#![allow(missing_debug_implementations)]
use std::collections::HashMap;
use std::mem;
use std::str::FromStr;

use miniserde::Deserialize;
use miniserde::de::Visitor;
use miniserde::json::{Number, Object, Value};

use crate::Currency;

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

pub trait FromValueOpt {
    fn from_value(v: Value) -> Option<Self>
    where
        Self: Sized;
}

impl FromValueOpt for bool {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }
}

impl FromValueOpt for i64 {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Number(num) => match num {
                Number::U64(u) => u.try_into().ok(),
                Number::I64(i) => Some(i),
                Number::F64(_) => None,
            },
            _ => None,
        }
    }
}

impl FromValueOpt for u64 {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Number(num) => match num {
                Number::U64(u) => Some(u),
                Number::I64(i) => Some(i.try_into().ok()?),
                Number::F64(_) => None,
            },
            _ => None,
        }
    }
}

impl FromValueOpt for f64 {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Number(num) => match num {
                Number::U64(u) => Some(u as f64),
                Number::I64(i) => Some(i as f64),
                Number::F64(f) => Some(f),
            },
            _ => None,
        }
    }
}
impl FromValueOpt for u32 {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Number(num) => match num {
                Number::U64(u) => Some(u.try_into().ok()?),
                Number::I64(i) => Some(i.try_into().ok()?),
                Number::F64(_) => None,
            },
            _ => None,
        }
    }
}
impl FromValueOpt for u8 {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Number(num) => match num {
                Number::U64(u) => Some(u.try_into().ok()?),
                Number::I64(i) => Some(i.try_into().ok()?),
                Number::F64(_) => None,
            },
            _ => None,
        }
    }
}

impl FromValueOpt for String {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::String(str) => Some(str),
            _ => None,
        }
    }
}

impl<T: FromValueOpt> FromValueOpt for Option<T> {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Null => Some(None),
            val => T::from_value(val).map(Some),
        }
    }
}

impl<T: FromValueOpt> FromValueOpt for Vec<T> {
    fn from_value(v: Value) -> Option<Self> {
        match v {
            Value::Array(items) => items.into_iter().map(|i| T::from_value(i)).collect(),
            _ => None,
        }
    }
}

impl<V: FromValueOpt> FromValueOpt for HashMap<String, V> {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        obj.into_iter()
            .map(|(k, v)| {
                let v = V::from_value(v)?;
                Some((k, v))
            })
            .collect::<Option<HashMap<_, _>>>()
    }
}

impl<V: FromValueOpt> FromValueOpt for HashMap<Currency, V> {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        obj.into_iter()
            .map(|(k, v)| {
                let k = Currency::from_str(&k).ok()?;
                let v = V::from_value(v)?;
                Some((k, v))
            })
            .collect::<Option<HashMap<_, _>>>()
    }
}

/// Replay an already parsed `Value` into a streaming `Visitor`.
///
/// Lets types build `FromValueOpt` on top of their `miniserde::Deserialize` impl instead of a
/// second, generated field-by-field parser. Not generic, so it is compiled once. Fails for arrays
/// and objects nested deeper than `MAX_DEPTH`, so the recursion cannot overflow the stack.
pub fn visit_value(value: Value, visitor: &mut dyn Visitor) -> miniserde::Result<()> {
    visit_value_at(value, visitor, 0)
}

fn visit_value_at(value: Value, visitor: &mut dyn Visitor, depth: usize) -> miniserde::Result<()> {
    if matches!(value, Value::Array(_) | Value::Object(_)) && depth >= MAX_DEPTH {
        return Err(miniserde::Error);
    }
    match value {
        Value::Null => visitor.null(),
        Value::Bool(b) => visitor.boolean(b),
        Value::Number(Number::U64(n)) => visitor.nonnegative(n),
        Value::Number(Number::I64(n)) => match u64::try_from(n) {
            Ok(n) => visitor.nonnegative(n),
            Err(_) => visitor.negative(n),
        },
        Value::Number(Number::F64(n)) => visitor.float(n),
        Value::String(s) => visitor.string(&s),
        Value::Array(items) => {
            let mut seq = visitor.seq()?;
            for item in items {
                visit_value_at(item, seq.element()?, depth + 1)?;
            }
            seq.finish()
        }
        Value::Object(obj) => {
            let mut map = visitor.map()?;
            for (k, v) in obj {
                visit_value_at(v, map.key(&k)?, depth + 1)?;
            }
            map.finish()
        }
    }
}

