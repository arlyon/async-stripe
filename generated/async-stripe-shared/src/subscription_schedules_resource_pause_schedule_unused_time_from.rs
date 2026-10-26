#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom {
    /// The type of unused time credit behavior.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromBuilder {
    type_: Option<SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom>,
        builder: SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromBuilder {
        type Out = SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(type_),) = (self.type_.take(),) else {
                return None;
            };
            Some(Self::Out { type_ })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom {
        type Builder = SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromBuilder;
    }

    impl FromValueOpt for SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of unused time credit behavior.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType {
    ItemCurrentPeriodStart,
    None,
    PauseAt,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType {
    pub fn as_str(&self) -> &str {
        use SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType::*;
        match self {
            ItemCurrentPeriodStart => "item_current_period_start",
            None => "none",
            PauseAt => "pause_at",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType::*;
        match s {
            "item_current_period_start" => Ok(ItemCurrentPeriodStart),
            "none" => Ok(None),
            "pause_at" => Ok(PauseAt),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
