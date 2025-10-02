/// The billing mode of the quote.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceSubscriptionDataBillingMode {
    pub flexible: Option<stripe_shared::SubscriptionsResourceBillingModeFlexible>,
    /// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: QuotesResourceSubscriptionDataBillingModeType,
}
#[doc(hidden)]
pub struct QuotesResourceSubscriptionDataBillingModeBuilder {
    flexible: Option<Option<stripe_shared::SubscriptionsResourceBillingModeFlexible>>,
    type_: Option<QuotesResourceSubscriptionDataBillingModeType>,
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
                builder: QuotesResourceSubscriptionDataBillingModeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for QuotesResourceSubscriptionDataBillingModeBuilder {
        type Out = QuotesResourceSubscriptionDataBillingMode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "flexible" => Deserialize::begin(&mut self.flexible),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { flexible: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(flexible), Some(type_)) = (self.flexible, self.type_) else {
                return None;
            };
            Some(Self::Out { flexible, type_ })
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

    impl ObjectDeser for QuotesResourceSubscriptionDataBillingMode {
        type Builder = QuotesResourceSubscriptionDataBillingModeBuilder;
    }

    impl FromValueOpt for QuotesResourceSubscriptionDataBillingMode {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = QuotesResourceSubscriptionDataBillingModeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "flexible" => b.flexible = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum QuotesResourceSubscriptionDataBillingModeType {
    Classic,
    Flexible,
}
impl QuotesResourceSubscriptionDataBillingModeType {
    pub fn as_str(self) -> &'static str {
        use QuotesResourceSubscriptionDataBillingModeType::*;
        match self {
            Classic => "classic",
            Flexible => "flexible",
        }
    }
}

impl std::str::FromStr for QuotesResourceSubscriptionDataBillingModeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuotesResourceSubscriptionDataBillingModeType::*;
        match s {
            "classic" => Ok(Classic),
            "flexible" => Ok(Flexible),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for QuotesResourceSubscriptionDataBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for QuotesResourceSubscriptionDataBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for QuotesResourceSubscriptionDataBillingModeType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<QuotesResourceSubscriptionDataBillingModeType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            QuotesResourceSubscriptionDataBillingModeType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(QuotesResourceSubscriptionDataBillingModeType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for QuotesResourceSubscriptionDataBillingModeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for QuotesResourceSubscriptionDataBillingModeType",
            )
        })
    }
}
