#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalSubscriptionUpdate {
    /// Determines the value to use for the billing cycle anchor on subscription updates.
    /// Valid values are `now` or `unchanged`, and the default value is `unchanged`.
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
    /// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
    pub billing_cycle_anchor: Option<PortalSubscriptionUpdateBillingCycleAnchor>,
    /// The types of subscription updates that are supported for items listed in the `products` attribute.
    /// When empty, subscriptions are not updateable.
    pub default_allowed_updates: Vec<PortalSubscriptionUpdateDefaultAllowedUpdates>,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The list of up to 10 products that support subscription updates.
    pub products: Option<Vec<stripe_billing::PortalSubscriptionUpdateProduct>>,
    /// Determines how to handle prorations resulting from subscription updates.
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    /// Defaults to a value of `none` if you don't set it during creation.
    pub proration_behavior: PortalSubscriptionUpdateProrationBehavior,
    pub schedule_at_period_end: stripe_billing::PortalResourceScheduleUpdateAtPeriodEnd,
    /// Determines how handle updates to trialing subscriptions.
    /// Valid values are `end_trial` and `continue_trial`.
    /// Defaults to a value of `end_trial` if you don't set it during creation.
    pub trial_update_behavior: PortalSubscriptionUpdateTrialUpdateBehavior,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalSubscriptionUpdate").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalSubscriptionUpdateBuilder {
    billing_cycle_anchor: Option<Option<PortalSubscriptionUpdateBillingCycleAnchor>>,
    default_allowed_updates: Option<Vec<PortalSubscriptionUpdateDefaultAllowedUpdates>>,
    enabled: Option<bool>,
    products: Option<Option<Vec<stripe_billing::PortalSubscriptionUpdateProduct>>>,
    proration_behavior: Option<PortalSubscriptionUpdateProrationBehavior>,
    schedule_at_period_end: Option<stripe_billing::PortalResourceScheduleUpdateAtPeriodEnd>,
    trial_update_behavior: Option<PortalSubscriptionUpdateTrialUpdateBehavior>,
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
                builder: PortalSubscriptionUpdateBuilder {
                    billing_cycle_anchor: Deserialize::default(),
                    default_allowed_updates: Deserialize::default(),
                    enabled: Deserialize::default(),
                    products: Deserialize::default(),
                    proration_behavior: Deserialize::default(),
                    schedule_at_period_end: Deserialize::default(),
                    trial_update_behavior: Deserialize::default(),
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
                "default_allowed_updates" => {
                    Deserialize::begin(&mut self.builder.default_allowed_updates)
                }
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "products" => Deserialize::begin(&mut self.builder.products),
                "proration_behavior" => Deserialize::begin(&mut self.builder.proration_behavior),
                "schedule_at_period_end" => {
                    Deserialize::begin(&mut self.builder.schedule_at_period_end)
                }
                "trial_update_behavior" => {
                    Deserialize::begin(&mut self.builder.trial_update_behavior)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(billing_cycle_anchor),
                Some(default_allowed_updates),
                Some(enabled),
                Some(products),
                Some(proration_behavior),
                Some(schedule_at_period_end),
                Some(trial_update_behavior),
            ) = (
                self.builder.billing_cycle_anchor.take(),
                self.builder.default_allowed_updates.take(),
                self.builder.enabled,
                self.builder.products.take(),
                self.builder.proration_behavior.take(),
                self.builder.schedule_at_period_end.take(),
                self.builder.trial_update_behavior.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PortalSubscriptionUpdate {
                billing_cycle_anchor,
                default_allowed_updates,
                enabled,
                products,
                proration_behavior,
                schedule_at_period_end,
                trial_update_behavior,
            });
            Ok(())
        }
    }
};
/// Determines the value to use for the billing cycle anchor on subscription updates.
/// Valid values are `now` or `unchanged`, and the default value is `unchanged`.
/// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
/// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalSubscriptionUpdateBillingCycleAnchor {
    Now,
    Unchanged,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalSubscriptionUpdateBillingCycleAnchor {
    pub fn as_str(&self) -> &str {
        use PortalSubscriptionUpdateBillingCycleAnchor::*;
        match self {
            Now => "now",
            Unchanged => "unchanged",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalSubscriptionUpdateBillingCycleAnchor {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalSubscriptionUpdateBillingCycleAnchor::*;
        match s {
            "now" => Ok(Now),
            "unchanged" => Ok(Unchanged),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PortalSubscriptionUpdateBillingCycleAnchor"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PortalSubscriptionUpdateBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PortalSubscriptionUpdateBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionUpdateBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PortalSubscriptionUpdateBillingCycleAnchor))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PortalSubscriptionUpdateBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PortalSubscriptionUpdateBillingCycleAnchor {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PortalSubscriptionUpdateBillingCycleAnchor> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PortalSubscriptionUpdateBillingCycleAnchor::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalSubscriptionUpdateBillingCycleAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The types of subscription updates that are supported for items listed in the `products` attribute.
/// When empty, subscriptions are not updateable.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalSubscriptionUpdateDefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalSubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(&self) -> &str {
        use PortalSubscriptionUpdateDefaultAllowedUpdates::*;
        match self {
            Price => "price",
            PromotionCode => "promotion_code",
            Quantity => "quantity",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalSubscriptionUpdateDefaultAllowedUpdates {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalSubscriptionUpdateDefaultAllowedUpdates::*;
        match s {
            "price" => Ok(Price),
            "promotion_code" => Ok(PromotionCode),
            "quantity" => Ok(Quantity),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PortalSubscriptionUpdateDefaultAllowedUpdates"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PortalSubscriptionUpdateDefaultAllowedUpdates))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PortalSubscriptionUpdateDefaultAllowedUpdates> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PortalSubscriptionUpdateDefaultAllowedUpdates::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Determines how to handle prorations resulting from subscription updates.
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
/// Defaults to a value of `none` if you don't set it during creation.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalSubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalSubscriptionUpdateProrationBehavior {
    pub fn as_str(&self) -> &str {
        use PortalSubscriptionUpdateProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalSubscriptionUpdateProrationBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalSubscriptionUpdateProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PortalSubscriptionUpdateProrationBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PortalSubscriptionUpdateProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PortalSubscriptionUpdateProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionUpdateProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PortalSubscriptionUpdateProrationBehavior))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PortalSubscriptionUpdateProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PortalSubscriptionUpdateProrationBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PortalSubscriptionUpdateProrationBehavior> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PortalSubscriptionUpdateProrationBehavior::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalSubscriptionUpdateProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Determines how handle updates to trialing subscriptions.
/// Valid values are `end_trial` and `continue_trial`.
/// Defaults to a value of `end_trial` if you don't set it during creation.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalSubscriptionUpdateTrialUpdateBehavior {
    ContinueTrial,
    EndTrial,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalSubscriptionUpdateTrialUpdateBehavior {
    pub fn as_str(&self) -> &str {
        use PortalSubscriptionUpdateTrialUpdateBehavior::*;
        match self {
            ContinueTrial => "continue_trial",
            EndTrial => "end_trial",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalSubscriptionUpdateTrialUpdateBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalSubscriptionUpdateTrialUpdateBehavior::*;
        match s {
            "continue_trial" => Ok(ContinueTrial),
            "end_trial" => Ok(EndTrial),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PortalSubscriptionUpdateTrialUpdateBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PortalSubscriptionUpdateTrialUpdateBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PortalSubscriptionUpdateTrialUpdateBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionUpdateTrialUpdateBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PortalSubscriptionUpdateTrialUpdateBehavior))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PortalSubscriptionUpdateTrialUpdateBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PortalSubscriptionUpdateTrialUpdateBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PortalSubscriptionUpdateTrialUpdateBehavior> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PortalSubscriptionUpdateTrialUpdateBehavior::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalSubscriptionUpdateTrialUpdateBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
