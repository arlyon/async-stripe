/// You can reverse some [ReceivedDebits](https://api.stripe.com#received_debits) depending on their network and source flow.
/// Reversing a ReceivedDebit leads to the creation of a new object known as a DebitReversal.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryDebitReversal {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The FinancialAccount to reverse funds from.
    pub financial_account: Option<String>,
    /// A [hosted transaction receipt](https://docs.stripe.com/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryDebitReversalId,
    /// Other flows linked to a DebitReversal.
    pub linked_flows:
        Option<stripe_treasury::TreasuryReceivedDebitsResourceDebitReversalLinkedFlows>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The rails used to reverse the funds.
    pub network: TreasuryDebitReversalNetwork,
    /// The ReceivedDebit being reversed.
    pub received_debit: String,
    /// Status of the DebitReversal
    pub status: TreasuryDebitReversalStatus,
    pub status_transitions: stripe_treasury::TreasuryReceivedDebitsResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryDebitReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryDebitReversal").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryDebitReversalBuilder {
    amount: Option<i64>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    financial_account: Option<Option<String>>,
    hosted_regulatory_receipt_url: Option<Option<String>>,
    id: Option<stripe_treasury::TreasuryDebitReversalId>,
    linked_flows:
        Option<Option<stripe_treasury::TreasuryReceivedDebitsResourceDebitReversalLinkedFlows>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    network: Option<TreasuryDebitReversalNetwork>,
    received_debit: Option<String>,
    status: Option<TreasuryDebitReversalStatus>,
    status_transitions: Option<stripe_treasury::TreasuryReceivedDebitsResourceStatusTransitions>,
    transaction: Option<Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>>,
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

    impl Deserialize for TreasuryDebitReversal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryDebitReversal>,
        builder: TreasuryDebitReversalBuilder,
    }

    impl Visitor for Place<TreasuryDebitReversal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryDebitReversalBuilder {
                    amount: Deserialize::default(),
                    created: Deserialize::default(),
                    currency: Deserialize::default(),
                    financial_account: Deserialize::default(),
                    hosted_regulatory_receipt_url: Deserialize::default(),
                    id: Deserialize::default(),
                    linked_flows: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    network: Deserialize::default(),
                    received_debit: Deserialize::default(),
                    status: Deserialize::default(),
                    status_transitions: Deserialize::default(),
                    transaction: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "created" => Deserialize::begin(&mut self.builder.created),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "financial_account" => Deserialize::begin(&mut self.builder.financial_account),
                "hosted_regulatory_receipt_url" => {
                    Deserialize::begin(&mut self.builder.hosted_regulatory_receipt_url)
                }
                "id" => Deserialize::begin(&mut self.builder.id),
                "linked_flows" => Deserialize::begin(&mut self.builder.linked_flows),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "network" => Deserialize::begin(&mut self.builder.network),
                "received_debit" => Deserialize::begin(&mut self.builder.received_debit),
                "status" => Deserialize::begin(&mut self.builder.status),
                "status_transitions" => Deserialize::begin(&mut self.builder.status_transitions),
                "transaction" => Deserialize::begin(&mut self.builder.transaction),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(created),
                Some(currency),
                Some(financial_account),
                Some(hosted_regulatory_receipt_url),
                Some(id),
                Some(linked_flows),
                Some(livemode),
                Some(metadata),
                Some(network),
                Some(received_debit),
                Some(status),
                Some(status_transitions),
                Some(transaction),
            ) = (
                self.builder.amount,
                self.builder.created,
                self.builder.currency.take(),
                self.builder.financial_account.take(),
                self.builder.hosted_regulatory_receipt_url.take(),
                self.builder.id.take(),
                self.builder.linked_flows.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.network.take(),
                self.builder.received_debit.take(),
                self.builder.status.take(),
                self.builder.status_transitions,
                self.builder.transaction.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TreasuryDebitReversal {
                amount,
                created,
                currency,
                financial_account,
                hosted_regulatory_receipt_url,
                id,
                linked_flows,
                livemode,
                metadata,
                network,
                received_debit,
                status,
                status_transitions,
                transaction,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryDebitReversal {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TreasuryDebitReversal", 15)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("financial_account", &self.financial_account)?;
        s.serialize_field("hosted_regulatory_receipt_url", &self.hosted_regulatory_receipt_url)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("linked_flows", &self.linked_flows)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("network", &self.network)?;
        s.serialize_field("received_debit", &self.received_debit)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_transitions", &self.status_transitions)?;
        s.serialize_field("transaction", &self.transaction)?;

        s.serialize_field("object", "treasury.debit_reversal")?;
        s.end()
    }
}
/// The rails used to reverse the funds.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryDebitReversalNetwork {
    Ach,
    Card,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryDebitReversalNetwork {
    pub fn as_str(&self) -> &str {
        use TreasuryDebitReversalNetwork::*;
        match self {
            Ach => "ach",
            Card => "card",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryDebitReversalNetwork {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryDebitReversalNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "card" => Ok(Card),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryDebitReversalNetwork"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryDebitReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryDebitReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryDebitReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryDebitReversalNetwork)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryDebitReversalNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TreasuryDebitReversalNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryDebitReversalNetwork> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryDebitReversalNetwork::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryDebitReversalNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Status of the DebitReversal
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryDebitReversalStatus {
    Failed,
    Processing,
    Succeeded,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryDebitReversalStatus {
    pub fn as_str(&self) -> &str {
        use TreasuryDebitReversalStatus::*;
        match self {
            Failed => "failed",
            Processing => "processing",
            Succeeded => "succeeded",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryDebitReversalStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryDebitReversalStatus::*;
        match s {
            "failed" => Ok(Failed),
            "processing" => Ok(Processing),
            "succeeded" => Ok(Succeeded),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryDebitReversalStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryDebitReversalStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryDebitReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TreasuryDebitReversalStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryDebitReversalStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryDebitReversalStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryDebitReversalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for TreasuryDebitReversal {
    type Id = stripe_treasury::TreasuryDebitReversalId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TreasuryDebitReversalId);
