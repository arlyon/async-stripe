// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::ids::TreasuryOutboundPaymentId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Currency, TreasurySharedResourceBillingDetails, TreasuryTransaction};

/// The resource representing a Stripe "OutboundPaymentsResourceTreasuryOutboundPayment".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryOutboundPayment {
    /// Unique identifier for the object.
    pub id: TreasuryOutboundPaymentId,

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

    /// ID of the [customer](https://stripe.com/docs/api/customers) to whom an OutboundPayment is sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The PaymentMethod via which an OutboundPayment is sent.
    ///
    /// This field can be empty if the OutboundPayment was created using `destination_payment_method_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method: Option<String>,

    /// Details about the PaymentMethod for an OutboundPayment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method_details: Option<OutboundPaymentsPaymentMethodDetails>,

    /// Details about the end user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_details:
        Option<OutboundPaymentsResourceTreasuryOutboundPaymentResourceEndUserDetails>,

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

    /// Details about a returned OutboundPayment.
    ///
    /// Only set when the status is `returned`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_details: Option<OutboundPaymentsResourceTreasuryReturnedStatus>,

    /// The description that appears on the receiving end for an OutboundPayment (for example, bank statement for external bank transfer).
    pub statement_descriptor: String,

    /// Current status of the OutboundPayment: `processing`, `failed`, `posted`, `returned`, `canceled`.
    ///
    /// An OutboundPayment is `processing` if it has been created and is pending.
    /// The status changes to `posted` once the OutboundPayment has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
    /// If an OutboundPayment fails to arrive at its destination, its status will change to `returned`.
    pub status: TreasuryOutboundPaymentStatus,

    pub status_transitions:
        OutboundPaymentsResourceTreasuryOutboundPaymentResourceStatusTransitions,

    /// The Transaction associated with this object.
    pub transaction: Expandable<TreasuryTransaction>,
}

impl Object for TreasuryOutboundPayment {
    type Id = TreasuryOutboundPaymentId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "treasury.outbound_payment"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutboundPaymentsPaymentMethodDetails {
    pub billing_details: TreasurySharedResourceBillingDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<OutboundPaymentsPaymentMethodDetailsFinancialAccount>,

    /// The type of the payment method used in the OutboundPayment.
    #[serde(rename = "type")]
    pub type_: OutboundPaymentsPaymentMethodDetailsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutboundPaymentsPaymentMethodDetailsFinancialAccount {
    /// Token of the FinancialAccount.
    pub id: String,

    /// The rails used to send funds.
    pub network: OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutboundPaymentsPaymentMethodDetailsUsBankAccount {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType>,

    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType>,

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
    pub network: OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork,

    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutboundPaymentsResourceTreasuryOutboundPaymentResourceEndUserDetails {
    /// IP address of the user initiating the OutboundPayment.
    ///
    /// Set if `present` is set to `true`.
    /// IP address collection is required for risk and compliance reasons.
    /// This will be used to help determine if the OutboundPayment is authorized or should be blocked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// `true`` if the OutboundPayment creation request is being made on behalf of an end user by a platform.
    ///
    /// Otherwise, `false`.
    pub present: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutboundPaymentsResourceTreasuryOutboundPaymentResourceStatusTransitions {
    /// Timestamp describing when an OutboundPayment changed status to `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<Timestamp>,

    /// Timestamp describing when an OutboundPayment changed status to `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<Timestamp>,

    /// Timestamp describing when an OutboundPayment changed status to `posted`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<Timestamp>,

    /// Timestamp describing when an OutboundPayment changed status to `returned`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutboundPaymentsResourceTreasuryReturnedStatus {
    /// Reason for the return.
    pub code: OutboundPaymentsResourceTreasuryReturnedStatusCode,

    /// The Transaction associated with this object.
    pub transaction: Expandable<TreasuryTransaction>,
}

/// An enum representing the possible values of an `OutboundPaymentsPaymentMethodDetailsFinancialAccount`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    Stripe,
}

impl OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn default() -> Self {
        Self::Stripe
    }
}

/// An enum representing the possible values of an `OutboundPaymentsPaymentMethodDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentsPaymentMethodDetailsType {
    FinancialAccount,
    UsBankAccount,
}

impl OutboundPaymentsPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            OutboundPaymentsPaymentMethodDetailsType::FinancialAccount => "financial_account",
            OutboundPaymentsPaymentMethodDetailsType::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OutboundPaymentsPaymentMethodDetailsType {
    fn default() -> Self {
        Self::FinancialAccount
    }
}

/// An enum representing the possible values of an `OutboundPaymentsPaymentMethodDetailsUsBankAccount`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType::Company => {
                "company"
            }
            OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType::Individual => {
                "individual"
            }
        }
    }
}

impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `OutboundPaymentsPaymentMethodDetailsUsBankAccount`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}

impl OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType::Checking => "checking",
            OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType::Savings => "savings",
        }
    }
}

impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `OutboundPaymentsPaymentMethodDetailsUsBankAccount`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}

impl OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::Ach => "ach",
            OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::UsDomesticWire => {
                "us_domestic_wire"
            }
        }
    }
}

impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `OutboundPaymentsResourceTreasuryReturnedStatus`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentsResourceTreasuryReturnedStatusCode {
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

impl OutboundPaymentsResourceTreasuryReturnedStatusCode {
    pub fn as_str(self) -> &'static str {
        match self {
            OutboundPaymentsResourceTreasuryReturnedStatusCode::AccountClosed => "account_closed",
            OutboundPaymentsResourceTreasuryReturnedStatusCode::AccountFrozen => "account_frozen",
            OutboundPaymentsResourceTreasuryReturnedStatusCode::BankAccountRestricted => {
                "bank_account_restricted"
            }
            OutboundPaymentsResourceTreasuryReturnedStatusCode::BankOwnershipChanged => {
                "bank_ownership_changed"
            }
            OutboundPaymentsResourceTreasuryReturnedStatusCode::Declined => "declined",
            OutboundPaymentsResourceTreasuryReturnedStatusCode::IncorrectAccountHolderName => {
                "incorrect_account_holder_name"
            }
            OutboundPaymentsResourceTreasuryReturnedStatusCode::InvalidAccountNumber => {
                "invalid_account_number"
            }
            OutboundPaymentsResourceTreasuryReturnedStatusCode::InvalidCurrency => {
                "invalid_currency"
            }
            OutboundPaymentsResourceTreasuryReturnedStatusCode::NoAccount => "no_account",
            OutboundPaymentsResourceTreasuryReturnedStatusCode::Other => "other",
        }
    }
}

impl AsRef<str> for OutboundPaymentsResourceTreasuryReturnedStatusCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentsResourceTreasuryReturnedStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OutboundPaymentsResourceTreasuryReturnedStatusCode {
    fn default() -> Self {
        Self::AccountClosed
    }
}

/// An enum representing the possible values of an `TreasuryOutboundPayment`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryOutboundPaymentStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl TreasuryOutboundPaymentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryOutboundPaymentStatus::Canceled => "canceled",
            TreasuryOutboundPaymentStatus::Failed => "failed",
            TreasuryOutboundPaymentStatus::Posted => "posted",
            TreasuryOutboundPaymentStatus::Processing => "processing",
            TreasuryOutboundPaymentStatus::Returned => "returned",
        }
    }
}

impl AsRef<str> for TreasuryOutboundPaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryOutboundPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryOutboundPaymentStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
