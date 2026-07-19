/// Transactions represent changes to a [FinancialAccount's](https://api.stripe.com#financial_accounts) balance.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Status of the Transaction.
    pub status: stripe_treasury::TreasuryTransactionStatus,
    pub status_transitions:
        stripe_treasury::TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryTransaction").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TreasuryTransactionBuilder {
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
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "balance_impact" => Deserialize::begin(&mut self.builder.balance_impact),
                "created" => Deserialize::begin(&mut self.builder.created),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "description" => Deserialize::begin(&mut self.builder.description),
                "entries" => Deserialize::begin(&mut self.builder.entries),
                "financial_account" => Deserialize::begin(&mut self.builder.financial_account),
                "flow" => Deserialize::begin(&mut self.builder.flow),
                "flow_details" => Deserialize::begin(&mut self.builder.flow_details),
                "flow_type" => Deserialize::begin(&mut self.builder.flow_type),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "status" => Deserialize::begin(&mut self.builder.status),
                "status_transitions" => Deserialize::begin(&mut self.builder.status_transitions),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.amount,
                self.builder.balance_impact,
                self.builder.created,
                self.builder.currency.take(),
                self.builder.description.take(),
                self.builder.entries.take(),
                self.builder.financial_account.take(),
                self.builder.flow.take(),
                self.builder.flow_details.take(),
                self.builder.flow_type.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.status.take(),
                self.builder.status_transitions,
            )
            else {
                return Ok(());
            };
            *self.out = Some(TreasuryTransaction {
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
            });
            Ok(())
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryTransactionFlowType {
    pub fn as_str(&self) -> &str {
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryTransactionFlowType {
    type Err = std::convert::Infallible;
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
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryTransactionFlowType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryTransactionFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryTransactionFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryTransactionFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryTransactionFlowType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for TreasuryTransactionFlowType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryTransactionFlowType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryTransactionFlowType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryTransactionFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryTransactionStatus {
    Open,
    Posted,
    Void,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryTransactionStatus {
    pub fn as_str(&self) -> &str {
        use TreasuryTransactionStatus::*;
        match self {
            Open => "open",
            Posted => "posted",
            Void => "void",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryTransactionStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryTransactionStatus::*;
        match s {
            "open" => Ok(Open),
            "posted" => Ok(Posted),
            "void" => Ok(Void),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "TreasuryTransactionStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryTransactionStatus)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for TreasuryTransactionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryTransactionStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryTransactionStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryTransactionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
