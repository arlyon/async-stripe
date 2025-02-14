// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{TreasuryTransactionId};
use crate::params::{List, Object, Timestamp};
use crate::resources::{Currency, TreasuryTransactionEntry, TreasuryTransactionsResourceBalanceImpact, TreasuryTransactionsResourceFlowDetails};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasuryTransactionsResourceTransaction".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryTransaction {
    /// Unique identifier for the object.
    pub id: TreasuryTransactionId,

    /// Amount (in cents) transferred.
    pub amount: i64,

    pub balance_impact: TreasuryTransactionsResourceBalanceImpact,

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
    pub description: String,

    /// A list of TransactionEntries that are part of this Transaction.
    ///
    /// This cannot be expanded in any list endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<List<TreasuryTransactionEntry>>,

    /// The FinancialAccount associated with this object.
    pub financial_account: String,

    /// ID of the flow that created the Transaction.
    pub flow: Option<String>,

    /// Details of the flow that created the Transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_details: Option<TreasuryTransactionsResourceFlowDetails>,

    /// Type of the flow that created the Transaction.
    pub flow_type: TreasuryTransactionFlowType,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Status of the Transaction.
    pub status: TreasuryTransactionStatus,

    pub status_transitions: TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
}

impl Object for TreasuryTransaction {
    type Id = TreasuryTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "treasury.transaction"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {

    /// Timestamp describing when the Transaction changed status to `posted`.
    pub posted_at: Option<Timestamp>,

    /// Timestamp describing when the Transaction changed status to `void`.
    pub void_at: Option<Timestamp>,
}

/// An enum representing the possible values of an `TreasuryTransaction`'s `flow_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionFlowType {
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

impl TreasuryTransactionFlowType {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryTransactionFlowType::CreditReversal => "credit_reversal",
            TreasuryTransactionFlowType::DebitReversal => "debit_reversal",
            TreasuryTransactionFlowType::InboundTransfer => "inbound_transfer",
            TreasuryTransactionFlowType::IssuingAuthorization => "issuing_authorization",
            TreasuryTransactionFlowType::Other => "other",
            TreasuryTransactionFlowType::OutboundPayment => "outbound_payment",
            TreasuryTransactionFlowType::OutboundTransfer => "outbound_transfer",
            TreasuryTransactionFlowType::ReceivedCredit => "received_credit",
            TreasuryTransactionFlowType::ReceivedDebit => "received_debit",
        }
    }
}

impl AsRef<str> for TreasuryTransactionFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryTransactionFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryTransactionFlowType {
    fn default() -> Self {
        Self::CreditReversal
    }
}

/// An enum representing the possible values of an `TreasuryTransaction`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionStatus {
    Open,
    Posted,
    Void,
}

impl TreasuryTransactionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryTransactionStatus::Open => "open",
            TreasuryTransactionStatus::Posted => "posted",
            TreasuryTransactionStatus::Void => "void",
        }
    }
}

impl AsRef<str> for TreasuryTransactionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryTransactionStatus {
    fn default() -> Self {
        Self::Open
    }
}
