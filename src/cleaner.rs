use serde_json::{Map, Value};
use std::io::{self};

pub fn remove_null_values(json_data: &mut Value) -> Option<Value> {
    let result = process_value(json_data);

    if result.is_ok() {
        Some(json_data.to_owned())
    } else {
        None
    }
}

fn process_value(json_data: &mut Value) -> Result<(), io::Error> {
    match json_data {
        Value::Object(object) => remove_null_values_from_object(object),
        Value::Array(array) => remove_null_values_from_array(array),
        _ => Ok(()),
    }
}

fn remove_null_values_from_object(object: &mut Map<String, Value>) -> Result<(), io::Error> {
    object.retain(|_key, value| !value.is_null());

    // Go a lever deeper w recursion
    for value in object.values_mut() {
        process_value(value)?;
    }

    Ok(())
}

fn remove_null_values_from_array(array: &mut Vec<Value>) -> Result<(), io::Error> {
    array.retain(|value| !value.is_null());

    // Go a lever deeper w recursion
    for value in array.iter_mut() {
        process_value(value)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cleaner::remove_null_values;
    use serde_json::json;

    #[test]
    fn test_remove_null_values() {
        // Create a test JSON object with null values
        let mut json_data = json!({
            "key1": "value1",
            "key2": null,
            "key3": {
                "nested_key": null,
                "another_key": "value2"
            },
            "key4": [null, "item1", null, "item2"],
        });

        remove_null_values(&mut json_data).unwrap();

        let expected = json!({
            "key1": "value1",
            "key3": {
                "another_key": "value2"
            },
            "key4": ["item1", "item2"],
        });

        assert_eq!(json_data, expected);
    }
}
