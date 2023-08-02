/// Transactions represent changes to a [FinancialAccount's](https://stripe.com/docs/api#financial_accounts) balance.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    /// Amount (in cents) transferred.
    pub amount: i64,
    pub balance_impact: stripe_treasury::balance_impact::BalanceImpact,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: String,
    /// A list of TransactionEntries that are part of this Transaction.
    ///
    /// This cannot be expanded in any list endpoints.
    pub entries: stripe_types::List<stripe_treasury::treasury::transaction_entry::TransactionEntry>,
    /// The FinancialAccount associated with this object.
    pub financial_account: String,
    /// ID of the flow that created the Transaction.
    pub flow: Option<String>,
    /// Details of the flow that created the Transaction.
    pub flow_details: Option<stripe_treasury::treasury::flow_details::FlowDetails>,
    /// Type of the flow that created the Transaction.
    pub flow_type: TransactionFlowType,
    /// Unique identifier for the object.
    pub id: stripe_treasury::treasury::transaction::TreasuryTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TransactionObject,
    /// Status of the Transaction.
    pub status: TransactionStatus,
    pub status_transitions: stripe_treasury::status_transitions::StatusTransitions,
}
/// Type of the flow that created the Transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
        use TransactionFlowType::*;
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

impl std::str::FromStr for TransactionFlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransactionFlowType::*;
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
impl serde::Serialize for TransactionFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransactionFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransactionFlowType"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionObject {
    TreasuryTransaction,
}

impl TransactionObject {
    pub fn as_str(self) -> &'static str {
        use TransactionObject::*;
        match self {
            TreasuryTransaction => "treasury.transaction",
        }
    }
}

impl std::str::FromStr for TransactionObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransactionObject::*;
        match s {
            "treasury.transaction" => Ok(TreasuryTransaction),
            _ => Err(()),
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
impl serde::Serialize for TransactionObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransactionObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransactionObject"))
    }
}
/// Status of the Transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionStatus {
    Open,
    Posted,
    Void,
}

impl TransactionStatus {
    pub fn as_str(self) -> &'static str {
        use TransactionStatus::*;
        match self {
            Open => "open",
            Posted => "posted",
            Void => "void",
        }
    }
}

impl std::str::FromStr for TransactionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransactionStatus::*;
        match s {
            "open" => Ok(Open),
            "posted" => Ok(Posted),
            "void" => Ok(Void),
            _ => Err(()),
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
impl serde::Serialize for TransactionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransactionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransactionStatus"))
    }
}
impl stripe_types::Object for Transaction {
    type Id = stripe_treasury::treasury::transaction::TreasuryTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryTransactionId);
pub mod requests;
