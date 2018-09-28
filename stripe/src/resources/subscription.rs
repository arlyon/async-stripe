use client::Client;
use error::Error;
use params::{List, Metadata, Timestamp};
use resources::{Discount, Plan};
use serde_qs as qs;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CancelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_period_end: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct ItemParams<'a> {
    pub plan: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}

/// The set of parameters that can be used when creating or updating a subscription.
///
/// For more details see https://stripe.com/docs/api#create_subscription and https://stripe.com/docs/api#update_subscription.
#[derive(Default, Serialize, Debug)]
pub struct SubscriptionParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ItemParams<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<TrialEnd<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u64>,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum TrialEnd<'a> {
    Timestamp(Timestamp),
    Special(&'a str),
}

/// The resource representing a Stripe subscription item.
///
/// For more details see https://stripe.com/docs/api#subscription_items.
#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionItem {
    pub id: String,
    pub created: Timestamp,
    pub plan: Plan,
    pub quantity: u64,
}

/// The resource representing a Stripe subscription.
///
/// For more details see https://stripe.com/docs/api#subscriptions.
#[derive(Debug, Deserialize, Serialize)]
pub struct Subscription {
    pub id: String,
    pub application_fee_percent: Option<f64>,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<Timestamp>,
    pub created: Option<Timestamp>,
    pub current_period_start: Timestamp,
    pub current_period_end: Timestamp,
    pub customer: String,
    pub discount: Option<Discount>,
    pub ended_at: Option<Timestamp>,
    pub items: List<SubscriptionItem>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub plan: Plan,
    pub quantity: u64,
    pub start: Timestamp,
    pub status: String, // (trialing, active, past_due, canceled, unpaid)
    pub tax_percent: Option<f64>,
    pub trial_start: Option<Timestamp>,
    pub trial_end: Option<Timestamp>,
}

impl Subscription {
    /// Creates a new subscription for a customer.
    ///
    /// For more details see https://stripe.com/docs/api#create_subscription.
    pub fn create(client: &Client, params: SubscriptionParams) -> Result<Subscription, Error> {
        client.post("/subscriptions", params)
    }

    /// Retrieves the details of a subscription.
    ///
    /// For more details see https://stripe.com/docs/api#retrieve_subscription.
    pub fn retrieve(client: &Client, subscription_id: &str) -> Result<Subscription, Error> {
        client.get(&format!("/subscriptions/{}", subscription_id))
    }

    /// Updates a subscription's properties.
    /// For more details see https://stripe.com/docs/api#update_subscription.
    pub fn update(
        client: &Client,
        subscription_id: &str,
        params: SubscriptionParams,
    ) -> Result<Subscription, Error> {
        client.post(&format!("/subscriptions/{}", subscription_id), params)
    }

    /// Cancels a subscription.
    ///
    /// For more details see https://stripe.com/docs/api#cancel_subscription.
    pub fn cancel(
        client: &Client,
        subscription_id: &str,
        params: CancelParams,
    ) -> Result<Subscription, Error> {
        client.delete(&format!("/subscriptions/{}?{}", subscription_id, qs::to_string(&params)?))
    }
}
