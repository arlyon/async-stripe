/// You can reverse some [ReceivedCredits](https://api.stripe.com#received_credits) depending on their network and source flow.
/// Reversing a ReceivedCredit leads to the creation of a new object known as a CreditReversal.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryCreditReversal {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The FinancialAccount to reverse funds from.
    pub financial_account: String,
    /// A [hosted transaction receipt](https://docs.stripe.com/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryCreditReversalId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The rails used to reverse the funds.
    pub network: TreasuryCreditReversalNetwork,
    /// The ReceivedCredit being reversed.
    pub received_credit: String,
    /// Status of the CreditReversal
    pub status: stripe_treasury::TreasuryCreditReversalStatus,
    pub status_transitions: stripe_treasury::TreasuryReceivedCreditsResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryCreditReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryCreditReversal").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryCreditReversalBuilder {
    amount: Option<i64>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    financial_account: Option<String>,
    hosted_regulatory_receipt_url: Option<Option<String>>,
    id: Option<stripe_treasury::TreasuryCreditReversalId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    network: Option<TreasuryCreditReversalNetwork>,
    received_credit: Option<String>,
    status: Option<stripe_treasury::TreasuryCreditReversalStatus>,
    status_transitions: Option<stripe_treasury::TreasuryReceivedCreditsResourceStatusTransitions>,
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

    impl Deserialize for TreasuryCreditReversal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryCreditReversal>,
        builder: TreasuryCreditReversalBuilder,
    }

    impl Visitor for Place<TreasuryCreditReversal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryCreditReversalBuilder {
                    amount: Deserialize::default(),
                    created: Deserialize::default(),
                    currency: Deserialize::default(),
                    financial_account: Deserialize::default(),
                    hosted_regulatory_receipt_url: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    network: Deserialize::default(),
                    received_credit: Deserialize::default(),
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
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "network" => Deserialize::begin(&mut self.builder.network),
                "received_credit" => Deserialize::begin(&mut self.builder.received_credit),
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
                Some(livemode),
                Some(metadata),
                Some(network),
                Some(received_credit),
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
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.network.take(),
                self.builder.received_credit.take(),
                self.builder.status.take(),
                self.builder.status_transitions,
                self.builder.transaction.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TreasuryCreditReversal {
                amount,
                created,
                currency,
                financial_account,
                hosted_regulatory_receipt_url,
                id,
                livemode,
                metadata,
                network,
                received_credit,
                status,
                status_transitions,
                transaction,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryCreditReversal {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TreasuryCreditReversal", 14)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("financial_account", &self.financial_account)?;
        s.serialize_field("hosted_regulatory_receipt_url", &self.hosted_regulatory_receipt_url)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("network", &self.network)?;
        s.serialize_field("received_credit", &self.received_credit)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_transitions", &self.status_transitions)?;
        s.serialize_field("transaction", &self.transaction)?;

        s.serialize_field("object", "treasury.credit_reversal")?;
        s.end()
    }
}
/// The rails used to reverse the funds.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryCreditReversalNetwork {
    Ach,
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryCreditReversalNetwork {
    pub fn as_str(&self) -> &str {
        use TreasuryCreditReversalNetwork::*;
        match self {
            Ach => "ach",
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryCreditReversalNetwork {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryCreditReversalNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryCreditReversalNetwork"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryCreditReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryCreditReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryCreditReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryCreditReversalNetwork)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryCreditReversalNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TreasuryCreditReversalNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryCreditReversalNetwork> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryCreditReversalNetwork::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryCreditReversalNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for TreasuryCreditReversal {
    type Id = stripe_treasury::TreasuryCreditReversalId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TreasuryCreditReversalId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryCreditReversalStatus {
    Canceled,
    Posted,
    Processing,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryCreditReversalStatus {
    pub fn as_str(&self) -> &str {
        use TreasuryCreditReversalStatus::*;
        match self {
            Canceled => "canceled",
            Posted => "posted",
            Processing => "processing",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryCreditReversalStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryCreditReversalStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "posted" => Ok(Posted),
            "processing" => Ok(Processing),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryCreditReversalStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryCreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryCreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryCreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryCreditReversalStatus)).finish_non_exhaustive()
    }
}
impl serde::Serialize for TreasuryCreditReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TreasuryCreditReversalStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryCreditReversalStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryCreditReversalStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryCreditReversalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
