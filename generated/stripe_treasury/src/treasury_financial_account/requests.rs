#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTreasuryFinancialAccount<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// An object ID cursor for use in pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit ranging from 1 to 100 (defaults to 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// An object ID cursor for use in pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListTreasuryFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListTreasuryFinancialAccount<'a> {
    /// Returns a list of FinancialAccounts.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryFinancialAccount>> {
        client.get_query("/treasury/financial_accounts", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_treasury::TreasuryFinancialAccount>> {
        stripe::ListPaginator::from_list_params("/treasury/financial_accounts", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryFinancialAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryFinancialAccount<'a> {
    /// Retrieves the details of a FinancialAccount.
    pub fn send(
        &self,
        client: &stripe::Client,
        financial_account: &stripe_treasury::TreasuryFinancialAccountId,
    ) -> stripe::Response<stripe_treasury::TreasuryFinancialAccount> {
        client.get_query(&format!("/treasury/financial_accounts/{financial_account}"), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveFeaturesTreasuryFinancialAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFeaturesTreasuryFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveFeaturesTreasuryFinancialAccount<'a> {
    /// Retrieves Features information associated with the FinancialAccount.
    pub fn send(
        &self,
        client: &stripe::Client,
        financial_account: &stripe_treasury::TreasuryFinancialAccountId,
    ) -> stripe::Response<stripe_treasury::TreasuryFinancialAccountFeatures> {
        client
            .get_query(&format!("/treasury/financial_accounts/{financial_account}/features"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Encodes whether a FinancialAccount has access to a particular feature.
    /// Stripe or the platform can control features via the requested field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateTreasuryFinancialAccountFeatures>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<CreateTreasuryFinancialAccountPlatformRestrictions>,
    /// The currencies the FinancialAccount can hold a balance in.
    pub supported_currencies: &'a [&'a str],
}
impl<'a> CreateTreasuryFinancialAccount<'a> {
    pub fn new(supported_currencies: &'a [&'a str]) -> Self {
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<CreateTreasuryFinancialAccountFeaturesFinancialAddressesAba>,
}
impl CreateTreasuryFinancialAccountFeaturesFinancialAddresses {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountFeaturesInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<CreateTreasuryFinancialAccountFeaturesInboundTransfersAch>,
}
impl CreateTreasuryFinancialAccountFeaturesInboundTransfers {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(()),
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
impl<'a> CreateTreasuryFinancialAccount<'a> {
    /// Creates a new FinancialAccount. For now, each connected account can only have one FinancialAccount.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_treasury::TreasuryFinancialAccount> {
        client.send_form("/treasury/financial_accounts", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Encodes whether a FinancialAccount has access to a particular feature, with a status enum and associated `status_details`.
    /// Stripe or the platform may control features via the requested field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<UpdateTreasuryFinancialAccountFeatures>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<UpdateTreasuryFinancialAccountPlatformRestrictions>,
}
impl<'a> UpdateTreasuryFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Encodes whether a FinancialAccount has access to a particular feature, with a status enum and associated `status_details`.
/// Stripe or the platform may control features via the requested field.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<UpdateTreasuryFinancialAccountFeaturesFinancialAddressesAba>,
}
impl UpdateTreasuryFinancialAccountFeaturesFinancialAddresses {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountFeaturesInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateTreasuryFinancialAccountFeaturesInboundTransfersAch>,
}
impl UpdateTreasuryFinancialAccountFeaturesInboundTransfers {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateTreasuryFinancialAccountPlatformRestrictionsInboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateTreasuryFinancialAccountPlatformRestrictionsOutboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(()),
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
impl<'a> UpdateTreasuryFinancialAccount<'a> {
    /// Updates the details of a FinancialAccount.
    pub fn send(
        &self,
        client: &stripe::Client,
        financial_account: &stripe_treasury::TreasuryFinancialAccountId,
    ) -> stripe::Response<stripe_treasury::TreasuryFinancialAccount> {
        client.send_form(
            &format!("/treasury/financial_accounts/{financial_account}"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccount<'a> {
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<UpdateFeaturesTreasuryFinancialAccountCardIssuing>,
    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    /// Various factors determine the insurance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<UpdateFeaturesTreasuryFinancialAccountDepositInsurance>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<UpdateFeaturesTreasuryFinancialAccountFinancialAddresses>,
    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<UpdateFeaturesTreasuryFinancialAccountInboundTransfers>,
    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<UpdateFeaturesTreasuryFinancialAccountIntraStripeFlows>,
    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<UpdateFeaturesTreasuryFinancialAccountOutboundPayments>,
    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<UpdateFeaturesTreasuryFinancialAccountOutboundTransfers>,
}
impl<'a> UpdateFeaturesTreasuryFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<UpdateFeaturesTreasuryFinancialAccountFinancialAddressesAba>,
}
impl UpdateFeaturesTreasuryFinancialAccountFinancialAddresses {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFeaturesTreasuryFinancialAccountInboundTransfersAch>,
}
impl UpdateFeaturesTreasuryFinancialAccountInboundTransfers {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
impl<'a> UpdateFeaturesTreasuryFinancialAccount<'a> {
    /// Updates the Features associated with a FinancialAccount.
    pub fn send(
        &self,
        client: &stripe::Client,
        financial_account: &stripe_treasury::TreasuryFinancialAccountId,
    ) -> stripe::Response<stripe_treasury::TreasuryFinancialAccountFeatures> {
        client.send_form(
            &format!("/treasury/financial_accounts/{financial_account}/features"),
            self,
            http_types::Method::Post,
        )
    }
}
