use serde_json::{Map, Value};

pub fn get_without_null_values(json_data: Value) -> Option<Value> {
    match json_data {
        Value::Object(object) => get_obj_without_null_values(object),
        Value::Array(array) => get_arr_without_null_values(array),
        Value::Null => None,
        val => Some(val.to_owned()),
    }
}

fn get_obj_without_null_values(object: Map<String, Value>) -> Option<Value> {
    // iterate through all entries in object
    // call "get without null values" on all

    let cleaned_obj: Map<String, Value> = object
        .iter()
        .filter_map(|(key, value)| {
            let v = get_without_null_values(value.clone());
            if v.is_some() {
                Some((key.clone(), v.unwrap()))
            } else {
                None
            }
        })
        .collect();

    Some(Value::Object(cleaned_obj))
}

fn get_arr_without_null_values(array: Vec<Value>) -> Option<Value> {
    let cleaned_arr: Vec<Value> = array
        .iter()
        .filter_map(|value| get_without_null_values(value.clone()))
        .collect();

    Some(Value::Array(cleaned_arr))
}

#[cfg(test)]
mod tests {
    use crate::assembler::get_without_null_values;
    use serde_json::json;

    #[test]
    fn test_get_without_null_values() {
        // Create a test JSON object with null values
        let json_data = json!({
            "key1": "value1",
            "key2": null,
            "key3": {
                "nested_key": null,
                "another_key": "value2"
            },
            "key4": [{"some_key": null, "good_key": "yes i am"}, null, "item1", null, "item2"],
        });

        let res = get_without_null_values(json_data).unwrap();

        let expected = json!({
            "key1": "value1",
            "key3": {
                "another_key": "value2"
            },
            "key4": [{"good_key": "yes i am"}, "item1", "item2"],
        });

        assert_eq!(res, expected);
    }
}
