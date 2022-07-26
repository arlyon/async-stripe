// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::ids::TreasuryFinancialAccountId;
use crate::params::{Metadata, Object, Timestamp};
use crate::resources::TreasuryFinancialAccountFeatures;

/// The resource representing a Stripe "AccountServiceResourceTreasuryFinancialAccount".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccount {
    /// Unique identifier for the object.
    pub id: TreasuryFinancialAccountId,

    /// The array of paths to active Features in the Features hash.
    pub active_features: Vec<TreasuryFinancialAccountActiveFeatures>,

    pub balance: AccountServiceResourceTreasuryBalance,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<TreasuryFinancialAccountFeatures>,

    /// The set of credentials that resolve to a FinancialAccount.
    pub financial_addresses: Vec<AccountServiceResourceTreasuryFinancialAddress>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The array of paths to pending Features in the Features hash.
    pub pending_features: Vec<TreasuryFinancialAccountPendingFeatures>,

    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<AccountServiceResourceTreasuryPlatformRestrictions>,

    /// The array of paths to restricted Features in the Features hash.
    pub restricted_features: Vec<TreasuryFinancialAccountRestrictedFeatures>,

    /// The enum specifying what state the account is in.
    pub status: TreasuryFinancialAccountStatus,

    pub status_details: AccountServiceResourceTreasuryStatusDetails,

    /// The currencies the FinancialAccount can hold a balance in.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    pub supported_currencies: Vec<String>,
}

impl Object for TreasuryFinancialAccount {
    type Id = TreasuryFinancialAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "treasury.financial_account"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceTreasuryBalance {
    /// Funds the user can spend right now.
    pub cash: i64,

    /// Funds not spendable yet, but will become available at a later time.
    pub inbound_pending: i64,

    /// Funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceTreasuryFinancialAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<AccountServiceResourceTreasuryAbaRecord>,

    /// The list of networks that the address supports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks:
        Option<Vec<AccountServiceResourceTreasuryFinancialAddressSupportedNetworks>>,

    /// The type of financial address.
    #[serde(rename = "type")]
    pub type_: AccountServiceResourceTreasuryFinancialAddressType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceTreasuryAbaRecord {
    /// The name of the person or business that owns the bank account.
    pub account_holder_name: String,

    /// The account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    /// The last four characters of the account number.
    pub account_number_last4: String,

    /// Name of the bank.
    pub bank_name: String,

    /// Routing number for the account.
    pub routing_number: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceTreasuryPlatformRestrictions {
    /// Restricts all inbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_flows: Option<AccountServiceResourceTreasuryPlatformRestrictionsInboundFlows>,

    /// Restricts all outbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_flows: Option<AccountServiceResourceTreasuryPlatformRestrictionsOutboundFlows>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceTreasuryStatusDetails {
    /// Details related to the closure of this FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<AccountServiceResourceTreasuryClosedStatusDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceTreasuryClosedStatusDetails {
    /// The array that contains reasons for a FinancialAccount closure.
    pub reasons: Vec<AccountServiceResourceTreasuryClosedStatusDetailsReasons>,
}

/// An enum representing the possible values of an `AccountServiceResourceTreasuryClosedStatusDetails`'s `reasons` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceTreasuryClosedStatusDetailsReasons {
    AccountRejected,
    ClosedByPlatform,
    Other,
}

impl AccountServiceResourceTreasuryClosedStatusDetailsReasons {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceTreasuryClosedStatusDetailsReasons::AccountRejected => {
                "account_rejected"
            }
            AccountServiceResourceTreasuryClosedStatusDetailsReasons::ClosedByPlatform => {
                "closed_by_platform"
            }
            AccountServiceResourceTreasuryClosedStatusDetailsReasons::Other => "other",
        }
    }
}

impl AsRef<str> for AccountServiceResourceTreasuryClosedStatusDetailsReasons {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceTreasuryClosedStatusDetailsReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountServiceResourceTreasuryClosedStatusDetailsReasons {
    fn default() -> Self {
        Self::AccountRejected
    }
}

/// An enum representing the possible values of an `AccountServiceResourceTreasuryFinancialAddress`'s `supported_networks` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceTreasuryFinancialAddressSupportedNetworks {
    Ach,
    UsDomesticWire,
}

impl AccountServiceResourceTreasuryFinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceTreasuryFinancialAddressSupportedNetworks::Ach => "ach",
            AccountServiceResourceTreasuryFinancialAddressSupportedNetworks::UsDomesticWire => {
                "us_domestic_wire"
            }
        }
    }
}

