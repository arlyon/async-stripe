// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{TreasuryTransactionEntryId};
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{Currency, TreasuryTransaction, TreasuryTransactionsResourceBalanceImpact, TreasuryTransactionsResourceFlowDetails};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasuryTransactionsResourceTransactionEntry".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryTransactionEntry {
    /// Unique identifier for the object.
    pub id: TreasuryTransactionEntryId,

    pub balance_impact: TreasuryTransactionsResourceBalanceImpact,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// When the TransactionEntry will impact the FinancialAccount's balance.
    pub effective_at: Timestamp,

    /// The FinancialAccount associated with this object.
    pub financial_account: String,

    /// Token of the flow associated with the TransactionEntry.
    pub flow: Option<String>,

    /// Details of the flow associated with the TransactionEntry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_details: Option<TreasuryTransactionsResourceFlowDetails>,

    /// Type of the flow associated with the TransactionEntry.
    pub flow_type: TreasuryTransactionEntryFlowType,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The Transaction associated with this object.
    pub transaction: Expandable<TreasuryTransaction>,

    /// The specific money movement that generated the TransactionEntry.
    #[serde(rename = "type")]
    pub type_: TreasuryTransactionEntryType,
}

impl Object for TreasuryTransactionEntry {
    type Id = TreasuryTransactionEntryId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "treasury.transaction_entry"
    }
}

/// An enum representing the possible values of an `TreasuryTransactionEntry`'s `flow_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionEntryFlowType {
    CreditReversal,
    DebitReversal,
    InboundTransfer,
    IssuingAuthorization,
    Other,
    OutboundPayment,
    OutboundTransfer,
    ReceivedCredit,
    ReceivedDebit,
}

impl TreasuryTransactionEntryFlowType {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryTransactionEntryFlowType::CreditReversal => "credit_reversal",
            TreasuryTransactionEntryFlowType::DebitReversal => "debit_reversal",
            TreasuryTransactionEntryFlowType::InboundTransfer => "inbound_transfer",
            TreasuryTransactionEntryFlowType::IssuingAuthorization => "issuing_authorization",
            TreasuryTransactionEntryFlowType::Other => "other",
            TreasuryTransactionEntryFlowType::OutboundPayment => "outbound_payment",
            TreasuryTransactionEntryFlowType::OutboundTransfer => "outbound_transfer",
            TreasuryTransactionEntryFlowType::ReceivedCredit => "received_credit",
            TreasuryTransactionEntryFlowType::ReceivedDebit => "received_debit",
        }
    }
}

impl AsRef<str> for TreasuryTransactionEntryFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryTransactionEntryFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryTransactionEntryFlowType {
    fn default() -> Self {
        Self::CreditReversal
    }
}

/// An enum representing the possible values of an `TreasuryTransactionEntry`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionEntryType {
    CreditReversal,
    CreditReversalPosting,
    DebitReversal,
    InboundTransfer,
    InboundTransferReturn,
    IssuingAuthorizationHold,
    IssuingAuthorizationRelease,
    Other,
    OutboundPayment,
    OutboundPaymentCancellation,
    OutboundPaymentFailure,
    OutboundPaymentPosting,
    OutboundPaymentReturn,
    OutboundTransfer,
    OutboundTransferCancellation,
    OutboundTransferFailure,
    OutboundTransferPosting,
    OutboundTransferReturn,
    ReceivedCredit,
    ReceivedDebit,
}

impl TreasuryTransactionEntryType {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryTransactionEntryType::CreditReversal => "credit_reversal",
            TreasuryTransactionEntryType::CreditReversalPosting => "credit_reversal_posting",
            TreasuryTransactionEntryType::DebitReversal => "debit_reversal",
            TreasuryTransactionEntryType::InboundTransfer => "inbound_transfer",
            TreasuryTransactionEntryType::InboundTransferReturn => "inbound_transfer_return",
            TreasuryTransactionEntryType::IssuingAuthorizationHold => "issuing_authorization_hold",
            TreasuryTransactionEntryType::IssuingAuthorizationRelease => "issuing_authorization_release",
            TreasuryTransactionEntryType::Other => "other",
            TreasuryTransactionEntryType::OutboundPayment => "outbound_payment",
            TreasuryTransactionEntryType::OutboundPaymentCancellation => "outbound_payment_cancellation",
            TreasuryTransactionEntryType::OutboundPaymentFailure => "outbound_payment_failure",
            TreasuryTransactionEntryType::OutboundPaymentPosting => "outbound_payment_posting",
            TreasuryTransactionEntryType::OutboundPaymentReturn => "outbound_payment_return",
            TreasuryTransactionEntryType::OutboundTransfer => "outbound_transfer",
            TreasuryTransactionEntryType::OutboundTransferCancellation => "outbound_transfer_cancellation",
            TreasuryTransactionEntryType::OutboundTransferFailure => "outbound_transfer_failure",
            TreasuryTransactionEntryType::OutboundTransferPosting => "outbound_transfer_posting",
            TreasuryTransactionEntryType::OutboundTransferReturn => "outbound_transfer_return",
            TreasuryTransactionEntryType::ReceivedCredit => "received_credit",
            TreasuryTransactionEntryType::ReceivedDebit => "received_debit",
        }
    }
}

impl AsRef<str> for TreasuryTransactionEntryType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryTransactionEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryTransactionEntryType {
    fn default() -> Self {
        Self::CreditReversal
    }
}
