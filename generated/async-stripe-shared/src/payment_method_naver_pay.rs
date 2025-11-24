#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodNaverPay {
    /// Uniquely identifies this particular Naver Pay account.
    /// You can use this attribute to check whether two Naver Pay accounts are the same.
    pub buyer_id: Option<String>,
    /// Whether to fund this transaction with Naver Pay points or a card.
    pub funding: PaymentMethodNaverPayFunding,
}
#[doc(hidden)]
pub struct PaymentMethodNaverPayBuilder {
    buyer_id: Option<Option<String>>,
    funding: Option<PaymentMethodNaverPayFunding>,
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

    impl Deserialize for PaymentMethodNaverPay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodNaverPay>,
        builder: PaymentMethodNaverPayBuilder,
    }

    impl Visitor for Place<PaymentMethodNaverPay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodNaverPayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodNaverPayBuilder {
        type Out = PaymentMethodNaverPay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buyer_id" => Deserialize::begin(&mut self.buyer_id),
                "funding" => Deserialize::begin(&mut self.funding),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { buyer_id: Deserialize::default(), funding: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(buyer_id), Some(funding)) = (self.buyer_id.take(), self.funding.take())
            else {
                return None;
            };
            Some(Self::Out { buyer_id, funding })
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

    impl ObjectDeser for PaymentMethodNaverPay {
        type Builder = PaymentMethodNaverPayBuilder;
    }

    impl FromValueOpt for PaymentMethodNaverPay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodNaverPayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "buyer_id" => b.buyer_id = FromValueOpt::from_value(v),
                    "funding" => b.funding = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Whether to fund this transaction with Naver Pay points or a card.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodNaverPayFunding {
    Card,
    Points,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodNaverPayFunding {
    pub fn as_str(&self) -> &str {
        use PaymentMethodNaverPayFunding::*;
        match self {
            Card => "card",
            Points => "points",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodNaverPayFunding {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodNaverPayFunding::*;
        match s {
            "card" => Ok(Card),
            "points" => Ok(Points),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodNaverPayFunding"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodNaverPayFunding {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodNaverPayFunding {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodNaverPayFunding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodNaverPayFunding {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodNaverPayFunding> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodNaverPayFunding::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodNaverPayFunding);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodNaverPayFunding {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
