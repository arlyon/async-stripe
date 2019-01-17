use crate::config::{Client, Response};
use crate::params::{Identifiable, Metadata, Timestamp};
use crate::resources::{Currency, Deleted};
use serde_derive::{Deserialize, Serialize};

/// The set of parameters that can be used when creating or updating a plan.
///
/// For more details see https://stripe.com/docs/api#create_plan and https://stripe.com/docs/api#update_plan.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlanParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<&'a str>, // (day, week, month, year)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u64>,
}

/// The resource representing a Stripe plan.
///
/// For more details see https://stripe.com/docs/api#plans.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plan {
    pub id: String,
    pub amount: u64,
    pub created: Timestamp,
    pub currency: Currency,
    pub interval: String, // (day, week, month, year)
    pub interval_count: u64,
    pub livemode: bool,
    pub metadata: Metadata,
    pub nickname: Option<String>,
    pub statement_descriptor: Option<String>,
    pub trial_period_days: Option<u64>,
}

impl Plan {
    /// Creates a new plan.
    ///
    /// For more details see https://stripe.com/docs/api#create_plan.
    pub fn create(client: &Client, params: PlanParams<'_>) -> Response<Plan> {
        client.post_form("/plans", params)
    }

    /// Retrieves the details of a plan.
    ///
    /// For more details see https://stripe.com/docs/api#retrieve_plan.
    pub fn retrieve(client: &Client, plan_id: &str) -> Response<Plan> {
        client.get(&format!("/plans/{}", plan_id))
    }

    /// Updates a plan's properties.
    ///
    /// For more details see https://stripe.com/docs/api#update_plan.
    pub fn update(client: &Client, plan_id: &str, params: PlanParams<'_>) -> Response<Plan> {
        client.post_form(&format!("/plans/{}", plan_id), params)
    }

    /// Deletes a plan.
    ///
    /// For more details see https://stripe.com/docs/api#delete_plan.
    pub fn delete(client: &Client, plan_id: &str) -> Response<Deleted> {
        client.delete(&format!("/plans/{}", plan_id))
    }
}

impl Identifiable for Plan {
    fn id(&self) -> &str {
        &self.id
    }
}
