use std::fmt::{Display, Formatter, Write};
use std::fs::File;

use serde_json::Value;

#[test]
fn fixtures() {
    let data: Value =
        serde_json::from_reader(File::open("../openapi/fixtures.json").unwrap()).unwrap();
    let resources = &data["resources"];
    super::generated::check_fixtures(resources);
}

const SKIP_PATHS: &[&str] = &[
    // Fixture data includes "id": "" and we don't deserialize the empty string to a valid id
    "deleted_discount",
    // https://github.com/stripe/openapi/issues/68
    "product",
    // We expect the "plan" field to be required when deserializing the `SubscriptionItem` at the path
    // `subscription.items.data`, but the field is not included in the provided fixture. See
    // https://github.com/stripe/openapi/issues/138 and linked issue for some context.
    "subscription",
    "subscription_item",
    "subscription_schedule",
    // We expect a "billing_details" field at the path `treasury.outbound_payment.destination_payment_method_details`,
    // but it is not present
    "treasury.outbound_payment",
];

pub fn check_object<T: miniserde::Deserialize + serde::Serialize>(resources: &Value, key: &str) {
    if SKIP_PATHS.contains(&key) {
        return;
    }

    let Some(fixture) = resources.as_object().unwrap().get(key) else {
        println!("skipping component {key} since it was not found in the fixture data");
        return;
    };
    let as_str = serde_json::to_string(fixture).unwrap();
    let result: T = miniserde::json::from_str(&as_str).unwrap_or_else(|_| {
        panic!("could not deserialize fixture for {key}, data was {as_str}");
    });

    let result_str = serde_json::to_string(&result).unwrap();
    let as_val: Value = serde_json::from_str(&result_str).unwrap();
    assert_json_matches(&as_val, fixture, &mut JsonKeyPath::new(key));
}

struct JsonKeyPath {
    keys: Vec<String>,
}

impl JsonKeyPath {
    pub fn new(base: impl ToString) -> Self {
        Self { keys: vec![base.to_string()] }
    }

    pub fn add_key(&mut self, new_key: impl ToString) {
        self.keys.push(new_key.to_string());
    }

    pub fn pop_key(&mut self) {
        self.keys.pop();
    }
}

impl Display for JsonKeyPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, key) in self.keys.iter().enumerate() {
            if i != 0 {
                f.write_char('.')?;
            }
            write!(f, "{key}")?;
        }
        Ok(())
    }
}

fn assert_json_matches(result: &Value, expected: &Value, key_path: &mut JsonKeyPath) {
    let result = match result {
        Value::Array(arr) => {
            let Some(expected_arr) = expected.as_array() else {
                panic!("Key {key_path}, result was array, but expected was {expected:?}")
            };
            if arr.len() != expected_arr.len() {
                panic!("Key {key_path}, mismatched array lengths, result was {result:?}, but expected was {expected:?}")
            }
            for (i, item) in arr.iter().enumerate() {
                key_path.add_key(i);
                assert_json_matches(item, &expected_arr[i], key_path);

                key_path.pop_key();
            }
            return;
        }
        Value::Object(obj) => obj,
        Value::Number(number) => {
            let Some(expected_num) = expected.as_number() else {
                panic!("Key {key_path}, result was number, but expected was {expected:?}")
            };
            // NB: we compare floats here because the raw types will not be equivalent -> for example
            // `19` in raw JSON can be deserialized into an `f64` field, which will then print as `19.0`.
            assert_eq!(number.as_f64().unwrap(), expected_num.as_f64().unwrap());
            return;
        }
        _ => {
            return assert_eq!(
                result, expected,
                "Key {key_path}, result was {result:?}, expected was {expected:?}"
            );
        }
    };

    let Some(expected) = expected.as_object() else {
        panic!("Key {key_path}, result was object, but expected was {expected:?}")
    };

    for (key, val) in result {
        key_path.add_key(key);
        match expected.get(key) {
            None => {
                // `expected` should only be missing a key result has if it was just not provided in the input json
                assert!(
                    val.is_null(),
                    "Key {key_path}, result was {val:?}, expected did not include this key"
                )
            }
            Some(expected_val) => {
                assert_json_matches(val, expected_val, key_path);
            }
        }
        key_path.pop_key();
    }
    for (key, val) in expected {
        key_path.add_key(key);
        match result.get(key) {
            None => {
                // This is a possible case if the fixture contains keys that have since been removed
                // from the OpenAPI schema
                println!("Key {key_path}, expected had value {val:?}, result did not have key.",);
            }
            Some(result_val) => {
                assert_json_matches(result_val, val, key_path);
            }
        }
        key_path.pop_key();
    }
}
