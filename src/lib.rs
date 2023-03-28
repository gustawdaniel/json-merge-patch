use serde_json::{Map, Value};

pub fn json_merge_patch(target: &mut Value, patch: &Value) {
    match patch {
        Value::Object(patch_obj) => {
            if !target.is_object() && !(target.is_array() && patch_obj.keys().all(|key| key.parse::<usize>().is_ok())) {
                *target = Value::Object(serde_json::Map::new());
            }

            if let Value::Object(target_obj) = target {
                for (key, value) in patch_obj {
                    if value.is_null() {
                        target_obj.remove(key);
                    } else {
                        let target_value = target_obj
                            .entry(key.clone())
                            .or_insert_with(|| Value::Null);
                        json_merge_patch(target_value, value);
                    }
                }
            } else if let Value::Array(target_arr) = target {
                for (key, value) in patch_obj {
                    if let Ok(index) = key.parse::<usize>() {
                        if value.is_null() && index < target_arr.len() {
                            target_arr.remove(index);
                        } else if index < target_arr.len() {
                            json_merge_patch(&mut target_arr[index], value);
                        } else {
                            // Handling the case where the index is greater than the current length of the target array
                            while target_arr.len() < index {
                                target_arr.push(Value::Null);
                            }
                            target_arr.push(value.clone());
                        }
                    }
                }
            }
        }
        Value::Array(patch_arr) => {
            *target = serde_json::Value::Array(patch_arr.clone().into_iter().filter(|value| !value.is_null()).collect());
        }
        _ => *target = patch.clone(),
    }
}

// pub fn json_merge_patch(target: &mut Value, patch: &Value) {
//     match patch {
//         Value::Object(patch_obj) => {
//             if !target.is_object() {
//                 *target = Value::Array(Vec::new());
//             }
//
//             if let Value::Array(target_arr) = target {
//                 let patch_keys: Vec<_> = patch_obj.keys().collect();
//                 for (index, target_value) in target_arr.iter_mut().enumerate() {
//                     if let Value::Object(target_obj) = target_value {
//                         let mut patch_obj = Map::new();
//                         for key in patch_keys.iter() {
//                             if let Some(value) = patch_obj.remove(&key) {
//                                 json_merge_patch(&mut target_obj[&key], &value);
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         _ => *target = patch.clone(),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_merge_patch_should_merge_objects_and_override_field() {
        let mut target = serde_json::from_str(r#"{"a": "b", "c": {"d": "e", "f": "g"}}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": "z", "c": {"f": null}}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        let expected: serde_json::Value = serde_json::from_str(r#"{"a": "z", "c": {"d": "e"}}"#).unwrap();
        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_override_field_in_object() {
        let mut target = serde_json::from_str(r#"{"a": "b"}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": "c"}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"{"a": "c"}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_add_field_to_object() {
        let mut target = serde_json::from_str(r#"{"a": "b"}"#).unwrap();
        let patch = serde_json::from_str(r#"{"b": "c"}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"{"a": "b", "b": "c"}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_remove_field_from_object() {
        let mut target = serde_json::from_str(r#"{"a": "b", "b": "c"}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": null}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"{"b": "c"}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_override_field_in_array() {
        let mut target = serde_json::from_str(r#"{"a": ["b"]}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": "c"}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"{"a": "c"}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_replace_array_with_scalar() {
        let mut target = serde_json::from_str(r#"{"a": "c"}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": ["b"]}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"{"a": ["b"]}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_merge_objects_in_object() {
        let mut target = serde_json::from_str(r#"{"a": {"b": "c"}}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": {"b": "d", "c": null}}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"{"a": {"b": "d"}}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_replace_array_with_value() {
        let mut target = serde_json::from_str(r#"{"a": [{"b": "c"}]}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": [1]}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"{"a": [1]}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_merge_nested_objects_and_remove_leaf_nodes() {
        let mut target = serde_json::from_str(r#"{}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": {"bb": {"ccc": null}}}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"{"a": {"bb": {}}}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_replace_scalar_with_scalar() {
        let mut target = serde_json::from_str(r#"{"a": "b"}"#).unwrap();
        let patch = serde_json::from_str(r#"["c"]"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"["c"]"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_replace_scalar_with_null() {
        let mut target = serde_json::from_str(r#"{"a": "foo"}"#).unwrap();
        let patch = serde_json::Value::Null;

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, serde_json::Value::Null);
    }

    #[test]
    fn test_json_merge_patch_should_replace_scalar_with_string() {
        let mut target = serde_json::from_str(r#"{"a": "foo"}"#).unwrap();
        let patch = serde_json::from_str(r#""bar""#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#""bar""#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_merge_null_with_scalar() {
        let mut target = serde_json::from_str(r#"{"e": null}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": 1}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"{"e": null, "a": 1}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_replace_array_with_object() {
        let mut target = serde_json::from_str(r#"{"a": []}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": {"b": "c"}}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"{"a": {"b": "c"}}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_merge_objects_in_array() {
        let mut target = serde_json::from_str(r#"[{"a": "b"}, {"c": "d"}]"#).unwrap();
        let patch = serde_json::from_str(r#"{"1": {"e": "f"}}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"[{"a": "b"}, {"c": "d", "e": "f"}]"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_replace_object_with_array() {
        let mut target = serde_json::from_str(r#"{"a": {"b": "c"}}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": []}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"{"a": []}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_merge_arrays() {
        let mut target = serde_json::from_str(r#"["a", "b"]"#).unwrap();
        let patch = serde_json::from_str(r#"["c", "d"]"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"["c", "d"]"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_remove_key_from_object() {
        let mut target = serde_json::from_str(r#"{"a": "b"}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": null}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"{}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_remove_index_from_array() {
        let mut target = serde_json::from_str(r#"["a", "b"]"#).unwrap();
        let patch = serde_json::from_str(r#"{"1": null}"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"["a"]"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }

    #[test]
    fn test_json_merge_patch_should_remove_array_element() {
        let mut target = serde_json::from_str(r#"[1, 2, 3]"#).unwrap();
        let patch = serde_json::from_str(r#"[null, 2]"#).unwrap();
        let expected: serde_json::Value = serde_json::from_str(r#"[2]"#).unwrap();

        json_merge_patch(&mut target, &patch);

        assert_eq!(target, expected);
    }
}

