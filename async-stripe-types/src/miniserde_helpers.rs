#![allow(missing_debug_implementations)]
use std::mem;

use miniserde::Deserialize;
use miniserde::de::Visitor;
use miniserde::json::{Number, Object, Value};

#[derive(Default)]
pub struct ObjectBuilderInner {
    object: Object,
    key: Option<String>,
    value: Option<Value>,
}

impl ObjectBuilderInner {
    pub fn shift(&mut self) {
        if let (Some(k), Some(v)) = (self.key.take(), self.value.take()) {
            self.object.insert(k, v);
        }
    }

    pub fn key_inner(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
        self.shift();
        self.key = Some(k.to_owned());
        Ok(Deserialize::begin(&mut self.value))
    }

    pub fn finish_inner(&mut self) -> Option<(String, Object)> {
        self.shift();
        let object_key = self.object.get("object").and_then(|o| match o {
            Value::String(object_key) => Some(object_key.clone()),
            _ => None,
        })?;
        let final_object = mem::replace(&mut self.object, Object::new());
        Some((object_key, final_object))
    }
}

#[derive(Default)]
pub struct MaybeDeletedBuilderInner {
    object: Object,
    key: Option<String>,
    value: Option<Value>,
}

impl MaybeDeletedBuilderInner {
    pub fn shift(&mut self) {
        if let (Some(k), Some(v)) = (self.key.take(), self.value.take()) {
            self.object.insert(k, v);
        }
    }

    pub fn key_inner(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
        self.shift();
        self.key = Some(k.to_owned());
        Ok(Deserialize::begin(&mut self.value))
    }

    pub fn finish_inner(&mut self) -> Option<(bool, Object)> {
        self.shift();
        let deleted = self
            .object
            .get("deleted")
            .map(|o| match o {
                Value::Bool(bool) => *bool,
                _ => false,
            })
            .unwrap_or(false);
        let final_object = mem::replace(&mut self.object, Object::new());
        Some((deleted, final_object))
    }
}

/// Deserialize a parsed JSON value using the type's miniserde visitor.
///
/// All `miniserde::Deserialize` types implement this trait automatically.
pub trait FromValueOpt {
    fn from_value(v: Value) -> Option<Self>
    where
        Self: Sized;
}

/// Replay a parsed JSON value through miniserde's streaming deserializer.
///
/// This uses the same visitors as JSON text deserialization, including their
/// validation of optional fields and numeric ranges.
pub fn drive(value: &Value, visitor: &mut dyn Visitor) -> miniserde::Result<()> {
    match value {
        Value::Null => visitor.null(),
        Value::Bool(b) => visitor.boolean(*b),
        Value::Number(Number::U64(n)) => visitor.nonnegative(*n),
        Value::Number(Number::I64(n)) if *n >= 0 => visitor.nonnegative(*n as u64),
        Value::Number(Number::I64(n)) => visitor.negative(*n),
        Value::Number(Number::F64(n)) => visitor.float(*n),
        Value::String(s) => visitor.string(s),
        Value::Array(items) => {
            let mut seq = visitor.seq()?;
            for item in items {
                drive(item, seq.element()?)?;
            }
            seq.finish()
        }
        Value::Object(obj) => {
            let mut map = visitor.map()?;
            for (k, v) in obj {
                drive(v, map.key(k)?)?;
            }
            map.finish()
        }
    }
}

impl<T: Deserialize> FromValueOpt for T {
    fn from_value(v: Value) -> Option<Self> {
        let mut out = None;
        drive(&v, T::begin(&mut out)).ok()?;
        out
    }
}

pub fn extract_object_discr(value: Value) -> Option<(String, Object)> {
    let Value::Object(obj) = value else { return None };
    let obj_name = obj.get("object")?;
    let obj_str = match obj_name {
        Value::String(str) => str.to_string(),
        _ => return None,
    };
    Some((obj_str, obj))
}

/// Retained for compatibility; implement `miniserde::Deserialize` instead.
#[macro_export]
#[deprecated(note = "FromValueOpt is now implemented for all miniserde::Deserialize types")]
macro_rules! impl_from_val_with_from_str {
    ($struct_name:ident) => {};
}

