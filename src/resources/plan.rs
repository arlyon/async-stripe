use error::Error;
use client::Client;
use params::{Metadata, Timestamp};
use resources::{Currency, Deleted};

#[derive(Default, Serialize)]
pub struct PlanParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")] pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")] pub interval: Option<&'a str>, // (day, week, month, year)
    #[serde(skip_serializing_if = "Option::is_none")] pub name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")] pub interval_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")] pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub trial_period_days: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Plan {
    pub id: String,
    pub amount: u64,
    pub created: Timestamp,
    pub currency: Currency,
    pub interval: String, // (day, week, month, year)
    pub interval_count: u64,
    pub livemode: bool,
    pub metadata: Metadata,
    pub name: String,
    pub statement_descriptor: Option<String>,
    pub trial_period_days: Option<u64>,
}

impl Plan {
    pub fn create(client: &Client, params: PlanParams) -> Result<Plan, Error> {
        client.post("/plans", params)
    }

    pub fn get(client: &Client, plan_id: &str) -> Result<Plan, Error> {
        client.get(&format!("/plans/{}", plan_id))
    }

    pub fn update(client: &Client, plan_id: &str, params: PlanParams) -> Result<Plan, Error> {
        client.post(&format!("/plans/{}", plan_id), params)
    }

    pub fn delete(client: &Client, plan_id: &str) -> Result<Deleted, Error> {
        client.delete(&format!("/plans/{}", plan_id))
    }
}
