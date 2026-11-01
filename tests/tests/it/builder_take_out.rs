use miniserde::json::Value as MiniValue;
use serde_json::{Value, json};
use stripe_core::{Customer, File, FilePurpose};
use stripe_types::miniserde_helpers::FromValueOpt;
use stripe_types::{MapBuilder, ObjectDeser};

fn file_input() -> Value {
    json!({
        "created": 123,
        "filename": "retained.txt",
        "id": "file_test",
        "purpose": "account_requirement",
        "size": 42,
    })
}

fn file_from_value(input: &Value) -> Option<File> {
    let value: MiniValue = miniserde::json::from_str(&input.to_string()).unwrap();
    File::from_value(value)
}

#[test]
fn complete_builder_preserves_owned_and_copy_fields() {
    let input = file_input();
    let streamed: File = miniserde::json::from_str(&input.to_string()).unwrap();
    let from_value = file_from_value(&input).unwrap();
    for file in [streamed, from_value] {
        assert_eq!(file.created, 123);
        assert_eq!(file.id.as_str(), "file_test");
        assert_eq!(file.purpose, FilePurpose::AccountRequirement);
        assert_eq!(file.size, 42);
        assert_eq!(file.filename.as_deref(), Some("retained.txt"));
        assert!(file.expires_at.is_none());
        assert!(file.links.is_none());
    }
}

#[test]
fn missing_or_invalid_required_fields_still_fail() {
    for field in ["created", "id", "purpose", "size"] {
        let mut missing = file_input();
        missing.as_object_mut().unwrap().remove(field);
        assert!(miniserde::json::from_str::<File>(&missing.to_string()).is_err(), "{field}");
        assert!(file_from_value(&missing).is_none(), "{field}");

        for invalid in [Value::Null, Value::Bool(false)] {
            let mut input = file_input();
            input[field] = invalid;
            assert!(miniserde::json::from_str::<File>(&input.to_string()).is_err(), "{field}");
            assert!(file_from_value(&input).is_none(), "{field}");
        }
    }
}

#[test]
fn optional_fields_preserve_missing_null_and_present_values() {
    for address in [
        None,
        Some(Value::Null),
        Some(json!({})),
        Some(json!({"city": null})),
        Some(json!({"city": "New York"})),
    ] {
        let mut input = json!({"id": "cus_test", "created": 123, "livemode": false});
        if let Some(address) = address {
            input["address"] = address;
        }
        let streamed: Customer = miniserde::json::from_str(&input.to_string()).unwrap();
        let value: MiniValue = miniserde::json::from_str(&input.to_string()).unwrap();
        let from_value = Customer::from_value(value).unwrap();
        assert_eq!(streamed.address, from_value.address);
        match streamed.address {
            Some(address) => {
                assert_eq!(
                    address.city.as_deref(),
                    input["address"].get("city").and_then(Value::as_str)
                );
                assert!(address.line1.is_none());
            }
            None => assert!(input["address"].is_null()),
        }
    }
}

#[test]
fn failed_take_out_retains_fields_for_retry() {
    let mut builder = <File as ObjectDeser>::Builder::deser_default();
    builder.key("created").unwrap().nonnegative(123).unwrap();
    builder.key("id").unwrap().string("file_test").unwrap();
    builder.key("purpose").unwrap().string("account_requirement").unwrap();
    builder.key("filename").unwrap().string("retained.txt").unwrap();
    assert!(builder.take_out().is_none());
    assert!(builder.take_out().is_none());

    builder.key("size").unwrap().nonnegative(42).unwrap();
    let file = builder.take_out().unwrap();
    assert_eq!(file.created, 123);
    assert_eq!(file.id.as_str(), "file_test");
    assert_eq!(file.size, 42);
    assert_eq!(file.filename.as_deref(), Some("retained.txt"));
    assert!(builder.take_out().is_none());
}
