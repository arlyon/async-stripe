/// TransactionEntries represent individual units of money movements within a single [Transaction](https://stripe.com/docs/api#transactions).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TransactionEntry {
    pub balance_impact: crate::treasury::transaction::balance_impact::BalanceImpact,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// When the TransactionEntry will impact the FinancialAccount's balance.
    pub effective_at: crate::Timestamp,
    /// The FinancialAccount associated with this object.
    pub financial_account: String,
    /// Token of the flow associated with the TransactionEntry.
    pub flow: Option<String>,
    /// Details of the flow associated with the TransactionEntry.
    pub flow_details: Option<crate::treasury::flow_details::FlowDetails>,
    /// Type of the flow associated with the TransactionEntry.
    pub flow_type: TransactionEntryFlowType,
    /// Unique identifier for the object.
    pub id: crate::treasury::transaction_entry::TreasuryTransactionEntryId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TransactionEntryObject,
    /// The Transaction associated with this object.
    pub transaction: crate::Expandable<crate::treasury::transaction::Transaction>,
    /// The specific money movement that generated the TransactionEntry.
    #[serde(rename = "type")]
    pub type_: TransactionEntryType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransactionEntry {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of the flow associated with the TransactionEntry.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TransactionEntryFlowType {
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

impl TransactionEntryFlowType {
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

impl AsRef<str> for TransactionEntryFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionEntryFlowType {
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
pub enum TransactionEntryObject {
    #[serde(rename = "treasury.transaction_entry")]
    TreasuryTransactionEntry,
}

impl TransactionEntryObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryTransactionEntry => "treasury.transaction_entry",
        }
    }
}

impl AsRef<str> for TransactionEntryObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionEntryObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The specific money movement that generated the TransactionEntry.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TransactionEntryType {
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

impl TransactionEntryType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditReversal => "credit_reversal",
            Self::CreditReversalPosting => "credit_reversal_posting",
            Self::DebitReversal => "debit_reversal",
            Self::InboundTransfer => "inbound_transfer",
            Self::InboundTransferReturn => "inbound_transfer_return",
            Self::IssuingAuthorizationHold => "issuing_authorization_hold",
            Self::IssuingAuthorizationRelease => "issuing_authorization_release",
            Self::Other => "other",
            Self::OutboundPayment => "outbound_payment",
            Self::OutboundPaymentCancellation => "outbound_payment_cancellation",
            Self::OutboundPaymentFailure => "outbound_payment_failure",
            Self::OutboundPaymentPosting => "outbound_payment_posting",
            Self::OutboundPaymentReturn => "outbound_payment_return",
            Self::OutboundTransfer => "outbound_transfer",
            Self::OutboundTransferCancellation => "outbound_transfer_cancellation",
            Self::OutboundTransferFailure => "outbound_transfer_failure",
            Self::OutboundTransferPosting => "outbound_transfer_posting",
            Self::OutboundTransferReturn => "outbound_transfer_return",
            Self::ReceivedCredit => "received_credit",
            Self::ReceivedDebit => "received_debit",
        }
    }
}

impl AsRef<str> for TransactionEntryType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for TransactionEntry {
    type Id = crate::treasury::transaction_entry::TreasuryTransactionEntryId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(TreasuryTransactionEntryId);
pub mod requests;
