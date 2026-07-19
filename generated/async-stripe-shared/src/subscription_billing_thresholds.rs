#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionBillingThresholds {
    /// Monetary threshold that triggers the subscription to create an invoice
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    /// This value may not be `true` if the subscription contains items with plans that have `aggregate_usage=last_ever`.
    pub reset_billing_cycle_anchor: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionBillingThresholds {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionBillingThresholds").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionBillingThresholdsBuilder {
    amount_gte: Option<Option<i64>>,
    reset_billing_cycle_anchor: Option<Option<bool>>,
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

    impl Deserialize for SubscriptionBillingThresholds {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionBillingThresholds>,
        builder: SubscriptionBillingThresholdsBuilder,
    }

    impl Visitor for Place<SubscriptionBillingThresholds> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionBillingThresholdsBuilder {
                    amount_gte: Deserialize::default(),
                    reset_billing_cycle_anchor: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_gte" => Deserialize::begin(&mut self.builder.amount_gte),
                "reset_billing_cycle_anchor" => {
                    Deserialize::begin(&mut self.builder.reset_billing_cycle_anchor)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount_gte), Some(reset_billing_cycle_anchor)) =
                (self.builder.amount_gte, self.builder.reset_billing_cycle_anchor)
            else {
                return Ok(());
            };
            *self.out =
                Some(SubscriptionBillingThresholds { amount_gte, reset_billing_cycle_anchor });
            Ok(())
        }
    }
};
