use miniserde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use stripe_types::miniserde_helpers::FromValueOpt;

/// Canonical outputs make comparisons independent of HashMap iteration order.
pub fn outputs<T: Deserialize + Serialize + FromValueOpt>(
    input: &Value,
) -> (Option<Value>, Option<Value>) {
    let json = serde_json::to_string(input).unwrap();
    let streaming = miniserde::json::from_str::<T>(&json).ok();
    let value = T::from_value(miniserde::json::from_str(&json).unwrap());
    (streaming.map(canonical), value.map(canonical))
}

fn canonical<T: Serialize>(value: T) -> Value {
    serde_json::to_value(value).unwrap()
}

/// Corrupt every non-null node through depth four, visiting at most two array elements.
pub fn mutations(input: &Value) -> Vec<(String, Value)> {
    fn paths(value: &Value, path: String, depth: usize, out: &mut Vec<String>) {
        if value.is_null() || depth > 4 {
            return;
        }
        out.push(path.clone());
        match value {
            Value::Object(object) => {
                for (key, value) in object {
                    let key = key.replace('~', "~0").replace('/', "~1");
                    paths(value, format!("{path}/{key}"), depth + 1, out);
                }
            }
            Value::Array(array) => {
                for (index, value) in array.iter().take(2).enumerate() {
                    paths(value, format!("{path}/{index}"), depth + 1, out);
                }
            }
            _ => {}
        }
    }

    let mut all_paths = Vec::new();
    paths(input, String::new(), 0, &mut all_paths);
    all_paths
        .into_iter()
        .map(|path| {
            let mut corrupted = input.clone();
            let node = corrupted.pointer_mut(&path).unwrap();
            *node = if node.is_string() { Value::from(12345) } else { Value::from("corrupted") };
            (path, corrupted)
        })
        .collect()
}
