#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalResourceScheduleUpdateAtPeriodEndCondition {
    /// The type of condition.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PortalResourceScheduleUpdateAtPeriodEndConditionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalResourceScheduleUpdateAtPeriodEndCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalResourceScheduleUpdateAtPeriodEndCondition").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalResourceScheduleUpdateAtPeriodEndConditionBuilder {
    type_: Option<PortalResourceScheduleUpdateAtPeriodEndConditionType>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for PortalResourceScheduleUpdateAtPeriodEndCondition {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalResourceScheduleUpdateAtPeriodEndCondition>,
        builder: PortalResourceScheduleUpdateAtPeriodEndConditionBuilder,
    }

    impl Visitor for Place<PortalResourceScheduleUpdateAtPeriodEndCondition> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalResourceScheduleUpdateAtPeriodEndConditionBuilder {
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(type_),) = (self.builder.type_.take(),) else {
                return Ok(());
            };
            *self.out = Some(PortalResourceScheduleUpdateAtPeriodEndCondition { type_ });
            Ok(())
        }
    }
};
/// The type of condition.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalResourceScheduleUpdateAtPeriodEndConditionType {
    DecreasingItemAmount,
    ShorteningInterval,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalResourceScheduleUpdateAtPeriodEndConditionType {
    pub fn as_str(&self) -> &str {
        use PortalResourceScheduleUpdateAtPeriodEndConditionType::*;
        match self {
            DecreasingItemAmount => "decreasing_item_amount",
            ShorteningInterval => "shortening_interval",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalResourceScheduleUpdateAtPeriodEndConditionType::*;
        match s {
            "decreasing_item_amount" => Ok(DecreasingItemAmount),
            "shortening_interval" => Ok(ShorteningInterval),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PortalResourceScheduleUpdateAtPeriodEndConditionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PortalResourceScheduleUpdateAtPeriodEndConditionType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PortalResourceScheduleUpdateAtPeriodEndConditionType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PortalResourceScheduleUpdateAtPeriodEndConditionType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
