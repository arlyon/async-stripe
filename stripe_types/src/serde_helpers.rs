use miniserde::json::{Array, Number};
use miniserde::json::{Object, Value as MiniValue};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use serde_json::Value;

struct Wrapper<'a>(&'a miniserde::json::Value);

impl Serialize for Wrapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            MiniValue::Null => serializer.serialize_none(),
            MiniValue::Bool(bool) => serializer.serialize_bool(*bool),
            MiniValue::Number(num) => match num {
                Number::U64(uint) => serializer.serialize_u64(*uint),
                Number::I64(int) => serializer.serialize_i64(*int),
                Number::F64(float) => serializer.serialize_f64(*float),
            },
            MiniValue::String(str) => serializer.serialize_str(str),
            MiniValue::Array(items) => {
                let mut seq = serializer.serialize_seq(Some(items.len()))?;
                for item in items {
                    seq.serialize_element(&Wrapper(item))?;
                }
                seq.end()
            }
            MiniValue::Object(items) => {
                let mut seq = serializer.serialize_map(Some(items.len()))?;
                for (key, v) in items {
                    seq.serialize_entry(key, &Wrapper(v))?;
                }
                seq.end()
            }
        }
    }
}

pub mod with_serde_json {
    use miniserde::json::Value as MiniValue;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_json::Value;

    use crate::serde_helpers::{serde_json_to_mini, Wrapper};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<MiniValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        serde_json_to_mini(value).ok_or_else(|| {
            serde::de::Error::custom("could not convert serde_json::Value to miniserde::Value")
        })
    }

    pub fn serialize<S: Serializer>(val: &MiniValue, s: S) -> Result<S::Ok, S::Error> {
        Wrapper(val).serialize(s)
    }
}

pub mod with_serde_json_opt {
    use miniserde::json::Value as MiniValue;
    use serde::{Deserialize, Deserializer, Serializer};
    use serde_json::Value;

    use crate::serde_helpers::{serde_json_to_mini, Wrapper};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<MiniValue>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Option<Value> = Deserialize::deserialize(deserializer)?;
        match value {
            None => Ok(None),
            Some(s) => Ok(Some(serde_json_to_mini(s).ok_or_else(|| {
                serde::de::Error::custom("could not convert serde_json::Value to miniserde::Value")
            })?)),
        }
    }

    pub fn serialize<S: Serializer>(val: &Option<MiniValue>, s: S) -> Result<S::Ok, S::Error> {
        match val {
            None => s.serialize_none(),
            Some(v) => s.serialize_some(&Wrapper(v)),
        }
    }
}

fn serde_json_to_mini(val: Value) -> Option<MiniValue> {
    Some(match val {
        Value::Null => MiniValue::Null,
        Value::Bool(bool) => MiniValue::Bool(bool),
        Value::Number(num) => {
            if let Some(float) = num.as_f64() {
                MiniValue::Number(Number::F64(float))
            } else if let Some(uint) = num.as_u64() {
                MiniValue::Number(Number::U64(uint))
            } else if let Some(int) = num.as_i64() {
                MiniValue::Number(Number::I64(int))
            } else {
                return None;
            }
        }
        Value::String(str) => MiniValue::String(str),
        Value::Array(arr) => {
            let arr_conv = arr.into_iter().map(serde_json_to_mini).collect::<Option<Vec<_>>>()?;
            MiniValue::Array(Array::from_iter(arr_conv))
        }
        Value::Object(obj) => {
            let items = obj
                .into_iter()
                .map(|(key, val)| serde_json_to_mini(val).map(|v| (key, v)))
                .collect::<Option<Vec<(String, MiniValue)>>>()?;
            MiniValue::Object(Object::from_iter(items))
        }
    })
}
