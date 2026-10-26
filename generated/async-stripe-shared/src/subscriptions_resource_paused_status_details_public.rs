/// Indicates when and why the subscription transitioned to the paused status.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourcePausedStatusDetailsPublic {
    pub subscription: stripe_shared::SubscriptionsResourcePausedStatusDetailsSubscription,
    /// Unix timestamp in seconds of when the subscription status transitioned to `paused`.
    pub transitioned_at: stripe_types::Timestamp,
    /// The type of pause.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: SubscriptionsResourcePausedStatusDetailsPublicType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourcePausedStatusDetailsPublic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourcePausedStatusDetailsPublic").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourcePausedStatusDetailsPublicBuilder {
    subscription: Option<stripe_shared::SubscriptionsResourcePausedStatusDetailsSubscription>,
    transitioned_at: Option<stripe_types::Timestamp>,
    type_: Option<SubscriptionsResourcePausedStatusDetailsPublicType>,
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

    impl Deserialize for SubscriptionsResourcePausedStatusDetailsPublic {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourcePausedStatusDetailsPublic>,
        builder: SubscriptionsResourcePausedStatusDetailsPublicBuilder,
    }

    impl Visitor for Place<SubscriptionsResourcePausedStatusDetailsPublic> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourcePausedStatusDetailsPublicBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourcePausedStatusDetailsPublicBuilder {
        type Out = SubscriptionsResourcePausedStatusDetailsPublic;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "subscription" => Deserialize::begin(&mut self.subscription),
                "transitioned_at" => Deserialize::begin(&mut self.transitioned_at),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { subscription: None, transitioned_at: None, type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(subscription), Some(transitioned_at), Some(type_)) =
                (self.subscription.take(), self.transitioned_at, self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { subscription, transitioned_at, type_ })
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

    impl ObjectDeser for SubscriptionsResourcePausedStatusDetailsPublic {
        type Builder = SubscriptionsResourcePausedStatusDetailsPublicBuilder;
    }

    impl FromValueOpt for SubscriptionsResourcePausedStatusDetailsPublic {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourcePausedStatusDetailsPublicBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "subscription" => b.subscription = FromValueOpt::from_value(v),
                    "transitioned_at" => b.transitioned_at = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of pause.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionsResourcePausedStatusDetailsPublicType {
    Subscription,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionsResourcePausedStatusDetailsPublicType {
    pub fn as_str(&self) -> &str {
        use SubscriptionsResourcePausedStatusDetailsPublicType::*;
        match self {
            Subscription => "subscription",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionsResourcePausedStatusDetailsPublicType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourcePausedStatusDetailsPublicType::*;
        match s {
            "subscription" => Ok(Subscription),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionsResourcePausedStatusDetailsPublicType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionsResourcePausedStatusDetailsPublicType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionsResourcePausedStatusDetailsPublicType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourcePausedStatusDetailsPublicType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionsResourcePausedStatusDetailsPublicType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionsResourcePausedStatusDetailsPublicType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionsResourcePausedStatusDetailsPublicType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SubscriptionsResourcePausedStatusDetailsPublicType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionsResourcePausedStatusDetailsPublicType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SubscriptionsResourcePausedStatusDetailsPublicType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionsResourcePausedStatusDetailsPublicType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
