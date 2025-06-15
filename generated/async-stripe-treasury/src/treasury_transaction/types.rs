/// Transactions represent changes to a [FinancialAccount's](https://stripe.com/docs/api#financial_accounts) balance.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
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
#[doc(hidden)]
pub struct TreasuryTransactionBuilder {
    amount: Option<i64>,
    balance_impact: Option<stripe_treasury::TreasuryTransactionsResourceBalanceImpact>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    description: Option<String>,
    entries: Option<Option<stripe_types::List<stripe_treasury::TreasuryTransactionEntry>>>,
    financial_account: Option<String>,
    flow: Option<Option<String>>,
    flow_details: Option<Option<stripe_treasury::TreasuryTransactionsResourceFlowDetails>>,
    flow_type: Option<TreasuryTransactionFlowType>,
    id: Option<stripe_treasury::TreasuryTransactionId>,
    livemode: Option<bool>,
    status: Option<stripe_treasury::TreasuryTransactionStatus>,
    status_transitions: Option<
        stripe_treasury::TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
    >,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryTransaction>,
        builder: TreasuryTransactionBuilder,
    }

    impl Visitor for Place<TreasuryTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryTransactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryTransactionBuilder {
        type Out = TreasuryTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "balance_impact" => Deserialize::begin(&mut self.balance_impact),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "entries" => Deserialize::begin(&mut self.entries),
                "financial_account" => Deserialize::begin(&mut self.financial_account),
                "flow" => Deserialize::begin(&mut self.flow),
                "flow_details" => Deserialize::begin(&mut self.flow_details),
                "flow_type" => Deserialize::begin(&mut self.flow_type),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "status" => Deserialize::begin(&mut self.status),
                "status_transitions" => Deserialize::begin(&mut self.status_transitions),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                balance_impact: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                entries: Deserialize::default(),
                financial_account: Deserialize::default(),
                flow: Deserialize::default(),
                flow_details: Deserialize::default(),
                flow_type: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                status: Deserialize::default(),
                status_transitions: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(balance_impact),
                Some(created),
                Some(currency),
                Some(description),
                Some(entries),
                Some(financial_account),
                Some(flow),
                Some(flow_details),
                Some(flow_type),
                Some(id),
                Some(livemode),
                Some(status),
                Some(status_transitions),
            ) = (
                self.amount,
                self.balance_impact,
                self.created,
                self.currency.take(),
                self.description.take(),
                self.entries.take(),
                self.financial_account.take(),
                self.flow.take(),
                self.flow_details.take(),
                self.flow_type,
                self.id.take(),
                self.livemode,
                self.status,
                self.status_transitions,
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                balance_impact,
                created,
                currency,
                description,
                entries,
                financial_account,
                flow,
                flow_details,
                flow_type,
                id,
                livemode,
                status,
                status_transitions,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TreasuryTransaction {
        type Builder = TreasuryTransactionBuilder;
    }

    impl FromValueOpt for TreasuryTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "balance_impact" => b.balance_impact = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "entries" => b.entries = FromValueOpt::from_value(v),
                    "financial_account" => b.financial_account = FromValueOpt::from_value(v),
                    "flow" => b.flow = FromValueOpt::from_value(v),
                    "flow_details" => b.flow_details = FromValueOpt::from_value(v),
                    "flow_type" => b.flow_type = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "status_transitions" => b.status_transitions = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryTransaction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TreasuryTransaction", 15)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("balance_impact", &self.balance_impact)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("entries", &self.entries)?;
        s.serialize_field("financial_account", &self.financial_account)?;
        s.serialize_field("flow", &self.flow)?;
        s.serialize_field("flow_details", &self.flow_details)?;
        s.serialize_field("flow_type", &self.flow_type)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_transitions", &self.status_transitions)?;

        s.serialize_field("object", "treasury.transaction")?;
        s.end()
    }
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
    type Err = stripe_types::StripeParseError;
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
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryTransactionFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryTransactionFlowType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TreasuryTransactionFlowType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryTransactionFlowType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryTransactionFlowType);
#[cfg(feature = "deserialize")]
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

    fn into_id(self) -> Self::Id {
        self.id
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryTransactionStatus::*;
        match s {
            "open" => Ok(Open),
            "posted" => Ok(Posted),
            "void" => Ok(Void),
            _ => Err(stripe_types::StripeParseError),
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
impl miniserde::Deserialize for TreasuryTransactionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TreasuryTransactionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryTransactionStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryTransactionStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryTransactionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TreasuryTransactionStatus"))
    }
}
