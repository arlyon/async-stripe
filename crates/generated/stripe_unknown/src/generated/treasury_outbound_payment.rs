// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_common::resources::Currency;
use async_stripe_core::{
    ids::TreasuryOutboundPaymentId,
    params::{Expandable, Metadata, Object, Timestamp},
};
use serde::{Deserialize, Serialize};

use crate::resources::{OutboundPaymentsPaymentMethodDetails, TreasuryTransaction};

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
        Option<TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails>,

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
    pub returned_details: Option<TreasuryOutboundPaymentsResourceReturnedStatus>,

    /// The description that appears on the receiving end for an OutboundPayment (for example, bank statement for external bank transfer).
    pub statement_descriptor: String,

    /// Current status of the OutboundPayment: `processing`, `failed`, `posted`, `returned`, `canceled`.
    ///
    /// An OutboundPayment is `processing` if it has been created and is pending.
    /// The status changes to `posted` once the OutboundPayment has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
    /// If an OutboundPayment fails to arrive at its destination, its status will change to `returned`.
    pub status: TreasuryOutboundPaymentStatus,

    pub status_transitions:
        TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions,

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
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
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
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
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
pub struct TreasuryOutboundPaymentsResourceReturnedStatus {
    /// Reason for the return.
    pub code: TreasuryOutboundPaymentsResourceReturnedStatusCode,

    /// The Transaction associated with this object.
    pub transaction: Expandable<TreasuryTransaction>,
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
            TreasuryOutboundPaymentsResourceReturnedStatusCode::BankAccountRestricted => {
                "bank_account_restricted"
            }
            TreasuryOutboundPaymentsResourceReturnedStatusCode::BankOwnershipChanged => {
                "bank_ownership_changed"
            }
            TreasuryOutboundPaymentsResourceReturnedStatusCode::Declined => "declined",
            TreasuryOutboundPaymentsResourceReturnedStatusCode::IncorrectAccountHolderName => {
                "incorrect_account_holder_name"
            }
            TreasuryOutboundPaymentsResourceReturnedStatusCode::InvalidAccountNumber => {
                "invalid_account_number"
            }
            TreasuryOutboundPaymentsResourceReturnedStatusCode::InvalidCurrency => {
                "invalid_currency"
            }
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
