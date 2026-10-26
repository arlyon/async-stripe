use wasm_bindgen::prelude::*;

/// Returns a list of all types that can be parsed.
#[wasm_bindgen]
pub fn list_types() -> Vec<String> {
    vec!["Event".to_string()]
}

/// Generic function to parse and validate a payload of type T using serde.
/// Returns Ok(()) if the payload is valid, or an error message with path information if parsing fails.
fn parse_type_serde<T: serde::de::DeserializeOwned>(payload: &str) -> Result<(), String> {
    let mut deserializer = serde_json::Deserializer::from_str(payload);
    let _: T = serde_path_to_error::deserialize(&mut deserializer).map_err(|e| format!("{e}"))?;
    deserializer.end().map_err(|e| e.to_string())
}

/// Parses and validates a payload based on the type name.
/// Returns Ok(()) if the payload is valid, or an error message with path information if parsing fails.
///
/// # Arguments
/// * `type_name` - The type to parse (e.g., "Event")
/// * `payload` - The JSON payload to parse
#[wasm_bindgen]
pub fn parse(type_name: &str, payload: &str) -> Result<(), String> {
    match type_name {
        "Event" => parse_type_serde::<stripe_webhook::Event>(payload),
        _ => Err(format!("Unknown type: {type_name}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_types() {
        let types = list_types();
        assert_eq!(types.len(), 1);
        assert!(types.contains(&"Event".to_string()));
    }

    fn valid_event_payload() -> &'static str {
        // Based on test_webhook_construct_event from webhook.rs
        r#"{
          "id": "evt_1SYARgFuzxtsmcCa56A419qD",
          "object": "event",
          "api_version": "2024-06-20",
          "created": 1764270096,
          "data": {
            "object": {
              "object": "entitlements.active_entitlement_summary",
              "customer": "cus_TVAmqNlyaYgEBn",
              "entitlements": {
                "object": "list",
                "data": [
                  {
                    "id": "ent_61ThYP2iCNV4w0FC341FuzxtsmcCaIqu",
                    "object": "entitlements.active_entitlement",
                    "feature": "feat_61RwMxrXpU9nzwARs41FuzxtsmcCa6hs",
                    "livemode": true,
                    "lookup_key": "ai-insights"
                  }
                ],
                "has_more": false,
                "url": "/v1/customer/cus_TVAmqNlyaYgEBn/entitlements"
              },
              "livemode": true
            },
            "previous_attributes": {
              "entitlements": {
                "data": []
              }
            }
          },
          "livemode": true,
          "pending_webhooks": 2,
          "request": {
            "id": null,
            "idempotency_key": null
          },
          "type": "entitlements.active_entitlement_summary.updated"
        }"#
    }

    #[test]
    fn test_parse_event_valid() {
        let result = parse("Event", valid_event_payload());
        if let Err(e) = &result {
            eprintln!("Parse error: {}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_event_rejects_trailing_garbage() {
        for suffix in [" NOT_JSON", "x", "\n// comment"] {
            let payload = format!("{}{suffix}", valid_event_payload());
            assert!(parse("Event", &payload).is_err(), "accepted suffix {suffix:?}");
        }
    }

    #[test]
    fn test_parse_event_rejects_multiple_values() {
        for suffix in [" {}", " null", " []", r#" "extra""#] {
            let payload = format!("{}{suffix}", valid_event_payload());
            assert!(parse("Event", &payload).is_err(), "accepted suffix {suffix:?}");
        }
    }

    #[test]
    fn test_parse_event_accepts_trailing_whitespace() {
        for suffix in ["", " ", "\r\n\t"] {
            let payload = format!("{}{suffix}", valid_event_payload());
            assert!(parse("Event", &payload).is_ok(), "rejected whitespace {suffix:?}");
        }
    }

    #[test]
    fn test_parse_unknown_type() {
        let payload = r#"{}"#;
        let result = parse("UnknownType", payload);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown type"));
    }

    #[test]
    fn test_parse_invalid_json() {
        let payload = r#"{"invalid": json}"#;
        let result = parse("Event", payload);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_field() {
        let payload = r#"{
            "id": "evt_test",
            "object": "event",
            "created": 1492774577,
            "livemode": false,
            "pending_webhooks": 1,
            "data": {
                "object": {}
            }
        }"#;
        let result = parse("Event", payload);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_unknown_event_type() {
        let payload = r#"{
          "id": "evt_1SYARgFuzxtsmcCa56A419qD",
          "object": "event",
          "api_version": "2024-06-20",
          "created": 1764270096,
          "data": {
            "object": {
              "object": "entitlements.active_entitlement_summary",
              "entitlements": {
                "object": "list",
                "data": [
                  {
                    "id": "ent_61ThYP2iCNV4w0FC341FuzxtsmcCaIqu",
                    "object": "entitlements.active_entitlement",
                    "livemode": true,
                    "lookup_key": "ai-insights"
                  }
                ],
                "has_more": false,
                "url": "/v1/customer/cus_TVAmqNlyaYgEBn/entitlements"
              },
              "livemode": true
            },
            "previous_attributes": {
              "entitlements": {
                "data": []
              }
            }
          },
          "livemode": true,
          "pending_webhooks": 2,
          "request": {
            "id": null,
            "idempotency_key": null
          },
          "type": "entitlements.active_entitlement_summary.upated"
        }"#;

        let result = parse("Event", payload);
        insta::assert_snapshot!(result.unwrap_err());
    }

    #[test]
    fn test_parse_invalid_event() {
        let payload = r#"{
          "id": "evt_1SYARgFuzxtsmcCa56A419qD",
          "object": "event",
          "api_version": "2024-06-20",
          "created": 1764270096,
          "data": {
            "object": {
              "object": "entitlements.active_entitlement_summary",
              "entitlements": {
                "object": "list",
                "data": [
                  {
                    "id": "ent_61ThYP2iCNV4w0FC341FuzxtsmcCaIqu",
                    "object": "entitlements.active_entitlement",
                    "livemode": true,
                    "lookup_key": "ai-insights"
                  }
                ],
                "has_more": false,
                "url": "/v1/customer/cus_TVAmqNlyaYgEBn/entitlements"
              },
              "livemode": true
            },
            "previous_attributes": {
              "entitlements": {
                "data": []
              }
            }
          },
          "livemode": true,
          "pending_webhooks": 2,
          "request": {
            "id": null,
            "idempotency_key": null
          },
          "type": "entitlements.active_entitlement_summary.updated"
        }"#;

        let result = parse("Event", payload);
        insta::assert_snapshot!(result.unwrap_err());
    }
    #[test]
    fn test_roundtrip() {
        let payload = r#"{
          "id": "evt_1SYARgFuzxtsmcCa56A419qD",
          "object": "event",
          "api_version": "2024-06-20",
          "created": 1764270096,
          "data": {
            "object": {
              "object": "entitlements.active_entitlement_summary",
              "customer": "cus_TVAmqNlyaYgEBn",
              "entitlements": {
                "object": "list",
                "data": [
                  {
                    "id": "ent_61ThYP2iCNV4w0FC341FuzxtsmcCaIqu",
                    "object": "entitlements.active_entitlement",
                    "feature": "feat_61RwMxrXpU9nzwARs41FuzxtsmcCa6hs",
                    "livemode": true,
                    "lookup_key": "ai-insights"
                  }
                ],
                "has_more": false,
                "url": "/v1/customer/cus_TVAmqNlyaYgEBn/entitlements"
              },
              "livemode": true
            },
            "previous_attributes": {
              "entitlements": {
                "data": []
              }
            }
          },
          "livemode": true,
          "pending_webhooks": 2,
          "request": {
            "id": null,
            "idempotency_key": null
          },
          "type": "entitlements.active_entitlement_summary.updated"
        }"#;

        let event: stripe_webhook::Event =
            serde_json::from_str(payload).expect("Failed to deserialize");
        let serialized = serde_json::to_string(&event).expect("Failed to serialize");
        println!("{}", serialized);
        let event_2: stripe_webhook::Event =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        let original_value: serde_json::Value = serde_json::from_str(payload).unwrap();
        let serialized_value: serde_json::Value = serde_json::from_str(&serialized).unwrap();

        println!("{:#?}\n\n{:#?}", event, event_2);

        assert_eq!(original_value, serialized_value);
    }
}
