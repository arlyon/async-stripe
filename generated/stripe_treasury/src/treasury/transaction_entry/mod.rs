/// TransactionEntries represent individual units of money movements within a single [Transaction](https://stripe.com/docs/api#transactions).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TransactionEntry {
    pub balance_impact: stripe_treasury::treasury::transaction::balance_impact::BalanceImpact,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// When the TransactionEntry will impact the FinancialAccount's balance.
    pub effective_at: stripe_types::Timestamp,
    /// The FinancialAccount associated with this object.
    pub financial_account: String,
    /// Token of the flow associated with the TransactionEntry.
    pub flow: Option<String>,
    /// Details of the flow associated with the TransactionEntry.
    pub flow_details: Option<stripe_treasury::treasury::flow_details::FlowDetails>,
    /// Type of the flow associated with the TransactionEntry.
    pub flow_type: TransactionEntryFlowType,
    /// Unique identifier for the object.
    pub id: stripe_treasury::treasury::transaction_entry::TreasuryTransactionEntryId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TransactionEntryObject,
    /// The Transaction associated with this object.
    pub transaction: stripe_types::Expandable<stripe_treasury::treasury::transaction::Transaction>,
    /// The specific money movement that generated the TransactionEntry.
    #[serde(rename = "type")]
    pub type_: TransactionEntryType,
}
/// Type of the flow associated with the TransactionEntry.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
        use TransactionEntryFlowType::*;
        match self {
            CreditReversal => "credit_reversal",
            DebitReversal => "debit_reversal",
            InboundTransfer => "inbound_transfer",
            IssuingAuthorization => "issuing_authorization",
            Other => "other",
            OutboundPayment => "outbound_payment",
            OutboundTransfer => "outbound_transfer",
            ReceivedCredit => "received_credit",
            ReceivedDebit => "received_debit",
        }
    }
}

impl std::str::FromStr for TransactionEntryFlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransactionEntryFlowType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "debit_reversal" => Ok(DebitReversal),
            "inbound_transfer" => Ok(InboundTransfer),
            "issuing_authorization" => Ok(IssuingAuthorization),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "outbound_transfer" => Ok(OutboundTransfer),
            "received_credit" => Ok(ReceivedCredit),
            "received_debit" => Ok(ReceivedDebit),
            _ => Err(()),
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
impl serde::Serialize for TransactionEntryFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransactionEntryFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransactionEntryFlowType"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionEntryObject {
    TreasuryTransactionEntry,
}

impl TransactionEntryObject {
    pub fn as_str(self) -> &'static str {
        use TransactionEntryObject::*;
        match self {
            TreasuryTransactionEntry => "treasury.transaction_entry",
        }
    }
}

impl std::str::FromStr for TransactionEntryObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransactionEntryObject::*;
        match s {
            "treasury.transaction_entry" => Ok(TreasuryTransactionEntry),
            _ => Err(()),
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
impl serde::Serialize for TransactionEntryObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransactionEntryObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransactionEntryObject"))
    }
}
/// The specific money movement that generated the TransactionEntry.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
        use TransactionEntryType::*;
        match self {
            CreditReversal => "credit_reversal",
            CreditReversalPosting => "credit_reversal_posting",
            DebitReversal => "debit_reversal",
            InboundTransfer => "inbound_transfer",
            InboundTransferReturn => "inbound_transfer_return",
            IssuingAuthorizationHold => "issuing_authorization_hold",
            IssuingAuthorizationRelease => "issuing_authorization_release",
            Other => "other",
            OutboundPayment => "outbound_payment",
            OutboundPaymentCancellation => "outbound_payment_cancellation",
            OutboundPaymentFailure => "outbound_payment_failure",
            OutboundPaymentPosting => "outbound_payment_posting",
            OutboundPaymentReturn => "outbound_payment_return",
            OutboundTransfer => "outbound_transfer",
            OutboundTransferCancellation => "outbound_transfer_cancellation",
            OutboundTransferFailure => "outbound_transfer_failure",
            OutboundTransferPosting => "outbound_transfer_posting",
            OutboundTransferReturn => "outbound_transfer_return",
            ReceivedCredit => "received_credit",
            ReceivedDebit => "received_debit",
        }
    }
}

impl std::str::FromStr for TransactionEntryType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransactionEntryType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "credit_reversal_posting" => Ok(CreditReversalPosting),
            "debit_reversal" => Ok(DebitReversal),
            "inbound_transfer" => Ok(InboundTransfer),
            "inbound_transfer_return" => Ok(InboundTransferReturn),
            "issuing_authorization_hold" => Ok(IssuingAuthorizationHold),
            "issuing_authorization_release" => Ok(IssuingAuthorizationRelease),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "outbound_payment_cancellation" => Ok(OutboundPaymentCancellation),
            "outbound_payment_failure" => Ok(OutboundPaymentFailure),
            "outbound_payment_posting" => Ok(OutboundPaymentPosting),
            "outbound_payment_return" => Ok(OutboundPaymentReturn),
            "outbound_transfer" => Ok(OutboundTransfer),
            "outbound_transfer_cancellation" => Ok(OutboundTransferCancellation),
            "outbound_transfer_failure" => Ok(OutboundTransferFailure),
            "outbound_transfer_posting" => Ok(OutboundTransferPosting),
            "outbound_transfer_return" => Ok(OutboundTransferReturn),
            "received_credit" => Ok(ReceivedCredit),
            "received_debit" => Ok(ReceivedDebit),
            _ => Err(()),
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
impl serde::Serialize for TransactionEntryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransactionEntryType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransactionEntryType"))
    }
}
impl stripe_types::Object for TransactionEntry {
    type Id = stripe_treasury::treasury::transaction_entry::TreasuryTransactionEntryId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryTransactionEntryId);
pub mod requests;
