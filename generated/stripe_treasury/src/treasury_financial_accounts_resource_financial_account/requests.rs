#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Encodes whether a FinancialAccount has access to a particular feature.
    ///
    /// Stripe or the platform can control features via the requested field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeatures>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictions>,
    /// The currencies the FinancialAccount can hold a balance in.
    pub supported_currencies: &'a [&'a str],
}
impl<'a> CreateTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    pub fn new(supported_currencies: &'a [&'a str]) -> Self {
        Self { expand: Default::default(), features: Default::default(), metadata: Default::default(), platform_restrictions: Default::default(), supported_currencies }
    }
}
/// Encodes whether a FinancialAccount has access to a particular feature.
///
/// Stripe or the platform can control features via the requested field.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeatures {
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesCardIssuing>,
    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    ///
    /// Various factors determine the insurance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesDepositInsurance>,
    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesFinancialAddresses>,
    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesInboundTransfers>,
    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesIntraStripeFlows>,
    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPayments>,
    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfers>,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeatures {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesCardIssuing {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesCardIssuing {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents whether this FinancialAccount is eligible for deposit insurance.
///
/// Various factors determine the insurance amount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesDepositInsurance {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesDepositInsurance {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains Features that add FinancialAddresses to the FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesFinancialAddressesAba>,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesFinancialAddresses {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Adds an ABA FinancialAddress to the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesFinancialAddressesAba {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesFinancialAddressesAba {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesInboundTransfersAch>,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesInboundTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH Debits via the InboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesInboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesInboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesIntraStripeFlows {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPayments {
    /// Enables ACH transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPaymentsAch>,
    /// Enables US domestic wire transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPaymentsUsDomesticWire>,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPaymentsAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPaymentsAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfers {
    /// Enables ACH transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfersAch>,
    /// Enables US domestic wire transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfersUsDomesticWire>,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// The set of functionalities that the platform can restrict on the FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictions {
    /// Restricts all inbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_flows: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows>,
    /// Restricts all outbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_flows: Option<CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows>,
}
impl CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Restricts all inbound money movement.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}

impl CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Restricts all outbound money movement.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}

impl CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    /// Creates a new FinancialAccount.
    ///
    /// For now, each connected account can only have one FinancialAccount.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAccount> {
        client.send_form("/treasury/financial_accounts", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Encodes whether a FinancialAccount has access to a particular feature, with a status enum and associated `status_details`.
    ///
    /// Stripe or the platform may control features via the requested field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeatures>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictions>,
}
impl<'a> UpdateTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Encodes whether a FinancialAccount has access to a particular feature, with a status enum and associated `status_details`.
///
/// Stripe or the platform may control features via the requested field.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeatures {
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesCardIssuing>,
    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    ///
    /// Various factors determine the insurance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesDepositInsurance>,
    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesFinancialAddresses>,
    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesInboundTransfers>,
    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesIntraStripeFlows>,
    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPayments>,
    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfers>,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeatures {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesCardIssuing {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesCardIssuing {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents whether this FinancialAccount is eligible for deposit insurance.
///
/// Various factors determine the insurance amount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesDepositInsurance {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesDepositInsurance {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains Features that add FinancialAddresses to the FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesFinancialAddressesAba>,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesFinancialAddresses {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Adds an ABA FinancialAddress to the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesFinancialAddressesAba {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesFinancialAddressesAba {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesInboundTransfersAch>,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesInboundTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH Debits via the InboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesInboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesInboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesIntraStripeFlows {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPayments {
    /// Enables ACH transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPaymentsAch>,
    /// Enables US domestic wire transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPaymentsUsDomesticWire>,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPaymentsAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPaymentsAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfers {
    /// Enables ACH transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfersAch>,
    /// Enables US domestic wire transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfersUsDomesticWire>,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// The set of functionalities that the platform can restrict on the FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictions {
    /// Restricts all inbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_flows: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows>,
    /// Restricts all outbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_flows: Option<UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows>,
}
impl UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Restricts all inbound money movement.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}

impl UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        use UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsInboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Restricts all outbound money movement.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}

impl UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        use UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateTreasuryFinancialAccountsResourceFinancialAccountPlatformRestrictionsOutboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> UpdateTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    /// Updates the details of a FinancialAccount.
    pub fn send(&self, client: &stripe::Client, financial_account: &stripe_treasury::treasury_financial_accounts_resource_financial_account::TreasuryFinancialAccountId) -> stripe::Response<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAccount> {
        client.send_form(&format!("/treasury/financial_accounts/{financial_account}", financial_account = financial_account), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountCardIssuing>,
    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    ///
    /// Various factors determine the insurance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountDepositInsurance>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountFinancialAddresses>,
    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountInboundTransfers>,
    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountIntraStripeFlows>,
    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundPayments>,
    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundTransfers>,
}
impl<'a> UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountCardIssuing {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountCardIssuing {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents whether this FinancialAccount is eligible for deposit insurance.
///
/// Various factors determine the insurance amount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountDepositInsurance {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountDepositInsurance {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains Features that add FinancialAddresses to the FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountFinancialAddressesAba>,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountFinancialAddresses {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Adds an ABA FinancialAddress to the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountFinancialAddressesAba {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountFinancialAddressesAba {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountInboundTransfersAch>,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountInboundTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH Debits via the InboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountInboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountInboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountIntraStripeFlows {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundPayments {
    /// Enables ACH transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundPaymentsAch>,
    /// Enables US domestic wire transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundPaymentsUsDomesticWire>,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundPaymentsAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundPaymentsAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundPaymentsUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundTransfers {
    /// Enables ACH transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundTransfersAch>,
    /// Enables US domestic wire transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundTransfersUsDomesticWire>,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccountOutboundTransfersUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
impl<'a> UpdateFeaturesTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    /// Updates the Features associated with a FinancialAccount.
    pub fn send(
        &self,
        client: &stripe::Client,
        financial_account: &stripe_treasury::treasury_financial_accounts_resource_financial_account::TreasuryFinancialAccountId,
    ) -> stripe::Response<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAccountFeatures> {
        client.send_form(&format!("/treasury/financial_accounts/{financial_account}/features", financial_account = financial_account), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTreasuryFinancialAccountsResourceFinancialAccount<'a> {
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
impl<'a> ListTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    /// Returns a list of FinancialAccounts.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAccount>> {
        client.get_query("/treasury/financial_accounts", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAccount> {
        stripe::ListPaginator::from_params("/treasury/financial_accounts", self)
    }
}
impl<'a> stripe::PaginationParams for ListTreasuryFinancialAccountsResourceFinancialAccount<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    /// Retrieves the details of a FinancialAccount.
    pub fn send(&self, client: &stripe::Client, financial_account: &stripe_treasury::treasury_financial_accounts_resource_financial_account::TreasuryFinancialAccountId) -> stripe::Response<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAccount> {
        client.get_query(&format!("/treasury/financial_accounts/{financial_account}", financial_account = financial_account), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveFeaturesTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFeaturesTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveFeaturesTreasuryFinancialAccountsResourceFinancialAccount<'a> {
    /// Retrieves Features information associated with the FinancialAccount.
    pub fn send(
        &self,
        client: &stripe::Client,
        financial_account: &stripe_treasury::treasury_financial_accounts_resource_financial_account::TreasuryFinancialAccountId,
    ) -> stripe::Response<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAccountFeatures> {
        client.get_query(&format!("/treasury/financial_accounts/{financial_account}/features", financial_account = financial_account), self)
    }
}
