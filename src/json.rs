/**
 *  RFC7396 JSON Merge Patch
 *
 *  define MergePatch(Target, Patch):
 *      if Patch is an Object:
 *          if Target is not an Object:
 *              Target = {} # Ignore the contents and set it to an empty Object
 *          for each Name/Value pair in Patch:
 *              if Value is null:
 *                  if Name exists in Target:
 *                      remove the Name/Value pair from Target
 *              else:
 *                  Target[Name] = MergePatch(Target[Name], Value)
 *          return Target
 *      else:
 *          return Patch
*/
pub fn patch_json(target: &serde_json::Value, patch: &serde_json::Value) -> serde_json::Value {
    match (target, patch) {
        (serde_json::Value::Object(target), serde_json::Value::Object(patch)) => {
            let mut result = target.clone();

            for (key, patch_value) in patch {
                match (result.get_mut(key), patch_value) {
                    (Some(_), serde_json::Value::Null) => {
                        result.remove(key);
                    }

                    (Some(target_value), serde_json::Value::Object(_)) => {
                        result[key] = patch_json(target_value, patch_value);
                    }

                    (_, _) => {
                        result.insert(key.clone(), patch_value.clone());
                    }
                }
            }

            result.into()
        }
        (_, patch) => patch.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::patch_json;

    #[test]
    fn test_patch_json() {
        let target = serde_json::json!({
            "a": 1,
            "b": 1,
            "c": {
                "d": 1,
                "e": 1,
            },
            "g": 1,
        });

        let patch = serde_json::json!({
            "a": 2,
            "c": {
                "d": 2,
                "f": 2,
            },
            "g": null,
        });

        let expected = serde_json::json!({
            "a": 2,
            "b": 1,
            "c": {
                "d": 2,
                "e": 1,
                "f": 2,
            },
        });

        assert_eq!(patch_json(&target, &patch), expected);
    }
}
