#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputePaymentMethodDetails {
    pub amazon_pay: Option<stripe_shared::DisputePaymentMethodDetailsAmazonPay>,
    pub card: Option<stripe_shared::DisputePaymentMethodDetailsCard>,
    pub klarna: Option<stripe_shared::DisputePaymentMethodDetailsKlarna>,
    pub paypal: Option<stripe_shared::DisputePaymentMethodDetailsPaypal>,
    /// Payment method type.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: DisputePaymentMethodDetailsType,
}
#[doc(hidden)]
pub struct DisputePaymentMethodDetailsBuilder {
    amazon_pay: Option<Option<stripe_shared::DisputePaymentMethodDetailsAmazonPay>>,
    card: Option<Option<stripe_shared::DisputePaymentMethodDetailsCard>>,
    klarna: Option<Option<stripe_shared::DisputePaymentMethodDetailsKlarna>>,
    paypal: Option<Option<stripe_shared::DisputePaymentMethodDetailsPaypal>>,
    type_: Option<DisputePaymentMethodDetailsType>,
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

    impl Deserialize for DisputePaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputePaymentMethodDetails>,
        builder: DisputePaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<DisputePaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputePaymentMethodDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputePaymentMethodDetailsBuilder {
        type Out = DisputePaymentMethodDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amazon_pay" => Deserialize::begin(&mut self.amazon_pay),
                "card" => Deserialize::begin(&mut self.card),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amazon_pay: Deserialize::default(),
                card: Deserialize::default(),
                klarna: Deserialize::default(),
                paypal: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amazon_pay), Some(card), Some(klarna), Some(paypal), Some(type_)) = (
                self.amazon_pay.take(),
                self.card.take(),
                self.klarna.take(),
                self.paypal.take(),
                self.type_.take(),
            ) else {
                return None;
            };
            Some(Self::Out { amazon_pay, card, klarna, paypal, type_ })
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

    impl ObjectDeser for DisputePaymentMethodDetails {
        type Builder = DisputePaymentMethodDetailsBuilder;
    }

    impl FromValueOpt for DisputePaymentMethodDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputePaymentMethodDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amazon_pay" => b.amazon_pay = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "klarna" => b.klarna = FromValueOpt::from_value(v),
                    "paypal" => b.paypal = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Payment method type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DisputePaymentMethodDetailsType {
    AmazonPay,
    Card,
    Klarna,
    Paypal,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DisputePaymentMethodDetailsType {
    pub fn as_str(&self) -> &str {
        use DisputePaymentMethodDetailsType::*;
        match self {
            AmazonPay => "amazon_pay",
            Card => "card",
            Klarna => "klarna",
            Paypal => "paypal",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DisputePaymentMethodDetailsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputePaymentMethodDetailsType::*;
        match s {
            "amazon_pay" => Ok(AmazonPay),
            "card" => Ok(Card),
            "klarna" => Ok(Klarna),
            "paypal" => Ok(Paypal),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "DisputePaymentMethodDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DisputePaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DisputePaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DisputePaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for DisputePaymentMethodDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<DisputePaymentMethodDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DisputePaymentMethodDetailsType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(DisputePaymentMethodDetailsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DisputePaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
