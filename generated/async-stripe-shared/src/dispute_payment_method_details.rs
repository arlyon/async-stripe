#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputePaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputePaymentMethodDetails").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: DisputePaymentMethodDetailsBuilder {
                    amazon_pay: Deserialize::default(),
                    card: Deserialize::default(),
                    klarna: Deserialize::default(),
                    paypal: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amazon_pay" => Deserialize::begin(&mut self.builder.amazon_pay),
                "card" => Deserialize::begin(&mut self.builder.card),
                "klarna" => Deserialize::begin(&mut self.builder.klarna),
                "paypal" => Deserialize::begin(&mut self.builder.paypal),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amazon_pay), Some(card), Some(klarna), Some(paypal), Some(type_)) = (
                self.builder.amazon_pay.take(),
                self.builder.card.take(),
                self.builder.klarna.take(),
                self.builder.paypal.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(DisputePaymentMethodDetails { amazon_pay, card, klarna, paypal, type_ });
            Ok(())
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for DisputePaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputePaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(DisputePaymentMethodDetailsType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for DisputePaymentMethodDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<DisputePaymentMethodDetailsType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DisputePaymentMethodDetailsType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DisputePaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
