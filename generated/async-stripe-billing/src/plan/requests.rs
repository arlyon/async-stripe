use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deleting plans means new subscribers can’t be added. Existing subscribers aren’t affected.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeletePlan {
    plan: stripe_shared::PlanId,
}
impl DeletePlan {
    /// Construct a new `DeletePlan`.
    pub fn new(plan: impl Into<stripe_shared::PlanId>) -> Self {
        Self { plan: plan.into() }
    }
}
impl DeletePlan {
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

impl StripeRequest for DeletePlan {
    type Output = stripe_shared::DeletedPlan;

    fn build(&self) -> RequestBuilder {
        let plan = &self.plan;
        RequestBuilder::new(StripeMethod::Delete, format!("/plans/{plan}"))
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ListPlanBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListPlanBuilder {
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
pub struct ListPlan {
    inner: ListPlanBuilder,
}
impl ListPlan {
    /// Construct a new `ListPlan`.
    pub fn new() -> Self {
        Self { inner: ListPlanBuilder::new() }
    }
    /// Only return plans that are active or inactive (e.g., pass `false` to list all inactive plans).
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// Only return plans for the given product.
    pub fn product(mut self, product: impl Into<String>) -> Self {
        self.inner.product = Some(product.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListPlan {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPlan {
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
        stripe_client_core::ListPaginator::new_list("/plans", &self.inner)
    }
}

impl StripeRequest for ListPlan {
    type Output = stripe_types::List<stripe_shared::Plan>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/plans").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrievePlanBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrievePlanBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the plan with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePlan {
    inner: RetrievePlanBuilder,
    plan: stripe_shared::PlanId,
}
impl RetrievePlan {
    /// Construct a new `RetrievePlan`.
    pub fn new(plan: impl Into<stripe_shared::PlanId>) -> Self {
        Self { plan: plan.into(), inner: RetrievePlanBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrievePlan {
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

impl StripeRequest for RetrievePlan {
    type Output = stripe_shared::Plan;

    fn build(&self) -> RequestBuilder {
        let plan = &self.plan;
        RequestBuilder::new(StripeMethod::Get, format!("/plans/{plan}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreatePlanBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_decimal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_scheme: Option<stripe_shared::PlanBillingScheme>,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    interval: stripe_shared::PlanInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<CreatePlanProduct>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers: Option<Vec<CreatePlanTiers>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers_mode: Option<stripe_shared::PlanTiersMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transform_usage: Option<CreatePlanTransformUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_period_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_type: Option<stripe_shared::PlanUsageType>,
}
impl CreatePlanBuilder {
    fn new(
        currency: impl Into<stripe_types::Currency>,
        interval: impl Into<stripe_shared::PlanInterval>,
    ) -> Self {
        Self {
            active: None,
            amount: None,
            amount_decimal: None,
            billing_scheme: None,
            currency: currency.into(),
            expand: None,
            id: None,
            interval: interval.into(),
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
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePlanProduct {
    #[serde(untagged)]
    InlineProductParams(CreatePlanInlineProductParams),
    #[serde(untagged)]
    Id(String),
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePlanInlineProductParams {
    /// Whether the product is currently available for purchase. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The identifier for the product.
    /// Must be unique.
    /// If not provided, an identifier will be randomly generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    pub name: String,
    /// An arbitrary string to be displayed on your customer's credit card or bank statement.
    /// While most banks display this information consistently, some may display it incorrectly or not at all.
    ///
    /// This may be up to 22 characters.
    /// The statement description may not include `<`, `>`, `\`, `"`, `'` characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    /// A [tax code](https://docs.stripe.com/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,
}
impl CreatePlanInlineProductParams {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            active: None,
            id: None,
            metadata: None,
            name: name.into(),
            statement_descriptor: None,
            tax_code: None,
            unit_label: None,
        }
    }
}
/// Each element represents a pricing tier.
/// This parameter requires `billing_scheme` to be set to `tiered`.
/// See also the documentation for `billing_scheme`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePlanTiers {
    /// The flat billing amount for an entire tier, regardless of the number of units in the tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<i64>,
    /// Same as `flat_amount`, but accepts a decimal value representing an integer in the minor units of the currency.
    /// Only one of `flat_amount` and `flat_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount_decimal: Option<String>,
    /// The per unit billing amount for each individual unit for which this tier applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
    /// Specifies the upper bound of this tier.
    /// The lower bound of a tier is the upper bound of the previous tier adding one.
    /// Use `inf` to define a fallback tier.
    pub up_to: CreatePlanTiersUpTo,
}
impl CreatePlanTiers {
    pub fn new(up_to: impl Into<CreatePlanTiersUpTo>) -> Self {
        Self {
            flat_amount: None,
            flat_amount_decimal: None,
            unit_amount: None,
            unit_amount_decimal: None,
            up_to: up_to.into(),
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePlanTransformUsage {
    /// Divide usage by this number.
    pub divide_by: i64,
    /// After division, either round the result `up` or `down`.
    pub round: CreatePlanTransformUsageRound,
}
impl CreatePlanTransformUsage {
    pub fn new(divide_by: impl Into<i64>, round: impl Into<CreatePlanTransformUsageRound>) -> Self {
        Self { divide_by: divide_by.into(), round: round.into() }
    }
}
/// After division, either round the result `up` or `down`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePlanTransformUsageRound {
    Down,
    Up,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePlanTransformUsageRound {
    pub fn as_str(&self) -> &str {
        use CreatePlanTransformUsageRound::*;
        match self {
            Down => "down",
            Up => "up",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePlanTransformUsageRound {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePlanTransformUsageRound::*;
        match s {
            "down" => Ok(Down),
            "up" => Ok(Up),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePlanTransformUsageRound"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// You can now model subscriptions more flexibly using the [Prices API](https://stripe.com/docs/api#prices).
/// It replaces the Plans API and is backwards compatible to simplify your migration.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePlan {
    inner: CreatePlanBuilder,
}
impl CreatePlan {
    /// Construct a new `CreatePlan`.
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        interval: impl Into<stripe_shared::PlanInterval>,
    ) -> Self {
        Self { inner: CreatePlanBuilder::new(currency.into(), interval.into()) }
    }
    /// Whether the plan is currently available for new subscriptions. Defaults to `true`.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// A positive integer in cents (or local equivalent) (or 0 for a free plan) representing how much to charge on a recurring basis.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// Same as `amount`, but accepts a decimal value with at most 12 decimal places.
    /// Only one of `amount` and `amount_decimal` can be set.
    pub fn amount_decimal(mut self, amount_decimal: impl Into<String>) -> Self {
        self.inner.amount_decimal = Some(amount_decimal.into());
        self
    }
    /// Describes how to compute the price per period.
    /// Either `per_unit` or `tiered`.
    /// `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`).
    /// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    pub fn billing_scheme(
        mut self,
        billing_scheme: impl Into<stripe_shared::PlanBillingScheme>,
    ) -> Self {
        self.inner.billing_scheme = Some(billing_scheme.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// An identifier randomly generated by Stripe.
    /// Used to identify this plan when subscribing a customer.
    /// You can optionally override this ID, but the ID must be unique across all plans in your Stripe account.
    /// You can, however, use the same plan ID in both live and test modes.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.inner.id = Some(id.into());
        self
    }
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    pub fn interval_count(mut self, interval_count: impl Into<u64>) -> Self {
        self.inner.interval_count = Some(interval_count.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The meter tracking the usage of a metered price
    pub fn meter(mut self, meter: impl Into<String>) -> Self {
        self.inner.meter = Some(meter.into());
        self
    }
    /// A brief description of the plan, hidden from customers.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.inner.nickname = Some(nickname.into());
        self
    }
    pub fn product(mut self, product: impl Into<CreatePlanProduct>) -> Self {
        self.inner.product = Some(product.into());
        self
    }
    /// Each element represents a pricing tier.
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    pub fn tiers(mut self, tiers: impl Into<Vec<CreatePlanTiers>>) -> Self {
        self.inner.tiers = Some(tiers.into());
        self
    }
    /// Defines if the tiering price should be `graduated` or `volume` based.
    /// In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows.
    pub fn tiers_mode(mut self, tiers_mode: impl Into<stripe_shared::PlanTiersMode>) -> Self {
        self.inner.tiers_mode = Some(tiers_mode.into());
        self
    }
    /// Apply a transformation to the reported usage or set quantity before computing the billed price.
    /// Cannot be combined with `tiers`.
    pub fn transform_usage(mut self, transform_usage: impl Into<CreatePlanTransformUsage>) -> Self {
        self.inner.transform_usage = Some(transform_usage.into());
        self
    }
    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://docs.stripe.com/api#create_subscription-trial_from_plan).
    pub fn trial_period_days(mut self, trial_period_days: impl Into<u32>) -> Self {
        self.inner.trial_period_days = Some(trial_period_days.into());
        self
    }
    /// Configures how the quantity per period should be determined.
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    pub fn usage_type(mut self, usage_type: impl Into<stripe_shared::PlanUsageType>) -> Self {
        self.inner.usage_type = Some(usage_type.into());
        self
    }
}
impl CreatePlan {
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

impl StripeRequest for CreatePlan {
    type Output = stripe_shared::Plan;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/plans").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdatePlanBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_period_days: Option<u32>,
}
impl UpdatePlanBuilder {
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
pub struct UpdatePlan {
    inner: UpdatePlanBuilder,
    plan: stripe_shared::PlanId,
}
impl UpdatePlan {
    /// Construct a new `UpdatePlan`.
    pub fn new(plan: impl Into<stripe_shared::PlanId>) -> Self {
        Self { plan: plan.into(), inner: UpdatePlanBuilder::new() }
    }
    /// Whether the plan is currently available for new subscriptions.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// A brief description of the plan, hidden from customers.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.inner.nickname = Some(nickname.into());
        self
    }
    /// The product the plan belongs to.
    /// This cannot be changed once it has been used in a subscription or subscription schedule.
    pub fn product(mut self, product: impl Into<String>) -> Self {
        self.inner.product = Some(product.into());
        self
    }
    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://docs.stripe.com/api#create_subscription-trial_from_plan).
    pub fn trial_period_days(mut self, trial_period_days: impl Into<u32>) -> Self {
        self.inner.trial_period_days = Some(trial_period_days.into());
        self
    }
}
impl UpdatePlan {
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

impl StripeRequest for UpdatePlan {
    type Output = stripe_shared::Plan;

    fn build(&self) -> RequestBuilder {
        let plan = &self.plan;
        RequestBuilder::new(StripeMethod::Post, format!("/plans/{plan}")).form(&self.inner)
    }
}
