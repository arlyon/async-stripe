/// A Transaction represents a real transaction that affects a Financial Connections Account balance.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FinancialConnectionsTransaction {
    /// The ID of the Financial Connections Account this transaction belongs to.
    pub account: String,
    /// The amount of this transaction, in cents (or local equivalent).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The description of this transaction.
    pub description: String,
    /// Unique identifier for the object.
    pub id: stripe_misc::FinancialConnectionsTransactionId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The status of the transaction.
    pub status: FinancialConnectionsTransactionStatus,
    pub status_transitions:
        stripe_misc::BankConnectionsResourceTransactionResourceStatusTransitions,
    /// Time at which the transaction was transacted. Measured in seconds since the Unix epoch.
    pub transacted_at: stripe_types::Timestamp,
    /// The token of the transaction refresh that last updated or created this transaction.
    pub transaction_refresh: String,
    /// Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FinancialConnectionsTransaction").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct FinancialConnectionsTransactionBuilder {
    account: Option<String>,
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    description: Option<String>,
    id: Option<stripe_misc::FinancialConnectionsTransactionId>,
    livemode: Option<bool>,
    status: Option<FinancialConnectionsTransactionStatus>,
    status_transitions:
        Option<stripe_misc::BankConnectionsResourceTransactionResourceStatusTransitions>,
    transacted_at: Option<stripe_types::Timestamp>,
    transaction_refresh: Option<String>,
    updated: Option<stripe_types::Timestamp>,
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

    impl Deserialize for FinancialConnectionsTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FinancialConnectionsTransaction>,
        builder: FinancialConnectionsTransactionBuilder,
    }

    impl Visitor for Place<FinancialConnectionsTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FinancialConnectionsTransactionBuilder {
                    account: Deserialize::default(),
                    amount: Deserialize::default(),
                    currency: Deserialize::default(),
                    description: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    status: Deserialize::default(),
                    status_transitions: Deserialize::default(),
                    transacted_at: Deserialize::default(),
                    transaction_refresh: Deserialize::default(),
                    updated: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.builder.account),
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "description" => Deserialize::begin(&mut self.builder.description),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "status" => Deserialize::begin(&mut self.builder.status),
                "status_transitions" => Deserialize::begin(&mut self.builder.status_transitions),
                "transacted_at" => Deserialize::begin(&mut self.builder.transacted_at),
                "transaction_refresh" => Deserialize::begin(&mut self.builder.transaction_refresh),
                "updated" => Deserialize::begin(&mut self.builder.updated),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account),
                Some(amount),
                Some(currency),
                Some(description),
                Some(id),
                Some(livemode),
                Some(status),
                Some(status_transitions),
                Some(transacted_at),
                Some(transaction_refresh),
                Some(updated),
            ) = (
                self.builder.account.take(),
                self.builder.amount,
                self.builder.currency.take(),
                self.builder.description.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.status.take(),
                self.builder.status_transitions,
                self.builder.transacted_at,
                self.builder.transaction_refresh.take(),
                self.builder.updated,
            )
            else {
                return Ok(());
            };
            *self.out = Some(FinancialConnectionsTransaction {
                account,
                amount,
                currency,
                description,
                id,
                livemode,
                status,
                status_transitions,
                transacted_at,
                transaction_refresh,
                updated,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsTransaction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("FinancialConnectionsTransaction", 12)?;
        s.serialize_field("account", &self.account)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_transitions", &self.status_transitions)?;
        s.serialize_field("transacted_at", &self.transacted_at)?;
        s.serialize_field("transaction_refresh", &self.transaction_refresh)?;
        s.serialize_field("updated", &self.updated)?;

        s.serialize_field("object", "financial_connections.transaction")?;
        s.end()
    }
}
/// The status of the transaction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FinancialConnectionsTransactionStatus {
    Pending,
    Posted,
    Void,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FinancialConnectionsTransactionStatus {
    pub fn as_str(&self) -> &str {
        use FinancialConnectionsTransactionStatus::*;
        match self {
            Pending => "pending",
            Posted => "posted",
            Void => "void",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FinancialConnectionsTransactionStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsTransactionStatus::*;
        match s {
            "pending" => Ok(Pending),
            "posted" => Ok(Posted),
            "void" => Ok(Void),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FinancialConnectionsTransactionStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FinancialConnectionsTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FinancialConnectionsTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FinancialConnectionsTransactionStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsTransactionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for FinancialConnectionsTransactionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<FinancialConnectionsTransactionStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialConnectionsTransactionStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FinancialConnectionsTransactionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for FinancialConnectionsTransaction {
    type Id = stripe_misc::FinancialConnectionsTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(FinancialConnectionsTransactionId);
