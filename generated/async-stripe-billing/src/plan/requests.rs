use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deleting plans means new subscribers can’t be added. Existing subscribers aren’t affected.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeletePlan<'a> {
    plan: &'a stripe_shared::PlanId,
}
impl<'a> DeletePlan<'a> {
    /// Construct a new `DeletePlan`.
    pub fn new(plan: &'a stripe_shared::PlanId) -> Self {
        Self { plan }
    }
}
impl DeletePlan<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for DeletePlan<'_> {
    type Output = stripe_shared::DeletedPlan;

    fn build(&self) -> RequestBuilder {
        let plan = self.plan;
        RequestBuilder::new(StripeMethod::Delete, format!("/plans/{plan}"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListPlanBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListPlanBuilder<'a> {
    fn new() -> Self {
        Self {
            active: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            product: None,
            starting_after: None,
        }
    }
}
/// Returns a list of your plans.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListPlan<'a> {
    inner: ListPlanBuilder<'a>,
}
impl<'a> ListPlan<'a> {
    /// Construct a new `ListPlan`.
    pub fn new() -> Self {
        Self { inner: ListPlanBuilder::new() }
    }
    /// Only return plans that are active or inactive (e.g., pass `false` to list all inactive plans).
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// Only return plans for the given product.
    pub fn product(mut self, product: &'a str) -> Self {
        self.inner.product = Some(product);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListPlan<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPlan<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Plan>> {
        stripe_client_core::ListPaginator::new_list("/plans", self.inner)
    }
}

impl StripeRequest for ListPlan<'_> {
    type Output = stripe_types::List<stripe_shared::Plan>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/plans").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrievePlanBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePlanBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the plan with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePlan<'a> {
    inner: RetrievePlanBuilder<'a>,
    plan: &'a stripe_shared::PlanId,
}
impl<'a> RetrievePlan<'a> {
    /// Construct a new `RetrievePlan`.
    pub fn new(plan: &'a stripe_shared::PlanId) -> Self {
        Self { plan, inner: RetrievePlanBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrievePlan<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrievePlan<'_> {
    type Output = stripe_shared::Plan;

    fn build(&self) -> RequestBuilder {
        let plan = self.plan;
        RequestBuilder::new(StripeMethod::Get, format!("/plans/{plan}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreatePlanBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregate_usage: Option<stripe_shared::PlanAggregateUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_decimal: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_scheme: Option<stripe_shared::PlanBillingScheme>,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<&'a str>,
    interval: stripe_shared::PlanInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meter: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<CreatePlanProduct<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers: Option<&'a [CreatePlanTiers<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers_mode: Option<stripe_shared::PlanTiersMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transform_usage: Option<CreatePlanTransformUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_period_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_type: Option<stripe_shared::PlanUsageType>,
}
impl<'a> CreatePlanBuilder<'a> {
    fn new(currency: stripe_types::Currency, interval: stripe_shared::PlanInterval) -> Self {
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
            meter: None,
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
#[serde(rename_all = "snake_case")]
pub enum CreatePlanProduct<'a> {
    #[serde(untagged)]
    InlineProductParams(CreatePlanInlineProductParams<'a>),
    #[serde(untagged)]
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
#[serde(rename_all = "snake_case")]
pub enum CreatePlanTiersUpTo {
    Inf,
    #[serde(untagged)]
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePlanTransformUsageRound::*;
        match s {
            "down" => Ok(Down),
            "up" => Ok(Up),
            _ => Err(stripe_types::StripeParseError),
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
/// You can now model subscriptions more flexibly using the [Prices API](https://stripe.com/docs/api#prices).
/// It replaces the Plans API and is backwards compatible to simplify your migration.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePlan<'a> {
    inner: CreatePlanBuilder<'a>,
}
impl<'a> CreatePlan<'a> {
    /// Construct a new `CreatePlan`.
    pub fn new(currency: stripe_types::Currency, interval: stripe_shared::PlanInterval) -> Self {
        Self { inner: CreatePlanBuilder::new(currency, interval) }
    }
    /// Whether the plan is currently available for new subscriptions. Defaults to `true`.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// Specifies a usage aggregation strategy for plans of `usage_type=metered`.
    /// Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period.
    /// Defaults to `sum`.
    pub fn aggregate_usage(mut self, aggregate_usage: stripe_shared::PlanAggregateUsage) -> Self {
        self.inner.aggregate_usage = Some(aggregate_usage);
        self
    }
    /// A positive integer in cents (or local equivalent) (or 0 for a free plan) representing how much to charge on a recurring basis.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// Same as `amount`, but accepts a decimal value with at most 12 decimal places.
    /// Only one of `amount` and `amount_decimal` can be set.
    pub fn amount_decimal(mut self, amount_decimal: &'a str) -> Self {
        self.inner.amount_decimal = Some(amount_decimal);
        self
    }
    /// Describes how to compute the price per period.
    /// Either `per_unit` or `tiered`.
    /// `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`).
    /// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    pub fn billing_scheme(mut self, billing_scheme: stripe_shared::PlanBillingScheme) -> Self {
        self.inner.billing_scheme = Some(billing_scheme);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// An identifier randomly generated by Stripe.
    /// Used to identify this plan when subscribing a customer.
    /// You can optionally override this ID, but the ID must be unique across all plans in your Stripe account.
    /// You can, however, use the same plan ID in both live and test modes.
    pub fn id(mut self, id: &'a str) -> Self {
        self.inner.id = Some(id);
        self
    }
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    pub fn interval_count(mut self, interval_count: u64) -> Self {
        self.inner.interval_count = Some(interval_count);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The meter tracking the usage of a metered price
    pub fn meter(mut self, meter: &'a str) -> Self {
        self.inner.meter = Some(meter);
        self
    }
    /// A brief description of the plan, hidden from customers.
    pub fn nickname(mut self, nickname: &'a str) -> Self {
        self.inner.nickname = Some(nickname);
        self
    }
    pub fn product(mut self, product: CreatePlanProduct<'a>) -> Self {
        self.inner.product = Some(product);
        self
    }
    /// Each element represents a pricing tier.
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    pub fn tiers(mut self, tiers: &'a [CreatePlanTiers<'a>]) -> Self {
        self.inner.tiers = Some(tiers);
        self
    }
    /// Defines if the tiering price should be `graduated` or `volume` based.
    /// In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows.
    pub fn tiers_mode(mut self, tiers_mode: stripe_shared::PlanTiersMode) -> Self {
        self.inner.tiers_mode = Some(tiers_mode);
        self
    }
    /// Apply a transformation to the reported usage or set quantity before computing the billed price.
    /// Cannot be combined with `tiers`.
    pub fn transform_usage(mut self, transform_usage: CreatePlanTransformUsage) -> Self {
        self.inner.transform_usage = Some(transform_usage);
        self
    }
    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    pub fn trial_period_days(mut self, trial_period_days: u32) -> Self {
        self.inner.trial_period_days = Some(trial_period_days);
        self
    }
    /// Configures how the quantity per period should be determined.
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    pub fn usage_type(mut self, usage_type: stripe_shared::PlanUsageType) -> Self {
        self.inner.usage_type = Some(usage_type);
        self
    }
}
impl CreatePlan<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreatePlan<'_> {
    type Output = stripe_shared::Plan;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/plans").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdatePlanBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_period_days: Option<u32>,
}
impl<'a> UpdatePlanBuilder<'a> {
    fn new() -> Self {
        Self {
            active: None,
            expand: None,
            metadata: None,
            nickname: None,
            product: None,
            trial_period_days: None,
        }
    }
}
/// Updates the specified plan by setting the values of the parameters passed.
/// Any parameters not provided are left unchanged.
/// By design, you cannot change a plan’s ID, amount, currency, or billing cycle.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePlan<'a> {
    inner: UpdatePlanBuilder<'a>,
    plan: &'a stripe_shared::PlanId,
}
impl<'a> UpdatePlan<'a> {
    /// Construct a new `UpdatePlan`.
    pub fn new(plan: &'a stripe_shared::PlanId) -> Self {
        Self { plan, inner: UpdatePlanBuilder::new() }
    }
    /// Whether the plan is currently available for new subscriptions.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// A brief description of the plan, hidden from customers.
    pub fn nickname(mut self, nickname: &'a str) -> Self {
        self.inner.nickname = Some(nickname);
        self
    }
    /// The product the plan belongs to.
    /// This cannot be changed once it has been used in a subscription or subscription schedule.
    pub fn product(mut self, product: &'a str) -> Self {
        self.inner.product = Some(product);
        self
    }
    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    pub fn trial_period_days(mut self, trial_period_days: u32) -> Self {
        self.inner.trial_period_days = Some(trial_period_days);
        self
    }
}
impl UpdatePlan<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdatePlan<'_> {
    type Output = stripe_shared::Plan;

    fn build(&self) -> RequestBuilder {
        let plan = self.plan;
        RequestBuilder::new(StripeMethod::Post, format!("/plans/{plan}")).form(&self.inner)
    }
}
