use error::Error;
use http;
use params::{Metadata, Timestamp};
use resources::Deleted;

#[derive(Serialize)]
pub struct PlanParams {
    #[serde(skip_serializing_if = "Option::is_none")] pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub currency: Option<String>, // eg. "usd"
    #[serde(skip_serializing_if = "Option::is_none")] pub interval: Option<String>, // (day, week, month, year)
    #[serde(skip_serializing_if = "Option::is_none")] pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] pub interval_count: Option<u64>,
    // NOTE: serde_urlencoded doesn't yet support struct/map style serialization
    // #[serde(skip_serializing_if = "Option::is_none")] pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")] pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub trial_period_days: Option<u64>,
}

#[derive(Deserialize)]
pub struct Plan {
    pub id: String,
    pub amount: u64,
    pub created: Timestamp,
    pub currency: String, // eg. "usd"
    pub interval: String, // (day, week, month, year)
    pub interval_count: u64,
    pub livemode: bool,
    pub metadata: Metadata,
    pub name: String,
    pub statement_descriptor: Option<String>,
    pub trial_period_days: Option<u64>,
}

impl Plan {
    pub fn create(params: PlanParams, key: &str) -> Result<Plan, Error> {
        return http::post("/plans", key, params);
    }

    pub fn get(plan_id: &str, key: &str) -> Result<Plan, Error> {
        return http::get(&format!("/plans/{}", plan_id), key);
    }

    pub fn update(plan_id: &str, params: PlanParams, key: &str) -> Result<Plan, Error> {
        return http::post(&format!("/plans/{}", plan_id), key, params);
    }

    pub fn delete(plan_id: &str, key: &str) -> Result<Deleted, Error> {
        return http::delete(&format!("/plans/{}", plan_id), key);
    }
}
