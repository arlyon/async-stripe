#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsFlowSubscriptionUpdateConfirm {
    /// The coupon or promotion code to apply to this subscription update.
    pub discounts: Option<Vec<stripe_billing::PortalFlowsSubscriptionUpdateConfirmDiscount>>,
    /// The [subscription item](https://docs.stripe.com/api/subscription_items) to be updated through this flow.
    /// Currently, only up to one may be specified and subscriptions with multiple items are not updatable.
    pub items: Vec<stripe_billing::PortalFlowsSubscriptionUpdateConfirmItem>,
    /// The ID of the subscription to be updated.
    pub subscription: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFlowsFlowSubscriptionUpdateConfirm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalFlowsFlowSubscriptionUpdateConfirm").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalFlowsFlowSubscriptionUpdateConfirmBuilder {
    discounts: Option<Option<Vec<stripe_billing::PortalFlowsSubscriptionUpdateConfirmDiscount>>>,
    items: Option<Vec<stripe_billing::PortalFlowsSubscriptionUpdateConfirmItem>>,
    subscription: Option<String>,
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

    impl Deserialize for PortalFlowsFlowSubscriptionUpdateConfirm {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsFlowSubscriptionUpdateConfirm>,
        builder: PortalFlowsFlowSubscriptionUpdateConfirmBuilder,
    }

    impl Visitor for Place<PortalFlowsFlowSubscriptionUpdateConfirm> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFlowsFlowSubscriptionUpdateConfirmBuilder {
                    discounts: Deserialize::default(),
                    items: Deserialize::default(),
                    subscription: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "discounts" => Deserialize::begin(&mut self.builder.discounts),
                "items" => Deserialize::begin(&mut self.builder.items),
                "subscription" => Deserialize::begin(&mut self.builder.subscription),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(discounts), Some(items), Some(subscription)) = (
                self.builder.discounts.take(),
                self.builder.items.take(),
                self.builder.subscription.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(PortalFlowsFlowSubscriptionUpdateConfirm { discounts, items, subscription });
            Ok(())
        }
    }
};
