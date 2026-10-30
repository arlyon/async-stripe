#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalSubscriptionPauseDuration {
    /// The preset pause durations customers can choose from.
    pub interval_options: Vec<stripe_billing::PortalSubscriptionPauseIntervalOption>,
    /// Whether subscriptions can be paused indefinitely or for a preset duration.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PortalSubscriptionPauseDurationType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionPauseDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalSubscriptionPauseDuration").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalSubscriptionPauseDurationBuilder {
    interval_options: Option<Vec<stripe_billing::PortalSubscriptionPauseIntervalOption>>,
    type_: Option<PortalSubscriptionPauseDurationType>,
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

    impl Deserialize for PortalSubscriptionPauseDuration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalSubscriptionPauseDuration>,
        builder: PortalSubscriptionPauseDurationBuilder,
    }

    impl Visitor for Place<PortalSubscriptionPauseDuration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalSubscriptionPauseDurationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalSubscriptionPauseDurationBuilder {
        type Out = PortalSubscriptionPauseDuration;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "interval_options" => Deserialize::begin(&mut self.interval_options),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { interval_options: None, type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(interval_options), Some(type_)) =
                (self.interval_options.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { interval_options, type_ })
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

    impl ObjectDeser for PortalSubscriptionPauseDuration {
        type Builder = PortalSubscriptionPauseDurationBuilder;
    }

    impl FromValueOpt for PortalSubscriptionPauseDuration {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalSubscriptionPauseDurationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "interval_options" => b.interval_options = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Whether subscriptions can be paused indefinitely or for a preset duration.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalSubscriptionPauseDurationType {
    Indefinite,
    IntervalOptions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalSubscriptionPauseDurationType {
    pub fn as_str(&self) -> &str {
        use PortalSubscriptionPauseDurationType::*;
        match self {
            Indefinite => "indefinite",
            IntervalOptions => "interval_options",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalSubscriptionPauseDurationType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalSubscriptionPauseDurationType::*;
        match s {
            "indefinite" => Ok(Indefinite),
            "interval_options" => Ok(IntervalOptions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PortalSubscriptionPauseDurationType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PortalSubscriptionPauseDurationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PortalSubscriptionPauseDurationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionPauseDurationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PortalSubscriptionPauseDurationType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PortalSubscriptionPauseDurationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PortalSubscriptionPauseDurationType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PortalSubscriptionPauseDurationType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalSubscriptionPauseDurationType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PortalSubscriptionPauseDurationType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalSubscriptionPauseDurationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
