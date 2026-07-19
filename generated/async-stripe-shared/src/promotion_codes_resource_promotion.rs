#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PromotionCodesResourcePromotion {
    /// If promotion `type` is `coupon`, the coupon for this promotion.
    pub coupon: Option<stripe_types::Expandable<stripe_shared::Coupon>>,
    /// The type of promotion.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PromotionCodesResourcePromotionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PromotionCodesResourcePromotion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PromotionCodesResourcePromotion").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PromotionCodesResourcePromotionBuilder {
    coupon: Option<Option<stripe_types::Expandable<stripe_shared::Coupon>>>,
    type_: Option<PromotionCodesResourcePromotionType>,
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

    impl Deserialize for PromotionCodesResourcePromotion {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PromotionCodesResourcePromotion>,
        builder: PromotionCodesResourcePromotionBuilder,
    }

    impl Visitor for Place<PromotionCodesResourcePromotion> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PromotionCodesResourcePromotionBuilder {
                    coupon: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "coupon" => Deserialize::begin(&mut self.builder.coupon),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(coupon), Some(type_)) =
                (self.builder.coupon.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(PromotionCodesResourcePromotion { coupon, type_ });
            Ok(())
        }
    }
};
/// The type of promotion.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PromotionCodesResourcePromotionType {
    Coupon,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PromotionCodesResourcePromotionType {
    pub fn as_str(&self) -> &str {
        use PromotionCodesResourcePromotionType::*;
        match self {
            Coupon => "coupon",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PromotionCodesResourcePromotionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PromotionCodesResourcePromotionType::*;
        match s {
            "coupon" => Ok(Coupon),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PromotionCodesResourcePromotionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PromotionCodesResourcePromotionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PromotionCodesResourcePromotionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PromotionCodesResourcePromotionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PromotionCodesResourcePromotionType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PromotionCodesResourcePromotionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PromotionCodesResourcePromotionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PromotionCodesResourcePromotionType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PromotionCodesResourcePromotionType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PromotionCodesResourcePromotionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
