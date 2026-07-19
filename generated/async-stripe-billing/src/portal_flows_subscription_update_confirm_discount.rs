#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsSubscriptionUpdateConfirmDiscount {
    /// The ID of the coupon to apply to this subscription update.
    pub coupon: Option<String>,
    /// The ID of a promotion code to apply to this subscription update.
    pub promotion_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFlowsSubscriptionUpdateConfirmDiscount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalFlowsSubscriptionUpdateConfirmDiscount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalFlowsSubscriptionUpdateConfirmDiscountBuilder {
    coupon: Option<Option<String>>,
    promotion_code: Option<Option<String>>,
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
                builder: PortalFlowsSubscriptionUpdateConfirmDiscountBuilder {
                    coupon: Deserialize::default(),
                    promotion_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "coupon" => Deserialize::begin(&mut self.builder.coupon),
                "promotion_code" => Deserialize::begin(&mut self.builder.promotion_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(coupon), Some(promotion_code)) =
                (self.builder.coupon.take(), self.builder.promotion_code.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(PortalFlowsSubscriptionUpdateConfirmDiscount { coupon, promotion_code });
            Ok(())
        }
    }
};
