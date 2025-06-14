#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditGrantsResourceAmount {
    /// The monetary amount.
    pub monetary: Option<stripe_shared::BillingCreditGrantsResourceMonetaryAmount>,
    /// The type of this amount. We currently only support `monetary` billing credits.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: BillingCreditGrantsResourceAmountType,
}
#[doc(hidden)]
pub struct BillingCreditGrantsResourceAmountBuilder {
    monetary: Option<Option<stripe_shared::BillingCreditGrantsResourceMonetaryAmount>>,
    type_: Option<BillingCreditGrantsResourceAmountType>,
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

    impl Deserialize for BillingCreditGrantsResourceAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingCreditGrantsResourceAmount>,
        builder: BillingCreditGrantsResourceAmountBuilder,
    }

    impl Visitor for Place<BillingCreditGrantsResourceAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingCreditGrantsResourceAmountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingCreditGrantsResourceAmountBuilder {
        type Out = BillingCreditGrantsResourceAmount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "monetary" => Deserialize::begin(&mut self.monetary),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { monetary: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(monetary), Some(type_)) = (self.monetary.take(), self.type_) else {
                return None;
            };
            Some(Self::Out { monetary, type_ })
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

    impl ObjectDeser for BillingCreditGrantsResourceAmount {
        type Builder = BillingCreditGrantsResourceAmountBuilder;
    }

    impl FromValueOpt for BillingCreditGrantsResourceAmount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingCreditGrantsResourceAmountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "monetary" => b.monetary = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of this amount. We currently only support `monetary` billing credits.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingCreditGrantsResourceAmountType {
    Monetary,
}
impl BillingCreditGrantsResourceAmountType {
    pub fn as_str(self) -> &'static str {
        use BillingCreditGrantsResourceAmountType::*;
        match self {
            Monetary => "monetary",
        }
    }
}

impl std::str::FromStr for BillingCreditGrantsResourceAmountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingCreditGrantsResourceAmountType::*;
        match s {
            "monetary" => Ok(Monetary),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingCreditGrantsResourceAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingCreditGrantsResourceAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingCreditGrantsResourceAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BillingCreditGrantsResourceAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingCreditGrantsResourceAmountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BillingCreditGrantsResourceAmountType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingCreditGrantsResourceAmountType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingCreditGrantsResourceAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BillingCreditGrantsResourceAmountType")
        })
    }
}