impl AsRef<str> for AccountServiceResourceTreasuryFinancialAddressSupportedNetworks {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceTreasuryFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountServiceResourceTreasuryFinancialAddressSupportedNetworks {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `AccountServiceResourceTreasuryFinancialAddress`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceTreasuryFinancialAddressType {
    Aba,
}

impl AccountServiceResourceTreasuryFinancialAddressType {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceTreasuryFinancialAddressType::Aba => "aba",
        }
    }
}

impl AsRef<str> for AccountServiceResourceTreasuryFinancialAddressType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceTreasuryFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountServiceResourceTreasuryFinancialAddressType {
    fn default() -> Self {
        Self::Aba
    }
}

/// An enum representing the possible values of an `AccountServiceResourceTreasuryPlatformRestrictions`'s `inbound_flows` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceTreasuryPlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}

impl AccountServiceResourceTreasuryPlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceTreasuryPlatformRestrictionsInboundFlows::Restricted => {
                "restricted"
            }
            AccountServiceResourceTreasuryPlatformRestrictionsInboundFlows::Unrestricted => {
                "unrestricted"
            }
        }
    }
}

impl AsRef<str> for AccountServiceResourceTreasuryPlatformRestrictionsInboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceTreasuryPlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountServiceResourceTreasuryPlatformRestrictionsInboundFlows {
    fn default() -> Self {
        Self::Restricted
    }
}

/// An enum representing the possible values of an `AccountServiceResourceTreasuryPlatformRestrictions`'s `outbound_flows` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceTreasuryPlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}

impl AccountServiceResourceTreasuryPlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceTreasuryPlatformRestrictionsOutboundFlows::Restricted => {
                "restricted"
            }
            AccountServiceResourceTreasuryPlatformRestrictionsOutboundFlows::Unrestricted => {
                "unrestricted"
            }
        }
    }
}

impl AsRef<str> for AccountServiceResourceTreasuryPlatformRestrictionsOutboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceTreasuryPlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountServiceResourceTreasuryPlatformRestrictionsOutboundFlows {
    fn default() -> Self {
        Self::Restricted
    }
}

/// An enum representing the possible values of an `TreasuryFinancialAccount`'s `active_features` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountActiveFeatures {
    CardIssuing,
    DepositInsurance,
    #[serde(rename = "financial_addresses.aba")]
    FinancialAddressesAba,
    #[serde(rename = "inbound_transfers.ach")]
    InboundTransfersAch,
    IntraStripeFlows,
    #[serde(rename = "outbound_payments.ach")]
    OutboundPaymentsAch,
    #[serde(rename = "outbound_payments.us_domestic_wire")]
    OutboundPaymentsUsDomesticWire,
    #[serde(rename = "outbound_transfers.ach")]
    OutboundTransfersAch,
    #[serde(rename = "outbound_transfers.us_domestic_wire")]
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

