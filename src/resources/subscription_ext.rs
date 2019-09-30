use crate::config::{Client, Response};
use crate::ids::{SubscriptionId, SubscriptionScheduleId};
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{CreateSubscriptionItems, PaymentMethod, Subscription};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_period_end: Option<bool>,
}

impl CancelSubscription {
    pub fn new() -> CancelSubscription {
        CancelSubscription { at_period_end: None }
    }
}

impl Subscription {
    /// Cancels a subscription.
    ///
    /// For more details see https://stripe.com/docs/api#cancel_subscription.
    pub fn cancel(
        client: &Client,
        subscription_id: &SubscriptionId,
        params: CancelSubscription,
    ) -> Response<Subscription> {
        client.delete_query(&format!("/subscriptions/{}", subscription_id), params)
    }
}

impl CreateSubscriptionItems {
    pub fn new(plan: impl AsRef<str>) -> Self {
        Self {
            billing_thresholds: Default::default(),
            metadata: Default::default(),
            plan: plan.as_ref().to_owned(),
            quantity: Default::default(),
            tax_rates: Default::default(),
        }
    }
}

/// The resource representing a Stripe "SubscriptionSchedule".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionSchedule {
    /// Unique identifier for the object.
    pub id: SubscriptionScheduleId,

    /// Time at which the subscription schedule was canceled.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<Timestamp>,

    /// Time at which the subscription schedule was completed.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<Timestamp>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Expandable<PaymentMethod>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// Time at which the subscription schedule was released.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<Timestamp>,

    /// ID of the subscription once managed by the subscription schedule (if it is released).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_subscription: Option<String>,

    /// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
    pub status: SubscriptionScheduleStatus,

    /// ID of the subscription managed by the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Expandable<Subscription>>,
}

impl Object for SubscriptionSchedule {
    type Id = SubscriptionScheduleId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription_schedule"
    }
}

/// An enum representing the possible values of an `SubscriptionSchedule`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
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
            SubscriptionScheduleStatus::Active => "active",
            SubscriptionScheduleStatus::Canceled => "canceled",
            SubscriptionScheduleStatus::Completed => "completed",
            SubscriptionScheduleStatus::NotStarted => "not_started",
            SubscriptionScheduleStatus::Released => "released",
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
