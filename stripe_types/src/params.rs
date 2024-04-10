use std::collections::HashMap;

use serde::{Serialize, Serializer};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AlwaysTrue;

impl serde::Serialize for AlwaysTrue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}

#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AlwaysTrue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bool_: bool = serde::Deserialize::deserialize(deserializer)?;
        if !bool_ {
            Err(serde::de::Error::custom("Expected value to always be `true`"))
        } else {
            Ok(AlwaysTrue)
        }
    }
}

#[doc(hidden)]
mod miniserde_deser {
    use miniserde::de::Visitor;
    use miniserde::json::Value;
    use miniserde::Deserialize;

    use crate::miniserde_helpers::FromValueOpt;
    use crate::{AlwaysTrue, Place};

    impl Deserialize for AlwaysTrue {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<AlwaysTrue> {
        fn boolean(&mut self, b: bool) -> miniserde::Result<()> {
            if b {
                self.out = Some(AlwaysTrue);
                Ok(())
            } else {
                Err(miniserde::Error)
            }
        }
    }

    impl FromValueOpt for AlwaysTrue {
        fn from_value(v: Value) -> Option<Self> {
            let b = bool::from_value(v)?;
            if b {
                Some(AlwaysTrue)
            } else {
                None
            }
        }
    }
}

pub type Metadata = HashMap<String, String>;
pub type Timestamp = i64;

#[derive(Copy, Clone, Debug, Serialize, Default)]
pub struct RangeBoundsTs {
    pub gt: Option<Timestamp>,
    pub gte: Option<Timestamp>,
    pub lt: Option<Timestamp>,
    pub lte: Option<Timestamp>,
}

/// A set of generic request parameters that can be used on
/// list endpoints to filter their results by some timestamp.
#[derive(Copy, Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum RangeQueryTs {
    Exact(Timestamp),
    Bounds(RangeBoundsTs),
}

impl RangeQueryTs {
    /// Filter results to exactly match a given value
    pub fn eq(value: Timestamp) -> Self {
        Self::Exact(value)
    }

    /// Filter results to be after a given value
    pub fn gt(value: Timestamp) -> Self {
        Self::Bounds(RangeBoundsTs { gt: Some(value), ..Default::default() })
    }

    /// Filter results to be after or equal to a given value
    pub fn gte(value: Timestamp) -> Self {
        Self::Bounds(RangeBoundsTs { gte: Some(value), ..Default::default() })
    }

    /// Filter results to be before to a given value
    pub fn lt(value: Timestamp) -> Self {
        Self::Bounds(RangeBoundsTs { lt: Some(value), ..Default::default() })
    }

    /// Filter results to be before or equal to a given value
    pub fn lte(value: Timestamp) -> Self {
        Self::Bounds(RangeBoundsTs { lte: Some(value), ..Default::default() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn urldecode(input: String) -> String {
        input.replace("%5B", "[").replace("%5D", "]")
    }

    // A smaller version of `ListCustomer`, used to test de/serialization of `RangeQueryTs`
    #[derive(Clone, Debug, Default, serde::Serialize)]
    struct TestListCustomer {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created: Option<RangeQueryTs>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ending_before: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub starting_after: Option<String>,
    }

    impl TestListCustomer {
        fn new() -> Self {
            Self::default()
        }
    }

    #[test]
    fn serialize_range_query() {
        let query = RangeQueryTs::Bounds(RangeBoundsTs {
            gt: None,
            gte: Some(1501598702),
            lt: Some(1504233902),
            lte: None,
        });
        assert_eq!(urldecode(serde_qs::to_string(&query).unwrap()), "gte=1501598702&lt=1504233902");

        let mut params = TestListCustomer::new();
        params.created = Some(RangeQueryTs::eq(1501598702));
        params.limit = Some(3);
        assert_eq!(urldecode(serde_qs::to_string(&params).unwrap()), "created=1501598702&limit=3");

        let mut params = TestListCustomer::new();
        params.created = Some(RangeQueryTs::gte(1501598702));
        params.limit = Some(3);
        assert_eq!(
            urldecode(serde_qs::to_string(&params).unwrap()),
            "created[gte]=1501598702&limit=3"
        );

        let mut params = TestListCustomer::new();
        params.created = Some(query);
        params.limit = Some(3);
        let encoded = urldecode(serde_qs::to_string(&params).unwrap());
        assert_eq!(encoded, "created[gte]=1501598702&created[lt]=1504233902&limit=3");
    }
}
