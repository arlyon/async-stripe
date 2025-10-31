use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListBillingCreditGrantBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListBillingCreditGrantBuilder {
    fn new() -> Self {
        Self {
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Retrieve a list of credit grants.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListBillingCreditGrant {
    inner: ListBillingCreditGrantBuilder,
}
impl ListBillingCreditGrant {
    /// Construct a new `ListBillingCreditGrant`.
    pub fn new() -> Self {
        Self { inner: ListBillingCreditGrantBuilder::new() }
    }
    /// Only return credit grants for this customer.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListBillingCreditGrant {
    fn default() -> Self {
        Self::new()
    }
}
impl ListBillingCreditGrant {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::BillingCreditGrant>>
    {
        stripe_client_core::ListPaginator::new_list("/billing/credit_grants", &self.inner)
    }
}

impl StripeRequest for ListBillingCreditGrant {
    type Output = stripe_types::List<stripe_shared::BillingCreditGrant>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/billing/credit_grants").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveBillingCreditGrantBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveBillingCreditGrantBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a credit grant.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveBillingCreditGrant {
    inner: RetrieveBillingCreditGrantBuilder,
    id: stripe_shared::BillingCreditGrantId,
}
impl RetrieveBillingCreditGrant {
    /// Construct a new `RetrieveBillingCreditGrant`.
    pub fn new(id: impl Into<stripe_shared::BillingCreditGrantId>) -> Self {
        Self { id: id.into(), inner: RetrieveBillingCreditGrantBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveBillingCreditGrant {
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

impl StripeRequest for RetrieveBillingCreditGrant {
    type Output = stripe_shared::BillingCreditGrant;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/billing/credit_grants/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateBillingCreditGrantBuilder {
    amount: CreateBillingCreditGrantAmount,
    applicability_config: CreateBillingCreditGrantApplicabilityConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<stripe_shared::BillingCreditGrantCategory>,
    customer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<i64>,
}
impl CreateBillingCreditGrantBuilder {
    fn new(
        amount: impl Into<CreateBillingCreditGrantAmount>,
        applicability_config: impl Into<CreateBillingCreditGrantApplicabilityConfig>,
        customer: impl Into<String>,
    ) -> Self {
        Self {
            amount: amount.into(),
            applicability_config: applicability_config.into(),
            category: None,
            customer: customer.into(),
            effective_at: None,
            expand: None,
            expires_at: None,
            metadata: None,
            name: None,
            priority: None,
        }
    }
}
/// Amount of this credit grant.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingCreditGrantAmount {
    /// The monetary amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monetary: Option<CreateBillingCreditGrantAmountMonetary>,
    /// The type of this amount. We currently only support `monetary` billing credits.
    #[serde(rename = "type")]
    pub type_: CreateBillingCreditGrantAmountType,
}
impl CreateBillingCreditGrantAmount {
    pub fn new(type_: impl Into<CreateBillingCreditGrantAmountType>) -> Self {
        Self { monetary: None, type_: type_.into() }
    }
}
/// The monetary amount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingCreditGrantAmountMonetary {
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `value` parameter.
    pub currency: stripe_types::Currency,
    /// A positive integer representing the amount of the credit grant.
    pub value: i64,
}
impl CreateBillingCreditGrantAmountMonetary {
    pub fn new(currency: impl Into<stripe_types::Currency>, value: impl Into<i64>) -> Self {
        Self { currency: currency.into(), value: value.into() }
    }
}
/// The type of this amount. We currently only support `monetary` billing credits.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingCreditGrantAmountType {
    Monetary,
}
impl CreateBillingCreditGrantAmountType {
    pub fn as_str(self) -> &'static str {
        use CreateBillingCreditGrantAmountType::*;
        match self {
            Monetary => "monetary",
        }
    }
}

impl std::str::FromStr for CreateBillingCreditGrantAmountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingCreditGrantAmountType::*;
        match s {
            "monetary" => Ok(Monetary),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateBillingCreditGrantAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingCreditGrantAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingCreditGrantAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateBillingCreditGrantAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateBillingCreditGrantAmountType")
        })
    }
}
/// Configuration specifying what this credit grant applies to.
/// We currently only support `metered` prices that have a [Billing Meter](https://docs.stripe.com/api/billing/meter) attached to them.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingCreditGrantApplicabilityConfig {
    /// Specify the scope of this applicability config.
    pub scope: CreateBillingCreditGrantApplicabilityConfigScope,
}
impl CreateBillingCreditGrantApplicabilityConfig {
    pub fn new(scope: impl Into<CreateBillingCreditGrantApplicabilityConfigScope>) -> Self {
        Self { scope: scope.into() }
    }
}
/// Specify the scope of this applicability config.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingCreditGrantApplicabilityConfigScope {
    /// The price type that credit grants can apply to.
    /// We currently only support the `metered` price type.
    /// Cannot be used in combination with `prices`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_type: Option<CreateBillingCreditGrantApplicabilityConfigScopePriceType>,
    /// A list of prices that the credit grant can apply to.
    /// We currently only support the `metered` prices.
    /// Cannot be used in combination with `price_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<CreateBillingCreditGrantApplicabilityConfigScopePrices>>,
}
impl CreateBillingCreditGrantApplicabilityConfigScope {
    pub fn new() -> Self {
        Self { price_type: None, prices: None }
    }
}
impl Default for CreateBillingCreditGrantApplicabilityConfigScope {
    fn default() -> Self {
        Self::new()
    }
}
/// The price type that credit grants can apply to.
/// We currently only support the `metered` price type.
/// Cannot be used in combination with `prices`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingCreditGrantApplicabilityConfigScopePriceType {
    Metered,
}
impl CreateBillingCreditGrantApplicabilityConfigScopePriceType {
    pub fn as_str(self) -> &'static str {
        use CreateBillingCreditGrantApplicabilityConfigScopePriceType::*;
        match self {
            Metered => "metered",
        }
    }
}