#[cfg(test)]
#[allow(deprecated)] // Exercise the compatibility macro without warning at every test build.
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{AlwaysTrue, Currency, List};

    fn parse<T: FromValueOpt>(json: &str) -> Option<T> {
        T::from_value(miniserde::json::from_str(json).unwrap())
    }

    #[test]
    fn numeric_ranges_match_streaming() {
        assert_eq!(parse::<i64>("9223372036854775807"), Some(i64::MAX));
        assert_eq!(parse::<i64>("-9223372036854775808"), Some(i64::MIN));
        assert_eq!(parse::<i64>("18446744073709551615"), None);
        assert_eq!(parse::<u64>("18446744073709551615"), Some(u64::MAX));
        assert_eq!(parse::<u64>("-1"), None);
        assert_eq!(parse::<u32>("4294967296"), None);
        assert_eq!(parse::<u8>("256"), None);
        assert_eq!(parse::<f64>("42"), Some(42.0));
        assert_eq!(parse::<f64>("-42"), Some(-42.0));
        assert_eq!(parse::<f64>("1.5"), Some(1.5));
        assert_eq!(parse::<i64>("1.5"), None);
        // Values constructed by callers can contain nonnegative signed numbers.
        assert_eq!(u64::from_value(Value::Number(Number::I64(42))), Some(42));
        assert_eq!(u64::from_value(Value::Number(Number::I64(0))), Some(0));
    }

    #[test]
    fn containers_and_currency_use_existing_visitors() {
        assert_eq!(parse::<Option<bool>>("null"), Some(None));
        assert_eq!(parse::<Option<bool>>("true"), Some(Some(true)));
        assert_eq!(parse::<Option<bool>>(r#""invalid""#), None);
        assert_eq!(parse::<Vec<Box<u8>>>("[1,2]"), Some(vec![Box::new(1), Box::new(2)]));
        assert_eq!(parse::<Vec<Option<u8>>>("[1,null]"), Some(vec![Some(1), None]));
        assert_eq!(parse::<Vec<Option<u8>>>(r#"[1,"invalid"]"#), None);
        let map = parse::<HashMap<Currency, i64>>(r#"{"usd":12,"new_currency":-3}"#).unwrap();
        assert_eq!(map.get(&Currency::USD), Some(&12));
        assert_eq!(map.get(&"new_currency".parse().unwrap()), Some(&-3));
        assert_eq!(parse::<HashMap<String, Option<bool>>>(r#"{"x":123}"#), None);
        let json = r#"{"a":[null,true,-1,2,1.5,"x"],"b":{}}"#;
        let value: Value = miniserde::json::from_str(json).unwrap();
        assert_eq!(
            miniserde::json::to_string(&parse::<Value>(json).unwrap()),
            miniserde::json::to_string(&value)
        );
        assert!(parse::<AlwaysTrue>("true").is_some());
        assert!(parse::<AlwaysTrue>("false").is_none());
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct OptionalField {
        value: Option<u8>,
    }

    // An old external macro invocation remains valid alongside Deserialize.
    crate::impl_from_val_with_from_str!(OptionalField);

    #[test]
    fn blanket_impl_supports_derived_types_and_rejects_bad_optional_fields() {
        assert_eq!(parse::<OptionalField>("{}"), Some(OptionalField { value: None }));
        assert_eq!(
            parse::<OptionalField>(r#"{"value":null}"#),
            Some(OptionalField { value: None })
        );
        assert_eq!(
            parse::<OptionalField>(r#"{"value":3}"#),
            Some(OptionalField { value: Some(3) })
        );
        assert_eq!(parse::<OptionalField>(r#"{"value":"invalid"}"#), None);
        assert_eq!(
            parse::<OptionalField>(r#"{"ignored":[{},null]}"#),
            Some(OptionalField { value: None })
        );
        let list = parse::<List<OptionalField>>(
            r#"{"data":[{"value":3}],"has_more":false,"url":"/test"}"#,
        )
        .unwrap();
        assert_eq!(list.data, vec![OptionalField { value: Some(3) }]);
        assert!(
            parse::<List<OptionalField>>(
                r#"{"data":[{"value":"invalid"}],"has_more":false,"url":"/test"}"#,
            )
            .is_none()
        );
    }
}
