use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListTreasuryFinancialAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListTreasuryFinancialAccountBuilder<'a> {
    fn new() -> Self {
        Self { created: None, ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of FinancialAccounts.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryFinancialAccount<'a> {
    inner: ListTreasuryFinancialAccountBuilder<'a>,
}
impl<'a> ListTreasuryFinancialAccount<'a> {
    /// Construct a new `ListTreasuryFinancialAccount`.
    pub fn new() -> Self {
        Self { inner: ListTreasuryFinancialAccountBuilder::new() }
    }
    /// Only return FinancialAccounts that were created during the given date interval.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// An object ID cursor for use in pagination.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit ranging from 1 to 100 (defaults to 10).
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// An object ID cursor for use in pagination.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListTreasuryFinancialAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTreasuryFinancialAccount<'_> {
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
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_treasury::TreasuryFinancialAccount>,
    > {
        stripe_client_core::ListPaginator::new_list("/treasury/financial_accounts", self.inner)
    }
}

impl StripeRequest for ListTreasuryFinancialAccount<'_> {
    type Output = stripe_types::List<stripe_treasury::TreasuryFinancialAccount>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/financial_accounts").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryFinancialAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryFinancialAccountBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryFinancialAccount<'a> {
    inner: RetrieveTreasuryFinancialAccountBuilder<'a>,
    financial_account: &'a stripe_treasury::TreasuryFinancialAccountId,
}
impl<'a> RetrieveTreasuryFinancialAccount<'a> {
    /// Construct a new `RetrieveTreasuryFinancialAccount`.
    pub fn new(financial_account: &'a stripe_treasury::TreasuryFinancialAccountId) -> Self {
        Self { financial_account, inner: RetrieveTreasuryFinancialAccountBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTreasuryFinancialAccount<'_> {
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

impl StripeRequest for RetrieveTreasuryFinancialAccount<'_> {
    type Output = stripe_treasury::TreasuryFinancialAccount;

    fn build(&self) -> RequestBuilder {
        let financial_account = self.financial_account;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/treasury/financial_accounts/{financial_account}"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveFeaturesTreasuryFinancialAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFeaturesTreasuryFinancialAccountBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves Features information associated with the FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveFeaturesTreasuryFinancialAccount<'a> {
    inner: RetrieveFeaturesTreasuryFinancialAccountBuilder<'a>,
    financial_account: &'a stripe_treasury::TreasuryFinancialAccountId,
}
impl<'a> RetrieveFeaturesTreasuryFinancialAccount<'a> {
    /// Construct a new `RetrieveFeaturesTreasuryFinancialAccount`.
    pub fn new(financial_account: &'a stripe_treasury::TreasuryFinancialAccountId) -> Self {
        Self { financial_account, inner: RetrieveFeaturesTreasuryFinancialAccountBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveFeaturesTreasuryFinancialAccount<'_> {
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

impl StripeRequest for RetrieveFeaturesTreasuryFinancialAccount<'_> {
    type Output = stripe_treasury::TreasuryFinancialAccountFeatures;

    fn build(&self) -> RequestBuilder {
        let financial_account = self.financial_account;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/treasury/financial_accounts/{financial_account}/features"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateTreasuryFinancialAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<CreateTreasuryFinancialAccountFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_restrictions: Option<CreateTreasuryFinancialAccountPlatformRestrictions>,
    supported_currencies: &'a [&'a str],
}
impl<'a> CreateTreasuryFinancialAccountBuilder<'a> {
    fn new(supported_currencies: &'a [&'a str]) -> Self {
        Self {
            expand: None,
            features: None,
            metadata: None,
            platform_restrictions: None,
            supported_currencies,
        }
    }
}
/// Encodes whether a FinancialAccount has access to a particular feature.
/// Stripe or the platform can control features via the requested field.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeatures {
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CreateTreasuryFinancialAccountFeaturesCardIssuing>,
    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    /// Various factors determine the insurance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<CreateTreasuryFinancialAccountFeaturesDepositInsurance>,
    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<CreateTreasuryFinancialAccountFeaturesFinancialAddresses>,
    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<CreateTreasuryFinancialAccountFeaturesInboundTransfers>,
    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<CreateTreasuryFinancialAccountFeaturesIntraStripeFlows>,
    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<CreateTreasuryFinancialAccountFeaturesOutboundPayments>,
    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<CreateTreasuryFinancialAccountFeaturesOutboundTransfers>,
}
impl CreateTreasuryFinancialAccountFeatures {
    pub fn new() -> Self {
        Self {
            card_issuing: None,
            deposit_insurance: None,
            financial_addresses: None,
            inbound_transfers: None,
            intra_stripe_flows: None,
            outbound_payments: None,
            outbound_transfers: None,
        }
    }
}
impl Default for CreateTreasuryFinancialAccountFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesCardIssuing {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountFeaturesCardIssuing {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents whether this FinancialAccount is eligible for deposit insurance.
/// Various factors determine the insurance amount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesDepositInsurance {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountFeaturesDepositInsurance {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains Features that add FinancialAddresses to the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<CreateTreasuryFinancialAccountFeaturesFinancialAddressesAba>,
}
impl CreateTreasuryFinancialAccountFeaturesFinancialAddresses {
    pub fn new() -> Self {
        Self { aba: None }
    }
}
impl Default for CreateTreasuryFinancialAccountFeaturesFinancialAddresses {
    fn default() -> Self {
        Self::new()
    }
}
/// Adds an ABA FinancialAddress to the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesFinancialAddressesAba {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountFeaturesFinancialAddressesAba {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<CreateTreasuryFinancialAccountFeaturesInboundTransfersAch>,
}
impl CreateTreasuryFinancialAccountFeaturesInboundTransfers {
    pub fn new() -> Self {
        Self { ach: None }
    }
}
impl Default for CreateTreasuryFinancialAccountFeaturesInboundTransfers {
    fn default() -> Self {
        Self::new()
    }
}
/// Enables ACH Debits via the InboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesInboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountFeaturesInboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountFeaturesIntraStripeFlows {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesOutboundPayments {
    /// Enables ACH transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<CreateTreasuryFinancialAccountFeaturesOutboundPaymentsAch>,
    /// Enables US domestic wire transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire:
        Option<CreateTreasuryFinancialAccountFeaturesOutboundPaymentsUsDomesticWire>,
}
impl CreateTreasuryFinancialAccountFeaturesOutboundPayments {
    pub fn new() -> Self {
        Self { ach: None, us_domestic_wire: None }
    }
}
impl Default for CreateTreasuryFinancialAccountFeaturesOutboundPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// Enables ACH transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesOutboundPaymentsAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountFeaturesOutboundPaymentsAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesOutboundTransfers {
    /// Enables ACH transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<CreateTreasuryFinancialAccountFeaturesOutboundTransfersAch>,
    /// Enables US domestic wire transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire:
        Option<CreateTreasuryFinancialAccountFeaturesOutboundTransfersUsDomesticWire>,
}
impl CreateTreasuryFinancialAccountFeaturesOutboundTransfers {
    pub fn new() -> Self {
        Self { ach: None, us_domestic_wire: None }
    }
}
impl Default for CreateTreasuryFinancialAccountFeaturesOutboundTransfers {
    fn default() -> Self {
        Self::new()
    }
}
/// Enables ACH transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesOutboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountFeaturesOutboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// The set of functionalities that the platform can restrict on the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountPlatformRestrictions {
    /// Restricts all inbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_flows: Option<CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows>,
    /// Restricts all outbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_flows: Option<CreateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows>,
}
impl CreateTreasuryFinancialAccountPlatformRestrictions {
    pub fn new() -> Self {
        Self { inbound_flows: None, outbound_flows: None }
    }
}
impl Default for CreateTreasuryFinancialAccountPlatformRestrictions {
    fn default() -> Self {
        Self::new()
    }
}
/// Restricts all inbound money movement.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}
impl CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows",
            )
        })
    }
}
/// Restricts all outbound money movement.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}
impl CreateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for CreateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows",
            )
        })
    }
}
/// Creates a new FinancialAccount. For now, each connected account can only have one FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccount<'a> {
    inner: CreateTreasuryFinancialAccountBuilder<'a>,
}
impl<'a> CreateTreasuryFinancialAccount<'a> {
    /// Construct a new `CreateTreasuryFinancialAccount`.
    pub fn new(supported_currencies: &'a [&'a str]) -> Self {
        Self { inner: CreateTreasuryFinancialAccountBuilder::new(supported_currencies) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Encodes whether a FinancialAccount has access to a particular feature.
    /// Stripe or the platform can control features via the requested field.
    pub fn features(mut self, features: CreateTreasuryFinancialAccountFeatures) -> Self {
        self.inner.features = Some(features);
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
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    pub fn platform_restrictions(
        mut self,
        platform_restrictions: CreateTreasuryFinancialAccountPlatformRestrictions,
    ) -> Self {
        self.inner.platform_restrictions = Some(platform_restrictions);
        self
    }
}
impl CreateTreasuryFinancialAccount<'_> {
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

impl StripeRequest for CreateTreasuryFinancialAccount<'_> {
    type Output = stripe_treasury::TreasuryFinancialAccount;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/treasury/financial_accounts").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateTreasuryFinancialAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<UpdateTreasuryFinancialAccountFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_restrictions: Option<UpdateTreasuryFinancialAccountPlatformRestrictions>,
}
impl<'a> UpdateTreasuryFinancialAccountBuilder<'a> {
    fn new() -> Self {
        Self { expand: None, features: None, metadata: None, platform_restrictions: None }
    }
}
/// Encodes whether a FinancialAccount has access to a particular feature, with a status enum and associated `status_details`.
/// Stripe or the platform may control features via the requested field.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeatures {
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<UpdateTreasuryFinancialAccountFeaturesCardIssuing>,
    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    /// Various factors determine the insurance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<UpdateTreasuryFinancialAccountFeaturesDepositInsurance>,
    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<UpdateTreasuryFinancialAccountFeaturesFinancialAddresses>,
    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<UpdateTreasuryFinancialAccountFeaturesInboundTransfers>,
    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<UpdateTreasuryFinancialAccountFeaturesIntraStripeFlows>,
    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<UpdateTreasuryFinancialAccountFeaturesOutboundPayments>,
    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<UpdateTreasuryFinancialAccountFeaturesOutboundTransfers>,
}
impl UpdateTreasuryFinancialAccountFeatures {
    pub fn new() -> Self {
        Self {
            card_issuing: None,
            deposit_insurance: None,
            financial_addresses: None,
            inbound_transfers: None,
            intra_stripe_flows: None,
            outbound_payments: None,
            outbound_transfers: None,
        }
    }
}
impl Default for UpdateTreasuryFinancialAccountFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesCardIssuing {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountFeaturesCardIssuing {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents whether this FinancialAccount is eligible for deposit insurance.
/// Various factors determine the insurance amount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesDepositInsurance {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountFeaturesDepositInsurance {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains Features that add FinancialAddresses to the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<UpdateTreasuryFinancialAccountFeaturesFinancialAddressesAba>,
}
impl UpdateTreasuryFinancialAccountFeaturesFinancialAddresses {
    pub fn new() -> Self {
        Self { aba: None }
    }
}
impl Default for UpdateTreasuryFinancialAccountFeaturesFinancialAddresses {
    fn default() -> Self {
        Self::new()
    }
}
/// Adds an ABA FinancialAddress to the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesFinancialAddressesAba {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountFeaturesFinancialAddressesAba {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateTreasuryFinancialAccountFeaturesInboundTransfersAch>,
}
impl UpdateTreasuryFinancialAccountFeaturesInboundTransfers {
    pub fn new() -> Self {
        Self { ach: None }
    }
}
impl Default for UpdateTreasuryFinancialAccountFeaturesInboundTransfers {
    fn default() -> Self {
        Self::new()
    }
}
/// Enables ACH Debits via the InboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesInboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountFeaturesInboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountFeaturesIntraStripeFlows {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesOutboundPayments {
    /// Enables ACH transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateTreasuryFinancialAccountFeaturesOutboundPaymentsAch>,
    /// Enables US domestic wire transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire:
        Option<UpdateTreasuryFinancialAccountFeaturesOutboundPaymentsUsDomesticWire>,
}
impl UpdateTreasuryFinancialAccountFeaturesOutboundPayments {
    pub fn new() -> Self {
        Self { ach: None, us_domestic_wire: None }
    }
}
impl Default for UpdateTreasuryFinancialAccountFeaturesOutboundPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// Enables ACH transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesOutboundPaymentsAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountFeaturesOutboundPaymentsAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesOutboundTransfers {
    /// Enables ACH transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateTreasuryFinancialAccountFeaturesOutboundTransfersAch>,
    /// Enables US domestic wire transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire:
        Option<UpdateTreasuryFinancialAccountFeaturesOutboundTransfersUsDomesticWire>,
}
impl UpdateTreasuryFinancialAccountFeaturesOutboundTransfers {
    pub fn new() -> Self {
        Self { ach: None, us_domestic_wire: None }
    }
}
impl Default for UpdateTreasuryFinancialAccountFeaturesOutboundTransfers {
    fn default() -> Self {
        Self::new()
    }
}
/// Enables ACH transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesOutboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountFeaturesOutboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// The set of functionalities that the platform can restrict on the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountPlatformRestrictions {
    /// Restricts all inbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_flows: Option<UpdateTreasuryFinancialAccountPlatformRestrictionsInboundFlows>,
    /// Restricts all outbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_flows: Option<UpdateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows>,
}
impl UpdateTreasuryFinancialAccountPlatformRestrictions {
    pub fn new() -> Self {
        Self { inbound_flows: None, outbound_flows: None }
    }
}
impl Default for UpdateTreasuryFinancialAccountPlatformRestrictions {
    fn default() -> Self {
        Self::new()
    }
}
/// Restricts all inbound money movement.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateTreasuryFinancialAccountPlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}
impl UpdateTreasuryFinancialAccountPlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        use UpdateTreasuryFinancialAccountPlatformRestrictionsInboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for UpdateTreasuryFinancialAccountPlatformRestrictionsInboundFlows {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateTreasuryFinancialAccountPlatformRestrictionsInboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateTreasuryFinancialAccountPlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateTreasuryFinancialAccountPlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateTreasuryFinancialAccountPlatformRestrictionsInboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateTreasuryFinancialAccountPlatformRestrictionsInboundFlows
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateTreasuryFinancialAccountPlatformRestrictionsInboundFlows",
            )
        })
    }
}
/// Restricts all outbound money movement.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}
impl UpdateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        use UpdateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for UpdateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows",
            )
        })
    }
}
/// Updates the details of a FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccount<'a> {
    inner: UpdateTreasuryFinancialAccountBuilder<'a>,
    financial_account: &'a stripe_treasury::TreasuryFinancialAccountId,
}
impl<'a> UpdateTreasuryFinancialAccount<'a> {
    /// Construct a new `UpdateTreasuryFinancialAccount`.
    pub fn new(financial_account: &'a stripe_treasury::TreasuryFinancialAccountId) -> Self {
        Self { financial_account, inner: UpdateTreasuryFinancialAccountBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Encodes whether a FinancialAccount has access to a particular feature, with a status enum and associated `status_details`.
    /// Stripe or the platform may control features via the requested field.
    pub fn features(mut self, features: UpdateTreasuryFinancialAccountFeatures) -> Self {
        self.inner.features = Some(features);
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
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    pub fn platform_restrictions(
        mut self,
        platform_restrictions: UpdateTreasuryFinancialAccountPlatformRestrictions,
    ) -> Self {
        self.inner.platform_restrictions = Some(platform_restrictions);
        self
    }
}
impl UpdateTreasuryFinancialAccount<'_> {
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

impl StripeRequest for UpdateTreasuryFinancialAccount<'_> {
    type Output = stripe_treasury::TreasuryFinancialAccount;

    fn build(&self) -> RequestBuilder {
        let financial_account = self.financial_account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/treasury/financial_accounts/{financial_account}"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateFeaturesTreasuryFinancialAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    card_issuing: Option<UpdateFeaturesTreasuryFinancialAccountCardIssuing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deposit_insurance: Option<UpdateFeaturesTreasuryFinancialAccountDepositInsurance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    financial_addresses: Option<UpdateFeaturesTreasuryFinancialAccountFinancialAddresses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inbound_transfers: Option<UpdateFeaturesTreasuryFinancialAccountInboundTransfers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intra_stripe_flows: Option<UpdateFeaturesTreasuryFinancialAccountIntraStripeFlows>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outbound_payments: Option<UpdateFeaturesTreasuryFinancialAccountOutboundPayments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outbound_transfers: Option<UpdateFeaturesTreasuryFinancialAccountOutboundTransfers>,
}
impl<'a> UpdateFeaturesTreasuryFinancialAccountBuilder<'a> {
    fn new() -> Self {
        Self {
            card_issuing: None,
            deposit_insurance: None,
            expand: None,
            financial_addresses: None,
            inbound_transfers: None,
            intra_stripe_flows: None,
            outbound_payments: None,
            outbound_transfers: None,
        }
    }
}
/// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountCardIssuing {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountCardIssuing {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents whether this FinancialAccount is eligible for deposit insurance.
/// Various factors determine the insurance amount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountDepositInsurance {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountDepositInsurance {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains Features that add FinancialAddresses to the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<UpdateFeaturesTreasuryFinancialAccountFinancialAddressesAba>,
}
impl UpdateFeaturesTreasuryFinancialAccountFinancialAddresses {
    pub fn new() -> Self {
        Self { aba: None }
    }
}
impl Default for UpdateFeaturesTreasuryFinancialAccountFinancialAddresses {
    fn default() -> Self {
        Self::new()
    }
}
/// Adds an ABA FinancialAddress to the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountFinancialAddressesAba {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountFinancialAddressesAba {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFeaturesTreasuryFinancialAccountInboundTransfersAch>,
}
impl UpdateFeaturesTreasuryFinancialAccountInboundTransfers {
    pub fn new() -> Self {
        Self { ach: None }
    }
}
impl Default for UpdateFeaturesTreasuryFinancialAccountInboundTransfers {
    fn default() -> Self {
        Self::new()
    }
}
/// Enables ACH Debits via the InboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountInboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountInboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountIntraStripeFlows {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountOutboundPayments {
    /// Enables ACH transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFeaturesTreasuryFinancialAccountOutboundPaymentsAch>,
    /// Enables US domestic wire transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire:
        Option<UpdateFeaturesTreasuryFinancialAccountOutboundPaymentsUsDomesticWire>,
}
impl UpdateFeaturesTreasuryFinancialAccountOutboundPayments {
    pub fn new() -> Self {
        Self { ach: None, us_domestic_wire: None }
    }
}
impl Default for UpdateFeaturesTreasuryFinancialAccountOutboundPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// Enables ACH transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountOutboundPaymentsAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountOutboundPaymentsAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountOutboundPaymentsUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountOutboundTransfers {
    /// Enables ACH transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFeaturesTreasuryFinancialAccountOutboundTransfersAch>,
    /// Enables US domestic wire transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire:
        Option<UpdateFeaturesTreasuryFinancialAccountOutboundTransfersUsDomesticWire>,
}
impl UpdateFeaturesTreasuryFinancialAccountOutboundTransfers {
    pub fn new() -> Self {
        Self { ach: None, us_domestic_wire: None }
    }
}
impl Default for UpdateFeaturesTreasuryFinancialAccountOutboundTransfers {
    fn default() -> Self {
        Self::new()
    }
}
/// Enables ACH transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountOutboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountOutboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountOutboundTransfersUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Updates the Features associated with a FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccount<'a> {
    inner: UpdateFeaturesTreasuryFinancialAccountBuilder<'a>,
    financial_account: &'a stripe_treasury::TreasuryFinancialAccountId,
}
impl<'a> UpdateFeaturesTreasuryFinancialAccount<'a> {
    /// Construct a new `UpdateFeaturesTreasuryFinancialAccount`.
    pub fn new(financial_account: &'a stripe_treasury::TreasuryFinancialAccountId) -> Self {
        Self { financial_account, inner: UpdateFeaturesTreasuryFinancialAccountBuilder::new() }
    }
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    pub fn card_issuing(
        mut self,
        card_issuing: UpdateFeaturesTreasuryFinancialAccountCardIssuing,
    ) -> Self {
        self.inner.card_issuing = Some(card_issuing);
        self
    }
    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    /// Various factors determine the insurance amount.
    pub fn deposit_insurance(
        mut self,
        deposit_insurance: UpdateFeaturesTreasuryFinancialAccountDepositInsurance,
    ) -> Self {
        self.inner.deposit_insurance = Some(deposit_insurance);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    pub fn financial_addresses(
        mut self,
        financial_addresses: UpdateFeaturesTreasuryFinancialAccountFinancialAddresses,
    ) -> Self {
        self.inner.financial_addresses = Some(financial_addresses);
        self
    }
    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    pub fn inbound_transfers(
        mut self,
        inbound_transfers: UpdateFeaturesTreasuryFinancialAccountInboundTransfers,
    ) -> Self {
        self.inner.inbound_transfers = Some(inbound_transfers);
        self
    }
    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    pub fn intra_stripe_flows(
        mut self,
        intra_stripe_flows: UpdateFeaturesTreasuryFinancialAccountIntraStripeFlows,
    ) -> Self {
        self.inner.intra_stripe_flows = Some(intra_stripe_flows);
        self
    }
    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    pub fn outbound_payments(
        mut self,
        outbound_payments: UpdateFeaturesTreasuryFinancialAccountOutboundPayments,
    ) -> Self {
        self.inner.outbound_payments = Some(outbound_payments);
        self
    }
    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    pub fn outbound_transfers(
        mut self,
        outbound_transfers: UpdateFeaturesTreasuryFinancialAccountOutboundTransfers,
    ) -> Self {
        self.inner.outbound_transfers = Some(outbound_transfers);
        self
    }
}
impl UpdateFeaturesTreasuryFinancialAccount<'_> {
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

impl StripeRequest for UpdateFeaturesTreasuryFinancialAccount<'_> {
    type Output = stripe_treasury::TreasuryFinancialAccountFeatures;

    fn build(&self) -> RequestBuilder {
        let financial_account = self.financial_account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/treasury/financial_accounts/{financial_account}/features"),
        )
        .form(&self.inner)
    }
}
