/// Pending Updates store the changes pending from a previous update that will be applied
/// to the Subscription upon successful payment.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourcePendingUpdate {
    /// If the update is applied, determines the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    /// The timestamp is in UTC format.
    pub billing_cycle_anchor: Option<stripe_types::Timestamp>,
    /// The point after which the changes reflected by this update will be discarded and no longer applied.
    pub expires_at: stripe_types::Timestamp,
    /// List of subscription items, each with an attached plan, that will be set if the update is applied.
    pub subscription_items: Option<Vec<stripe_shared::SubscriptionItem>>,
    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time, if the update is applied.
    pub trial_end: Option<stripe_types::Timestamp>,
    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://docs.stripe.com/billing/subscriptions/trials) to learn more.
    pub trial_from_plan: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourcePendingUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourcePendingUpdate").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourcePendingUpdateBuilder {
    billing_cycle_anchor: Option<Option<stripe_types::Timestamp>>,
    expires_at: Option<stripe_types::Timestamp>,
    subscription_items: Option<Option<Vec<stripe_shared::SubscriptionItem>>>,
    trial_end: Option<Option<stripe_types::Timestamp>>,
    trial_from_plan: Option<Option<bool>>,
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

    impl Deserialize for SubscriptionsResourcePendingUpdate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourcePendingUpdate>,
        builder: SubscriptionsResourcePendingUpdateBuilder,
    }

    impl Visitor for Place<SubscriptionsResourcePendingUpdate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourcePendingUpdateBuilder {
                    billing_cycle_anchor: Deserialize::default(),
                    expires_at: Deserialize::default(),
                    subscription_items: Deserialize::default(),
                    trial_end: Deserialize::default(),
                    trial_from_plan: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_cycle_anchor" => {
                    Deserialize::begin(&mut self.builder.billing_cycle_anchor)
                }
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "subscription_items" => Deserialize::begin(&mut self.builder.subscription_items),
                "trial_end" => Deserialize::begin(&mut self.builder.trial_end),
                "trial_from_plan" => Deserialize::begin(&mut self.builder.trial_from_plan),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(billing_cycle_anchor),
                Some(expires_at),
                Some(subscription_items),
                Some(trial_end),
                Some(trial_from_plan),
            ) = (
                self.builder.billing_cycle_anchor,
                self.builder.expires_at,
                self.builder.subscription_items.take(),
                self.builder.trial_end,
                self.builder.trial_from_plan,
            )
            else {
                return Ok(());
            };
            *self.out = Some(SubscriptionsResourcePendingUpdate {
                billing_cycle_anchor,
                expires_at,
                subscription_items,
                trial_end,
                trial_from_plan,
            });
            Ok(())
        }
    }
};
