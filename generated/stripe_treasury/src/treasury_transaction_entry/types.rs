/// TransactionEntries represent individual units of money movements within a single [Transaction](https://stripe.com/docs/api#transactions).
#[derive(Clone, Debug)]
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
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The Transaction associated with this object.
    pub transaction: stripe_types::Expandable<stripe_treasury::TreasuryTransaction>,
    /// The specific money movement that generated the TransactionEntry.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: TreasuryTransactionEntryType,
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

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: TreasuryTransactionEntryBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryTransactionEntryBuilder {
        type Out = TreasuryTransactionEntry;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "balance_impact" => Deserialize::begin(&mut self.balance_impact),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "effective_at" => Deserialize::begin(&mut self.effective_at),
                "financial_account" => Deserialize::begin(&mut self.financial_account),
                "flow" => Deserialize::begin(&mut self.flow),
                "flow_details" => Deserialize::begin(&mut self.flow_details),
                "flow_type" => Deserialize::begin(&mut self.flow_type),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "transaction" => Deserialize::begin(&mut self.transaction),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                balance_impact: self.balance_impact?,
                created: self.created?,
                currency: self.currency?,
                effective_at: self.effective_at?,
                financial_account: self.financial_account.take()?,
                flow: self.flow.take()?,
                flow_details: self.flow_details.take()?,
                flow_type: self.flow_type?,
                id: self.id.take()?,
                livemode: self.livemode?,
                transaction: self.transaction.take()?,
                type_: self.type_?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TreasuryTransactionEntry {
        type Builder = TreasuryTransactionEntryBuilder;
    }

    impl FromValueOpt for TreasuryTransactionEntry {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryTransactionEntryBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "balance_impact" => b.balance_impact = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "effective_at" => b.effective_at = Some(FromValueOpt::from_value(v)?),
                    "financial_account" => b.financial_account = Some(FromValueOpt::from_value(v)?),
                    "flow" => b.flow = Some(FromValueOpt::from_value(v)?),
                    "flow_details" => b.flow_details = Some(FromValueOpt::from_value(v)?),
                    "flow_type" => b.flow_type = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "transaction" => b.transaction = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
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
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryTransactionEntryFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryTransactionEntryFlowType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TreasuryTransactionEntryFlowType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(TreasuryTransactionEntryFlowType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryTransactionEntryFlowType);
#[cfg(feature = "deserialize")]
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
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryTransactionEntryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryTransactionEntryType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TreasuryTransactionEntryType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryTransactionEntryType::from_str(s)
                .unwrap_or(TreasuryTransactionEntryType::Unknown),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryTransactionEntryType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryTransactionEntryType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
impl stripe_types::Object for TreasuryTransactionEntry {
    type Id = stripe_treasury::TreasuryTransactionEntryId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryTransactionEntryId);