/// `FromValueOpt::from_value` for any type that implements `miniserde::Deserialize`.
pub fn from_value_via_deserialize<T: Deserialize>(value: Value) -> Option<T> {
    let mut out = None;
    match visit_value(value, T::begin(&mut out)) {
        Ok(()) => out,
        Err(_) => None,
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

impl FromValueOpt for Value {
    fn from_value(v: Value) -> Option<Self> {
        Some(v)
    }
}

impl<T: FromValueOpt> FromValueOpt for Box<T> {
    fn from_value(v: Value) -> Option<Self> {
        Some(Box::new(T::from_value(v)?))
    }
}

#[macro_export]
macro_rules! impl_from_val_with_from_str {
    ($struct_name:ident) => {
        impl $crate::miniserde_helpers::FromValueOpt for $struct_name {
            fn from_value(v: miniserde::json::Value) -> Option<Self> {
                match v {
                    miniserde::json::Value::String(str) => std::str::FromStr::from_str(&str).ok(),
                    _ => None,
                }
            }
        }
    };
}

impl_from_val_with_from_str!(Currency);

/// Nesting limit of `visit_value`, the same as the default recursion limit of serde_json.
const MAX_DEPTH: usize = 128;

#[cfg(test)]
mod tests {
    use miniserde::json;

    use super::*;

    #[test]
    fn optional_values_reject_invalid_non_null_input() {
        assert_eq!(Option::<String>::from_value(Value::Bool(true)), None);
        let invalid: Value = miniserde::json::from_str(r#"["valid", 123]"#).unwrap();
        assert_eq!(Option::<Vec<String>>::from_value(invalid), None);
    }

    #[test]
    fn optional_values_preserve_null_and_valid_input() {
        assert_eq!(Option::<String>::from_value(Value::Null), Some(None));
        assert_eq!(
            Option::<String>::from_value(Value::String("valid".into())),
            Some(Some("valid".into()))
        );
        let valid: Value = miniserde::json::from_str(r#"["first", "second"]"#).unwrap();
        assert_eq!(
            Option::<Vec<String>>::from_value(valid),
            Some(Some(vec!["first".into(), "second".into()]))
        );
    }

    #[test]
    fn signed_integers_reject_unsigned_overflow() {
        for value in [i64::MAX as u64 + 1, u64::MAX] {
            assert_eq!(i64::from_value(Value::Number(Number::U64(value))), None);
        }
        for value in [0, i64::MAX as u64] {
            assert_eq!(i64::from_value(Value::Number(Number::U64(value))), Some(value as i64));
        }
        assert_eq!(i64::from_value(Value::Number(Number::I64(i64::MIN))), Some(i64::MIN));
    }

    type Nested = HashMap<String, Vec<Option<i64>>>;

    #[test]
    fn from_value_matches_streaming_deserialize() {
        let raw = r#"{"a": [1, -2, null], "b": [], "c": [9223372036854775807]}"#;
        let streamed: Nested = json::from_str(raw).unwrap();
        let replayed: Nested = from_value_via_deserialize(json::from_str(raw).unwrap()).unwrap();
        assert_eq!(replayed, streamed);
    }

    #[test]
    fn from_value_rejects_deep_nesting() {
        fn nested(depth: usize) -> Value {
            let mut value = Value::Null;
            for _ in 0..depth {
                value = Value::Array(std::iter::once(value).collect());
            }
            value
        }
        assert!(from_value_via_deserialize::<Value>(nested(MAX_DEPTH)).is_some());
        assert!(from_value_via_deserialize::<Value>(nested(MAX_DEPTH + 1)).is_none());
        assert!(from_value_via_deserialize::<Value>(nested(100_000)).is_none());
    }

    #[test]
    fn from_value_accepts_nonnegative_i64_for_unsigned() {
        let replayed: Option<u64> = from_value_via_deserialize(Value::Number(Number::I64(5)));
        assert_eq!(replayed, Some(5));
    }

    #[test]
    fn from_value_rejects_mismatched_types() {
        let wrong_scalar: Option<i64> = from_value_via_deserialize(Value::String("1".into()));
        assert_eq!(wrong_scalar, None);

        let wrong_element: Option<Nested> =
            from_value_via_deserialize(json::from_str(r#"{"a": ["x"]}"#).unwrap());
        assert_eq!(wrong_element, None);

        let negative_for_unsigned: Option<u64> =
            from_value_via_deserialize(Value::Number(Number::I64(-1)));
        assert_eq!(negative_for_unsigned, None);
    }
}
