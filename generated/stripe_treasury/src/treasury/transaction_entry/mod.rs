/// TransactionEntries represent individual units of money movements within a single [Transaction](https://stripe.com/docs/api#transactions).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransactionEntry {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
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

impl std::str::FromStr for TransactionEntryFlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "credit_reversal" => Ok(Self::CreditReversal),
            "debit_reversal" => Ok(Self::DebitReversal),
            "inbound_transfer" => Ok(Self::InboundTransfer),
            "issuing_authorization" => Ok(Self::IssuingAuthorization),
            "other" => Ok(Self::Other),
            "outbound_payment" => Ok(Self::OutboundPayment),
            "outbound_transfer" => Ok(Self::OutboundTransfer),
            "received_credit" => Ok(Self::ReceivedCredit),
            "received_debit" => Ok(Self::ReceivedDebit),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransactionEntryFlowType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransactionEntryFlowType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TransactionEntryFlowType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TransactionEntryFlowType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
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
        match self {
            Self::TreasuryTransactionEntry => "treasury.transaction_entry",
        }
    }
}

impl std::str::FromStr for TransactionEntryObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "treasury.transaction_entry" => Ok(Self::TreasuryTransactionEntry),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransactionEntryObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransactionEntryObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TransactionEntryObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TransactionEntryObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
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

impl std::str::FromStr for TransactionEntryType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "credit_reversal" => Ok(Self::CreditReversal),
            "credit_reversal_posting" => Ok(Self::CreditReversalPosting),
            "debit_reversal" => Ok(Self::DebitReversal),
            "inbound_transfer" => Ok(Self::InboundTransfer),
            "inbound_transfer_return" => Ok(Self::InboundTransferReturn),
            "issuing_authorization_hold" => Ok(Self::IssuingAuthorizationHold),
            "issuing_authorization_release" => Ok(Self::IssuingAuthorizationRelease),
            "other" => Ok(Self::Other),
            "outbound_payment" => Ok(Self::OutboundPayment),
            "outbound_payment_cancellation" => Ok(Self::OutboundPaymentCancellation),
            "outbound_payment_failure" => Ok(Self::OutboundPaymentFailure),
            "outbound_payment_posting" => Ok(Self::OutboundPaymentPosting),
            "outbound_payment_return" => Ok(Self::OutboundPaymentReturn),
            "outbound_transfer" => Ok(Self::OutboundTransfer),
            "outbound_transfer_cancellation" => Ok(Self::OutboundTransferCancellation),
            "outbound_transfer_failure" => Ok(Self::OutboundTransferFailure),
            "outbound_transfer_posting" => Ok(Self::OutboundTransferPosting),
            "outbound_transfer_return" => Ok(Self::OutboundTransferReturn),
            "received_credit" => Ok(Self::ReceivedCredit),
            "received_debit" => Ok(Self::ReceivedDebit),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransactionEntryType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransactionEntryType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TransactionEntryType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TransactionEntryType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for TransactionEntry {
    type Id = stripe_treasury::treasury::transaction_entry::TreasuryTransactionEntryId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryTransactionEntryId);
