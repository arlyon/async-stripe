#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AmazonPayUnderlyingPaymentMethodFundingDetails {
    pub card: Option<stripe_shared::PaymentMethodDetailsPassthroughCard>,
    /// funding type of the underlying payment method.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<AmazonPayUnderlyingPaymentMethodFundingDetailsType>,
}
#[doc(hidden)]
pub struct AmazonPayUnderlyingPaymentMethodFundingDetailsBuilder {
    card: Option<Option<stripe_shared::PaymentMethodDetailsPassthroughCard>>,
    type_: Option<Option<AmazonPayUnderlyingPaymentMethodFundingDetailsType>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AmazonPayUnderlyingPaymentMethodFundingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AmazonPayUnderlyingPaymentMethodFundingDetails>,
        builder: AmazonPayUnderlyingPaymentMethodFundingDetailsBuilder,
    }

    impl Visitor for Place<AmazonPayUnderlyingPaymentMethodFundingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AmazonPayUnderlyingPaymentMethodFundingDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AmazonPayUnderlyingPaymentMethodFundingDetailsBuilder {
        type Out = AmazonPayUnderlyingPaymentMethodFundingDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card" => Deserialize::begin(&mut self.card),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(card), Some(type_)) = (self.card.take(), self.type_) else {
                return None;
            };
            Some(Self::Out { card, type_ })
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

    impl ObjectDeser for AmazonPayUnderlyingPaymentMethodFundingDetails {
        type Builder = AmazonPayUnderlyingPaymentMethodFundingDetailsBuilder;
    }

    impl FromValueOpt for AmazonPayUnderlyingPaymentMethodFundingDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AmazonPayUnderlyingPaymentMethodFundingDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card" => b.card = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// funding type of the underlying payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AmazonPayUnderlyingPaymentMethodFundingDetailsType {
    Card,
}
impl AmazonPayUnderlyingPaymentMethodFundingDetailsType {
    pub fn as_str(self) -> &'static str {
        use AmazonPayUnderlyingPaymentMethodFundingDetailsType::*;
        match self {
            Card => "card",
        }
    }
}

impl std::str::FromStr for AmazonPayUnderlyingPaymentMethodFundingDetailsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AmazonPayUnderlyingPaymentMethodFundingDetailsType::*;
        match s {
            "card" => Ok(Card),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for AmazonPayUnderlyingPaymentMethodFundingDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AmazonPayUnderlyingPaymentMethodFundingDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AmazonPayUnderlyingPaymentMethodFundingDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AmazonPayUnderlyingPaymentMethodFundingDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AmazonPayUnderlyingPaymentMethodFundingDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            AmazonPayUnderlyingPaymentMethodFundingDetailsType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AmazonPayUnderlyingPaymentMethodFundingDetailsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AmazonPayUnderlyingPaymentMethodFundingDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for AmazonPayUnderlyingPaymentMethodFundingDetailsType",
            )
        })
    }
}
