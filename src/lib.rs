use serde_json::{Map, Value};

pub fn json_merge_patch(target: &mut Value, patch: &Value) {
    match patch {
        Value::Object(patch_obj) => {
            if !target.is_object() {
                *target = Value::Object(Map::new());
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
            }
        }
        _ => *target = patch.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_merge_patch() {
        let mut target = serde_json::from_str(r#"{"a": "b", "c": {"d": "e", "f": "g"}}"#).unwrap();
        let patch = serde_json::from_str(r#"{"a": "z", "c": {"f": null}}"#).unwrap();

        json_merge_patch(&mut target, &patch);

        let expected: serde_json::Value = serde_json::from_str(r#"{"a": "z", "c": {"d": "e"}}"#).unwrap();
        assert_eq!(target, expected);
    }
}