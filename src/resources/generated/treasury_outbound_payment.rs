// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{TreasuryOutboundPaymentId};
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Currency, Mandate, TreasurySharedResourceBillingDetails, TreasuryTransaction};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasuryOutboundPaymentsResourceOutboundPayment".
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
    pub customer: Option<String>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// The PaymentMethod via which an OutboundPayment is sent.
    ///
    /// This field can be empty if the OutboundPayment was created using `destination_payment_method_data`.
    pub destination_payment_method: Option<String>,

    /// Details about the PaymentMethod for an OutboundPayment.
    pub destination_payment_method_details: Option<OutboundPaymentsPaymentMethodDetails>,

    /// Details about the end user.
    pub end_user_details: Option<TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails>,

    /// The date when funds are expected to arrive in the destination account.
    pub expected_arrival_date: Timestamp,

    /// The FinancialAccount that funds were pulled from.
    pub financial_account: String,

    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
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
    pub returned_details: Option<TreasuryOutboundPaymentsResourceReturnedStatus>,

    /// The description that appears on the receiving end for an OutboundPayment (for example, bank statement for external bank transfer).
    pub statement_descriptor: String,

    /// Current status of the OutboundPayment: `processing`, `failed`, `posted`, `returned`, `canceled`.
    ///
    /// An OutboundPayment is `processing` if it has been created and is pending.
    /// The status changes to `posted` once the OutboundPayment has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
    /// If an OutboundPayment fails to arrive at its destination, its status will change to `returned`.
    pub status: TreasuryOutboundPaymentStatus,

    pub status_transitions: TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions,

    /// Details about network-specific tracking information if available.
    pub tracking_details: Option<TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails>,

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
    pub account_holder_type: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType>,

    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    pub account_type: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType>,

    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// ID of the mandate used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<Expandable<Mandate>>,

    /// The network rails used.
    ///
    /// See the [docs](https://stripe.com/docs/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
    pub network: OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork,

    /// Routing number of the bank account.
    pub routing_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {

    /// IP address of the user initiating the OutboundPayment.
    ///
    /// Set if `present` is set to `true`.
    /// IP address collection is required for risk and compliance reasons.
    /// This will be used to help determine if the OutboundPayment is authorized or should be blocked.
    pub ip_address: Option<String>,

    /// `true` if the OutboundPayment creation request is being made on behalf of an end user by a platform.
    ///
    /// Otherwise, `false`.
    pub present: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {

    /// Timestamp describing when an OutboundPayment changed status to `canceled`.
    pub canceled_at: Option<Timestamp>,

    /// Timestamp describing when an OutboundPayment changed status to `failed`.
    pub failed_at: Option<Timestamp>,

    /// Timestamp describing when an OutboundPayment changed status to `posted`.
    pub posted_at: Option<Timestamp>,

    /// Timestamp describing when an OutboundPayment changed status to `returned`.
    pub returned_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<TreasuryOutboundPaymentsResourceAchTrackingDetails>,

    /// The US bank account network used to send funds.
    #[serde(rename = "type")]
    pub type_: TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryOutboundPaymentsResourceAchTrackingDetails {

    /// ACH trace ID of the OutboundPayment for payments sent over the `ach` network.
    pub trace_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryOutboundPaymentsResourceReturnedStatus {

    /// Reason for the return.
    pub code: TreasuryOutboundPaymentsResourceReturnedStatusCode,

    /// The Transaction associated with this object.
    pub transaction: Expandable<TreasuryTransaction>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails {

    /// CHIPS System Sequence Number (SSN) of the OutboundPayment for payments sent over the `us_domestic_wire` network.
    pub chips: Option<String>,

    /// IMAD of the OutboundPayment for payments sent over the `us_domestic_wire` network.
    pub imad: Option<String>,

    /// OMAD of the OutboundPayment for payments sent over the `us_domestic_wire` network.
    pub omad: Option<String>,
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
            OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType::Company => "company",
            OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType::Individual => "individual",
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
            OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::UsDomesticWire => "us_domestic_wire",
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

/// An enum representing the possible values of an `TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType {
    Ach,
    UsDomesticWire,
}

impl TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType::Ach => "ach",
            TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `TreasuryOutboundPaymentsResourceReturnedStatus`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryOutboundPaymentsResourceReturnedStatusCode {
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

impl TreasuryOutboundPaymentsResourceReturnedStatusCode {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryOutboundPaymentsResourceReturnedStatusCode::AccountClosed => "account_closed",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::AccountFrozen => "account_frozen",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::BankAccountRestricted => "bank_account_restricted",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::BankOwnershipChanged => "bank_ownership_changed",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::Declined => "declined",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::IncorrectAccountHolderName => "incorrect_account_holder_name",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::InvalidAccountNumber => "invalid_account_number",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::InvalidCurrency => "invalid_currency",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::NoAccount => "no_account",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::Other => "other",
        }
    }
}

impl AsRef<str> for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn default() -> Self {
        Self::AccountClosed
    }
}