impl TreasuryFinancialAccountActiveFeatures {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountActiveFeatures::CardIssuing => "card_issuing",
            TreasuryFinancialAccountActiveFeatures::DepositInsurance => "deposit_insurance",
            TreasuryFinancialAccountActiveFeatures::FinancialAddressesAba => {
                "financial_addresses.aba"
            }
            TreasuryFinancialAccountActiveFeatures::InboundTransfersAch => "inbound_transfers.ach",
            TreasuryFinancialAccountActiveFeatures::IntraStripeFlows => "intra_stripe_flows",
            TreasuryFinancialAccountActiveFeatures::OutboundPaymentsAch => "outbound_payments.ach",
            TreasuryFinancialAccountActiveFeatures::OutboundPaymentsUsDomesticWire => {
                "outbound_payments.us_domestic_wire"
            }
            TreasuryFinancialAccountActiveFeatures::OutboundTransfersAch => {
                "outbound_transfers.ach"
            }
            TreasuryFinancialAccountActiveFeatures::OutboundTransfersUsDomesticWire => {
                "outbound_transfers.us_domestic_wire"
            }
            TreasuryFinancialAccountActiveFeatures::RemoteDepositCapture => {
                "remote_deposit_capture"
            }
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountActiveFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountActiveFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountActiveFeatures {
    fn default() -> Self {
        Self::CardIssuing
    }
}

/// An enum representing the possible values of an `TreasuryFinancialAccount`'s `pending_features` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountPendingFeatures {
    CardIssuing,
    DepositInsurance,
    #[serde(rename = "financial_addresses.aba")]
    FinancialAddressesAba,
    #[serde(rename = "inbound_transfers.ach")]
    InboundTransfersAch,
    IntraStripeFlows,
    #[serde(rename = "outbound_payments.ach")]
    OutboundPaymentsAch,
    #[serde(rename = "outbound_payments.us_domestic_wire")]
    OutboundPaymentsUsDomesticWire,
    #[serde(rename = "outbound_transfers.ach")]
    OutboundTransfersAch,
    #[serde(rename = "outbound_transfers.us_domestic_wire")]
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

impl TreasuryFinancialAccountPendingFeatures {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountPendingFeatures::CardIssuing => "card_issuing",
            TreasuryFinancialAccountPendingFeatures::DepositInsurance => "deposit_insurance",
            TreasuryFinancialAccountPendingFeatures::FinancialAddressesAba => {
                "financial_addresses.aba"
            }
            TreasuryFinancialAccountPendingFeatures::InboundTransfersAch => "inbound_transfers.ach",
            TreasuryFinancialAccountPendingFeatures::IntraStripeFlows => "intra_stripe_flows",
            TreasuryFinancialAccountPendingFeatures::OutboundPaymentsAch => "outbound_payments.ach",
            TreasuryFinancialAccountPendingFeatures::OutboundPaymentsUsDomesticWire => {
                "outbound_payments.us_domestic_wire"
            }
            TreasuryFinancialAccountPendingFeatures::OutboundTransfersAch => {
                "outbound_transfers.ach"
            }
            TreasuryFinancialAccountPendingFeatures::OutboundTransfersUsDomesticWire => {
                "outbound_transfers.us_domestic_wire"
            }
            TreasuryFinancialAccountPendingFeatures::RemoteDepositCapture => {
                "remote_deposit_capture"
            }
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountPendingFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountPendingFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountPendingFeatures {
    fn default() -> Self {
        Self::CardIssuing
    }
}

/// An enum representing the possible values of an `TreasuryFinancialAccount`'s `restricted_features` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountRestrictedFeatures {
    CardIssuing,
    DepositInsurance,
    #[serde(rename = "financial_addresses.aba")]
    FinancialAddressesAba,
    #[serde(rename = "inbound_transfers.ach")]
    InboundTransfersAch,
    IntraStripeFlows,
    #[serde(rename = "outbound_payments.ach")]
    OutboundPaymentsAch,
    #[serde(rename = "outbound_payments.us_domestic_wire")]
    OutboundPaymentsUsDomesticWire,
    #[serde(rename = "outbound_transfers.ach")]
    OutboundTransfersAch,
    #[serde(rename = "outbound_transfers.us_domestic_wire")]
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

impl TreasuryFinancialAccountRestrictedFeatures {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountRestrictedFeatures::CardIssuing => "card_issuing",
            TreasuryFinancialAccountRestrictedFeatures::DepositInsurance => "deposit_insurance",
            TreasuryFinancialAccountRestrictedFeatures::FinancialAddressesAba => {
                "financial_addresses.aba"
            }
            TreasuryFinancialAccountRestrictedFeatures::InboundTransfersAch => {
                "inbound_transfers.ach"
            }
            TreasuryFinancialAccountRestrictedFeatures::IntraStripeFlows => "intra_stripe_flows",
            TreasuryFinancialAccountRestrictedFeatures::OutboundPaymentsAch => {
                "outbound_payments.ach"
            }
            TreasuryFinancialAccountRestrictedFeatures::OutboundPaymentsUsDomesticWire => {
                "outbound_payments.us_domestic_wire"
            }
            TreasuryFinancialAccountRestrictedFeatures::OutboundTransfersAch => {
                "outbound_transfers.ach"
            }
            TreasuryFinancialAccountRestrictedFeatures::OutboundTransfersUsDomesticWire => {
                "outbound_transfers.us_domestic_wire"
            }
            TreasuryFinancialAccountRestrictedFeatures::RemoteDepositCapture => {
                "remote_deposit_capture"
            }
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountRestrictedFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountRestrictedFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountRestrictedFeatures {
    fn default() -> Self {
        Self::CardIssuing
    }
}

/// An enum representing the possible values of an `TreasuryFinancialAccount`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountStatus {
    Closed,
    Open,
}

impl TreasuryFinancialAccountStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountStatus::Closed => "closed",
            TreasuryFinancialAccountStatus::Open => "open",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountStatus {
    fn default() -> Self {
        Self::Closed
    }
}
