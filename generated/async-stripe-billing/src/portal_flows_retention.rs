#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsRetention {
    /// Configuration when `retention.type=coupon_offer`.
    pub coupon_offer: Option<stripe_billing::PortalFlowsCouponOffer>,
    /// Type of retention strategy that will be used.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::PortalFlowsRetentionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFlowsRetention {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalFlowsRetention").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalFlowsRetentionBuilder {
    coupon_offer: Option<Option<stripe_billing::PortalFlowsCouponOffer>>,
    type_: Option<stripe_shared::PortalFlowsRetentionType>,
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
            Self { coupon_offer: Some(None), type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(coupon_offer), Some(type_)) = (self.coupon_offer.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { coupon_offer, type_ })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
                    "coupon_offer" => b.coupon_offer = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of retention strategy that will be used.
pub type PortalFlowsRetentionType = stripe_shared::PortalFlowsRetentionType;
