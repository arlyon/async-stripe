// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::PlanId;
use crate::params::{Deleted, Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{Currency, Product, UpTo};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Plan".
///
/// For more details see [https://stripe.com/docs/api/plans/object](https://stripe.com/docs/api/plans/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plan {
    /// Unique identifier for the object.
    pub id: PlanId,

    /// Whether the plan is currently available for new subscriptions.
    #[serde(default)]
    pub active: bool,

    /// Specifies a usage aggregation strategy for plans of `usage_type=metered`.
    ///
    /// Allowed values are `sum` for summing up all usage during a period, `last_during_period` for picking the last usage record reported within a period, `last_ever` for picking the last usage record ever (across period bounds) or `max` which picks the usage record with the maximum reported usage during a period.
    /// Defaults to `sum`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<PlanAggregateUsage>,

    /// The amount in %s to be charged on the interval specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Describes how to compute the price per period.
    ///
    /// Either `per_unit` or `tiered`.
    /// `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`).
    /// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_scheme: Option<PlanBillingScheme>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    // Always true for a deleted object
    #[serde(default)]
    deleted: bool,

    /// One of `day`, `week`, `month` or `year`.
    ///
    /// The frequency with which a subscription should be billed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<PlanInterval>,

    /// The number of intervals (specified in the `interval` property) between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(default)]
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// A brief description of the plan, hidden from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// The product whose pricing this plan determines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Expandable<Product>>,

    /// Each element represents a pricing tier.
    ///
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PlanTier>>,

    /// Defines if the tiering price should be `graduated` or `volume` based.
    ///
    /// In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<PlanTiersMode>,

    /// Apply a transformation to the reported usage or set quantity before computing the billed price.
    ///
    /// Cannot be combined with `tiers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_usage: Option<TransformUsage>,

    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,

    /// Configures how the quantity per period should be determined, can be either `metered` or `licensed`.
    ///
    /// `licensed` will automatically bill the `quantity` set for a plan when adding it to a subscription, `metered` will aggregate the total usage based on usage records.
    /// Defaults to `licensed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<PlanUsageType>,
}

impl Plan {
    /// Returns a list of your plans.
    pub fn list(client: &Client, params: ListPlans<'_>) -> Response<List<Plan>> {
        client.get_query("/plans", &params)
    }

    /// You can create plans using the API, or in the Stripe [Dashboard](https://dashboard.stripe.com/subscriptions/products).
    pub fn create(client: &Client, params: CreatePlan<'_>) -> Response<Plan> {
        client.post_form("/plans", &params)
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlanTier {
    /// Price for the entire tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<i64>,

    /// Per unit price for units relevant to the tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Up to and including to this quantity will be contained in the tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_to: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransformUsage {
    /// Divide usage by this number.
    pub divide_by: i64,

    /// After division, either round the result `up` or `down`.
    pub round: TransformUsageRound,
}

/// The parameters for `Plan::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreatePlan<'a> {
    /// Whether the plan is currently available for new subscriptions.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,

    /// Specifies a usage aggregation strategy for plans of `usage_type=metered`.
    ///
    /// Allowed values are `sum` for summing up all usage during a period, `last_during_period` for picking the last usage record reported within a period, `last_ever` for picking the last usage record ever (across period bounds) or `max` which picks the usage record with the maximum reported usage during a period.
    /// Defaults to `sum`.
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregate_usage: Option<PlanAggregateUsage>,

    /// A positive integer in %s (or 0 for a free plan) representing how much to charge on a recurring basis.
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,

    /// Describes how to compute the price per period.
    ///
    /// Either `per_unit` or `tiered`.
    /// `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`).
    /// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_scheme: Option<PlanBillingScheme>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    currency: Currency,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// An identifier randomly generated by Stripe.
    ///
    /// Used to identify this plan when subscribing a customer.
    /// You can optionally override this ID, but the ID must be unique across all plans in your Stripe account.
    /// You can, however, use the same plan ID in both live and test modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<&'a str>,

    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    interval: PlanInterval,

    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_count: Option<u64>,

    /// A set of key-value pairs that you can attach to a plan object.
    ///
    /// It can be useful for storing additional information about the plan in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Metadata>,

    /// A brief description of the plan, hidden from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers: Option<Vec<CreatePlanTiers>>,

    /// Defines if the tiering price should be `graduated` or `volume` based.
    ///
    /// In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows.
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers_mode: Option<PlanTiersMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transform_usage: Option<CreatePlanTransformUsage>,

    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_period_days: Option<u32>,

    /// Configures how the quantity per period should be determined, can be either `metered` or `licensed`.
    ///
    /// `licensed` will automatically bill the `quantity` set for a plan when adding it to a subscription, `metered` will aggregate the total usage based on usage records.
    /// Defaults to `licensed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_type: Option<PlanUsageType>,
}

impl<'a> CreatePlan<'a> {
    pub fn new(currency: Currency, interval: PlanInterval) -> Self {
        CreatePlan {
            active: Default::default(),
            aggregate_usage: Default::default(),
            amount: Default::default(),
            billing_scheme: Default::default(),
            currency,
            expand: Default::default(),
            id: Default::default(),
            interval,
            interval_count: Default::default(),
            metadata: Default::default(),
            nickname: Default::default(),
            tiers: Default::default(),
            tiers_mode: Default::default(),
            transform_usage: Default::default(),
            trial_period_days: Default::default(),
            usage_type: Default::default(),
        }
    }
}

/// The parameters for `Plan::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListPlans<'a> {
    /// Only return plans that are active or inactive (e.g., pass `false` to list all inactive products).
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,

    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a PlanId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a PlanId>,
}

impl<'a> ListPlans<'a> {
    pub fn new() -> Self {
        ListPlans {
            active: Default::default(),
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// The parameters for `Plan::update`.
#[derive(Clone, Debug, Serialize)]
pub struct UpdatePlan<'a> {
    /// Whether the plan is currently available for new subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A set of key-value pairs that you can attach to a plan object.
    ///
    /// It can be useful for storing additional information about the plan in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Metadata>,

    /// A brief description of the plan, hidden from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<&'a str>,

    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_period_days: Option<u32>,
}

impl<'a> UpdatePlan<'a> {
    pub fn new() -> Self {
        UpdatePlan {
            active: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            nickname: Default::default(),
            trial_period_days: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePlanTiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    pub up_to: Option<UpTo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePlanTransformUsage {
    pub divide_by: i64,

    pub round: CreatePlanTransformUsageRound,
}

/// An enum representing the possible values of an `CreatePlanTransformUsage`'s `round` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePlanTransformUsageRound {
    Down,
    Up,
}

/// An enum representing the possible values of an `Plan`'s `aggregate_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlanAggregateUsage {
    LastDuringPeriod,
    LastEver,
    Max,
    Sum,
}

/// An enum representing the possible values of an `Plan`'s `billing_scheme` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlanBillingScheme {
    PerUnit,
    Tiered,
}

/// An enum representing the possible values of an `Plan`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlanInterval {
    Day,
    Month,
    Week,
    Year,
}

/// An enum representing the possible values of an `Plan`'s `tiers_mode` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlanTiersMode {
    Graduated,
    Volume,
}

/// An enum representing the possible values of an `Plan`'s `usage_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlanUsageType {
    Licensed,
    Metered,
}

/// An enum representing the possible values of an `TransformUsage`'s `round` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransformUsageRound {
    Down,
    Up,
}
