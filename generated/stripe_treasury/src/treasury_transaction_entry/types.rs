/// TransactionEntries represent individual units of money movements within a single [Transaction](https://stripe.com/docs/api#transactions).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryTransactionEntry {
    pub balance_impact: stripe_treasury::TreasuryTransactionsResourceBalanceImpact,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// When the TransactionEntry will impact the FinancialAccount's balance.
    pub effective_at: stripe_types::Timestamp,
    /// The FinancialAccount associated with this object.
    pub financial_account: String,
    /// Token of the flow associated with the TransactionEntry.
    pub flow: Option<String>,
    /// Details of the flow associated with the TransactionEntry.
    pub flow_details: Option<stripe_treasury::TreasuryTransactionsResourceFlowDetails>,
    /// Type of the flow associated with the TransactionEntry.
    pub flow_type: TreasuryTransactionEntryFlowType,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryTransactionEntryId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The Transaction associated with this object.
    pub transaction: stripe_types::Expandable<stripe_treasury::TreasuryTransaction>,
    /// The specific money movement that generated the TransactionEntry.
    #[serde(rename = "type")]
    pub type_: TreasuryTransactionEntryType,
}
/// Type of the flow associated with the TransactionEntry.
#[derive(Copy, Clone, Eq, PartialEq)]
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
        use TreasuryTransactionEntryFlowType::*;
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

impl std::str::FromStr for TreasuryTransactionEntryFlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryTransactionEntryFlowType::*;
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
impl std::fmt::Display for TreasuryTransactionEntryFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryTransactionEntryFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryTransactionEntryFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryTransactionEntryFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TreasuryTransactionEntryFlowType")
        })
    }
}
/// The specific money movement that generated the TransactionEntry.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl TreasuryTransactionEntryType {
    pub fn as_str(self) -> &'static str {
        use TreasuryTransactionEntryType::*;
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
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for TreasuryTransactionEntryType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryTransactionEntryType::*;
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
impl std::fmt::Display for TreasuryTransactionEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryTransactionEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryTransactionEntryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryTransactionEntryType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(TreasuryTransactionEntryType::Unknown))
    }
}
impl stripe_types::Object for TreasuryTransactionEntry {
    type Id = stripe_treasury::TreasuryTransactionEntryId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryTransactionEntryId);
