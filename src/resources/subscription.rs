use error::Error;
use client::Client;
use resources::{Discount, Plan};
use params::{List, Metadata, Timestamp};

#[derive(Serialize)]
pub struct SubscriptionItemParams<'a> {
    pub plan: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")] pub quantity: Option<u64>,
}

#[derive(Default, Serialize)]
pub struct SubscriptionParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")] pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub items: Option<Vec<SubscriptionItemParams<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")] pub plan: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub prorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub proration_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")] pub quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub tax_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub trial_end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")] pub trial_period_days: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionItem {
    pub id: String,
    pub created: Timestamp,
    pub plan: Plan,
    pub quantity: u64,
}

#[derive(Debug, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub application_fee_percent: f64,
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
    pub tax_percent: f64,
    pub trial_start: Timestamp,
    pub trial_end: Timestamp,
}

impl Subscription {
    pub fn create(c: &Client, params: SubscriptionParams) -> Result<Subscription, Error> {
        return c.post("/subscriptions", params);
    }

    pub fn get(c: &Client, subscription_id: &str) -> Result<Subscription, Error> {
        return c.get(&format!("/subscriptions/{}", subscription_id));
    }

    pub fn update(c: &Client, subscription_id: &str, params: SubscriptionParams) -> Result<Subscription, Error> {
        return c.post(&format!("/subscriptions/{}", subscription_id), params);
    }

    pub fn cancel(c: &Client, subscription_id: &str, at_period_end: bool) -> Result<Subscription, Error> {
        let path = format!("/subscriptions/{}/cancel?at_period_end={}", subscription_id, at_period_end);
        return c.post(&path, ());
    }
}
