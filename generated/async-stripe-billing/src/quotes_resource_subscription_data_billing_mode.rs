/// The billing mode of the quote.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceSubscriptionDataBillingMode {
    pub flexible: Option<stripe_shared::SubscriptionsResourceBillingModeFlexible>,
    /// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: QuotesResourceSubscriptionDataBillingModeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for QuotesResourceSubscriptionDataBillingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QuotesResourceSubscriptionDataBillingMode").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct QuotesResourceSubscriptionDataBillingModeBuilder {
    flexible: Option<Option<stripe_shared::SubscriptionsResourceBillingModeFlexible>>,
    type_: Option<QuotesResourceSubscriptionDataBillingModeType>,
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

    impl Deserialize for QuotesResourceSubscriptionDataBillingMode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceSubscriptionDataBillingMode>,
        builder: QuotesResourceSubscriptionDataBillingModeBuilder,
    }

    impl Visitor for Place<QuotesResourceSubscriptionDataBillingMode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: QuotesResourceSubscriptionDataBillingModeBuilder {
                    flexible: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "flexible" => Deserialize::begin(&mut self.builder.flexible),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(flexible), Some(type_)) =
                (self.builder.flexible.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(QuotesResourceSubscriptionDataBillingMode { flexible, type_ });
            Ok(())
        }
    }
};
/// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum QuotesResourceSubscriptionDataBillingModeType {
    Classic,
    Flexible,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl QuotesResourceSubscriptionDataBillingModeType {
    pub fn as_str(&self) -> &str {
        use QuotesResourceSubscriptionDataBillingModeType::*;
        match self {
            Classic => "classic",
            Flexible => "flexible",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for QuotesResourceSubscriptionDataBillingModeType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuotesResourceSubscriptionDataBillingModeType::*;
        match s {
            "classic" => Ok(Classic),
            "flexible" => Ok(Flexible),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "QuotesResourceSubscriptionDataBillingModeType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for QuotesResourceSubscriptionDataBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for QuotesResourceSubscriptionDataBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for QuotesResourceSubscriptionDataBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(QuotesResourceSubscriptionDataBillingModeType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for QuotesResourceSubscriptionDataBillingModeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for QuotesResourceSubscriptionDataBillingModeType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<QuotesResourceSubscriptionDataBillingModeType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(QuotesResourceSubscriptionDataBillingModeType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for QuotesResourceSubscriptionDataBillingModeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
