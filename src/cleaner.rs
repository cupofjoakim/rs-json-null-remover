use serde_json::{Map, Value};
use std::io::{self};

pub fn remove_null_values(json_data: &mut Value) -> Result<(), io::Error> {
    match json_data {
        Value::Object(object) => remove_null_values_from_object(object),
        Value::Array(array) => remove_null_values_from_array(array),
        _ => Ok(()),
    }
}

fn remove_null_values_from_object(object: &mut Map<String, Value>) -> Result<(), io::Error> {
    // Collect keys to remove
    let keys_to_remove: Vec<String> = object
        .iter()
        .filter_map(|(key, value)| {
            if value.is_null() {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect();

    for key in keys_to_remove {
        log::debug!("Removing key {}", key);
        object.remove(&key);
    }

    // Go a lever deeper w recursion
    for value in object.values_mut() {
        remove_null_values(value)?;
    }

    Ok(())
}

fn remove_null_values_from_array(array: &mut Vec<Value>) -> Result<(), io::Error> {
    // Collect indices to remove
    let indices_to_remove: Vec<usize> = array
        .iter()
        .enumerate()
        .filter_map(|(index, value)| if value.is_null() { Some(index) } else { None })
        .collect();

    for index in indices_to_remove.iter().rev() {
        log::debug!("Removing item with index {}", index);
        array.remove(*index);
    }

    // Go a lever deeper w recursion
    for value in array.iter_mut() {
        remove_null_values(value)?;
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
