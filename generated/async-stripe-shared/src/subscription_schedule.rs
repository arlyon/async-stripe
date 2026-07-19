/// A subscription schedule allows you to create and manage the lifecycle of a subscription by predefining expected changes.
///
/// Related guide: [Subscription schedules](https://docs.stripe.com/billing/subscriptions/subscription-schedules).
///
/// For more details see <<https://stripe.com/docs/api/subscription_schedules/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedule {
    /// ID of the Connect Application that created the schedule.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    pub billing_mode: stripe_shared::SubscriptionsResourceBillingMode,
    /// Time at which the subscription schedule was canceled. Measured in seconds since the Unix epoch.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Time at which the subscription schedule was completed. Measured in seconds since the Unix epoch.
    pub completed_at: Option<stripe_types::Timestamp>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Object representing the start and end dates for the current phase of the subscription schedule, if it is `active`.
    pub current_phase: Option<stripe_shared::SubscriptionScheduleCurrentPhase>,
    /// ID of the customer who owns the subscription schedule.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    /// ID of the account who owns the subscription schedule.
    pub customer_account: Option<String>,
    pub default_settings: stripe_shared::SubscriptionSchedulesResourceDefaultSettings,
    /// Behavior of the subscription schedule and underlying subscription when it ends.
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.
    /// `cancel` will end the subscription schedule and cancel the underlying subscription.
    pub end_behavior: stripe_shared::SubscriptionScheduleEndBehavior,
    /// Unique identifier for the object.
    pub id: stripe_shared::SubscriptionScheduleId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Configuration for the subscription schedule's phases.
    pub phases: Vec<stripe_shared::SubscriptionSchedulePhaseConfiguration>,
    /// Time at which the subscription schedule was released. Measured in seconds since the Unix epoch.
    pub released_at: Option<stripe_types::Timestamp>,
    /// ID of the subscription once managed by the subscription schedule (if it is released).
    pub released_subscription: Option<String>,
    /// The present status of the subscription schedule.
    /// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
    /// You can read more about the different states in our [behavior guide](https://docs.stripe.com/billing/subscriptions/subscription-schedules).
    pub status: SubscriptionScheduleStatus,
    /// ID of the subscription managed by the subscription schedule.
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    /// ID of the test clock this subscription schedule belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedule").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionScheduleBuilder {
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    billing_mode: Option<stripe_shared::SubscriptionsResourceBillingMode>,
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    completed_at: Option<Option<stripe_types::Timestamp>>,
    created: Option<stripe_types::Timestamp>,
    current_phase: Option<Option<stripe_shared::SubscriptionScheduleCurrentPhase>>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    customer_account: Option<Option<String>>,
    default_settings: Option<stripe_shared::SubscriptionSchedulesResourceDefaultSettings>,
    end_behavior: Option<stripe_shared::SubscriptionScheduleEndBehavior>,
    id: Option<stripe_shared::SubscriptionScheduleId>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    phases: Option<Vec<stripe_shared::SubscriptionSchedulePhaseConfiguration>>,
    released_at: Option<Option<stripe_types::Timestamp>>,
    released_subscription: Option<Option<String>>,
    status: Option<SubscriptionScheduleStatus>,
    subscription: Option<Option<stripe_types::Expandable<stripe_shared::Subscription>>>,
    test_clock: Option<Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>>,
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

    impl Deserialize for SubscriptionSchedule {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedule>,
        builder: SubscriptionScheduleBuilder,
    }

    impl Visitor for Place<SubscriptionSchedule> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionScheduleBuilder {
                    application: Deserialize::default(),
                    billing_mode: Deserialize::default(),
                    canceled_at: Deserialize::default(),
                    completed_at: Deserialize::default(),
                    created: Deserialize::default(),
                    current_phase: Deserialize::default(),
                    customer: Deserialize::default(),
                    customer_account: Deserialize::default(),
                    default_settings: Deserialize::default(),
                    end_behavior: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    phases: Deserialize::default(),
                    released_at: Deserialize::default(),
                    released_subscription: Deserialize::default(),
                    status: Deserialize::default(),
                    subscription: Deserialize::default(),
                    test_clock: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "application" => Deserialize::begin(&mut self.builder.application),
                "billing_mode" => Deserialize::begin(&mut self.builder.billing_mode),
                "canceled_at" => Deserialize::begin(&mut self.builder.canceled_at),
                "completed_at" => Deserialize::begin(&mut self.builder.completed_at),
                "created" => Deserialize::begin(&mut self.builder.created),
                "current_phase" => Deserialize::begin(&mut self.builder.current_phase),
                "customer" => Deserialize::begin(&mut self.builder.customer),
                "customer_account" => Deserialize::begin(&mut self.builder.customer_account),
                "default_settings" => Deserialize::begin(&mut self.builder.default_settings),
                "end_behavior" => Deserialize::begin(&mut self.builder.end_behavior),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "phases" => Deserialize::begin(&mut self.builder.phases),
                "released_at" => Deserialize::begin(&mut self.builder.released_at),
                "released_subscription" => {
                    Deserialize::begin(&mut self.builder.released_subscription)
                }
                "status" => Deserialize::begin(&mut self.builder.status),
                "subscription" => Deserialize::begin(&mut self.builder.subscription),
                "test_clock" => Deserialize::begin(&mut self.builder.test_clock),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(application),
                Some(billing_mode),
                Some(canceled_at),
                Some(completed_at),
                Some(created),
                Some(current_phase),
                Some(customer),
                Some(customer_account),
                Some(default_settings),
                Some(end_behavior),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(phases),
                Some(released_at),
                Some(released_subscription),
                Some(status),
                Some(subscription),
                Some(test_clock),
            ) = (
                self.builder.application.take(),
                self.builder.billing_mode.take(),
                self.builder.canceled_at,
                self.builder.completed_at,
                self.builder.created,
                self.builder.current_phase,
                self.builder.customer.take(),
                self.builder.customer_account.take(),
                self.builder.default_settings.take(),
                self.builder.end_behavior.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.phases.take(),
                self.builder.released_at,
                self.builder.released_subscription.take(),
                self.builder.status.take(),
                self.builder.subscription.take(),
                self.builder.test_clock.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SubscriptionSchedule {
                application,
                billing_mode,
                canceled_at,
                completed_at,
                created,
                current_phase,
                customer,
                customer_account,
                default_settings,
                end_behavior,
                id,
                livemode,
                metadata,
                phases,
                released_at,
                released_subscription,
                status,
                subscription,
                test_clock,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedule {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("SubscriptionSchedule", 20)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("billing_mode", &self.billing_mode)?;
        s.serialize_field("canceled_at", &self.canceled_at)?;
        s.serialize_field("completed_at", &self.completed_at)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("current_phase", &self.current_phase)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_account", &self.customer_account)?;
        s.serialize_field("default_settings", &self.default_settings)?;
        s.serialize_field("end_behavior", &self.end_behavior)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("phases", &self.phases)?;
        s.serialize_field("released_at", &self.released_at)?;
        s.serialize_field("released_subscription", &self.released_subscription)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("subscription", &self.subscription)?;
        s.serialize_field("test_clock", &self.test_clock)?;

        s.serialize_field("object", "subscription_schedule")?;
        s.end()
    }
}
/// The present status of the subscription schedule.
/// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
/// You can read more about the different states in our [behavior guide](https://docs.stripe.com/billing/subscriptions/subscription-schedules).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionScheduleStatus {
    Active,
    Canceled,
    Completed,
    NotStarted,
    Released,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionScheduleStatus {
    pub fn as_str(&self) -> &str {
        use SubscriptionScheduleStatus::*;
        match self {
            Active => "active",
            Canceled => "canceled",
            Completed => "completed",
            NotStarted => "not_started",
            Released => "released",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionScheduleStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionScheduleStatus::*;
        match s {
            "active" => Ok(Active),
            "canceled" => Ok(Canceled),
            "completed" => Ok(Completed),
            "not_started" => Ok(NotStarted),
            "released" => Ok(Released),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "SubscriptionScheduleStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionScheduleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionScheduleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionScheduleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionScheduleStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionScheduleStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SubscriptionScheduleStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<SubscriptionScheduleStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionScheduleStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionScheduleStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for SubscriptionSchedule {
    type Id = stripe_shared::SubscriptionScheduleId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(SubscriptionScheduleId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionScheduleEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionScheduleEndBehavior {
    pub fn as_str(&self) -> &str {
        use SubscriptionScheduleEndBehavior::*;
        match self {
            Cancel => "cancel",
            None => "none",
            Release => "release",
            Renew => "renew",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionScheduleEndBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionScheduleEndBehavior::*;
        match s {
            "cancel" => Ok(Cancel),
            "none" => Ok(None),
            "release" => Ok(Release),
            "renew" => Ok(Renew),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionScheduleEndBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionScheduleEndBehavior)).finish_non_exhaustive()
    }
}
impl serde::Serialize for SubscriptionScheduleEndBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SubscriptionScheduleEndBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<SubscriptionScheduleEndBehavior> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionScheduleEndBehavior::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionScheduleEndBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
