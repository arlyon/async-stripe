/// Transactions represent changes to a [FinancialAccount's](https://stripe.com/docs/api#financial_accounts) balance.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryTransaction {
    /// Amount (in cents) transferred.
    pub amount: i64,
    pub balance_impact: stripe_treasury::TreasuryTransactionsResourceBalanceImpact,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: String,
    /// A list of TransactionEntries that are part of this Transaction.
    /// This cannot be expanded in any list endpoints.
    pub entries: Option<stripe_types::List<stripe_treasury::TreasuryTransactionEntry>>,
    /// The FinancialAccount associated with this object.
    pub financial_account: String,
    /// ID of the flow that created the Transaction.
    pub flow: Option<String>,
    /// Details of the flow that created the Transaction.
    pub flow_details: Option<stripe_treasury::TreasuryTransactionsResourceFlowDetails>,
    /// Type of the flow that created the Transaction.
    pub flow_type: TreasuryTransactionFlowType,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Status of the Transaction.
    pub status: stripe_treasury::TreasuryTransactionStatus,
    pub status_transitions:
        stripe_treasury::TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
}
/// Type of the flow that created the Transaction.
#[derive(Copy, Clone, Eq, PartialEq)]
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
        use TreasuryTransactionFlowType::*;
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

impl std::str::FromStr for TreasuryTransactionFlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryTransactionFlowType::*;
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
impl std::fmt::Display for TreasuryTransactionFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryTransactionFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryTransactionFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryTransactionFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TreasuryTransactionFlowType"))
    }
}
impl stripe_types::Object for TreasuryTransaction {
    type Id = stripe_treasury::TreasuryTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryTransactionId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryTransactionStatus {
    Open,
    Posted,
    Void,
}
impl TreasuryTransactionStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryTransactionStatus::*;
        match self {
            Open => "open",
            Posted => "posted",
            Void => "void",
        }
    }
}

impl std::str::FromStr for TreasuryTransactionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryTransactionStatus::*;
        match s {
            "open" => Ok(Open),
            "posted" => Ok(Posted),
            "void" => Ok(Void),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryTransactionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryTransactionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TreasuryTransactionStatus"))
    }
}
