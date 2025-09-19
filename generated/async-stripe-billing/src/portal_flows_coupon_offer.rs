#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsCouponOffer {
    /// The ID of the coupon to be offered.
    pub coupon: String,
}
#[doc(hidden)]
pub struct PortalFlowsCouponOfferBuilder {
    coupon: Option<String>,
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

    impl Deserialize for PortalFlowsCouponOffer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsCouponOffer>,
        builder: PortalFlowsCouponOfferBuilder,
    }

    impl Visitor for Place<PortalFlowsCouponOffer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFlowsCouponOfferBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalFlowsCouponOfferBuilder {
        type Out = PortalFlowsCouponOffer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "coupon" => Deserialize::begin(&mut self.coupon),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { coupon: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(coupon),) = (self.coupon.take(),) else {
                return None;
            };
            Some(Self::Out { coupon })
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

    impl ObjectDeser for PortalFlowsCouponOffer {
        type Builder = PortalFlowsCouponOfferBuilder;
    }

    impl FromValueOpt for PortalFlowsCouponOffer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalFlowsCouponOfferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "coupon" => b.coupon = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
