/// A Transaction represents a real transaction that affects a Financial Connections Account balance.
#[derive(Clone, Debug)]
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
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
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
                builder: FinancialConnectionsTransactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FinancialConnectionsTransactionBuilder {
        type Out = FinancialConnectionsTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "amount" => Deserialize::begin(&mut self.amount),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "status" => Deserialize::begin(&mut self.status),
                "status_transitions" => Deserialize::begin(&mut self.status_transitions),
                "transacted_at" => Deserialize::begin(&mut self.transacted_at),
                "transaction_refresh" => Deserialize::begin(&mut self.transaction_refresh),
                "updated" => Deserialize::begin(&mut self.updated),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.account.take(),
                self.amount,
                self.currency,
                self.description.take(),
                self.id.take(),
                self.livemode,
                self.status,
                self.status_transitions,
                self.transacted_at,
                self.transaction_refresh.take(),
                self.updated,
            )
            else {
                return None;
            };
            Some(Self::Out {
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

    impl ObjectDeser for FinancialConnectionsTransaction {
        type Builder = FinancialConnectionsTransactionBuilder;
    }

    impl FromValueOpt for FinancialConnectionsTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FinancialConnectionsTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = FromValueOpt::from_value(v),
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "status_transitions" => b.status_transitions = FromValueOpt::from_value(v),
                    "transacted_at" => b.transacted_at = FromValueOpt::from_value(v),
                    "transaction_refresh" => b.transaction_refresh = FromValueOpt::from_value(v),
                    "updated" => b.updated = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FinancialConnectionsTransactionStatus {
    Pending,
    Posted,
    Void,
}
impl FinancialConnectionsTransactionStatus {
    pub fn as_str(self) -> &'static str {
        use FinancialConnectionsTransactionStatus::*;
        match self {
            Pending => "pending",
            Posted => "posted",
            Void => "void",
        }
    }
}

impl std::str::FromStr for FinancialConnectionsTransactionStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsTransactionStatus::*;
        match s {
            "pending" => Ok(Pending),
            "posted" => Ok(Posted),
            "void" => Ok(Void),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for FinancialConnectionsTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FinancialConnectionsTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for FinancialConnectionsTransactionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FinancialConnectionsTransactionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(FinancialConnectionsTransactionStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FinancialConnectionsTransactionStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FinancialConnectionsTransactionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialConnectionsTransactionStatus")
        })
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
