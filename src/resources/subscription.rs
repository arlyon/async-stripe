use error::Error;
use http;
use resources::{Discount, Plan};
use params::{List, Metadata, Timestamp};

#[derive(Serialize)]
pub struct SubscriptionItemParams {
    pub plan: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub quantity: Option<u64>,
}

#[derive(Serialize)]
pub struct SubscriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")] pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub coupon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub items: Option<Vec<SubscriptionItemParams>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")] pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub prorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub proration_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")] pub quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub tax_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub trial_end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")] pub trial_period_days: Option<u64>,
}

#[derive(Deserialize)]
pub struct SubscriptionItem {
    pub id: String,
    pub created: Timestamp,
    pub plan: Plan,
    pub quantity: u64,
}

#[derive(Deserialize)]
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
    pub fn create(params: SubscriptionParams, key: &str) -> Result<Subscription, Error> {
        return http::post("/subscriptions", key, params);
    }

    pub fn get(subscription_id: &str, key: &str) -> Result<Subscription, Error> {
        return http::get(&format!("/subscriptions/{}", subscription_id), key);
    }

    pub fn update(subscription_id: &str, params: SubscriptionParams, key: &str) -> Result<Subscription, Error> {
        return http::post(&format!("/subscriptions/{}", subscription_id), key, params);
    }

    pub fn cancel(subscription_id: &str, at_period_end: bool, key: &str) -> Result<Subscription, Error> {
        let path = format!("/subscriptions/{}/cancel?at_period_end={}", subscription_id, at_period_end);
        return http::post(&path, key, ());
    }
}
