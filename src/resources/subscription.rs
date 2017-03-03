use resources::{Discount, Plan};
use params::{List, Metadata};

#[derive(Deserialize)]
pub struct SubscriptionItem {
    pub id: String,
    pub created: i64, // unix timestamp
    pub plan: Plan,
    pub quantity: u64,
}

#[derive(Deserialize)]
pub struct Subscription {
    pub id: String,
    pub application_fee_percent: f64,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<i64>, // unix timestamp
    pub created: Option<i64>, // unix timestamp
    pub current_period_start: i64, // unix timestamp
    pub current_period_end: i64, // unix timestamp
    pub customer: String,
    pub discount: Option<Discount>,
    pub ended_at: Option<i64>, // unix timestamp
    pub items: List<SubscriptionItem>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub plan: Plan,
    pub quantity: u64,
    pub start: i64, // unix timestamp
    pub status: String, // (trialing, active, past_due, canceled, unpaid)
    pub tax_percent: f64,
    pub trial_start: i64, // unix timestamp
    pub trial_end: i64, // unix timestamp
}
