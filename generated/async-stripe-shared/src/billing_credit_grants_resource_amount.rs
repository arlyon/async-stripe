#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditGrantsResourceAmount {
    /// The monetary amount.
    pub monetary: Option<stripe_shared::BillingCreditGrantsResourceMonetaryAmount>,
    /// The type of this amount. We currently only support `monetary` billing credits.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: BillingCreditGrantsResourceAmountType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingCreditGrantsResourceAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingCreditGrantsResourceAmount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingCreditGrantsResourceAmountBuilder {
    monetary: Option<Option<stripe_shared::BillingCreditGrantsResourceMonetaryAmount>>,
    type_: Option<BillingCreditGrantsResourceAmountType>,
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
                builder: BillingCreditGrantsResourceAmountBuilder {
                    monetary: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "monetary" => Deserialize::begin(&mut self.builder.monetary),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(monetary), Some(type_)) =
                (self.builder.monetary.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(BillingCreditGrantsResourceAmount { monetary, type_ });
            Ok(())
        }
    }
};
/// The type of this amount. We currently only support `monetary` billing credits.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingCreditGrantsResourceAmountType {
    Monetary,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingCreditGrantsResourceAmountType {
    pub fn as_str(&self) -> &str {
        use BillingCreditGrantsResourceAmountType::*;
        match self {
            Monetary => "monetary",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingCreditGrantsResourceAmountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingCreditGrantsResourceAmountType::*;
        match s {
            "monetary" => Ok(Monetary),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingCreditGrantsResourceAmountType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingCreditGrantsResourceAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingCreditGrantsResourceAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingCreditGrantsResourceAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingCreditGrantsResourceAmountType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for BillingCreditGrantsResourceAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingCreditGrantsResourceAmountType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingCreditGrantsResourceAmountType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingCreditGrantsResourceAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
