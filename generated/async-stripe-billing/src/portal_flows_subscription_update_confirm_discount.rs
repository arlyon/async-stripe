#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsSubscriptionUpdateConfirmDiscount {
    /// The ID of the coupon to apply to this subscription update.
    pub coupon: Option<String>,
    /// The ID of a promotion code to apply to this subscription update.
    pub promotion_code: Option<String>,
}
#[doc(hidden)]
pub struct PortalFlowsSubscriptionUpdateConfirmDiscountBuilder {
    coupon: Option<Option<String>>,
    promotion_code: Option<Option<String>>,
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

    impl Deserialize for PortalFlowsSubscriptionUpdateConfirmDiscount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsSubscriptionUpdateConfirmDiscount>,
        builder: PortalFlowsSubscriptionUpdateConfirmDiscountBuilder,
    }

    impl Visitor for Place<PortalFlowsSubscriptionUpdateConfirmDiscount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFlowsSubscriptionUpdateConfirmDiscountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalFlowsSubscriptionUpdateConfirmDiscountBuilder {
        type Out = PortalFlowsSubscriptionUpdateConfirmDiscount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "coupon" => Deserialize::begin(&mut self.coupon),
                "promotion_code" => Deserialize::begin(&mut self.promotion_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { coupon: Deserialize::default(), promotion_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(coupon), Some(promotion_code)) =
                (self.coupon.take(), self.promotion_code.take())
            else {
                return None;
            };
            Some(Self::Out { coupon, promotion_code })
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

    impl ObjectDeser for PortalFlowsSubscriptionUpdateConfirmDiscount {
        type Builder = PortalFlowsSubscriptionUpdateConfirmDiscountBuilder;
    }

    impl FromValueOpt for PortalFlowsSubscriptionUpdateConfirmDiscount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalFlowsSubscriptionUpdateConfirmDiscountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "coupon" => b.coupon = FromValueOpt::from_value(v),
                    "promotion_code" => b.promotion_code = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
