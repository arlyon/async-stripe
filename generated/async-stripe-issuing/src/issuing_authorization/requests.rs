use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListIssuingAuthorizationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cardholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::IssuingAuthorizationStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIssuingAuthorizationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIssuingAuthorizationBuilder").finish_non_exhaustive()
    }
}
impl ListIssuingAuthorizationBuilder {
    fn new() -> Self {
        Self {
            card: None,
            cardholder: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Returns a list of Issuing `Authorization` objects.
/// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListIssuingAuthorization {
    inner: ListIssuingAuthorizationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIssuingAuthorization").finish_non_exhaustive()
    }
}
impl ListIssuingAuthorization {
    /// Construct a new `ListIssuingAuthorization`.
    pub fn new() -> Self {
        Self { inner: ListIssuingAuthorizationBuilder::new() }
    }
    /// Only return authorizations that belong to the given card.
    pub fn card(mut self, card: impl Into<String>) -> Self {
        self.inner.card = Some(card.into());
        self
    }
    /// Only return authorizations that belong to the given cardholder.
    pub fn cardholder(mut self, cardholder: impl Into<String>) -> Self {
        self.inner.cardholder = Some(cardholder.into());
        self
    }
    /// Only return authorizations that were created during the given date interval.
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return authorizations with the given status. One of `pending`, `closed`, or `reversed`.
    pub fn status(mut self, status: impl Into<stripe_shared::IssuingAuthorizationStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListIssuingAuthorization {
    fn default() -> Self {
        Self::new()
    }
}
impl ListIssuingAuthorization {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::IssuingAuthorization>>
    {
        stripe_client_core::ListPaginator::new_list("/issuing/authorizations", &self.inner)
    }
}

impl StripeRequest for ListIssuingAuthorization {
    type Output = stripe_types::List<stripe_shared::IssuingAuthorization>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/issuing/authorizations").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveIssuingAuthorizationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIssuingAuthorizationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIssuingAuthorizationBuilder").finish_non_exhaustive()
    }
}
impl RetrieveIssuingAuthorizationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an Issuing `Authorization` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveIssuingAuthorization {
    inner: RetrieveIssuingAuthorizationBuilder,
    authorization: stripe_shared::IssuingAuthorizationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIssuingAuthorization").finish_non_exhaustive()
    }
}
impl RetrieveIssuingAuthorization {
    /// Construct a new `RetrieveIssuingAuthorization`.
    pub fn new(authorization: impl Into<stripe_shared::IssuingAuthorizationId>) -> Self {
        Self {
            authorization: authorization.into(),
            inner: RetrieveIssuingAuthorizationBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveIssuingAuthorization {
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

impl StripeRequest for RetrieveIssuingAuthorization {
    type Output = stripe_shared::IssuingAuthorization;

    fn build(&self) -> RequestBuilder {
        let authorization = &self.authorization;
        RequestBuilder::new(StripeMethod::Get, format!("/issuing/authorizations/{authorization}"))
            .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateIssuingAuthorizationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingAuthorizationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingAuthorizationBuilder").finish_non_exhaustive()
    }
}
impl UpdateIssuingAuthorizationBuilder {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// Updates the specified Issuing `Authorization` object by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateIssuingAuthorization {
    inner: UpdateIssuingAuthorizationBuilder,
    authorization: stripe_shared::IssuingAuthorizationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingAuthorization").finish_non_exhaustive()
    }
}
impl UpdateIssuingAuthorization {
    /// Construct a new `UpdateIssuingAuthorization`.
    pub fn new(authorization: impl Into<stripe_shared::IssuingAuthorizationId>) -> Self {
        Self {
            authorization: authorization.into(),
            inner: UpdateIssuingAuthorizationBuilder::new(),
        }
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
}
impl UpdateIssuingAuthorization {
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

impl StripeRequest for UpdateIssuingAuthorization {
    type Output = stripe_shared::IssuingAuthorization;

    fn build(&self) -> RequestBuilder {
        let authorization = &self.authorization;
        RequestBuilder::new(StripeMethod::Post, format!("/issuing/authorizations/{authorization}"))
            .form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ApproveIssuingAuthorizationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ApproveIssuingAuthorizationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ApproveIssuingAuthorizationBuilder").finish_non_exhaustive()
    }
}
impl ApproveIssuingAuthorizationBuilder {
    fn new() -> Self {
        Self { amount: None, expand: None, metadata: None }
    }
}
/// \[Deprecated\] Approves a pending Issuing `Authorization` object.
/// This request should be made within the timeout window of the [real-time authorization](https://stripe.com/docs/issuing/controls/real-time-authorizations) flow.
///
/// This method is deprecated.
/// Instead, [respond directly to the webhook request to approve an authorization](https://stripe.com/docs/issuing/controls/real-time-authorizations#authorization-handling).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ApproveIssuingAuthorization {
    inner: ApproveIssuingAuthorizationBuilder,
    authorization: stripe_shared::IssuingAuthorizationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ApproveIssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ApproveIssuingAuthorization").finish_non_exhaustive()
    }
}
impl ApproveIssuingAuthorization {
    /// Construct a new `ApproveIssuingAuthorization`.
    pub fn new(authorization: impl Into<stripe_shared::IssuingAuthorizationId>) -> Self {
        Self {
            authorization: authorization.into(),
            inner: ApproveIssuingAuthorizationBuilder::new(),
        }
    }
    /// If the authorization's `pending_request.is_amount_controllable` property is `true`, you may provide this value to control how much to hold for the authorization.
    /// Must be positive (use [`decline`](https://docs.stripe.com/api/issuing/authorizations/decline) to decline an authorization request).
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
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
}
impl ApproveIssuingAuthorization {
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

impl StripeRequest for ApproveIssuingAuthorization {
    type Output = stripe_shared::IssuingAuthorization;

    fn build(&self) -> RequestBuilder {
        let authorization = &self.authorization;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/issuing/authorizations/{authorization}/approve"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct DeclineIssuingAuthorizationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeclineIssuingAuthorizationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeclineIssuingAuthorizationBuilder").finish_non_exhaustive()
    }
}
impl DeclineIssuingAuthorizationBuilder {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// \[Deprecated\] Declines a pending Issuing `Authorization` object.
/// This request should be made within the timeout window of the [real time authorization](https://stripe.com/docs/issuing/controls/real-time-authorizations) flow.
/// This method is deprecated.
/// Instead, [respond directly to the webhook request to decline an authorization](https://stripe.com/docs/issuing/controls/real-time-authorizations#authorization-handling).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeclineIssuingAuthorization {
    inner: DeclineIssuingAuthorizationBuilder,
    authorization: stripe_shared::IssuingAuthorizationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeclineIssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeclineIssuingAuthorization").finish_non_exhaustive()
    }
}
impl DeclineIssuingAuthorization {
    /// Construct a new `DeclineIssuingAuthorization`.
    pub fn new(authorization: impl Into<stripe_shared::IssuingAuthorizationId>) -> Self {
        Self {
            authorization: authorization.into(),
            inner: DeclineIssuingAuthorizationBuilder::new(),
        }
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
}
impl DeclineIssuingAuthorization {
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

impl StripeRequest for DeclineIssuingAuthorization {
    type Output = stripe_shared::IssuingAuthorization;

    fn build(&self) -> RequestBuilder {
        let authorization = &self.authorization;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/issuing/authorizations/{authorization}/decline"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateIssuingAuthorizationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_details: Option<CreateIssuingAuthorizationAmountDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_method: Option<stripe_shared::AuthorizationMethod>,
    card: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet: Option<CreateIssuingAuthorizationFleet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fraud_disputability_likelihood: Option<CreateIssuingAuthorizationFraudDisputabilityLikelihood>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fuel: Option<FuelSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_amount_controllable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merchant_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merchant_currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merchant_data: Option<CreateIssuingAuthorizationMerchantData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_data: Option<CreateIssuingAuthorizationNetworkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    risk_assessment: Option<CreateIssuingAuthorizationRiskAssessment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_data: Option<CreateIssuingAuthorizationVerificationData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wallet: Option<stripe_shared::IssuingTransactionWallet>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorizationBuilder").finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorizationBuilder {
    fn new(card: impl Into<String>) -> Self {
        Self {
            amount: None,
            amount_details: None,
            authorization_method: None,
            card: card.into(),
            currency: None,
            expand: None,
            fleet: None,
            fraud_disputability_likelihood: None,
            fuel: None,
            is_amount_controllable: None,
            merchant_amount: None,
            merchant_currency: None,
            merchant_data: None,
            network_data: None,
            risk_assessment: None,
            verification_data: None,
            wallet: None,
        }
    }
}
/// Detailed breakdown of amount components.
/// These amounts are denominated in `currency` and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingAuthorizationAmountDetails {
    /// The ATM withdrawal fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atm_fee: Option<i64>,
    /// The amount of cash requested by the cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashback_amount: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationAmountDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorizationAmountDetails").finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorizationAmountDetails {
    pub fn new() -> Self {
        Self { atm_fee: None, cashback_amount: None }
    }
}
impl Default for CreateIssuingAuthorizationAmountDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Fleet-specific information for authorizations using Fleet cards.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingAuthorizationFleet {
    /// Answers to prompts presented to the cardholder at the point of sale.
    /// Prompted fields vary depending on the configuration of your physical fleet cards.
    /// Typical points of sale support only numeric entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_prompt_data: Option<FleetCardholderPromptDataSpecs>,
    /// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_type: Option<stripe_shared::PurchaseType>,
    /// More information about the total amount.
    /// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_breakdown: Option<FleetReportedBreakdownSpecs>,
    /// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<stripe_shared::ServiceType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationFleet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorizationFleet").finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorizationFleet {
    pub fn new() -> Self {
        Self {
            cardholder_prompt_data: None,
            purchase_type: None,
            reported_breakdown: None,
            service_type: None,
        }
    }
}
impl Default for CreateIssuingAuthorizationFleet {
    fn default() -> Self {
        Self::new()
    }
}
/// Probability that this transaction can be disputed in the event of fraud.
/// Assessed by comparing the characteristics of the authorization to card network rules.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateIssuingAuthorizationFraudDisputabilityLikelihood {
    Neutral,
    Unknown,
    VeryLikely,
    VeryUnlikely,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    /// This variant is prefixed with an underscore to avoid conflicts with Stripe's 'Unknown' variant.
    _Unknown(String),
}
impl CreateIssuingAuthorizationFraudDisputabilityLikelihood {
    pub fn as_str(&self) -> &str {
        use CreateIssuingAuthorizationFraudDisputabilityLikelihood::*;
        match self {
            Neutral => "neutral",
            Unknown => "unknown",
            VeryLikely => "very_likely",
            VeryUnlikely => "very_unlikely",
            _Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateIssuingAuthorizationFraudDisputabilityLikelihood {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingAuthorizationFraudDisputabilityLikelihood::*;
        match s {
            "neutral" => Ok(Neutral),
            "unknown" => Ok(Unknown),
            "very_likely" => Ok(VeryLikely),
            "very_unlikely" => Ok(VeryUnlikely),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateIssuingAuthorizationFraudDisputabilityLikelihood"
                );
                Ok(_Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateIssuingAuthorizationFraudDisputabilityLikelihood {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateIssuingAuthorizationFraudDisputabilityLikelihood {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationFraudDisputabilityLikelihood {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateIssuingAuthorizationFraudDisputabilityLikelihood))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateIssuingAuthorizationFraudDisputabilityLikelihood {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateIssuingAuthorizationFraudDisputabilityLikelihood {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingAuthorizationMerchantData {
    /// A categorization of the seller's type of business.
    /// See our [merchant categories guide](https://docs.stripe.com/issuing/merchant-categories) for a list of possible values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<stripe_shared::CreateIssuingAuthorizationMerchantDataCategory>,
    /// City where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Country where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Name of the seller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier assigned to the seller by the card network.
    /// Different card networks may assign different network_id fields to the same merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// Postal code where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// An ID assigned by the seller to the location of the sale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_id: Option<String>,
    /// URL provided by the merchant on a 3DS request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationMerchantData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorizationMerchantData").finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorizationMerchantData {
    pub fn new() -> Self {
        Self {
            category: None,
            city: None,
            country: None,
            name: None,
            network_id: None,
            postal_code: None,
            state: None,
            terminal_id: None,
            url: None,
        }
    }
}
impl Default for CreateIssuingAuthorizationMerchantData {
    fn default() -> Self {
        Self::new()
    }
}
/// Details about the authorization, such as identifiers, set by the card network.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingAuthorizationNetworkData {
    /// Identifier assigned to the acquirer by the card network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquiring_institution_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationNetworkData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorizationNetworkData").finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorizationNetworkData {
    pub fn new() -> Self {
        Self { acquiring_institution_id: None }
    }
}
impl Default for CreateIssuingAuthorizationNetworkData {
    fn default() -> Self {
        Self::new()
    }
}
/// Stripe’s assessment of the fraud risk for this authorization.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingAuthorizationRiskAssessment {
    /// Stripe's assessment of this authorization's likelihood of being card testing activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_testing_risk: Option<CreateIssuingAuthorizationRiskAssessmentCardTestingRisk>,
    /// Stripe’s assessment of this authorization’s likelihood to be fraudulent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_risk: Option<CreateIssuingAuthorizationRiskAssessmentFraudRisk>,
    /// The dispute risk of the merchant (the seller on a purchase) on an authorization based on all Stripe Issuing activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_dispute_risk: Option<CreateIssuingAuthorizationRiskAssessmentMerchantDisputeRisk>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationRiskAssessment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorizationRiskAssessment").finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorizationRiskAssessment {
    pub fn new() -> Self {
        Self { card_testing_risk: None, fraud_risk: None, merchant_dispute_risk: None }
    }
}
impl Default for CreateIssuingAuthorizationRiskAssessment {
    fn default() -> Self {
        Self::new()
    }
}
/// Stripe's assessment of this authorization's likelihood of being card testing activity.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingAuthorizationRiskAssessmentCardTestingRisk {
    /// The % of declines due to a card number not existing in the past hour, taking place at the same merchant.
    /// Higher rates correspond to a greater probability of card testing activity, meaning bad actors may be attempting different card number combinations to guess a correct one.
    /// Takes on values between 0 and 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_account_number_decline_rate_past_hour: Option<i64>,
    /// The % of declines due to incorrect verification data (like CVV or expiry) in the past hour, taking place at the same merchant.
    /// Higher rates correspond to a greater probability of bad actors attempting to utilize valid card credentials at merchants with verification requirements.
    /// Takes on values between 0 and 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_credentials_decline_rate_past_hour: Option<i64>,
    /// The likelihood that this authorization is associated with card testing activity.
    /// This is assessed by evaluating decline activity over the last hour.
    pub level: stripe_shared::InsightsResourcesPaymentEvaluationSignalV2RiskLevel,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationRiskAssessmentCardTestingRisk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorizationRiskAssessmentCardTestingRisk")
            .finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorizationRiskAssessmentCardTestingRisk {
    pub fn new(
        level: impl Into<stripe_shared::InsightsResourcesPaymentEvaluationSignalV2RiskLevel>,
    ) -> Self {
        Self {
            invalid_account_number_decline_rate_past_hour: None,
            invalid_credentials_decline_rate_past_hour: None,
            level: level.into(),
        }
    }
}
/// Stripe’s assessment of this authorization’s likelihood to be fraudulent.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingAuthorizationRiskAssessmentFraudRisk {
    /// Stripe’s assessment of the likelihood of fraud on an authorization.
    pub level: stripe_shared::InsightsResourcesPaymentEvaluationSignalV2RiskLevel,
    /// Stripe’s numerical model score assessing the likelihood of fraudulent activity.
    /// A higher score means a higher likelihood of fraudulent activity, and anything above 25 is considered high risk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationRiskAssessmentFraudRisk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorizationRiskAssessmentFraudRisk").finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorizationRiskAssessmentFraudRisk {
    pub fn new(
        level: impl Into<stripe_shared::InsightsResourcesPaymentEvaluationSignalV2RiskLevel>,
    ) -> Self {
        Self { level: level.into(), score: None }
    }
}
/// The dispute risk of the merchant (the seller on a purchase) on an authorization based on all Stripe Issuing activity.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingAuthorizationRiskAssessmentMerchantDisputeRisk {
    /// The dispute rate observed across all Stripe Issuing authorizations for this merchant.
    /// For example, a value of 50 means 50% of authorizations from this merchant on Stripe Issuing have resulted in a dispute.
    /// Higher values mean a higher likelihood the authorization is disputed.
    /// Takes on values between 0 and 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_rate: Option<i64>,
    /// The likelihood that authorizations from this merchant will result in a dispute based on their history on Stripe Issuing.
    pub level: stripe_shared::InsightsResourcesPaymentEvaluationSignalV2RiskLevel,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationRiskAssessmentMerchantDisputeRisk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorizationRiskAssessmentMerchantDisputeRisk")
            .finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorizationRiskAssessmentMerchantDisputeRisk {
    pub fn new(
        level: impl Into<stripe_shared::InsightsResourcesPaymentEvaluationSignalV2RiskLevel>,
    ) -> Self {
        Self { dispute_rate: None, level: level.into() }
    }
}
/// Verifications that Stripe performed on information that the cardholder provided to the merchant.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingAuthorizationVerificationData {
    /// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<stripe_shared::IssuingAuthorizationVerificationDataCvcCheck>,
    /// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_postal_code_check:
        Option<stripe_shared::IssuingAuthorizationVerificationDataCvcCheck>,
    /// The exemption applied to this authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_exemption:
        Option<CreateIssuingAuthorizationVerificationDataAuthenticationExemption>,
    /// Whether the cardholder provided a CVC and if it matched Stripe’s record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<stripe_shared::IssuingAuthorizationVerificationDataCvcCheck>,
    /// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_check: Option<stripe_shared::IssuingAuthorizationVerificationDataCvcCheck>,
    /// 3D Secure details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<CreateIssuingAuthorizationVerificationDataThreeDSecure>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationVerificationData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorizationVerificationData").finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorizationVerificationData {
    pub fn new() -> Self {
        Self {
            address_line1_check: None,
            address_postal_code_check: None,
            authentication_exemption: None,
            cvc_check: None,
            expiry_check: None,
            three_d_secure: None,
        }
    }
}
impl Default for CreateIssuingAuthorizationVerificationData {
    fn default() -> Self {
        Self::new()
    }
}
/// The exemption applied to this authorization.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingAuthorizationVerificationDataAuthenticationExemption {
    /// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
    pub claimed_by: stripe_shared::ClaimedBy,
    /// The specific exemption claimed for this authorization.
    #[serde(rename = "type")]
    pub type_: stripe_shared::IssuingAuthorizationAuthenticationExemptionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationVerificationDataAuthenticationExemption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorizationVerificationDataAuthenticationExemption")
            .finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorizationVerificationDataAuthenticationExemption {
    pub fn new(
        claimed_by: impl Into<stripe_shared::ClaimedBy>,
        type_: impl Into<stripe_shared::IssuingAuthorizationAuthenticationExemptionType>,
    ) -> Self {
        Self { claimed_by: claimed_by.into(), type_: type_.into() }
    }
}
/// 3D Secure details.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingAuthorizationVerificationDataThreeDSecure {
    /// The outcome of the 3D Secure authentication request.
    pub result: stripe_shared::IssuingAuthorizationThreeDSecureResult,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorizationVerificationDataThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorizationVerificationDataThreeDSecure")
            .finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorizationVerificationDataThreeDSecure {
    pub fn new(result: impl Into<stripe_shared::IssuingAuthorizationThreeDSecureResult>) -> Self {
        Self { result: result.into() }
    }
}
/// Create a test-mode authorization.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingAuthorization {
    inner: CreateIssuingAuthorizationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingAuthorization").finish_non_exhaustive()
    }
}
impl CreateIssuingAuthorization {
    /// Construct a new `CreateIssuingAuthorization`.
    pub fn new(card: impl Into<String>) -> Self {
        Self { inner: CreateIssuingAuthorizationBuilder::new(card.into()) }
    }
    /// The total amount to attempt to authorize.
    /// This amount is in the provided currency, or defaults to the card's currency, and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// Detailed breakdown of amount components.
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub fn amount_details(
        mut self,
        amount_details: impl Into<CreateIssuingAuthorizationAmountDetails>,
    ) -> Self {
        self.inner.amount_details = Some(amount_details.into());
        self
    }
    /// How the card details were provided. Defaults to online.
    pub fn authorization_method(
        mut self,
        authorization_method: impl Into<stripe_shared::AuthorizationMethod>,
    ) -> Self {
        self.inner.authorization_method = Some(authorization_method.into());
        self
    }
    /// The currency of the authorization.
    /// If not provided, defaults to the currency of the card.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Fleet-specific information for authorizations using Fleet cards.
    pub fn fleet(mut self, fleet: impl Into<CreateIssuingAuthorizationFleet>) -> Self {
        self.inner.fleet = Some(fleet.into());
        self
    }
    /// Probability that this transaction can be disputed in the event of fraud.
    /// Assessed by comparing the characteristics of the authorization to card network rules.
    pub fn fraud_disputability_likelihood(
        mut self,
        fraud_disputability_likelihood: impl Into<
            CreateIssuingAuthorizationFraudDisputabilityLikelihood,
        >,
    ) -> Self {
        self.inner.fraud_disputability_likelihood = Some(fraud_disputability_likelihood.into());
        self
    }
    /// Information about fuel that was purchased with this transaction.
    pub fn fuel(mut self, fuel: impl Into<FuelSpecs>) -> Self {
        self.inner.fuel = Some(fuel.into());
        self
    }
    /// If set `true`, you may provide [amount](https://docs.stripe.com/api/issuing/authorizations/approve#approve_issuing_authorization-amount) to control how much to hold for the authorization.
    pub fn is_amount_controllable(mut self, is_amount_controllable: impl Into<bool>) -> Self {
        self.inner.is_amount_controllable = Some(is_amount_controllable.into());
        self
    }
    /// The total amount to attempt to authorize.
    /// This amount is in the provided merchant currency, and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub fn merchant_amount(mut self, merchant_amount: impl Into<i64>) -> Self {
        self.inner.merchant_amount = Some(merchant_amount.into());
        self
    }
    /// The currency of the authorization.
    /// If not provided, defaults to the currency of the card.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn merchant_currency(
        mut self,
        merchant_currency: impl Into<stripe_types::Currency>,
    ) -> Self {
        self.inner.merchant_currency = Some(merchant_currency.into());
        self
    }
    /// Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened.
    pub fn merchant_data(
        mut self,
        merchant_data: impl Into<CreateIssuingAuthorizationMerchantData>,
    ) -> Self {
        self.inner.merchant_data = Some(merchant_data.into());
        self
    }
    /// Details about the authorization, such as identifiers, set by the card network.
    pub fn network_data(
        mut self,
        network_data: impl Into<CreateIssuingAuthorizationNetworkData>,
    ) -> Self {
        self.inner.network_data = Some(network_data.into());
        self
    }
    /// Stripe’s assessment of the fraud risk for this authorization.
    pub fn risk_assessment(
        mut self,
        risk_assessment: impl Into<CreateIssuingAuthorizationRiskAssessment>,
    ) -> Self {
        self.inner.risk_assessment = Some(risk_assessment.into());
        self
    }
    /// Verifications that Stripe performed on information that the cardholder provided to the merchant.
    pub fn verification_data(
        mut self,
        verification_data: impl Into<CreateIssuingAuthorizationVerificationData>,
    ) -> Self {
        self.inner.verification_data = Some(verification_data.into());
        self
    }
    /// The digital wallet used for this transaction.
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    /// Will populate as `null` when no digital wallet was utilized.
    pub fn wallet(mut self, wallet: impl Into<stripe_shared::IssuingTransactionWallet>) -> Self {
        self.inner.wallet = Some(wallet.into());
        self
    }
}
impl CreateIssuingAuthorization {
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

impl StripeRequest for CreateIssuingAuthorization {
    type Output = stripe_shared::IssuingAuthorization;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/test_helpers/issuing/authorizations")
            .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CaptureIssuingAuthorizationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    capture_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_authorization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purchase_details: Option<CaptureIssuingAuthorizationPurchaseDetails>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CaptureIssuingAuthorizationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CaptureIssuingAuthorizationBuilder").finish_non_exhaustive()
    }
}
impl CaptureIssuingAuthorizationBuilder {
    fn new() -> Self {
        Self {
            capture_amount: None,
            close_authorization: None,
            expand: None,
            purchase_details: None,
        }
    }
}
/// Additional purchase information that is optionally provided by the merchant.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CaptureIssuingAuthorizationPurchaseDetails {
    /// Fleet-specific information for transactions using Fleet cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<CaptureIssuingAuthorizationPurchaseDetailsFleet>,
    /// Information about the flight that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight: Option<CaptureIssuingAuthorizationPurchaseDetailsFlight>,
    /// Information about fuel that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<FuelSpecs>,
    /// Information about lodging that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lodging: Option<CaptureIssuingAuthorizationPurchaseDetailsLodging>,
    /// The line items in the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Vec<CaptureIssuingAuthorizationPurchaseDetailsReceipt>>,
    /// A merchant-specific order number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CaptureIssuingAuthorizationPurchaseDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CaptureIssuingAuthorizationPurchaseDetails").finish_non_exhaustive()
    }
}
impl CaptureIssuingAuthorizationPurchaseDetails {
    pub fn new() -> Self {
        Self {
            fleet: None,
            flight: None,
            fuel: None,
            lodging: None,
            receipt: None,
            reference: None,
        }
    }
}
impl Default for CaptureIssuingAuthorizationPurchaseDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Fleet-specific information for transactions using Fleet cards.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CaptureIssuingAuthorizationPurchaseDetailsFleet {
    /// Answers to prompts presented to the cardholder at the point of sale.
    /// Prompted fields vary depending on the configuration of your physical fleet cards.
    /// Typical points of sale support only numeric entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_prompt_data: Option<FleetCardholderPromptDataSpecs>,
    /// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_type: Option<stripe_shared::PurchaseType>,
    /// More information about the total amount.
    /// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_breakdown: Option<FleetReportedBreakdownSpecs>,
    /// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<stripe_shared::ServiceType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CaptureIssuingAuthorizationPurchaseDetailsFleet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CaptureIssuingAuthorizationPurchaseDetailsFleet").finish_non_exhaustive()
    }
}
impl CaptureIssuingAuthorizationPurchaseDetailsFleet {
    pub fn new() -> Self {
        Self {
            cardholder_prompt_data: None,
            purchase_type: None,
            reported_breakdown: None,
            service_type: None,
        }
    }
}
impl Default for CaptureIssuingAuthorizationPurchaseDetailsFleet {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about the flight that was purchased with this transaction.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CaptureIssuingAuthorizationPurchaseDetailsFlight {
    /// The time that the flight departed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_at: Option<stripe_types::Timestamp>,
    /// The name of the passenger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_name: Option<String>,
    /// Whether the ticket is refundable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refundable: Option<bool>,
    /// The legs of the trip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<CaptureIssuingAuthorizationPurchaseDetailsFlightSegments>>,
    /// The travel agency that issued the ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_agency: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CaptureIssuingAuthorizationPurchaseDetailsFlight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CaptureIssuingAuthorizationPurchaseDetailsFlight").finish_non_exhaustive()
    }
}
impl CaptureIssuingAuthorizationPurchaseDetailsFlight {
    pub fn new() -> Self {
        Self {
            departure_at: None,
            passenger_name: None,
            refundable: None,
            segments: None,
            travel_agency: None,
        }
    }
}
impl Default for CaptureIssuingAuthorizationPurchaseDetailsFlight {
    fn default() -> Self {
        Self::new()
    }
}
/// The legs of the trip.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CaptureIssuingAuthorizationPurchaseDetailsFlightSegments {
    /// The three-letter IATA airport code of the flight's destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_airport_code: Option<String>,
    /// The airline carrier code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// The three-letter IATA airport code that the flight departed from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_airport_code: Option<String>,
    /// The flight number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_number: Option<String>,
    /// The flight's service class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_class: Option<String>,
    /// Whether a stopover is allowed on this flight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopover_allowed: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CaptureIssuingAuthorizationPurchaseDetailsFlightSegments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CaptureIssuingAuthorizationPurchaseDetailsFlightSegments")
            .finish_non_exhaustive()
    }
}
impl CaptureIssuingAuthorizationPurchaseDetailsFlightSegments {
    pub fn new() -> Self {
        Self {
            arrival_airport_code: None,
            carrier: None,
            departure_airport_code: None,
            flight_number: None,
            service_class: None,
            stopover_allowed: None,
        }
    }
}
impl Default for CaptureIssuingAuthorizationPurchaseDetailsFlightSegments {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about lodging that was purchased with this transaction.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CaptureIssuingAuthorizationPurchaseDetailsLodging {
    /// The time of checking into the lodging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in_at: Option<stripe_types::Timestamp>,
    /// The number of nights stayed at the lodging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nights: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CaptureIssuingAuthorizationPurchaseDetailsLodging {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CaptureIssuingAuthorizationPurchaseDetailsLodging").finish_non_exhaustive()
    }
}
impl CaptureIssuingAuthorizationPurchaseDetailsLodging {
    pub fn new() -> Self {
        Self { check_in_at: None, nights: None }
    }
}
impl Default for CaptureIssuingAuthorizationPurchaseDetailsLodging {
    fn default() -> Self {
        Self::new()
    }
}
/// The line items in the purchase.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CaptureIssuingAuthorizationPurchaseDetailsReceipt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CaptureIssuingAuthorizationPurchaseDetailsReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CaptureIssuingAuthorizationPurchaseDetailsReceipt").finish_non_exhaustive()
    }
}
impl CaptureIssuingAuthorizationPurchaseDetailsReceipt {
    pub fn new() -> Self {
        Self { description: None, quantity: None, total: None, unit_cost: None }
    }
}
impl Default for CaptureIssuingAuthorizationPurchaseDetailsReceipt {
    fn default() -> Self {
        Self::new()
    }
}
/// Capture a test-mode authorization.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CaptureIssuingAuthorization {
    inner: CaptureIssuingAuthorizationBuilder,
    authorization: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CaptureIssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CaptureIssuingAuthorization").finish_non_exhaustive()
    }
}
impl CaptureIssuingAuthorization {
    /// Construct a new `CaptureIssuingAuthorization`.
    pub fn new(authorization: impl Into<String>) -> Self {
        Self {
            authorization: authorization.into(),
            inner: CaptureIssuingAuthorizationBuilder::new(),
        }
    }
    /// The amount to capture from the authorization.
    /// If not provided, the full amount of the authorization will be captured.
    /// This amount is in the authorization currency and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub fn capture_amount(mut self, capture_amount: impl Into<i64>) -> Self {
        self.inner.capture_amount = Some(capture_amount.into());
        self
    }
    /// Whether to close the authorization after capture.
    /// Defaults to true.
    /// Set to false to enable multi-capture flows.
    pub fn close_authorization(mut self, close_authorization: impl Into<bool>) -> Self {
        self.inner.close_authorization = Some(close_authorization.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Additional purchase information that is optionally provided by the merchant.
    pub fn purchase_details(
        mut self,
        purchase_details: impl Into<CaptureIssuingAuthorizationPurchaseDetails>,
    ) -> Self {
        self.inner.purchase_details = Some(purchase_details.into());
        self
    }
}
impl CaptureIssuingAuthorization {
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

impl StripeRequest for CaptureIssuingAuthorization {
    type Output = stripe_shared::IssuingAuthorization;

    fn build(&self) -> RequestBuilder {
        let authorization = &self.authorization;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/issuing/authorizations/{authorization}/capture"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ExpireIssuingAuthorizationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ExpireIssuingAuthorizationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExpireIssuingAuthorizationBuilder").finish_non_exhaustive()
    }
}
impl ExpireIssuingAuthorizationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Expire a test-mode Authorization.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ExpireIssuingAuthorization {
    inner: ExpireIssuingAuthorizationBuilder,
    authorization: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ExpireIssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExpireIssuingAuthorization").finish_non_exhaustive()
    }
}
impl ExpireIssuingAuthorization {
    /// Construct a new `ExpireIssuingAuthorization`.
    pub fn new(authorization: impl Into<String>) -> Self {
        Self {
            authorization: authorization.into(),
            inner: ExpireIssuingAuthorizationBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ExpireIssuingAuthorization {
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

impl StripeRequest for ExpireIssuingAuthorization {
    type Output = stripe_shared::IssuingAuthorization;

    fn build(&self) -> RequestBuilder {
        let authorization = &self.authorization;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/issuing/authorizations/{authorization}/expire"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct FinalizeAmountIssuingAuthorizationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    final_amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet: Option<FinalizeAmountIssuingAuthorizationFleet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fuel: Option<FuelSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinalizeAmountIssuingAuthorizationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FinalizeAmountIssuingAuthorizationBuilder").finish_non_exhaustive()
    }
}
impl FinalizeAmountIssuingAuthorizationBuilder {
    fn new(final_amount: impl Into<i64>) -> Self {
        Self { expand: None, final_amount: final_amount.into(), fleet: None, fuel: None }
    }
}
/// Fleet-specific information for authorizations using Fleet cards.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FinalizeAmountIssuingAuthorizationFleet {
    /// Answers to prompts presented to the cardholder at the point of sale.
    /// Prompted fields vary depending on the configuration of your physical fleet cards.
    /// Typical points of sale support only numeric entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_prompt_data: Option<FleetCardholderPromptDataSpecs>,
    /// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_type: Option<stripe_shared::PurchaseType>,
    /// More information about the total amount.
    /// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_breakdown: Option<FleetReportedBreakdownSpecs>,
    /// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<stripe_shared::ServiceType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinalizeAmountIssuingAuthorizationFleet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FinalizeAmountIssuingAuthorizationFleet").finish_non_exhaustive()
    }
}
impl FinalizeAmountIssuingAuthorizationFleet {
    pub fn new() -> Self {
        Self {
            cardholder_prompt_data: None,
            purchase_type: None,
            reported_breakdown: None,
            service_type: None,
        }
    }
}
impl Default for FinalizeAmountIssuingAuthorizationFleet {
    fn default() -> Self {
        Self::new()
    }
}
/// Finalize the amount on an Authorization prior to capture, when the initial authorization was for an estimated amount.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FinalizeAmountIssuingAuthorization {
    inner: FinalizeAmountIssuingAuthorizationBuilder,
    authorization: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinalizeAmountIssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FinalizeAmountIssuingAuthorization").finish_non_exhaustive()
    }
}
impl FinalizeAmountIssuingAuthorization {
    /// Construct a new `FinalizeAmountIssuingAuthorization`.
    pub fn new(authorization: impl Into<String>, final_amount: impl Into<i64>) -> Self {
        Self {
            authorization: authorization.into(),
            inner: FinalizeAmountIssuingAuthorizationBuilder::new(final_amount.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Fleet-specific information for authorizations using Fleet cards.
    pub fn fleet(mut self, fleet: impl Into<FinalizeAmountIssuingAuthorizationFleet>) -> Self {
        self.inner.fleet = Some(fleet.into());
        self
    }
    /// Information about fuel that was purchased with this transaction.
    pub fn fuel(mut self, fuel: impl Into<FuelSpecs>) -> Self {
        self.inner.fuel = Some(fuel.into());
        self
    }
}
impl FinalizeAmountIssuingAuthorization {
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

impl StripeRequest for FinalizeAmountIssuingAuthorization {
    type Output = stripe_shared::IssuingAuthorization;

    fn build(&self) -> RequestBuilder {
        let authorization = &self.authorization;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/issuing/authorizations/{authorization}/finalize_amount"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RespondIssuingAuthorizationBuilder {
    confirmed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RespondIssuingAuthorizationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RespondIssuingAuthorizationBuilder").finish_non_exhaustive()
    }
}
impl RespondIssuingAuthorizationBuilder {
    fn new(confirmed: impl Into<bool>) -> Self {
        Self { confirmed: confirmed.into(), expand: None }
    }
}
/// Respond to a fraud challenge on a testmode Issuing authorization, simulating either a confirmation of fraud or a correction of legitimacy.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RespondIssuingAuthorization {
    inner: RespondIssuingAuthorizationBuilder,
    authorization: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RespondIssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RespondIssuingAuthorization").finish_non_exhaustive()
    }
}
impl RespondIssuingAuthorization {
    /// Construct a new `RespondIssuingAuthorization`.
    pub fn new(authorization: impl Into<String>, confirmed: impl Into<bool>) -> Self {
        Self {
            authorization: authorization.into(),
            inner: RespondIssuingAuthorizationBuilder::new(confirmed.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RespondIssuingAuthorization {
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

impl StripeRequest for RespondIssuingAuthorization {
    type Output = stripe_shared::IssuingAuthorization;

    fn build(&self) -> RequestBuilder {
        let authorization = &self.authorization;
        RequestBuilder::new(
            StripeMethod::Post,
            format!(
                "/test_helpers/issuing/authorizations/{authorization}/fraud_challenges/respond"
            ),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct IncrementIssuingAuthorizationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    increment_amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_amount_controllable: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IncrementIssuingAuthorizationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IncrementIssuingAuthorizationBuilder").finish_non_exhaustive()
    }
}
impl IncrementIssuingAuthorizationBuilder {
    fn new(increment_amount: impl Into<i64>) -> Self {
        Self {
            expand: None,
            increment_amount: increment_amount.into(),
            is_amount_controllable: None,
        }
    }
}
/// Increment a test-mode Authorization.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct IncrementIssuingAuthorization {
    inner: IncrementIssuingAuthorizationBuilder,
    authorization: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IncrementIssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IncrementIssuingAuthorization").finish_non_exhaustive()
    }
}
impl IncrementIssuingAuthorization {
    /// Construct a new `IncrementIssuingAuthorization`.
    pub fn new(authorization: impl Into<String>, increment_amount: impl Into<i64>) -> Self {
        Self {
            authorization: authorization.into(),
            inner: IncrementIssuingAuthorizationBuilder::new(increment_amount.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// If set `true`, you may provide [amount](https://docs.stripe.com/api/issuing/authorizations/approve#approve_issuing_authorization-amount) to control how much to hold for the authorization.
    pub fn is_amount_controllable(mut self, is_amount_controllable: impl Into<bool>) -> Self {
        self.inner.is_amount_controllable = Some(is_amount_controllable.into());
        self
    }
}
impl IncrementIssuingAuthorization {
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

impl StripeRequest for IncrementIssuingAuthorization {
    type Output = stripe_shared::IssuingAuthorization;

    fn build(&self) -> RequestBuilder {
        let authorization = &self.authorization;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/issuing/authorizations/{authorization}/increment"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ReverseIssuingAuthorizationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reverse_amount: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReverseIssuingAuthorizationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReverseIssuingAuthorizationBuilder").finish_non_exhaustive()
    }
}
impl ReverseIssuingAuthorizationBuilder {
    fn new() -> Self {
        Self { expand: None, reverse_amount: None }
    }
}
/// Reverse a test-mode Authorization.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ReverseIssuingAuthorization {
    inner: ReverseIssuingAuthorizationBuilder,
    authorization: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReverseIssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReverseIssuingAuthorization").finish_non_exhaustive()
    }
}
impl ReverseIssuingAuthorization {
    /// Construct a new `ReverseIssuingAuthorization`.
    pub fn new(authorization: impl Into<String>) -> Self {
        Self {
            authorization: authorization.into(),
            inner: ReverseIssuingAuthorizationBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The amount to reverse from the authorization.
    /// If not provided, the full amount of the authorization will be reversed.
    /// This amount is in the authorization currency and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub fn reverse_amount(mut self, reverse_amount: impl Into<i64>) -> Self {
        self.inner.reverse_amount = Some(reverse_amount.into());
        self
    }
}
impl ReverseIssuingAuthorization {
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

impl StripeRequest for ReverseIssuingAuthorization {
    type Output = stripe_shared::IssuingAuthorization;

    fn build(&self) -> RequestBuilder {
        let authorization = &self.authorization;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/issuing/authorizations/{authorization}/reverse"),
        )
        .form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FleetCardholderPromptDataSpecs {
    /// Driver ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_id: Option<String>,
    /// Odometer reading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odometer: Option<i64>,
    /// An alphanumeric ID.
    /// This field is used when a vehicle ID, driver ID, or generic ID is entered by the cardholder, but the merchant or card network did not specify the prompt type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unspecified_id: Option<String>,
    /// User ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Vehicle number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FleetCardholderPromptDataSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FleetCardholderPromptDataSpecs").finish_non_exhaustive()
    }
}
impl FleetCardholderPromptDataSpecs {
    pub fn new() -> Self {
        Self {
            driver_id: None,
            odometer: None,
            unspecified_id: None,
            user_id: None,
            vehicle_number: None,
        }
    }
}
impl Default for FleetCardholderPromptDataSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FleetReportedBreakdownFuelSpecs {
    /// Gross fuel amount that should equal Fuel Volume multiplied by Fuel Unit Cost, inclusive of taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FleetReportedBreakdownFuelSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FleetReportedBreakdownFuelSpecs").finish_non_exhaustive()
    }
}
impl FleetReportedBreakdownFuelSpecs {
    pub fn new() -> Self {
        Self { gross_amount_decimal: None }
    }
}
impl Default for FleetReportedBreakdownFuelSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FleetReportedBreakdownNonFuelSpecs {
    /// Gross non-fuel amount that should equal the sum of the line items, inclusive of taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FleetReportedBreakdownNonFuelSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FleetReportedBreakdownNonFuelSpecs").finish_non_exhaustive()
    }
}
impl FleetReportedBreakdownNonFuelSpecs {
    pub fn new() -> Self {
        Self { gross_amount_decimal: None }
    }
}
impl Default for FleetReportedBreakdownNonFuelSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FleetReportedBreakdownTaxSpecs {
    /// Amount of state or provincial Sales Tax included in the transaction amount.
    /// Null if not reported by merchant or not subject to tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amount_decimal: Option<String>,
    /// Amount of national Sales Tax or VAT included in the transaction amount.
    /// Null if not reported by merchant or not subject to tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FleetReportedBreakdownTaxSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FleetReportedBreakdownTaxSpecs").finish_non_exhaustive()
    }
}
impl FleetReportedBreakdownTaxSpecs {
    pub fn new() -> Self {
        Self { local_amount_decimal: None, national_amount_decimal: None }
    }
}
impl Default for FleetReportedBreakdownTaxSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FuelSpecs {
    /// [Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying the primary fuel product purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_product_code: Option<String>,
    /// The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_decimal: Option<String>,
    /// The type of fuel that was purchased.
    /// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<stripe_shared::IssuingAuthorizationFuelDataType>,
    /// The units for `quantity_decimal`.
    /// One of `charging_minute`, `imperial_gallon`, `kilogram`, `kilowatt_hour`, `liter`, `pound`, `us_gallon`, or `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<stripe_shared::IssuingAuthorizationFuelDataUnit>,
    /// The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FuelSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FuelSpecs").finish_non_exhaustive()
    }
}
impl FuelSpecs {
    pub fn new() -> Self {
        Self {
            industry_product_code: None,
            quantity_decimal: None,
            type_: None,
            unit: None,
            unit_cost_decimal: None,
        }
    }
}
impl Default for FuelSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FleetReportedBreakdownSpecs {
    /// Breakdown of fuel portion of the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<FleetReportedBreakdownFuelSpecs>,
    /// Breakdown of non-fuel portion of the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_fuel: Option<FleetReportedBreakdownNonFuelSpecs>,
    /// Information about tax included in this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<FleetReportedBreakdownTaxSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FleetReportedBreakdownSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FleetReportedBreakdownSpecs").finish_non_exhaustive()
    }
}
impl FleetReportedBreakdownSpecs {
    pub fn new() -> Self {
        Self { fuel: None, non_fuel: None, tax: None }
    }
}
impl Default for FleetReportedBreakdownSpecs {
    fn default() -> Self {
        Self::new()
    }
}
/// Only return authorizations with the given status. One of `pending`, `closed`, or `reversed`.
pub type ListIssuingAuthorizationStatus = stripe_shared::IssuingAuthorizationStatus;
/// How the card details were provided. Defaults to online.
pub type CreateIssuingAuthorizationAuthorizationMethod = stripe_shared::AuthorizationMethod;
/// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
pub type CreateIssuingAuthorizationFleetPurchaseType = stripe_shared::PurchaseType;
/// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
pub type CreateIssuingAuthorizationFleetServiceType = stripe_shared::ServiceType;
/// The type of fuel that was purchased.
/// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
pub type CreateIssuingAuthorizationFuelType = stripe_shared::IssuingAuthorizationFuelDataType;
/// The units for `quantity_decimal`.
/// One of `charging_minute`, `imperial_gallon`, `kilogram`, `kilowatt_hour`, `liter`, `pound`, `us_gallon`, or `other`.
pub type CreateIssuingAuthorizationFuelUnit = stripe_shared::IssuingAuthorizationFuelDataUnit;
/// A categorization of the seller's type of business.
/// See our [merchant categories guide](https://docs.stripe.com/issuing/merchant-categories) for a list of possible values.
pub type CreateIssuingAuthorizationMerchantDataCategory =
    stripe_shared::CreateIssuingAuthorizationMerchantDataCategory;
/// The likelihood that this authorization is associated with card testing activity.
/// This is assessed by evaluating decline activity over the last hour.
pub type CreateIssuingAuthorizationRiskAssessmentCardTestingRiskLevel =
    stripe_shared::InsightsResourcesPaymentEvaluationSignalV2RiskLevel;
/// Stripe’s assessment of the likelihood of fraud on an authorization.
pub type CreateIssuingAuthorizationRiskAssessmentFraudRiskLevel =
    stripe_shared::InsightsResourcesPaymentEvaluationSignalV2RiskLevel;
/// The likelihood that authorizations from this merchant will result in a dispute based on their history on Stripe Issuing.
pub type CreateIssuingAuthorizationRiskAssessmentMerchantDisputeRiskLevel =
    stripe_shared::InsightsResourcesPaymentEvaluationSignalV2RiskLevel;
/// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
pub type CreateIssuingAuthorizationVerificationDataAddressLine1Check =
    stripe_shared::IssuingAuthorizationVerificationDataCvcCheck;
/// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
pub type CreateIssuingAuthorizationVerificationDataAddressPostalCodeCheck =
    stripe_shared::IssuingAuthorizationVerificationDataCvcCheck;
/// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
pub type CreateIssuingAuthorizationVerificationDataAuthenticationExemptionClaimedBy =
    stripe_shared::ClaimedBy;
/// The specific exemption claimed for this authorization.
pub type CreateIssuingAuthorizationVerificationDataAuthenticationExemptionType =
    stripe_shared::IssuingAuthorizationAuthenticationExemptionType;
/// Whether the cardholder provided a CVC and if it matched Stripe’s record.
pub type CreateIssuingAuthorizationVerificationDataCvcCheck =
    stripe_shared::IssuingAuthorizationVerificationDataCvcCheck;
/// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
pub type CreateIssuingAuthorizationVerificationDataExpiryCheck =
    stripe_shared::IssuingAuthorizationVerificationDataCvcCheck;
/// The outcome of the 3D Secure authentication request.
pub type CreateIssuingAuthorizationVerificationDataThreeDSecureResult =
    stripe_shared::IssuingAuthorizationThreeDSecureResult;
/// The digital wallet used for this transaction.
/// One of `apple_pay`, `google_pay`, or `samsung_pay`.
/// Will populate as `null` when no digital wallet was utilized.
pub type CreateIssuingAuthorizationWallet = stripe_shared::IssuingTransactionWallet;
/// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
pub type CaptureIssuingAuthorizationPurchaseDetailsFleetPurchaseType = stripe_shared::PurchaseType;
/// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
pub type CaptureIssuingAuthorizationPurchaseDetailsFleetServiceType = stripe_shared::ServiceType;
/// The type of fuel that was purchased.
/// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
pub type CaptureIssuingAuthorizationPurchaseDetailsFuelType =
    stripe_shared::IssuingAuthorizationFuelDataType;
/// The units for `quantity_decimal`.
/// One of `charging_minute`, `imperial_gallon`, `kilogram`, `kilowatt_hour`, `liter`, `pound`, `us_gallon`, or `other`.
pub type CaptureIssuingAuthorizationPurchaseDetailsFuelUnit =
    stripe_shared::IssuingAuthorizationFuelDataUnit;
/// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
pub type FinalizeAmountIssuingAuthorizationFleetPurchaseType = stripe_shared::PurchaseType;
/// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
pub type FinalizeAmountIssuingAuthorizationFleetServiceType = stripe_shared::ServiceType;
/// The type of fuel that was purchased.
/// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
pub type FinalizeAmountIssuingAuthorizationFuelType =
    stripe_shared::IssuingAuthorizationFuelDataType;
/// The units for `quantity_decimal`.
/// One of `charging_minute`, `imperial_gallon`, `kilogram`, `kilowatt_hour`, `liter`, `pound`, `us_gallon`, or `other`.
pub type FinalizeAmountIssuingAuthorizationFuelUnit =
    stripe_shared::IssuingAuthorizationFuelDataUnit;
/// Answers to prompts presented to the cardholder at the point of sale.
/// Prompted fields vary depending on the configuration of your physical fleet cards.
/// Typical points of sale support only numeric entry.
pub type CreateIssuingAuthorizationFleetCardholderPromptData = FleetCardholderPromptDataSpecs;
/// Breakdown of fuel portion of the purchase.
pub type CreateIssuingAuthorizationFleetReportedBreakdownFuel = FleetReportedBreakdownFuelSpecs;
/// Breakdown of non-fuel portion of the purchase.
pub type CreateIssuingAuthorizationFleetReportedBreakdownNonFuel =
    FleetReportedBreakdownNonFuelSpecs;
/// Information about tax included in this transaction.
pub type CreateIssuingAuthorizationFleetReportedBreakdownTax = FleetReportedBreakdownTaxSpecs;
/// Information about fuel that was purchased with this transaction.
pub type CreateIssuingAuthorizationFuel = FuelSpecs;
/// Answers to prompts presented to the cardholder at the point of sale.
/// Prompted fields vary depending on the configuration of your physical fleet cards.
/// Typical points of sale support only numeric entry.
pub type CaptureIssuingAuthorizationPurchaseDetailsFleetCardholderPromptData =
    FleetCardholderPromptDataSpecs;
/// Breakdown of fuel portion of the purchase.
pub type CaptureIssuingAuthorizationPurchaseDetailsFleetReportedBreakdownFuel =
    FleetReportedBreakdownFuelSpecs;
/// Breakdown of non-fuel portion of the purchase.
pub type CaptureIssuingAuthorizationPurchaseDetailsFleetReportedBreakdownNonFuel =
    FleetReportedBreakdownNonFuelSpecs;
/// Information about tax included in this transaction.
pub type CaptureIssuingAuthorizationPurchaseDetailsFleetReportedBreakdownTax =
    FleetReportedBreakdownTaxSpecs;
/// Information about fuel that was purchased with this transaction.
pub type CaptureIssuingAuthorizationPurchaseDetailsFuel = FuelSpecs;
/// Answers to prompts presented to the cardholder at the point of sale.
/// Prompted fields vary depending on the configuration of your physical fleet cards.
/// Typical points of sale support only numeric entry.
pub type FinalizeAmountIssuingAuthorizationFleetCardholderPromptData =
    FleetCardholderPromptDataSpecs;
/// Breakdown of fuel portion of the purchase.
pub type FinalizeAmountIssuingAuthorizationFleetReportedBreakdownFuel =
    FleetReportedBreakdownFuelSpecs;
/// Breakdown of non-fuel portion of the purchase.
pub type FinalizeAmountIssuingAuthorizationFleetReportedBreakdownNonFuel =
    FleetReportedBreakdownNonFuelSpecs;
/// Information about tax included in this transaction.
pub type FinalizeAmountIssuingAuthorizationFleetReportedBreakdownTax =
    FleetReportedBreakdownTaxSpecs;
/// Information about fuel that was purchased with this transaction.
pub type FinalizeAmountIssuingAuthorizationFuel = FuelSpecs;
/// More information about the total amount.
/// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
pub type CreateIssuingAuthorizationFleetReportedBreakdown = FleetReportedBreakdownSpecs;
/// More information about the total amount.
/// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
pub type CaptureIssuingAuthorizationPurchaseDetailsFleetReportedBreakdown =
    FleetReportedBreakdownSpecs;
/// More information about the total amount.
/// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
pub type FinalizeAmountIssuingAuthorizationFleetReportedBreakdown = FleetReportedBreakdownSpecs;
