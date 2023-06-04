/// Transactions represent changes to a [FinancialAccount's](https://stripe.com/docs/api#financial_accounts) balance.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Transaction {
    /// Amount (in cents) transferred.
    pub amount: i64,
    pub balance_impact: crate::treasury::transaction::balance_impact::BalanceImpact,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: String,
    /// A list of TransactionEntries that are part of this Transaction.
    ///
    /// This cannot be expanded in any list endpoints.
    pub entries: crate::List<crate::treasury::transaction_entry::TransactionEntry>,
    /// The FinancialAccount associated with this object.
    pub financial_account: String,
    /// ID of the flow that created the Transaction.
    pub flow: Option<String>,
    /// Details of the flow that created the Transaction.
    pub flow_details: Option<crate::treasury::flow_details::FlowDetails>,
    /// Type of the flow that created the Transaction.
    pub flow_type: TransactionFlowType,
    /// Unique identifier for the object.
    pub id: crate::treasury::transaction::TreasuryTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TransactionObject,
    /// Status of the Transaction.
    pub status: TransactionStatus,
    pub status_transitions: crate::treasury::transaction::status_transitions::StatusTransitions,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Transaction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of the flow that created the Transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TransactionFlowType {
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

impl TransactionFlowType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditReversal => "credit_reversal",
            Self::DebitReversal => "debit_reversal",
            Self::InboundTransfer => "inbound_transfer",
            Self::IssuingAuthorization => "issuing_authorization",
            Self::Other => "other",
            Self::OutboundPayment => "outbound_payment",
            Self::OutboundTransfer => "outbound_transfer",
            Self::ReceivedCredit => "received_credit",
            Self::ReceivedDebit => "received_debit",
        }
    }
}

impl AsRef<str> for TransactionFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TransactionObject {
    #[serde(rename = "treasury.transaction")]
    TreasuryTransaction,
}

impl TransactionObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryTransaction => "treasury.transaction",
        }
    }
}

impl AsRef<str> for TransactionObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Status of the Transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Open,
    Posted,
    Void,
}

impl TransactionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Posted => "posted",
            Self::Void => "void",
        }
    }
}

impl AsRef<str> for TransactionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for Transaction {
    type Id = crate::treasury::transaction::TreasuryTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(TreasuryTransactionId);
pub mod requests;
pub mod status_transitions;
pub use status_transitions::StatusTransitions;
pub mod balance_impact;
pub use balance_impact::BalanceImpact;
