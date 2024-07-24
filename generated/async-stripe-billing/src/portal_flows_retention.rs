#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsRetention {
    /// Configuration when `retention.type=coupon_offer`.
    pub coupon_offer: Option<stripe_billing::PortalFlowsCouponOffer>,
    /// Type of retention strategy that will be used.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PortalFlowsRetentionType,
}
#[doc(hidden)]
pub struct PortalFlowsRetentionBuilder {
    coupon_offer: Option<Option<stripe_billing::PortalFlowsCouponOffer>>,
    type_: Option<PortalFlowsRetentionType>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsRetention {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsRetention>,
        builder: PortalFlowsRetentionBuilder,
    }

    impl Visitor for Place<PortalFlowsRetention> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFlowsRetentionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalFlowsRetentionBuilder {
        type Out = PortalFlowsRetention;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "coupon_offer" => Deserialize::begin(&mut self.coupon_offer),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { coupon_offer: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { coupon_offer: self.coupon_offer.take()?, type_: self.type_? })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PortalFlowsRetention {
        type Builder = PortalFlowsRetentionBuilder;
    }

    impl FromValueOpt for PortalFlowsRetention {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalFlowsRetentionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "coupon_offer" => b.coupon_offer = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of retention strategy that will be used.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalFlowsRetentionType {
    CouponOffer,
}
impl PortalFlowsRetentionType {
    pub fn as_str(self) -> &'static str {
        use PortalFlowsRetentionType::*;
        match self {
            CouponOffer => "coupon_offer",
        }
    }
}

impl std::str::FromStr for PortalFlowsRetentionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalFlowsRetentionType::*;
        match s {
            "coupon_offer" => Ok(CouponOffer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PortalFlowsRetentionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalFlowsRetentionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PortalFlowsRetentionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PortalFlowsRetentionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PortalFlowsRetentionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalFlowsRetentionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PortalFlowsRetentionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalFlowsRetentionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PortalFlowsRetentionType"))
    }
}
