/// A subscription schedule allows you to create and manage the lifecycle of a subscription by predefining expected changes.
///
/// Related guide: [Subscription schedules](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
///
/// For more details see <<https://stripe.com/docs/api/subscription_schedules/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedule {
    /// ID of the Connect Application that created the schedule.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
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
    pub default_settings: stripe_shared::SubscriptionSchedulesResourceDefaultSettings,
    /// Behavior of the subscription schedule and underlying subscription when it ends.
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.
    /// `cancel` will end the subscription schedule and cancel the underlying subscription.
    pub end_behavior: stripe_shared::SubscriptionScheduleEndBehavior,
    /// Unique identifier for the object.
    pub id: stripe_shared::SubscriptionScheduleId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    /// You can read more about the different states in our [behavior guide](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
    pub status: SubscriptionScheduleStatus,
    /// ID of the subscription managed by the subscription schedule.
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    /// ID of the test clock this subscription schedule belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
}
#[doc(hidden)]
pub struct SubscriptionScheduleBuilder {
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    completed_at: Option<Option<stripe_types::Timestamp>>,
    created: Option<stripe_types::Timestamp>,
    current_phase: Option<Option<stripe_shared::SubscriptionScheduleCurrentPhase>>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
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

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: SubscriptionScheduleBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionScheduleBuilder {
        type Out = SubscriptionSchedule;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "application" => Deserialize::begin(&mut self.application),
                "canceled_at" => Deserialize::begin(&mut self.canceled_at),
                "completed_at" => Deserialize::begin(&mut self.completed_at),
                "created" => Deserialize::begin(&mut self.created),
                "current_phase" => Deserialize::begin(&mut self.current_phase),
                "customer" => Deserialize::begin(&mut self.customer),
                "default_settings" => Deserialize::begin(&mut self.default_settings),
                "end_behavior" => Deserialize::begin(&mut self.end_behavior),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "phases" => Deserialize::begin(&mut self.phases),
                "released_at" => Deserialize::begin(&mut self.released_at),
                "released_subscription" => Deserialize::begin(&mut self.released_subscription),
                "status" => Deserialize::begin(&mut self.status),
                "subscription" => Deserialize::begin(&mut self.subscription),
                "test_clock" => Deserialize::begin(&mut self.test_clock),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                application: Deserialize::default(),
                canceled_at: Deserialize::default(),
                completed_at: Deserialize::default(),
                created: Deserialize::default(),
                current_phase: Deserialize::default(),
                customer: Deserialize::default(),
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                application: self.application.take()?,
                canceled_at: self.canceled_at?,
                completed_at: self.completed_at?,
                created: self.created?,
                current_phase: self.current_phase?,
                customer: self.customer.take()?,
                default_settings: self.default_settings.take()?,
                end_behavior: self.end_behavior?,
                id: self.id.take()?,
                livemode: self.livemode?,
                metadata: self.metadata.take()?,
                phases: self.phases.take()?,
                released_at: self.released_at?,
                released_subscription: self.released_subscription.take()?,
                status: self.status?,
                subscription: self.subscription.take()?,
                test_clock: self.test_clock.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SubscriptionSchedule {
        type Builder = SubscriptionScheduleBuilder;
    }

    impl FromValueOpt for SubscriptionSchedule {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionScheduleBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "application" => b.application = Some(FromValueOpt::from_value(v)?),
                    "canceled_at" => b.canceled_at = Some(FromValueOpt::from_value(v)?),
                    "completed_at" => b.completed_at = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "current_phase" => b.current_phase = Some(FromValueOpt::from_value(v)?),
                    "customer" => b.customer = Some(FromValueOpt::from_value(v)?),
                    "default_settings" => b.default_settings = Some(FromValueOpt::from_value(v)?),
                    "end_behavior" => b.end_behavior = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "phases" => b.phases = Some(FromValueOpt::from_value(v)?),
                    "released_at" => b.released_at = Some(FromValueOpt::from_value(v)?),
                    "released_subscription" => {
                        b.released_subscription = Some(FromValueOpt::from_value(v)?)
                    }
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "subscription" => b.subscription = Some(FromValueOpt::from_value(v)?),
                    "test_clock" => b.test_clock = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedule {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("SubscriptionSchedule", 18)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("canceled_at", &self.canceled_at)?;
        s.serialize_field("completed_at", &self.completed_at)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("current_phase", &self.current_phase)?;
        s.serialize_field("customer", &self.customer)?;
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
/// You can read more about the different states in our [behavior guide](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionScheduleStatus {
    Active,
    Canceled,
    Completed,
    NotStarted,
    Released,
}
impl SubscriptionScheduleStatus {
    pub fn as_str(self) -> &'static str {
        use SubscriptionScheduleStatus::*;
        match self {
            Active => "active",
            Canceled => "canceled",
            Completed => "completed",
            NotStarted => "not_started",
            Released => "released",
        }
    }
}

impl std::str::FromStr for SubscriptionScheduleStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionScheduleStatus::*;
        match s {
            "active" => Ok(Active),
            "canceled" => Ok(Canceled),
            "completed" => Ok(Completed),
            "not_started" => Ok(NotStarted),
            "released" => Ok(Released),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SubscriptionScheduleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionScheduleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for SubscriptionScheduleStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SubscriptionScheduleStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionScheduleStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SubscriptionScheduleStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionScheduleStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionScheduleStatus"))
    }
}
impl stripe_types::Object for SubscriptionSchedule {
    type Id = stripe_shared::SubscriptionScheduleId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(SubscriptionScheduleId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionScheduleEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
}
impl SubscriptionScheduleEndBehavior {
    pub fn as_str(self) -> &'static str {
        use SubscriptionScheduleEndBehavior::*;
        match self {
            Cancel => "cancel",
            None => "none",
            Release => "release",
            Renew => "renew",
        }
    }
}

impl std::str::FromStr for SubscriptionScheduleEndBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionScheduleEndBehavior::*;
        match s {
            "cancel" => Ok(Cancel),
            "none" => Ok(None),
            "release" => Ok(Release),
            "renew" => Ok(Renew),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for SubscriptionScheduleEndBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SubscriptionScheduleEndBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(SubscriptionScheduleEndBehavior::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SubscriptionScheduleEndBehavior);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionScheduleEndBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SubscriptionScheduleEndBehavior")
        })
    }
}
