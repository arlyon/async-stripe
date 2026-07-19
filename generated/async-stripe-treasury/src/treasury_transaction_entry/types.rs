/// TransactionEntries represent individual units of money movements within a single [Transaction](https://api.stripe.com#transactions).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The Transaction associated with this object.
    pub transaction: stripe_types::Expandable<stripe_treasury::TreasuryTransaction>,
    /// The specific money movement that generated the TransactionEntry.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: TreasuryTransactionEntryType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryTransactionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryTransactionEntry").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryTransactionEntryBuilder {
    balance_impact: Option<stripe_treasury::TreasuryTransactionsResourceBalanceImpact>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    effective_at: Option<stripe_types::Timestamp>,
    financial_account: Option<String>,
    flow: Option<Option<String>>,
    flow_details: Option<Option<stripe_treasury::TreasuryTransactionsResourceFlowDetails>>,
    flow_type: Option<TreasuryTransactionEntryFlowType>,
    id: Option<stripe_treasury::TreasuryTransactionEntryId>,
    livemode: Option<bool>,
    transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
    type_: Option<TreasuryTransactionEntryType>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for TreasuryTransactionEntry {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryTransactionEntry>,
        builder: TreasuryTransactionEntryBuilder,
    }

    impl Visitor for Place<TreasuryTransactionEntry> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryTransactionEntryBuilder {
                    balance_impact: Deserialize::default(),
                    created: Deserialize::default(),
                    currency: Deserialize::default(),
                    effective_at: Deserialize::default(),
                    financial_account: Deserialize::default(),
                    flow: Deserialize::default(),
                    flow_details: Deserialize::default(),
                    flow_type: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    transaction: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "balance_impact" => Deserialize::begin(&mut self.builder.balance_impact),
                "created" => Deserialize::begin(&mut self.builder.created),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "effective_at" => Deserialize::begin(&mut self.builder.effective_at),
                "financial_account" => Deserialize::begin(&mut self.builder.financial_account),
                "flow" => Deserialize::begin(&mut self.builder.flow),
                "flow_details" => Deserialize::begin(&mut self.builder.flow_details),
                "flow_type" => Deserialize::begin(&mut self.builder.flow_type),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "transaction" => Deserialize::begin(&mut self.builder.transaction),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(balance_impact),
                Some(created),
                Some(currency),
                Some(effective_at),
                Some(financial_account),
                Some(flow),
                Some(flow_details),
                Some(flow_type),
                Some(id),
                Some(livemode),
                Some(transaction),
                Some(type_),
            ) = (
                self.builder.balance_impact,
                self.builder.created,
                self.builder.currency.take(),
                self.builder.effective_at,
                self.builder.financial_account.take(),
                self.builder.flow.take(),
                self.builder.flow_details.take(),
                self.builder.flow_type.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.transaction.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TreasuryTransactionEntry {
                balance_impact,
                created,
                currency,
                effective_at,
                financial_account,
                flow,
                flow_details,
                flow_type,
                id,
                livemode,
                transaction,
                type_,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryTransactionEntry {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TreasuryTransactionEntry", 13)?;
        s.serialize_field("balance_impact", &self.balance_impact)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("effective_at", &self.effective_at)?;
        s.serialize_field("financial_account", &self.financial_account)?;
        s.serialize_field("flow", &self.flow)?;
        s.serialize_field("flow_details", &self.flow_details)?;
        s.serialize_field("flow_type", &self.flow_type)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("transaction", &self.transaction)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "treasury.transaction_entry")?;
        s.end()
    }
}
/// Type of the flow associated with the TransactionEntry.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryTransactionEntryFlowType {
    pub fn as_str(&self) -> &str {
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryTransactionEntryFlowType {
    type Err = std::convert::Infallible;
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
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryTransactionEntryFlowType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryTransactionEntryFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryTransactionEntryFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryTransactionEntryFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryTransactionEntryFlowType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryTransactionEntryFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TreasuryTransactionEntryFlowType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryTransactionEntryFlowType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryTransactionEntryFlowType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryTransactionEntryFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The specific money movement that generated the TransactionEntry.
#[derive(Clone, Eq, PartialEq)]
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
    Unknown(String),
}
impl TreasuryTransactionEntryType {
    pub fn as_str(&self) -> &str {
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryTransactionEntryType {
    type Err = std::convert::Infallible;
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
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryTransactionEntryType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryTransactionEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryTransactionEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryTransactionEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryTransactionEntryType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryTransactionEntryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TreasuryTransactionEntryType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryTransactionEntryType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryTransactionEntryType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryTransactionEntryType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for TreasuryTransactionEntry {
    type Id = stripe_treasury::TreasuryTransactionEntryId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TreasuryTransactionEntryId);
