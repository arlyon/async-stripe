/// Specifies the end of billing period.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceBillingSchedulesBillUntil {
    /// The timestamp the billing schedule will apply until.
    pub computed_timestamp: stripe_types::Timestamp,
    /// Specifies the billing period.
    pub duration: Option<stripe_shared::SubscriptionsResourceBillingSchedulesBillUntilDuration>,
    /// If specified, the billing schedule will apply until the specified timestamp.
    pub timestamp: Option<stripe_types::Timestamp>,
    /// Describes how the billing schedule will determine the end date. Either `duration` or `timestamp`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: SubscriptionsResourceBillingSchedulesBillUntilType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceBillingSchedulesBillUntil {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourceBillingSchedulesBillUntil").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourceBillingSchedulesBillUntilBuilder {
    computed_timestamp: Option<stripe_types::Timestamp>,
    duration: Option<Option<stripe_shared::SubscriptionsResourceBillingSchedulesBillUntilDuration>>,
    timestamp: Option<Option<stripe_types::Timestamp>>,
    type_: Option<SubscriptionsResourceBillingSchedulesBillUntilType>,
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

    impl Deserialize for SubscriptionsResourceBillingSchedulesBillUntil {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourceBillingSchedulesBillUntil>,
        builder: SubscriptionsResourceBillingSchedulesBillUntilBuilder,
    }

    impl Visitor for Place<SubscriptionsResourceBillingSchedulesBillUntil> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourceBillingSchedulesBillUntilBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourceBillingSchedulesBillUntilBuilder {
        type Out = SubscriptionsResourceBillingSchedulesBillUntil;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "computed_timestamp" => Deserialize::begin(&mut self.computed_timestamp),
                "duration" => Deserialize::begin(&mut self.duration),
                "timestamp" => Deserialize::begin(&mut self.timestamp),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                computed_timestamp: None,
                duration: Some(None),
                timestamp: Some(None),
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(computed_timestamp), Some(duration), Some(timestamp), Some(type_)) =
                (self.computed_timestamp, self.duration.take(), self.timestamp, self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { computed_timestamp, duration, timestamp, type_ })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SubscriptionsResourceBillingSchedulesBillUntil {
        type Builder = SubscriptionsResourceBillingSchedulesBillUntilBuilder;
    }

    impl FromValueOpt for SubscriptionsResourceBillingSchedulesBillUntil {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourceBillingSchedulesBillUntilBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "computed_timestamp" => b.computed_timestamp = FromValueOpt::from_value(v),
                    "duration" => b.duration = FromValueOpt::from_value(v),
                    "timestamp" => b.timestamp = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Describes how the billing schedule will determine the end date. Either `duration` or `timestamp`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionsResourceBillingSchedulesBillUntilType {
    Duration,
    Timestamp,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionsResourceBillingSchedulesBillUntilType {
    pub fn as_str(&self) -> &str {
        use SubscriptionsResourceBillingSchedulesBillUntilType::*;
        match self {
            Duration => "duration",
            Timestamp => "timestamp",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionsResourceBillingSchedulesBillUntilType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourceBillingSchedulesBillUntilType::*;
        match s {
            "duration" => Ok(Duration),
            "timestamp" => Ok(Timestamp),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionsResourceBillingSchedulesBillUntilType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionsResourceBillingSchedulesBillUntilType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionsResourceBillingSchedulesBillUntilType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceBillingSchedulesBillUntilType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionsResourceBillingSchedulesBillUntilType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionsResourceBillingSchedulesBillUntilType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionsResourceBillingSchedulesBillUntilType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SubscriptionsResourceBillingSchedulesBillUntilType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionsResourceBillingSchedulesBillUntilType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SubscriptionsResourceBillingSchedulesBillUntilType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionsResourceBillingSchedulesBillUntilType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
