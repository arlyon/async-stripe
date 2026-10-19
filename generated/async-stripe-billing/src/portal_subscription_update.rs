#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalSubscriptionUpdate {
    /// Determines the value to use for the billing cycle anchor on subscription updates.
    /// Valid values are `now` or `unchanged`, and the default value is `unchanged`.
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
    /// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
    pub billing_cycle_anchor: Option<stripe_shared::BillingCycleAnchor>,
    /// The types of subscription updates that are supported for items listed in the `products` attribute.
    /// When empty, subscriptions are not updateable.
    pub default_allowed_updates: Vec<stripe_shared::DefaultAllowedUpdates>,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The list of up to 10 products that support subscription updates.
    pub products: Option<Vec<stripe_billing::PortalSubscriptionUpdateProduct>>,
    /// Determines how to handle prorations resulting from subscription updates.
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    /// Defaults to a value of `none` if you don't set it during creation.
    pub proration_behavior: stripe_shared::ProrationBehavior,
    pub schedule_at_period_end: stripe_billing::PortalResourceScheduleUpdateAtPeriodEnd,
    /// Determines how handle updates to trialing subscriptions.
    /// Valid values are `end_trial` and `continue_trial`.
    /// Defaults to a value of `end_trial` if you don't set it during creation.
    pub trial_update_behavior: stripe_shared::TrialUpdateBehavior,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalSubscriptionUpdate").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalSubscriptionUpdateBuilder {
    billing_cycle_anchor: Option<Option<stripe_shared::BillingCycleAnchor>>,
    default_allowed_updates: Option<Vec<stripe_shared::DefaultAllowedUpdates>>,
    enabled: Option<bool>,
    products: Option<Option<Vec<stripe_billing::PortalSubscriptionUpdateProduct>>>,
    proration_behavior: Option<stripe_shared::ProrationBehavior>,
    schedule_at_period_end: Option<stripe_billing::PortalResourceScheduleUpdateAtPeriodEnd>,
    trial_update_behavior: Option<stripe_shared::TrialUpdateBehavior>,
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

    impl Deserialize for PortalSubscriptionUpdate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalSubscriptionUpdate>,
        builder: PortalSubscriptionUpdateBuilder,
    }

    impl Visitor for Place<PortalSubscriptionUpdate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalSubscriptionUpdateBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalSubscriptionUpdateBuilder {
        type Out = PortalSubscriptionUpdate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_cycle_anchor" => Deserialize::begin(&mut self.billing_cycle_anchor),
                "default_allowed_updates" => Deserialize::begin(&mut self.default_allowed_updates),
                "enabled" => Deserialize::begin(&mut self.enabled),
                "products" => Deserialize::begin(&mut self.products),
                "proration_behavior" => Deserialize::begin(&mut self.proration_behavior),
                "schedule_at_period_end" => Deserialize::begin(&mut self.schedule_at_period_end),
                "trial_update_behavior" => Deserialize::begin(&mut self.trial_update_behavior),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                billing_cycle_anchor: Some(None),
                default_allowed_updates: None,
                enabled: None,
                products: Some(None),
                proration_behavior: None,
                schedule_at_period_end: None,
                trial_update_behavior: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(billing_cycle_anchor),
                Some(default_allowed_updates),
                Some(enabled),
                Some(products),
                Some(proration_behavior),
                Some(schedule_at_period_end),
                Some(trial_update_behavior),
            ) = (
                self.billing_cycle_anchor.take(),
                self.default_allowed_updates.take(),
                self.enabled,
                self.products.take(),
                self.proration_behavior.take(),
                self.schedule_at_period_end.take(),
                self.trial_update_behavior.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                billing_cycle_anchor,
                default_allowed_updates,
                enabled,
                products,
                proration_behavior,
                schedule_at_period_end,
                trial_update_behavior,
            })
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

    impl ObjectDeser for PortalSubscriptionUpdate {
        type Builder = PortalSubscriptionUpdateBuilder;
    }

    impl FromValueOpt for PortalSubscriptionUpdate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalSubscriptionUpdateBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "billing_cycle_anchor" => b.billing_cycle_anchor = FromValueOpt::from_value(v),
                    "default_allowed_updates" => {
                        b.default_allowed_updates = FromValueOpt::from_value(v)
                    }
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "products" => b.products = FromValueOpt::from_value(v),
                    "proration_behavior" => b.proration_behavior = FromValueOpt::from_value(v),
                    "schedule_at_period_end" => {
                        b.schedule_at_period_end = FromValueOpt::from_value(v)
                    }
                    "trial_update_behavior" => {
                        b.trial_update_behavior = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Determines the value to use for the billing cycle anchor on subscription updates.
/// Valid values are `now` or `unchanged`, and the default value is `unchanged`.
/// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
/// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
pub type PortalSubscriptionUpdateBillingCycleAnchor = stripe_shared::BillingCycleAnchor;
/// The types of subscription updates that are supported for items listed in the `products` attribute.
/// When empty, subscriptions are not updateable.
pub type PortalSubscriptionUpdateDefaultAllowedUpdates = stripe_shared::DefaultAllowedUpdates;
/// Determines how to handle prorations resulting from subscription updates.
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
/// Defaults to a value of `none` if you don't set it during creation.
pub type PortalSubscriptionUpdateProrationBehavior = stripe_shared::ProrationBehavior;
/// Determines how handle updates to trialing subscriptions.
/// Valid values are `end_trial` and `continue_trial`.
/// Defaults to a value of `end_trial` if you don't set it during creation.
pub type PortalSubscriptionUpdateTrialUpdateBehavior = stripe_shared::TrialUpdateBehavior;
