// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::params::{Deleted, Expand, IdOrCreate, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::CreateProduct;
use crate::Plan;

impl Plan {
    /// Returns a list of your plans.
    pub fn list(client: &Client, params: ListPlans<'_>) -> Response<List<Plan>> {
        client.get_query("/plans", &params)
    }

    /// Retrieves the plan with the given ID.
    pub fn retrieve(client: &Client, id: &PlanId, expand: &[&str]) -> Response<Plan> {
        client.get_query(&format!("/plans/{}", id), &Expand { expand })
    }

    /// Updates the specified plan by setting the values of the parameters passed.
    ///
    /// Any parameters not provided are left unchanged.
    /// By design, you cannot change a plan’s ID, amount, currency, or billing cycle.
    pub fn update(client: &Client, id: &PlanId, params: UpdatePlan<'_>) -> Response<Plan> {
        client.post_form(&format!("/plans/{}", id), &params)
    }

    /// Deleting plans means new subscribers can’t be added.
    ///
    /// Existing subscribers aren’t affected.
    pub fn delete(client: &Client, id: &PlanId) -> Response<Deleted<PlanId>> {
        client.delete(&format!("/plans/{}", id))
    }
}

impl Object for Plan {
    type Id = PlanId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "plan"
    }
}

// written at 597
/// The parameters for `Plan::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListPlans<'a> {
    /// Only return plans that are active or inactive (e.g., pass `false` to list all inactive plans).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<PlanId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return plans for the given product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<IdOrCreate<'a, CreateProduct<'a>>>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<PlanId>,
}

impl<'a> ListPlans<'a> {
    pub fn new() -> Self {
        ListPlans {
            active: Default::default(),
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            product: Default::default(),
            starting_after: Default::default(),
        }
    }
}

// written at 597
/// The parameters for `Plan::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdatePlan<'a> {
    /// Whether the plan is currently available for new subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// A brief description of the plan, hidden from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<&'a str>,

    /// The product the plan belongs to.
    ///
    /// This cannot be changed once it has been used in a subscription or subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<IdOrCreate<'a, CreateProduct<'a>>>,

    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}

impl<'a> UpdatePlan<'a> {
    pub fn new() -> Self {
        UpdatePlan {
            active: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            nickname: Default::default(),
            product: Default::default(),
            trial_period_days: Default::default(),
        }
    }
}
