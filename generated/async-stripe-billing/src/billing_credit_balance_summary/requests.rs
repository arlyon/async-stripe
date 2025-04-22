use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveForMyAccountBillingCreditBalanceSummaryBuilder {
    customer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    filter: RetrieveForMyAccountBillingCreditBalanceSummaryFilter,
}
impl RetrieveForMyAccountBillingCreditBalanceSummaryBuilder {
    fn new(
        customer: impl Into<String>,
        filter: impl Into<RetrieveForMyAccountBillingCreditBalanceSummaryFilter>,
    ) -> Self {
        Self { customer: customer.into(), expand: None, filter: filter.into() }
    }
}
/// The filter criteria for the credit balance summary.
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType {
    Metered,
}
impl RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType {
    pub fn as_str(self) -> &'static str {
        use RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType::*;
        match self {
            Metered => "metered",
        }
    }
}

impl std::str::FromStr
    for RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType::*;
        match s {
            "metered" => Ok(Metered),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for RetrieveForMyAccountBillingCreditBalanceSummaryFilterApplicabilityScopePriceType"))
    }
}
/// A list of prices that the credit grant can apply to.
/// We currently only support the `metered` prices.
/// Cannot be used in combination with `price_type`.
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RetrieveForMyAccountBillingCreditBalanceSummaryFilterType {
    ApplicabilityScope,
    CreditGrant,
}
impl RetrieveForMyAccountBillingCreditBalanceSummaryFilterType {
    pub fn as_str(self) -> &'static str {
        use RetrieveForMyAccountBillingCreditBalanceSummaryFilterType::*;
        match self {
            ApplicabilityScope => "applicability_scope",
            CreditGrant => "credit_grant",
        }
    }
}

impl std::str::FromStr for RetrieveForMyAccountBillingCreditBalanceSummaryFilterType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RetrieveForMyAccountBillingCreditBalanceSummaryFilterType::*;
        match s {
            "applicability_scope" => Ok(ApplicabilityScope),
            "credit_grant" => Ok(CreditGrant),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for RetrieveForMyAccountBillingCreditBalanceSummaryFilterType",
            )
        })
    }
}
/// Retrieves the credit balance summary for a customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveForMyAccountBillingCreditBalanceSummary {
    inner: RetrieveForMyAccountBillingCreditBalanceSummaryBuilder,
}
impl RetrieveForMyAccountBillingCreditBalanceSummary {
    /// Construct a new `RetrieveForMyAccountBillingCreditBalanceSummary`.
    pub fn new(
        customer: impl Into<String>,
        filter: impl Into<RetrieveForMyAccountBillingCreditBalanceSummaryFilter>,
    ) -> Self {
        Self {
            inner: RetrieveForMyAccountBillingCreditBalanceSummaryBuilder::new(
                customer.into(),
                filter.into(),
            ),
        }
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
