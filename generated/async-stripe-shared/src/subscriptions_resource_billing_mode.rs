/// The billing mode of the subscription.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceBillingMode {
    /// Configure behavior for flexible billing mode
    pub flexible: Option<stripe_shared::SubscriptionsResourceBillingModeFlexible>,
    /// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: SubscriptionsResourceBillingModeType,
    /// Details on when the current billing_mode was adopted.
    pub updated_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceBillingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourceBillingMode").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourceBillingModeBuilder {
    flexible: Option<Option<stripe_shared::SubscriptionsResourceBillingModeFlexible>>,
    type_: Option<SubscriptionsResourceBillingModeType>,
    updated_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for SubscriptionsResourceBillingMode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourceBillingMode>,
        builder: SubscriptionsResourceBillingModeBuilder,
    }

    impl Visitor for Place<SubscriptionsResourceBillingMode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourceBillingModeBuilder {
                    flexible: Deserialize::default(),
                    type_: Deserialize::default(),
                    updated_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "flexible" => Deserialize::begin(&mut self.builder.flexible),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "updated_at" => Deserialize::begin(&mut self.builder.updated_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(flexible), Some(type_), Some(updated_at)) =
                (self.builder.flexible.take(), self.builder.type_.take(), self.builder.updated_at)
            else {
                return Ok(());
            };
            *self.out = Some(SubscriptionsResourceBillingMode { flexible, type_, updated_at });
            Ok(())
        }
    }
};
/// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionsResourceBillingModeType {
    Classic,
    Flexible,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionsResourceBillingModeType {
    pub fn as_str(&self) -> &str {
        use SubscriptionsResourceBillingModeType::*;
        match self {
            Classic => "classic",
            Flexible => "flexible",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionsResourceBillingModeType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourceBillingModeType::*;
        match s {
            "classic" => Ok(Classic),
            "flexible" => Ok(Flexible),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionsResourceBillingModeType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionsResourceBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionsResourceBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionsResourceBillingModeType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionsResourceBillingModeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SubscriptionsResourceBillingModeType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<SubscriptionsResourceBillingModeType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionsResourceBillingModeType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionsResourceBillingModeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
