// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::ids::TreasuryOutboundTransferId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Currency, TreasurySharedResourceBillingDetails, TreasuryTransaction};

/// The resource representing a Stripe "TreasuryOutboundTransfersResourceOutboundTransfer".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryOutboundTransfer {
    /// Unique identifier for the object.
    pub id: TreasuryOutboundTransferId,

    /// Amount (in cents) transferred.
    pub amount: i64,

    /// Returns `true` if the object can be canceled, and `false` otherwise.
    pub cancelable: bool,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The PaymentMethod used as the payment instrument for an OutboundTransfer.
    pub destination_payment_method: String,

    pub destination_payment_method_details: OutboundTransfersPaymentMethodDetails,

    /// The date when funds are expected to arrive in the destination account.
    pub expected_arrival_date: Timestamp,

    /// The FinancialAccount that funds were pulled from.
    pub financial_account: String,

    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Details about a returned OutboundTransfer.
    ///
    /// Only set when the status is `returned`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_details: Option<TreasuryOutboundTransfersResourceReturnedDetails>,

    /// Information about the OutboundTransfer to be sent to the recipient account.
    pub statement_descriptor: String,

    /// Current status of the OutboundTransfer: `processing`, `failed`, `canceled`, `posted`, `returned`.
    ///
    /// An OutboundTransfer is `processing` if it has been created and is pending.
    /// The status changes to `posted` once the OutboundTransfer has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
    /// If an OutboundTransfer fails to arrive at its destination, its status will change to `returned`.
    pub status: TreasuryOutboundTransferStatus,

    pub status_transitions: TreasuryOutboundTransfersResourceStatusTransitions,

    /// The Transaction associated with this object.
    pub transaction: Expandable<TreasuryTransaction>,
}

impl Object for TreasuryOutboundTransfer {
    type Id = TreasuryOutboundTransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "treasury.outbound_transfer"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutboundTransfersPaymentMethodDetails {
    pub billing_details: TreasurySharedResourceBillingDetails,

    /// The type of the payment method used in the OutboundTransfer.
    #[serde(rename = "type")]
    pub type_: OutboundTransfersPaymentMethodDetailsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<OutboundTransfersPaymentMethodDetailsUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutboundTransfersPaymentMethodDetailsUsBankAccount {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType>,

    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType>,

    /// Name of the bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    /// The US bank account network used to send funds.
    pub network: OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork,

    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryOutboundTransfersResourceReturnedDetails {
    /// Reason for the return.
    pub code: TreasuryOutboundTransfersResourceReturnedDetailsCode,

    /// The Transaction associated with this object.
    pub transaction: Expandable<TreasuryTransaction>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryOutboundTransfersResourceStatusTransitions {
    /// Timestamp describing when an OutboundTransfer changed status to `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<Timestamp>,

    /// Timestamp describing when an OutboundTransfer changed status to `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<Timestamp>,

    /// Timestamp describing when an OutboundTransfer changed status to `posted`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<Timestamp>,

    /// Timestamp describing when an OutboundTransfer changed status to `returned`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<Timestamp>,
}

/// An enum representing the possible values of an `OutboundTransfersPaymentMethodDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutboundTransfersPaymentMethodDetailsType {
    UsBankAccount,
}

impl OutboundTransfersPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            OutboundTransfersPaymentMethodDetailsType::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for OutboundTransfersPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundTransfersPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OutboundTransfersPaymentMethodDetailsType {
    fn default() -> Self {
        Self::UsBankAccount
    }
}

/// An enum representing the possible values of an `OutboundTransfersPaymentMethodDetailsUsBankAccount`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::Company => {
                "company"
            }
            OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::Individual => {
                "individual"
            }
        }
    }
}

impl AsRef<str> for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `OutboundTransfersPaymentMethodDetailsUsBankAccount`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}

impl OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType::Checking => "checking",
            OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType::Savings => "savings",
        }
    }
}

impl AsRef<str> for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `OutboundTransfersPaymentMethodDetailsUsBankAccount`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}

impl OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork::Ach => "ach",
            OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork::UsDomesticWire => {
                "us_domestic_wire"
            }
        }
    }
}

impl AsRef<str> for OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `TreasuryOutboundTransfer`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryOutboundTransferStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl TreasuryOutboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryOutboundTransferStatus::Canceled => "canceled",
            TreasuryOutboundTransferStatus::Failed => "failed",
            TreasuryOutboundTransferStatus::Posted => "posted",
            TreasuryOutboundTransferStatus::Processing => "processing",
            TreasuryOutboundTransferStatus::Returned => "returned",
        }
    }
}

impl AsRef<str> for TreasuryOutboundTransferStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryOutboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryOutboundTransferStatus {
    fn default() -> Self {
        Self::Canceled
    }
}

/// An enum representing the possible values of an `TreasuryOutboundTransfersResourceReturnedDetails`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryOutboundTransfersResourceReturnedDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    Declined,
    IncorrectAccountHolderName,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
}

impl TreasuryOutboundTransfersResourceReturnedDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryOutboundTransfersResourceReturnedDetailsCode::AccountClosed => "account_closed",
            TreasuryOutboundTransfersResourceReturnedDetailsCode::AccountFrozen => "account_frozen",
            TreasuryOutboundTransfersResourceReturnedDetailsCode::BankAccountRestricted => {
                "bank_account_restricted"
            }
            TreasuryOutboundTransfersResourceReturnedDetailsCode::BankOwnershipChanged => {
                "bank_ownership_changed"
            }
            TreasuryOutboundTransfersResourceReturnedDetailsCode::Declined => "declined",
            TreasuryOutboundTransfersResourceReturnedDetailsCode::IncorrectAccountHolderName => {
                "incorrect_account_holder_name"
            }
            TreasuryOutboundTransfersResourceReturnedDetailsCode::InvalidAccountNumber => {
                "invalid_account_number"
            }
            TreasuryOutboundTransfersResourceReturnedDetailsCode::InvalidCurrency => {
                "invalid_currency"
            }
            TreasuryOutboundTransfersResourceReturnedDetailsCode::NoAccount => "no_account",
            TreasuryOutboundTransfersResourceReturnedDetailsCode::Other => "other",
        }
    }
}

impl AsRef<str> for TreasuryOutboundTransfersResourceReturnedDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryOutboundTransfersResourceReturnedDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryOutboundTransfersResourceReturnedDetailsCode {
    fn default() -> Self {
        Self::AccountClosed
    }
}
