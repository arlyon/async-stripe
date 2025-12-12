/// Pending Updates store the changes pending from a previous update that will be applied
/// to the Subscription upon successful payment.
#[derive(Clone, Debug)]
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
                builder: SubscriptionsResourcePendingUpdateBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourcePendingUpdateBuilder {
        type Out = SubscriptionsResourcePendingUpdate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_cycle_anchor" => Deserialize::begin(&mut self.billing_cycle_anchor),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "subscription_items" => Deserialize::begin(&mut self.subscription_items),
                "trial_end" => Deserialize::begin(&mut self.trial_end),
                "trial_from_plan" => Deserialize::begin(&mut self.trial_from_plan),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                billing_cycle_anchor: Deserialize::default(),
                expires_at: Deserialize::default(),
                subscription_items: Deserialize::default(),
                trial_end: Deserialize::default(),
                trial_from_plan: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(billing_cycle_anchor),
                Some(expires_at),
                Some(subscription_items),
                Some(trial_end),
                Some(trial_from_plan),
            ) = (
                self.billing_cycle_anchor,
                self.expires_at,
                self.subscription_items.take(),
                self.trial_end,
                self.trial_from_plan,
            )
            else {
                return None;
            };
            Some(Self::Out {
                billing_cycle_anchor,
                expires_at,
                subscription_items,
                trial_end,
                trial_from_plan,
            })
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

    impl ObjectDeser for SubscriptionsResourcePendingUpdate {
        type Builder = SubscriptionsResourcePendingUpdateBuilder;
    }

    impl FromValueOpt for SubscriptionsResourcePendingUpdate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourcePendingUpdateBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "billing_cycle_anchor" => b.billing_cycle_anchor = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "subscription_items" => b.subscription_items = FromValueOpt::from_value(v),
                    "trial_end" => b.trial_end = FromValueOpt::from_value(v),
                    "trial_from_plan" => b.trial_from_plan = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
