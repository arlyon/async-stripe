use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveForMyAccountBillingCreditBalanceSummaryBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    filter: RetrieveForMyAccountBillingCreditBalanceSummaryFilter,
}
impl RetrieveForMyAccountBillingCreditBalanceSummaryBuilder {
    fn new(filter: impl Into<RetrieveForMyAccountBillingCreditBalanceSummaryFilter>) -> Self {
        Self { customer: None, customer_account: None, expand: None, filter: filter.into() }
    }
}
/// The filter criteria for the credit balance summary.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct RetrieveForMyAccountBillingCreditBalanceSummaryFilter {
    /// The billing credit applicability scope for which to fetch credit balance summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicability_scope:
        Option<RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScope>,
    /// The credit grant for which to fetch credit balance summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_grant: Option<String>,
    /// Specify the type of this filter.
    #[serde(rename = "type")]
    pub type_: RetrieveForMyAccountBillingCreditBalanceSummaryFilterType,
}
impl RetrieveForMyAccountBillingCreditBalanceSummaryFilter {
    pub fn new(
        type_: impl Into<RetrieveForMyAccountBillingCreditBalanceSummaryFilterType>,
    ) -> Self {
        Self { applicability_scope: None, credit_grant: None, type_: type_.into() }
    }
}
/// The billing credit applicability scope for which to fetch credit balance summary.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScope {
    /// The price type that credit grants can apply to.
    /// We currently only support the `metered` price type.
    /// Cannot be used in combination with `prices`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_type:
        Option<RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType>,
    /// A list of prices that the credit grant can apply to.
    /// We currently only support the `metered` prices.
    /// Cannot be used in combination with `price_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prices:
        Option<Vec<RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePrices>>,
}
impl RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScope {
    pub fn new() -> Self {
        Self { price_type: None, prices: None }
    }
}
impl Default for RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScope {
    fn default() -> Self {
        Self::new()
    }
}
/// The price type that credit grants can apply to.
/// We currently only support the `metered` price type.
/// Cannot be used in combination with `prices`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType {
    Metered,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType {
    pub fn as_str(&self) -> &str {
        use RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType::*;
        match self {
            Metered => "metered",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType::*;
        match s {
            "metered" => Ok(Metered),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// A list of prices that the credit grant can apply to.
/// We currently only support the `metered` prices.
/// Cannot be used in combination with `price_type`.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePrices {
    /// The price ID this credit grant should apply to.
    pub id: String,
}
impl RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePrices {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
/// Specify the type of this filter.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RetrieveForMyAccountBillingCreditBalanceSummaryFilterType {
    ApplicabilityScope,
    CreditGrant,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RetrieveForMyAccountBillingCreditBalanceSummaryFilterType {
    pub fn as_str(&self) -> &str {
        use RetrieveForMyAccountBillingCreditBalanceSummaryFilterType::*;
        match self {
            ApplicabilityScope => "applicability_scope",
            CreditGrant => "credit_grant",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for RetrieveForMyAccountBillingCreditBalanceSummaryFilterType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RetrieveForMyAccountBillingCreditBalanceSummaryFilterType::*;
        match s {
            "applicability_scope" => Ok(ApplicabilityScope),
            "credit_grant" => Ok(CreditGrant),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "RetrieveForMyAccountBillingCreditBalanceSummaryFilterType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for RetrieveForMyAccountBillingCreditBalanceSummaryFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RetrieveForMyAccountBillingCreditBalanceSummaryFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RetrieveForMyAccountBillingCreditBalanceSummaryFilterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RetrieveForMyAccountBillingCreditBalanceSummaryFilterType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Retrieves the credit balance summary for a customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveForMyAccountBillingCreditBalanceSummary {
    inner: RetrieveForMyAccountBillingCreditBalanceSummaryBuilder,
}
impl RetrieveForMyAccountBillingCreditBalanceSummary {
    /// Construct a new `RetrieveForMyAccountBillingCreditBalanceSummary`.
    pub fn new(filter: impl Into<RetrieveForMyAccountBillingCreditBalanceSummaryFilter>) -> Self {
        Self { inner: RetrieveForMyAccountBillingCreditBalanceSummaryBuilder::new(filter.into()) }
    }
    /// The customer whose credit balance summary you're retrieving.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// The account representing the customer whose credit balance summary you're retrieving.
    pub fn customer_account(mut self, customer_account: impl Into<String>) -> Self {
        self.inner.customer_account = Some(customer_account.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveForMyAccountBillingCreditBalanceSummary {
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

impl StripeRequest for RetrieveForMyAccountBillingCreditBalanceSummary {
    type Output = stripe_billing::BillingCreditBalanceSummary;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/billing/credit_balance_summary").query(&self.inner)
    }
}
