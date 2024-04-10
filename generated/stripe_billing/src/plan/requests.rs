#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeletePlan {}
impl DeletePlan {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeletePlan {
    /// Deleting plans means new subscribers can’t be added. Existing subscribers aren’t affected.
    pub fn send(
        &self,
        client: &stripe::Client,
        plan: &stripe_shared::PlanId,
    ) -> stripe::Response<stripe_shared::DeletedPlan> {
        client.send_form(&format!("/plans/{plan}"), self, http_types::Method::Delete)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListPlan<'a> {
    /// Only return plans that are active or inactive (e.g., pass `false` to list all inactive plans).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return plans for the given product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListPlan<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListPlan<'a> {
    /// Returns a list of your plans.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_shared::Plan>> {
        client.get_query("/plans", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::List<stripe_shared::Plan>> {
        stripe::ListPaginator::from_list_params("/plans", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePlan<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePlan<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrievePlan<'a> {
    /// Retrieves the plan with the given ID.
    pub fn send(
        &self,
        client: &stripe::Client,
        plan: &stripe_shared::PlanId,
    ) -> stripe::Response<stripe_shared::Plan> {
        client.get_query(&format!("/plans/{plan}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePlan<'a> {
    /// Whether the plan is currently available for new subscriptions. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Specifies a usage aggregation strategy for plans of `usage_type=metered`.
    /// Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period.
    /// Defaults to `sum`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<stripe_shared::PlanAggregateUsage>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free plan) representing how much to charge on a recurring basis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Same as `amount`, but accepts a decimal value with at most 12 decimal places.
    /// Only one of `amount` and `amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_decimal: Option<&'a str>,
    /// Describes how to compute the price per period.
    /// Either `per_unit` or `tiered`.
    /// `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`).
    /// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_scheme: Option<stripe_shared::PlanBillingScheme>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// An identifier randomly generated by Stripe.
    /// Used to identify this plan when subscribing a customer.
    /// You can optionally override this ID, but the ID must be unique across all plans in your Stripe account.
    /// You can, however, use the same plan ID in both live and test modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: stripe_shared::PlanInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A brief description of the plan, hidden from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<CreatePlanProduct<'a>>,
    /// Each element represents a pricing tier.
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<&'a [CreatePlanTiers<'a>]>,
    /// Defines if the tiering price should be `graduated` or `volume` based.
    /// In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<stripe_shared::PlanTiersMode>,
    /// Apply a transformation to the reported usage or set quantity before computing the billed price.
    /// Cannot be combined with `tiers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_usage: Option<CreatePlanTransformUsage>,
    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
    /// Configures how the quantity per period should be determined.
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<stripe_shared::PlanUsageType>,
}
impl<'a> CreatePlan<'a> {
    pub fn new(currency: stripe_types::Currency, interval: stripe_shared::PlanInterval) -> Self {
        Self {
            active: None,
            aggregate_usage: None,
            amount: None,
            amount_decimal: None,
            billing_scheme: None,
            currency,
            expand: None,
            id: None,
            interval,
            interval_count: None,
            metadata: None,
            nickname: None,
            product: None,
            tiers: None,
            tiers_mode: None,
            transform_usage: None,
            trial_period_days: None,
            usage_type: None,
        }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum CreatePlanProduct<'a> {
    InlineProductParams(CreatePlanInlineProductParams<'a>),
    Id(&'a str),
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePlanInlineProductParams<'a> {
    /// Whether the product is currently available for purchase. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The identifier for the product.
    /// Must be unique.
    /// If not provided, an identifier will be randomly generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    pub name: &'a str,
    /// An arbitrary string to be displayed on your customer's credit card or bank statement.
    /// While most banks display this information consistently, some may display it incorrectly or not at all.
    ///
    /// This may be up to 22 characters.
    /// The statement description may not include `<`, `>`, `\`, `"`, `'` characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<&'a str>,
}
impl<'a> CreatePlanInlineProductParams<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            active: None,
            id: None,
            metadata: None,
            name,
            statement_descriptor: None,
            tax_code: None,
            unit_label: None,
        }
    }
}
/// Each element represents a pricing tier.
/// This parameter requires `billing_scheme` to be set to `tiered`.
/// See also the documentation for `billing_scheme`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePlanTiers<'a> {
    /// The flat billing amount for an entire tier, regardless of the number of units in the tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<i64>,
    /// Same as `flat_amount`, but accepts a decimal value representing an integer in the minor units of the currency.
    /// Only one of `flat_amount` and `flat_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount_decimal: Option<&'a str>,
    /// The per unit billing amount for each individual unit for which this tier applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
    /// Specifies the upper bound of this tier.
    /// The lower bound of a tier is the upper bound of the previous tier adding one.
    /// Use `inf` to define a fallback tier.
    pub up_to: CreatePlanTiersUpTo,
}
impl<'a> CreatePlanTiers<'a> {
    pub fn new(up_to: CreatePlanTiersUpTo) -> Self {
        Self {
            flat_amount: None,
            flat_amount_decimal: None,
            unit_amount: None,
            unit_amount_decimal: None,
            up_to,
        }
    }
}
/// Specifies the upper bound of this tier.
/// The lower bound of a tier is the upper bound of the previous tier adding one.
/// Use `inf` to define a fallback tier.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum CreatePlanTiersUpTo {
    Inf,
    I64(i64),
}
/// Apply a transformation to the reported usage or set quantity before computing the billed price.
/// Cannot be combined with `tiers`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePlanTransformUsage {
    /// Divide usage by this number.
    pub divide_by: i64,
    /// After division, either round the result `up` or `down`.
    pub round: CreatePlanTransformUsageRound,
}
impl CreatePlanTransformUsage {
    pub fn new(divide_by: i64, round: CreatePlanTransformUsageRound) -> Self {
        Self { divide_by, round }
    }
}
/// After division, either round the result `up` or `down`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePlanTransformUsageRound {
    Down,
    Up,
}
impl CreatePlanTransformUsageRound {
    pub fn as_str(self) -> &'static str {
        use CreatePlanTransformUsageRound::*;
        match self {
            Down => "down",
            Up => "up",
        }
    }
}

impl std::str::FromStr for CreatePlanTransformUsageRound {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePlanTransformUsageRound::*;
        match s {
            "down" => Ok(Down),
            "up" => Ok(Up),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreatePlanTransformUsageRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePlanTransformUsageRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePlanTransformUsageRound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePlanTransformUsageRound {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePlanTransformUsageRound")
        })
    }
}
impl<'a> CreatePlan<'a> {
    /// You can now model subscriptions more flexibly using the [Prices API](https://stripe.com/docs/api#prices).
    /// It replaces the Plans API and is backwards compatible to simplify your migration.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_shared::Plan> {
        client.send_form("/plans", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePlan<'a> {
    /// Whether the plan is currently available for new subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A brief description of the plan, hidden from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<&'a str>,
    /// The product the plan belongs to.
    /// This cannot be changed once it has been used in a subscription or subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}
impl<'a> UpdatePlan<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdatePlan<'a> {
    /// Updates the specified plan by setting the values of the parameters passed.
    /// Any parameters not provided are left unchanged.
    /// By design, you cannot change a plan’s ID, amount, currency, or billing cycle.
    pub fn send(
        &self,
        client: &stripe::Client,
        plan: &stripe_shared::PlanId,
    ) -> stripe::Response<stripe_shared::Plan> {
        client.send_form(&format!("/plans/{plan}"), self, http_types::Method::Post)
    }
}
