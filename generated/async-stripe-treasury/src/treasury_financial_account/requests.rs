use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListTreasuryFinancialAccountBuilder {
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
}
impl ListTreasuryFinancialAccountBuilder {
    fn new() -> Self {
        Self { created: None, ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of FinancialAccounts.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryFinancialAccount {
    inner: ListTreasuryFinancialAccountBuilder,
}
impl ListTreasuryFinancialAccount {
    /// Construct a new `ListTreasuryFinancialAccount`.
    pub fn new() -> Self {
        Self { inner: ListTreasuryFinancialAccountBuilder::new() }
    }
    /// Only return FinancialAccounts that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// An object ID cursor for use in pagination.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit ranging from 1 to 100 (defaults to 10).
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// An object ID cursor for use in pagination.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListTreasuryFinancialAccount {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTreasuryFinancialAccount {
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
        stripe_client_core::ListPaginator::new_list("/treasury/financial_accounts", &self.inner)
    }
}

impl StripeRequest for ListTreasuryFinancialAccount {
    type Output = stripe_types::List<stripe_treasury::TreasuryFinancialAccount>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/financial_accounts").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryFinancialAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTreasuryFinancialAccountBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryFinancialAccount {
    inner: RetrieveTreasuryFinancialAccountBuilder,
    financial_account: stripe_treasury::TreasuryFinancialAccountId,
}
impl RetrieveTreasuryFinancialAccount {
    /// Construct a new `RetrieveTreasuryFinancialAccount`.
    pub fn new(financial_account: impl Into<stripe_treasury::TreasuryFinancialAccountId>) -> Self {
        Self {
            financial_account: financial_account.into(),
            inner: RetrieveTreasuryFinancialAccountBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTreasuryFinancialAccount {
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

impl StripeRequest for RetrieveTreasuryFinancialAccount {
    type Output = stripe_treasury::TreasuryFinancialAccount;

    fn build(&self) -> RequestBuilder {
        let financial_account = &self.financial_account;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/treasury/financial_accounts/{financial_account}"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveFeaturesTreasuryFinancialAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveFeaturesTreasuryFinancialAccountBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves Features information associated with the FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveFeaturesTreasuryFinancialAccount {
    inner: RetrieveFeaturesTreasuryFinancialAccountBuilder,
    financial_account: stripe_treasury::TreasuryFinancialAccountId,
}
impl RetrieveFeaturesTreasuryFinancialAccount {
    /// Construct a new `RetrieveFeaturesTreasuryFinancialAccount`.
    pub fn new(financial_account: impl Into<stripe_treasury::TreasuryFinancialAccountId>) -> Self {
        Self {
            financial_account: financial_account.into(),
            inner: RetrieveFeaturesTreasuryFinancialAccountBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveFeaturesTreasuryFinancialAccount {
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

impl StripeRequest for RetrieveFeaturesTreasuryFinancialAccount {
    type Output = stripe_treasury::TreasuryFinancialAccountFeatures;

    fn build(&self) -> RequestBuilder {
        let financial_account = &self.financial_account;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/treasury/financial_accounts/{financial_account}/features"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateTreasuryFinancialAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<CreateTreasuryFinancialAccountFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_restrictions: Option<CreateTreasuryFinancialAccountPlatformRestrictions>,
    supported_currencies: Vec<String>,
}
impl CreateTreasuryFinancialAccountBuilder {
    fn new(supported_currencies: impl Into<Vec<String>>) -> Self {
        Self {
            expand: None,
            features: None,
            metadata: None,
            nickname: None,
            platform_restrictions: None,
            supported_currencies: supported_currencies.into(),
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
    }
}
/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountFeaturesIntraStripeFlows {
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
    }
}
/// Enables US domestic wire transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
    }
}
/// Enables US domestic wire transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
/// Creates a new FinancialAccount.
/// Each connected account can have up to three FinancialAccounts by default.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccount {
    inner: CreateTreasuryFinancialAccountBuilder,
}
impl CreateTreasuryFinancialAccount {
    /// Construct a new `CreateTreasuryFinancialAccount`.
    pub fn new(supported_currencies: impl Into<Vec<String>>) -> Self {
        Self { inner: CreateTreasuryFinancialAccountBuilder::new(supported_currencies.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Encodes whether a FinancialAccount has access to a particular feature.
    /// Stripe or the platform can control features via the requested field.
    pub fn features(mut self, features: impl Into<CreateTreasuryFinancialAccountFeatures>) -> Self {
        self.inner.features = Some(features.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    /// The nickname for the FinancialAccount.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.inner.nickname = Some(nickname.into());
        self
    }
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    pub fn platform_restrictions(
        mut self,
        platform_restrictions: impl Into<CreateTreasuryFinancialAccountPlatformRestrictions>,
    ) -> Self {
        self.inner.platform_restrictions = Some(platform_restrictions.into());
        self
    }
}
impl CreateTreasuryFinancialAccount {
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

impl StripeRequest for CreateTreasuryFinancialAccount {
    type Output = stripe_treasury::TreasuryFinancialAccount;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/treasury/financial_accounts").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateTreasuryFinancialAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<UpdateTreasuryFinancialAccountFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_settings: Option<UpdateTreasuryFinancialAccountForwardingSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_restrictions: Option<UpdateTreasuryFinancialAccountPlatformRestrictions>,
}
impl UpdateTreasuryFinancialAccountBuilder {
    fn new() -> Self {
        Self {
            expand: None,
            features: None,
            forwarding_settings: None,
            metadata: None,
            nickname: None,
            platform_restrictions: None,
        }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
    }
}
/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountFeaturesIntraStripeFlows {
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
    }
}
/// Enables US domestic wire transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
    }
}
/// Enables US domestic wire transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
    }
}
/// A different bank account where funds can be deposited/debited in order to get the closing FA's balance to $0.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountForwardingSettings {
    /// The financial_account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    /// The payment_method or bank account id. This needs to be a verified bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// The type of the bank account provided. This can be either "financial_account" or "payment_method"
    #[serde(rename = "type")]
    pub type_: UpdateTreasuryFinancialAccountForwardingSettingsType,
}
impl UpdateTreasuryFinancialAccountForwardingSettings {
    pub fn new(type_: impl Into<UpdateTreasuryFinancialAccountForwardingSettingsType>) -> Self {
        Self { financial_account: None, payment_method: None, type_: type_.into() }
    }
}
/// The type of the bank account provided. This can be either "financial_account" or "payment_method"
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateTreasuryFinancialAccountForwardingSettingsType {
    FinancialAccount,
    PaymentMethod,
}
impl UpdateTreasuryFinancialAccountForwardingSettingsType {
    pub fn as_str(self) -> &'static str {
        use UpdateTreasuryFinancialAccountForwardingSettingsType::*;
        match self {
            FinancialAccount => "financial_account",
            PaymentMethod => "payment_method",
        }
    }
}

impl std::str::FromStr for UpdateTreasuryFinancialAccountForwardingSettingsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateTreasuryFinancialAccountForwardingSettingsType::*;
        match s {
            "financial_account" => Ok(FinancialAccount),
            "payment_method" => Ok(PaymentMethod),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateTreasuryFinancialAccountForwardingSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateTreasuryFinancialAccountForwardingSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateTreasuryFinancialAccountForwardingSettingsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateTreasuryFinancialAccountForwardingSettingsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateTreasuryFinancialAccountForwardingSettingsType",
            )
        })
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
pub struct UpdateTreasuryFinancialAccount {
    inner: UpdateTreasuryFinancialAccountBuilder,
    financial_account: stripe_treasury::TreasuryFinancialAccountId,
}
impl UpdateTreasuryFinancialAccount {
    /// Construct a new `UpdateTreasuryFinancialAccount`.
    pub fn new(financial_account: impl Into<stripe_treasury::TreasuryFinancialAccountId>) -> Self {
        Self {
            financial_account: financial_account.into(),
            inner: UpdateTreasuryFinancialAccountBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Encodes whether a FinancialAccount has access to a particular feature, with a status enum and associated `status_details`.
    /// Stripe or the platform may control features via the requested field.
    pub fn features(mut self, features: impl Into<UpdateTreasuryFinancialAccountFeatures>) -> Self {
        self.inner.features = Some(features.into());
        self
    }
    /// A different bank account where funds can be deposited/debited in order to get the closing FA's balance to $0.
    pub fn forwarding_settings(
        mut self,
        forwarding_settings: impl Into<UpdateTreasuryFinancialAccountForwardingSettings>,
    ) -> Self {
        self.inner.forwarding_settings = Some(forwarding_settings.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    /// The nickname for the FinancialAccount.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.inner.nickname = Some(nickname.into());
        self
    }
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    pub fn platform_restrictions(
        mut self,
        platform_restrictions: impl Into<UpdateTreasuryFinancialAccountPlatformRestrictions>,
    ) -> Self {
        self.inner.platform_restrictions = Some(platform_restrictions.into());
        self
    }
}
impl UpdateTreasuryFinancialAccount {
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

impl StripeRequest for UpdateTreasuryFinancialAccount {
    type Output = stripe_treasury::TreasuryFinancialAccount;

    fn build(&self) -> RequestBuilder {
        let financial_account = &self.financial_account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/treasury/financial_accounts/{financial_account}"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CloseTreasuryFinancialAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_settings: Option<CloseTreasuryFinancialAccountForwardingSettings>,
}
impl CloseTreasuryFinancialAccountBuilder {
    fn new() -> Self {
        Self { expand: None, forwarding_settings: None }
    }
}
/// A different bank account where funds can be deposited/debited in order to get the closing FA's balance to $0.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CloseTreasuryFinancialAccountForwardingSettings {
    /// The financial_account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    /// The payment_method or bank account id. This needs to be a verified bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// The type of the bank account provided. This can be either "financial_account" or "payment_method"
    #[serde(rename = "type")]
    pub type_: CloseTreasuryFinancialAccountForwardingSettingsType,
}
impl CloseTreasuryFinancialAccountForwardingSettings {
    pub fn new(type_: impl Into<CloseTreasuryFinancialAccountForwardingSettingsType>) -> Self {
        Self { financial_account: None, payment_method: None, type_: type_.into() }
    }
}
/// The type of the bank account provided. This can be either "financial_account" or "payment_method"
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CloseTreasuryFinancialAccountForwardingSettingsType {
    FinancialAccount,
    PaymentMethod,
}
impl CloseTreasuryFinancialAccountForwardingSettingsType {
    pub fn as_str(self) -> &'static str {
        use CloseTreasuryFinancialAccountForwardingSettingsType::*;
        match self {
            FinancialAccount => "financial_account",
            PaymentMethod => "payment_method",
        }
    }
}

impl std::str::FromStr for CloseTreasuryFinancialAccountForwardingSettingsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CloseTreasuryFinancialAccountForwardingSettingsType::*;
        match s {
            "financial_account" => Ok(FinancialAccount),
            "payment_method" => Ok(PaymentMethod),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CloseTreasuryFinancialAccountForwardingSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CloseTreasuryFinancialAccountForwardingSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CloseTreasuryFinancialAccountForwardingSettingsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CloseTreasuryFinancialAccountForwardingSettingsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CloseTreasuryFinancialAccountForwardingSettingsType",
            )
        })
    }
}
/// Closes a FinancialAccount.
/// A FinancialAccount can only be closed if it has a zero balance, has no pending InboundTransfers, and has canceled all attached Issuing cards.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CloseTreasuryFinancialAccount {
    inner: CloseTreasuryFinancialAccountBuilder,
    financial_account: stripe_treasury::TreasuryFinancialAccountId,
}
impl CloseTreasuryFinancialAccount {
    /// Construct a new `CloseTreasuryFinancialAccount`.
    pub fn new(financial_account: impl Into<stripe_treasury::TreasuryFinancialAccountId>) -> Self {
        Self {
            financial_account: financial_account.into(),
            inner: CloseTreasuryFinancialAccountBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A different bank account where funds can be deposited/debited in order to get the closing FA's balance to $0.
    pub fn forwarding_settings(
        mut self,
        forwarding_settings: impl Into<CloseTreasuryFinancialAccountForwardingSettings>,
    ) -> Self {
        self.inner.forwarding_settings = Some(forwarding_settings.into());
        self
    }
}
impl CloseTreasuryFinancialAccount {
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

impl StripeRequest for CloseTreasuryFinancialAccount {
    type Output = stripe_treasury::TreasuryFinancialAccount;

    fn build(&self) -> RequestBuilder {
        let financial_account = &self.financial_account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/treasury/financial_accounts/{financial_account}/close"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateFeaturesTreasuryFinancialAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    card_issuing: Option<UpdateFeaturesTreasuryFinancialAccountCardIssuing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deposit_insurance: Option<UpdateFeaturesTreasuryFinancialAccountDepositInsurance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
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
impl UpdateFeaturesTreasuryFinancialAccountBuilder {
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
    }
}
/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountIntraStripeFlows {
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
    }
}
/// Enables US domestic wire transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountOutboundPaymentsUsDomesticWire {
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
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
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
    }
}
/// Enables US domestic wire transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountOutboundTransfersUsDomesticWire {
    pub fn new(requested: impl Into<bool>) -> Self {
        Self { requested: requested.into() }
    }
}
/// Updates the Features associated with a FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccount {
    inner: UpdateFeaturesTreasuryFinancialAccountBuilder,
    financial_account: stripe_treasury::TreasuryFinancialAccountId,
}
impl UpdateFeaturesTreasuryFinancialAccount {
    /// Construct a new `UpdateFeaturesTreasuryFinancialAccount`.
    pub fn new(financial_account: impl Into<stripe_treasury::TreasuryFinancialAccountId>) -> Self {
        Self {
            financial_account: financial_account.into(),
            inner: UpdateFeaturesTreasuryFinancialAccountBuilder::new(),
        }
    }
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    pub fn card_issuing(
        mut self,
        card_issuing: impl Into<UpdateFeaturesTreasuryFinancialAccountCardIssuing>,
    ) -> Self {
        self.inner.card_issuing = Some(card_issuing.into());
        self
    }
    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    /// Various factors determine the insurance amount.
    pub fn deposit_insurance(
        mut self,
        deposit_insurance: impl Into<UpdateFeaturesTreasuryFinancialAccountDepositInsurance>,
    ) -> Self {
        self.inner.deposit_insurance = Some(deposit_insurance.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    pub fn financial_addresses(
        mut self,
        financial_addresses: impl Into<UpdateFeaturesTreasuryFinancialAccountFinancialAddresses>,
    ) -> Self {
        self.inner.financial_addresses = Some(financial_addresses.into());
        self
    }
    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    pub fn inbound_transfers(
        mut self,
        inbound_transfers: impl Into<UpdateFeaturesTreasuryFinancialAccountInboundTransfers>,
    ) -> Self {
        self.inner.inbound_transfers = Some(inbound_transfers.into());
        self
    }
    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    pub fn intra_stripe_flows(
        mut self,
        intra_stripe_flows: impl Into<UpdateFeaturesTreasuryFinancialAccountIntraStripeFlows>,
    ) -> Self {
        self.inner.intra_stripe_flows = Some(intra_stripe_flows.into());
        self
    }
    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    pub fn outbound_payments(
        mut self,
        outbound_payments: impl Into<UpdateFeaturesTreasuryFinancialAccountOutboundPayments>,
    ) -> Self {
        self.inner.outbound_payments = Some(outbound_payments.into());
        self
    }
    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    pub fn outbound_transfers(
        mut self,
        outbound_transfers: impl Into<UpdateFeaturesTreasuryFinancialAccountOutboundTransfers>,
    ) -> Self {
        self.inner.outbound_transfers = Some(outbound_transfers.into());
        self
    }
}
impl UpdateFeaturesTreasuryFinancialAccount {
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

impl StripeRequest for UpdateFeaturesTreasuryFinancialAccount {
    type Output = stripe_treasury::TreasuryFinancialAccountFeatures;

    fn build(&self) -> RequestBuilder {
        let financial_account = &self.financial_account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/treasury/financial_accounts/{financial_account}/features"),
        )
        .form(&self.inner)
    }
}
