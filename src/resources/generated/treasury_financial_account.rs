// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{TreasuryFinancialAccountId};
use crate::params::{Metadata, Object, Timestamp};
use crate::resources::{TreasuryFinancialAccountFeatures};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasuryFinancialAccountsResourceFinancialAccount".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccount {
    /// Unique identifier for the object.
    pub id: TreasuryFinancialAccountId,

    /// The array of paths to active Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_features: Option<Vec<TreasuryFinancialAccountActiveFeatures>>,

    pub balance: TreasuryFinancialAccountsResourceBalance,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<TreasuryFinancialAccountFeatures>,

    /// The set of credentials that resolve to a FinancialAccount.
    pub financial_addresses: Vec<TreasuryFinancialAccountsResourceFinancialAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// The nickname for the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// The array of paths to pending Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_features: Option<Vec<TreasuryFinancialAccountPendingFeatures>>,

    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<TreasuryFinancialAccountsResourcePlatformRestrictions>,

    /// The array of paths to restricted Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_features: Option<Vec<TreasuryFinancialAccountRestrictedFeatures>>,

    /// Status of this FinancialAccount.
    pub status: TreasuryFinancialAccountStatus,

    pub status_details: TreasuryFinancialAccountsResourceStatusDetails,

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
pub struct TreasuryFinancialAccountsResourceBalance {

    /// Funds the user can spend right now.
    pub cash: i64,

    /// Funds not spendable yet, but will become available at a later time.
    pub inbound_pending: i64,

    /// Funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountsResourceFinancialAddress {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<TreasuryFinancialAccountsResourceAbaRecord>,

    /// The list of networks that the address supports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks: Option<Vec<TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks>>,

    /// The type of financial address.
    #[serde(rename = "type")]
    pub type_: TreasuryFinancialAccountsResourceFinancialAddressType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountsResourceAbaRecord {

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
pub struct TreasuryFinancialAccountsResourcePlatformRestrictions {

    /// Restricts all inbound money movement.
    pub inbound_flows: Option<TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows>,

    /// Restricts all outbound money movement.
    pub outbound_flows: Option<TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountsResourceStatusDetails {

    /// Details related to the closure of this FinancialAccount.
    pub closed: Option<TreasuryFinancialAccountsResourceClosedStatusDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountsResourceClosedStatusDetails {

    /// The array that contains reasons for a FinancialAccount closure.
    pub reasons: Vec<TreasuryFinancialAccountsResourceClosedStatusDetailsReasons>,
}

/// An enum representing the possible values of an `TreasuryFinancialAccount`'s `active_features` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountActiveFeatures {
    CardIssuing,
    DepositInsurance,
    #[serde(rename = "financial_addresses.aba")]
    FinancialAddressesAba,
    #[serde(rename = "financial_addresses.aba.forwarding")]
    FinancialAddressesAbaForwarding,
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
            TreasuryFinancialAccountActiveFeatures::FinancialAddressesAba => "financial_addresses.aba",
            TreasuryFinancialAccountActiveFeatures::FinancialAddressesAbaForwarding => "financial_addresses.aba.forwarding",
            TreasuryFinancialAccountActiveFeatures::InboundTransfersAch => "inbound_transfers.ach",
            TreasuryFinancialAccountActiveFeatures::IntraStripeFlows => "intra_stripe_flows",
            TreasuryFinancialAccountActiveFeatures::OutboundPaymentsAch => "outbound_payments.ach",
            TreasuryFinancialAccountActiveFeatures::OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            TreasuryFinancialAccountActiveFeatures::OutboundTransfersAch => "outbound_transfers.ach",
            TreasuryFinancialAccountActiveFeatures::OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            TreasuryFinancialAccountActiveFeatures::RemoteDepositCapture => "remote_deposit_capture",
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
    #[serde(rename = "financial_addresses.aba.forwarding")]
    FinancialAddressesAbaForwarding,
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
            TreasuryFinancialAccountPendingFeatures::FinancialAddressesAba => "financial_addresses.aba",
            TreasuryFinancialAccountPendingFeatures::FinancialAddressesAbaForwarding => "financial_addresses.aba.forwarding",
            TreasuryFinancialAccountPendingFeatures::InboundTransfersAch => "inbound_transfers.ach",
            TreasuryFinancialAccountPendingFeatures::IntraStripeFlows => "intra_stripe_flows",
            TreasuryFinancialAccountPendingFeatures::OutboundPaymentsAch => "outbound_payments.ach",
            TreasuryFinancialAccountPendingFeatures::OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            TreasuryFinancialAccountPendingFeatures::OutboundTransfersAch => "outbound_transfers.ach",
            TreasuryFinancialAccountPendingFeatures::OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            TreasuryFinancialAccountPendingFeatures::RemoteDepositCapture => "remote_deposit_capture",
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
    #[serde(rename = "financial_addresses.aba.forwarding")]
    FinancialAddressesAbaForwarding,
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
            TreasuryFinancialAccountRestrictedFeatures::FinancialAddressesAba => "financial_addresses.aba",
            TreasuryFinancialAccountRestrictedFeatures::FinancialAddressesAbaForwarding => "financial_addresses.aba.forwarding",
            TreasuryFinancialAccountRestrictedFeatures::InboundTransfersAch => "inbound_transfers.ach",
            TreasuryFinancialAccountRestrictedFeatures::IntraStripeFlows => "intra_stripe_flows",
            TreasuryFinancialAccountRestrictedFeatures::OutboundPaymentsAch => "outbound_payments.ach",
            TreasuryFinancialAccountRestrictedFeatures::OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            TreasuryFinancialAccountRestrictedFeatures::OutboundTransfersAch => "outbound_transfers.ach",
            TreasuryFinancialAccountRestrictedFeatures::OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            TreasuryFinancialAccountRestrictedFeatures::RemoteDepositCapture => "remote_deposit_capture",
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

/// An enum representing the possible values of an `TreasuryFinancialAccountsResourceClosedStatusDetails`'s `reasons` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    AccountRejected,
    ClosedByPlatform,
    Other,
}

impl TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountsResourceClosedStatusDetailsReasons::AccountRejected => "account_rejected",
            TreasuryFinancialAccountsResourceClosedStatusDetailsReasons::ClosedByPlatform => "closed_by_platform",
            TreasuryFinancialAccountsResourceClosedStatusDetailsReasons::Other => "other",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn default() -> Self {
        Self::AccountRejected
    }
}

/// An enum representing the possible values of an `TreasuryFinancialAccountsResourceFinancialAddress`'s `supported_networks` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    Ach,
    UsDomesticWire,
}

impl TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks::Ach => "ach",
            TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `TreasuryFinancialAccountsResourceFinancialAddress`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceFinancialAddressType {
    Aba,
}

impl TreasuryFinancialAccountsResourceFinancialAddressType {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountsResourceFinancialAddressType::Aba => "aba",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn default() -> Self {
        Self::Aba
    }
}

/// An enum representing the possible values of an `TreasuryFinancialAccountsResourcePlatformRestrictions`'s `inbound_flows` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}

impl TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::Restricted => "restricted",
            TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::Unrestricted => "unrestricted",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn default() -> Self {
        Self::Restricted
    }
}

/// An enum representing the possible values of an `TreasuryFinancialAccountsResourcePlatformRestrictions`'s `outbound_flows` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}

impl TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows::Restricted => "restricted",
            TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows::Unrestricted => "unrestricted",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn default() -> Self {
        Self::Restricted
    }
}