impl std::str::FromStr for CreateBillingCreditGrantApplicabilityConfigScopePriceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingCreditGrantApplicabilityConfigScopePriceType::*;
        match s {
            "metered" => Ok(Metered),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateBillingCreditGrantApplicabilityConfigScopePriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingCreditGrantApplicabilityConfigScopePriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingCreditGrantApplicabilityConfigScopePriceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateBillingCreditGrantApplicabilityConfigScopePriceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateBillingCreditGrantApplicabilityConfigScopePriceType",
            )
        })
    }
}
/// A list of prices that the credit grant can apply to.
/// We currently only support the `metered` prices.
/// Cannot be used in combination with `price_type`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingCreditGrantApplicabilityConfigScopePrices {
    /// The price ID this credit grant should apply to.
    pub id: String,
}
impl CreateBillingCreditGrantApplicabilityConfigScopePrices {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
/// Creates a credit grant.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingCreditGrant {
    inner: CreateBillingCreditGrantBuilder,
}
impl CreateBillingCreditGrant {
    /// Construct a new `CreateBillingCreditGrant`.
    pub fn new(
        amount: impl Into<CreateBillingCreditGrantAmount>,
        applicability_config: impl Into<CreateBillingCreditGrantApplicabilityConfig>,
        customer: impl Into<String>,
    ) -> Self {
        Self {
            inner: CreateBillingCreditGrantBuilder::new(
                amount.into(),
                applicability_config.into(),
                customer.into(),
            ),
        }
    }
    /// The category of this credit grant. It defaults to `paid` if not specified.
    pub fn category(
        mut self,
        category: impl Into<stripe_shared::BillingCreditGrantCategory>,
    ) -> Self {
        self.inner.category = Some(category.into());
        self
    }
    /// The time when the billing credits become effective-when they're eligible for use.
    /// It defaults to the current timestamp if not specified.
    pub fn effective_at(mut self, effective_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.effective_at = Some(effective_at.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The time when the billing credits expire. If not specified, the billing credits don't expire.
    pub fn expires_at(mut self, expires_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.expires_at = Some(expires_at.into());
        self
    }
    /// Set of key-value pairs that you can attach to an object.
    /// You can use this to store additional information about the object (for example, cost basis) in a structured format.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// A descriptive name shown in the Dashboard.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
    /// The desired priority for applying this credit grant.
    /// If not specified, it will be set to the default value of 50.
    /// The highest priority is 0 and the lowest is 100.
    pub fn priority(mut self, priority: impl Into<i64>) -> Self {
        self.inner.priority = Some(priority.into());
        self
    }
}
impl CreateBillingCreditGrant {
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

impl StripeRequest for CreateBillingCreditGrant {
    type Output = stripe_shared::BillingCreditGrant;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/billing/credit_grants").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateBillingCreditGrantBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl UpdateBillingCreditGrantBuilder {
    fn new() -> Self {
        Self { expand: None, expires_at: None, metadata: None }
    }
}
/// Updates a credit grant.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingCreditGrant {
    inner: UpdateBillingCreditGrantBuilder,
    id: stripe_shared::BillingCreditGrantId,
}
impl UpdateBillingCreditGrant {
    /// Construct a new `UpdateBillingCreditGrant`.
    pub fn new(id: impl Into<stripe_shared::BillingCreditGrantId>) -> Self {
        Self { id: id.into(), inner: UpdateBillingCreditGrantBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The time when the billing credits created by this credit grant expire.
    /// If set to empty, the billing credits never expire.
    pub fn expires_at(mut self, expires_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.expires_at = Some(expires_at.into());
        self
    }
    /// Set of key-value pairs you can attach to an object.
    /// You can use this to store additional information about the object (for example, cost basis) in a structured format.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl UpdateBillingCreditGrant {
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

impl StripeRequest for UpdateBillingCreditGrant {
    type Output = stripe_shared::BillingCreditGrant;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/credit_grants/{id}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ExpireBillingCreditGrantBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl ExpireBillingCreditGrantBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Expires a credit grant.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ExpireBillingCreditGrant {
    inner: ExpireBillingCreditGrantBuilder,
    id: stripe_shared::BillingCreditGrantId,
}
impl ExpireBillingCreditGrant {
    /// Construct a new `ExpireBillingCreditGrant`.
    pub fn new(id: impl Into<stripe_shared::BillingCreditGrantId>) -> Self {
        Self { id: id.into(), inner: ExpireBillingCreditGrantBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ExpireBillingCreditGrant {
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

impl StripeRequest for ExpireBillingCreditGrant {
    type Output = stripe_shared::BillingCreditGrant;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/credit_grants/{id}/expire"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct VoidGrantBillingCreditGrantBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl VoidGrantBillingCreditGrantBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Voids a credit grant.
#[derive(Clone, Debug, serde::Serialize)]
pub struct VoidGrantBillingCreditGrant {
    inner: VoidGrantBillingCreditGrantBuilder,
    id: stripe_shared::BillingCreditGrantId,
}
impl VoidGrantBillingCreditGrant {
    /// Construct a new `VoidGrantBillingCreditGrant`.
    pub fn new(id: impl Into<stripe_shared::BillingCreditGrantId>) -> Self {
        Self { id: id.into(), inner: VoidGrantBillingCreditGrantBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl VoidGrantBillingCreditGrant {
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

impl StripeRequest for VoidGrantBillingCreditGrant {
    type Output = stripe_shared::BillingCreditGrant;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/credit_grants/{id}/void"))
            .form(&self.inner)
    }
}
