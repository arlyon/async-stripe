use stripe::{Client, Response};

impl stripe_treasury::treasury::financial_account::FinancialAccount {
    /// Creates a new FinancialAccount.
    ///
    /// For now, each connected account can only have one FinancialAccount.
    pub fn create(
        client: &Client,
        params: CreateFinancialAccount,
    ) -> Response<stripe_treasury::treasury::financial_account::FinancialAccount> {
        client.send_form("/treasury/financial_accounts", params, http_types::Method::Post)
    }
    /// Updates the details of a FinancialAccount.
    pub fn update(
        client: &Client,
        financial_account: &str,
        params: UpdateFinancialAccount,
    ) -> Response<stripe_treasury::treasury::financial_account::FinancialAccount> {
        client.send_form(
            &format!(
                "/treasury/financial_accounts/{financial_account}",
                financial_account = financial_account
            ),
            params,
            http_types::Method::Post,
        )
    }
    /// Updates the Features associated with a FinancialAccount.
    pub fn update_features(
        client: &Client,
        financial_account: &str,
        params: UpdateFeaturesFinancialAccount,
    ) -> Response<stripe_treasury::treasury::financial_account_features::FinancialAccountFeatures>
    {
        client.send_form(
            &format!(
                "/treasury/financial_accounts/{financial_account}/features",
                financial_account = financial_account
            ),
            params,
            http_types::Method::Post,
        )
    }
    /// Returns a list of FinancialAccounts.
    pub fn list(
        client: &Client,
        params: ListFinancialAccount,
    ) -> Response<stripe_types::List<stripe_treasury::treasury::financial_account::FinancialAccount>>
    {
        client.get_query("/treasury/financial_accounts", params)
    }
    /// Retrieves the details of a FinancialAccount.
    pub fn retrieve(
        client: &Client,
        financial_account: &str,
        params: RetrieveFinancialAccount,
    ) -> Response<stripe_treasury::treasury::financial_account::FinancialAccount> {
        client.get_query(
            &format!(
                "/treasury/financial_accounts/{financial_account}",
                financial_account = financial_account
            ),
            params,
        )
    }
    /// Retrieves Features information associated with the FinancialAccount.
    pub fn retrieve_features(
        client: &Client,
        financial_account: &str,
        params: RetrieveFeaturesFinancialAccount,
    ) -> Response<stripe_treasury::treasury::financial_account_features::FinancialAccountFeatures>
    {
        client.get_query(
            &format!(
                "/treasury/financial_accounts/{financial_account}/features",
                financial_account = financial_account
            ),
            params,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Encodes whether a FinancialAccount has access to a particular feature.
    ///
    /// Stripe or the platform can control features via the requested field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateFinancialAccountFeatures>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<CreateFinancialAccountPlatformRestrictions>,
    /// The currencies the FinancialAccount can hold a balance in.
    pub supported_currencies: &'a [&'a str],
}
impl<'a> CreateFinancialAccount<'a> {
    pub fn new(supported_currencies: &'a [&'a str]) -> Self {
        Self {
            expand: Default::default(),
            features: Default::default(),
            metadata: Default::default(),
            platform_restrictions: Default::default(),
            supported_currencies,
        }
    }
}
/// Encodes whether a FinancialAccount has access to a particular feature.
///
/// Stripe or the platform can control features via the requested field.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateFinancialAccountFeatures {
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CreateFinancialAccountFeaturesCardIssuing>,
    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    ///
    /// Various factors determine the insurance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<CreateFinancialAccountFeaturesDepositInsurance>,
    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<CreateFinancialAccountFeaturesFinancialAddresses>,
    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<CreateFinancialAccountFeaturesInboundTransfers>,
    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<CreateFinancialAccountFeaturesIntraStripeFlows>,
    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<CreateFinancialAccountFeaturesOutboundPayments>,
    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<CreateFinancialAccountFeaturesOutboundTransfers>,
}
impl CreateFinancialAccountFeatures {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesCardIssuing {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateFinancialAccountFeaturesCardIssuing {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents whether this FinancialAccount is eligible for deposit insurance.
///
/// Various factors determine the insurance amount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesDepositInsurance {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateFinancialAccountFeaturesDepositInsurance {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains Features that add FinancialAddresses to the FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<CreateFinancialAccountFeaturesFinancialAddressesAba>,
}
impl CreateFinancialAccountFeaturesFinancialAddresses {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Adds an ABA FinancialAddress to the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesFinancialAddressesAba {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateFinancialAccountFeaturesFinancialAddressesAba {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<CreateFinancialAccountFeaturesInboundTransfersAch>,
}
impl CreateFinancialAccountFeaturesInboundTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH Debits via the InboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesInboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateFinancialAccountFeaturesInboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateFinancialAccountFeaturesIntraStripeFlows {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesOutboundPayments {
    /// Enables ACH transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<CreateFinancialAccountFeaturesOutboundPaymentsAch>,
    /// Enables US domestic wire tranfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<CreateFinancialAccountFeaturesOutboundPaymentsUsDomesticWire>,
}
impl CreateFinancialAccountFeaturesOutboundPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesOutboundPaymentsAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateFinancialAccountFeaturesOutboundPaymentsAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire tranfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesOutboundTransfers {
    /// Enables ACH transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<CreateFinancialAccountFeaturesOutboundTransfersAch>,
    /// Enables US domestic wire tranfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<CreateFinancialAccountFeaturesOutboundTransfersUsDomesticWire>,
}
impl CreateFinancialAccountFeaturesOutboundTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesOutboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateFinancialAccountFeaturesOutboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire tranfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl CreateFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// The set of functionalities that the platform can restrict on the FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateFinancialAccountPlatformRestrictions {
    /// Restricts all inbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_flows: Option<CreateFinancialAccountPlatformRestrictionsInboundFlows>,
    /// Restricts all outbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_flows: Option<CreateFinancialAccountPlatformRestrictionsOutboundFlows>,
}
impl CreateFinancialAccountPlatformRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Restricts all inbound money movement.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateFinancialAccountPlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}

impl CreateFinancialAccountPlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for CreateFinancialAccountPlatformRestrictionsInboundFlows {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "restricted" => Ok(Self::Restricted),
            "unrestricted" => Ok(Self::Unrestricted),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateFinancialAccountPlatformRestrictionsInboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateFinancialAccountPlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateFinancialAccountPlatformRestrictionsInboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Restricts all outbound money movement.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateFinancialAccountPlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}

impl CreateFinancialAccountPlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for CreateFinancialAccountPlatformRestrictionsOutboundFlows {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "restricted" => Ok(Self::Restricted),
            "unrestricted" => Ok(Self::Unrestricted),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateFinancialAccountPlatformRestrictionsOutboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateFinancialAccountPlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateFinancialAccountPlatformRestrictionsOutboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFinancialAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Encodes whether a FinancialAccount has access to a particular feature, with a status enum and associated `status_details`.
    ///
    /// Stripe or the platform may control features via the requested field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<UpdateFinancialAccountFeatures>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<UpdateFinancialAccountPlatformRestrictions>,
}
impl<'a> UpdateFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Encodes whether a FinancialAccount has access to a particular feature, with a status enum and associated `status_details`.
///
/// Stripe or the platform may control features via the requested field.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFinancialAccountFeatures {
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<UpdateFinancialAccountFeaturesCardIssuing>,
    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    ///
    /// Various factors determine the insurance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<UpdateFinancialAccountFeaturesDepositInsurance>,
    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<UpdateFinancialAccountFeaturesFinancialAddresses>,
    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<UpdateFinancialAccountFeaturesInboundTransfers>,
    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<UpdateFinancialAccountFeaturesIntraStripeFlows>,
    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<UpdateFinancialAccountFeaturesOutboundPayments>,
    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<UpdateFinancialAccountFeaturesOutboundTransfers>,
}
impl UpdateFinancialAccountFeatures {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesCardIssuing {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFinancialAccountFeaturesCardIssuing {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents whether this FinancialAccount is eligible for deposit insurance.
///
/// Various factors determine the insurance amount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesDepositInsurance {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFinancialAccountFeaturesDepositInsurance {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains Features that add FinancialAddresses to the FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<UpdateFinancialAccountFeaturesFinancialAddressesAba>,
}
impl UpdateFinancialAccountFeaturesFinancialAddresses {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Adds an ABA FinancialAddress to the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesFinancialAddressesAba {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFinancialAccountFeaturesFinancialAddressesAba {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFinancialAccountFeaturesInboundTransfersAch>,
}
impl UpdateFinancialAccountFeaturesInboundTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH Debits via the InboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesInboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFinancialAccountFeaturesInboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFinancialAccountFeaturesIntraStripeFlows {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesOutboundPayments {
    /// Enables ACH transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFinancialAccountFeaturesOutboundPaymentsAch>,
    /// Enables US domestic wire tranfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<UpdateFinancialAccountFeaturesOutboundPaymentsUsDomesticWire>,
}
impl UpdateFinancialAccountFeaturesOutboundPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesOutboundPaymentsAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFinancialAccountFeaturesOutboundPaymentsAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire tranfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFinancialAccountFeaturesOutboundPaymentsUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesOutboundTransfers {
    /// Enables ACH transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFinancialAccountFeaturesOutboundTransfersAch>,
    /// Enables US domestic wire tranfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<UpdateFinancialAccountFeaturesOutboundTransfersUsDomesticWire>,
}
impl UpdateFinancialAccountFeaturesOutboundTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesOutboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFinancialAccountFeaturesOutboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire tranfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFinancialAccountFeaturesOutboundTransfersUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// The set of functionalities that the platform can restrict on the FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFinancialAccountPlatformRestrictions {
    /// Restricts all inbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_flows: Option<UpdateFinancialAccountPlatformRestrictionsInboundFlows>,
    /// Restricts all outbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_flows: Option<UpdateFinancialAccountPlatformRestrictionsOutboundFlows>,
}
impl UpdateFinancialAccountPlatformRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Restricts all inbound money movement.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateFinancialAccountPlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}

impl UpdateFinancialAccountPlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for UpdateFinancialAccountPlatformRestrictionsInboundFlows {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "restricted" => Ok(Self::Restricted),
            "unrestricted" => Ok(Self::Unrestricted),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateFinancialAccountPlatformRestrictionsInboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateFinancialAccountPlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UpdateFinancialAccountPlatformRestrictionsInboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Restricts all outbound money movement.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateFinancialAccountPlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}

impl UpdateFinancialAccountPlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for UpdateFinancialAccountPlatformRestrictionsOutboundFlows {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "restricted" => Ok(Self::Restricted),
            "unrestricted" => Ok(Self::Unrestricted),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateFinancialAccountPlatformRestrictionsOutboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateFinancialAccountPlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UpdateFinancialAccountPlatformRestrictionsOutboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccount<'a> {
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<UpdateFeaturesFinancialAccountCardIssuing>,
    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    ///
    /// Various factors determine the insurance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<UpdateFeaturesFinancialAccountDepositInsurance>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<UpdateFeaturesFinancialAccountFinancialAddresses>,
    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<UpdateFeaturesFinancialAccountInboundTransfers>,
    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<UpdateFeaturesFinancialAccountIntraStripeFlows>,
    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<UpdateFeaturesFinancialAccountOutboundPayments>,
    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<UpdateFeaturesFinancialAccountOutboundTransfers>,
}
impl<'a> UpdateFeaturesFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountCardIssuing {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesFinancialAccountCardIssuing {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents whether this FinancialAccount is eligible for deposit insurance.
///
/// Various factors determine the insurance amount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountDepositInsurance {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesFinancialAccountDepositInsurance {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains Features that add FinancialAddresses to the FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<UpdateFeaturesFinancialAccountFinancialAddressesAba>,
}
impl UpdateFeaturesFinancialAccountFinancialAddresses {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Adds an ABA FinancialAddress to the FinancialAccount.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountFinancialAddressesAba {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesFinancialAccountFinancialAddressesAba {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFeaturesFinancialAccountInboundTransfersAch>,
}
impl UpdateFeaturesFinancialAccountInboundTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH Debits via the InboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountInboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesFinancialAccountInboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesFinancialAccountIntraStripeFlows {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountOutboundPayments {
    /// Enables ACH transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFeaturesFinancialAccountOutboundPaymentsAch>,
    /// Enables US domestic wire tranfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<UpdateFeaturesFinancialAccountOutboundPaymentsUsDomesticWire>,
}
impl UpdateFeaturesFinancialAccountOutboundPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH transfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountOutboundPaymentsAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesFinancialAccountOutboundPaymentsAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire tranfers via the OutboundPayments API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesFinancialAccountOutboundPaymentsUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountOutboundTransfers {
    /// Enables ACH transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<UpdateFeaturesFinancialAccountOutboundTransfersAch>,
    /// Enables US domestic wire tranfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<UpdateFeaturesFinancialAccountOutboundTransfersUsDomesticWire>,
}
impl UpdateFeaturesFinancialAccountOutboundTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Enables ACH transfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountOutboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesFinancialAccountOutboundTransfersAch {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
/// Enables US domestic wire tranfers via the OutboundTransfers API.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateFeaturesFinancialAccountOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}
impl UpdateFeaturesFinancialAccountOutboundTransfersUsDomesticWire {
    pub fn new(requested: bool) -> Self {
        Self { requested }
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListFinancialAccount<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// An object ID cursor for use in pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit ranging from 1 to 100 (defaults to 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// An object ID cursor for use in pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveFinancialAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveFeaturesFinancialAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFeaturesFinancialAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
