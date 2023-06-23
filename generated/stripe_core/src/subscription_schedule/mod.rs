/// A subscription schedule allows you to create and manage the lifecycle of a subscription by predefining expected changes.
///
/// Related guide: [Subscription Schedules](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionSchedule {
    /// ID of the Connect Application that created the schedule.
    pub application: Option<stripe_types::Expandable<stripe_types::application::Application>>,
    /// Time at which the subscription schedule was canceled.
    ///
    /// Measured in seconds since the Unix epoch.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Time at which the subscription schedule was completed.
    ///
    /// Measured in seconds since the Unix epoch.
    pub completed_at: Option<stripe_types::Timestamp>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Object representing the start and end dates for the current phase of the subscription schedule, if it is `active`.
    pub current_phase: Option<stripe_core::subscription_schedule::current_phase::CurrentPhase>,
    /// ID of the customer who owns the subscription schedule.
    pub customer: stripe_types::Expandable<stripe_core::customer::Customer>,
    pub default_settings: stripe_core::subscription_schedule::default_settings::DefaultSettings,
    /// Behavior of the subscription schedule and underlying subscription when it ends.
    ///
    /// Possible values are `release` and `cancel`.
    pub end_behavior: SubscriptionScheduleEndBehavior,
    /// Unique identifier for the object.
    pub id: stripe_core::subscription_schedule::SubscriptionScheduleId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SubscriptionScheduleObject,
    /// Configuration for the subscription schedule's phases.
    pub phases: Vec<stripe_core::subscription_schedule::phase::Phase>,
    /// Time at which the subscription schedule was released.
    ///
    /// Measured in seconds since the Unix epoch.
    pub released_at: Option<stripe_types::Timestamp>,
    /// ID of the subscription once managed by the subscription schedule (if it is released).
    pub released_subscription: Option<String>,
    /// The present status of the subscription schedule.
    ///
    /// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
    /// You can read more about the different states in our [behavior guide](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
    pub status: SubscriptionScheduleStatus,
    /// ID of the subscription managed by the subscription schedule.
    pub subscription: Option<stripe_types::Expandable<stripe_core::subscription::Subscription>>,
    /// ID of the test clock this subscription schedule belongs to.
    pub test_clock:
        Option<stripe_types::Expandable<stripe_core::test_helpers::test_clock::TestClock>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionSchedule {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Behavior of the subscription schedule and underlying subscription when it ends.
///
/// Possible values are `release` and `cancel`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
}

impl SubscriptionScheduleEndBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cancel => "cancel",
            Self::None => "none",
            Self::Release => "release",
            Self::Renew => "renew",
        }
    }
}

impl AsRef<str> for SubscriptionScheduleEndBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleObject {
    SubscriptionSchedule,
}

impl SubscriptionScheduleObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SubscriptionSchedule => "subscription_schedule",
        }
    }
}

impl AsRef<str> for SubscriptionScheduleObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The present status of the subscription schedule.
///
/// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
/// You can read more about the different states in our [behavior guide](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleStatus {
    Active,
    Canceled,
    Completed,
    NotStarted,
    Released,
}

impl SubscriptionScheduleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Canceled => "canceled",
            Self::Completed => "completed",
            Self::NotStarted => "not_started",
            Self::Released => "released",
        }
    }
}

impl AsRef<str> for SubscriptionScheduleStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for SubscriptionSchedule {
    type Id = stripe_core::subscription_schedule::SubscriptionScheduleId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(SubscriptionScheduleId, "sub_sched_");
pub mod invoice_settings;
pub mod requests;
pub use invoice_settings::InvoiceSettings;
pub mod add_invoice_item;
pub use add_invoice_item::AddInvoiceItem;
pub mod phase_item;
pub use phase_item::PhaseItem;
pub mod current_phase;
pub use current_phase::CurrentPhase;
pub mod phase;
pub use phase::Phase;
pub mod default_settings;
pub use default_settings::DefaultSettings;
