/// To top up your Stripe balance, you create a top-up object. You can retrieve
/// individual top-ups, as well as list all top-ups. Top-ups are identified by a
/// unique, random ID.
///
/// Related guide: [Topping up your platform account](https://stripe.com/docs/connect/top-ups)
///
/// For more details see <<https://stripe.com/docs/api/topups/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Topup {
    /// Amount transferred.
    pub amount: i64,
    /// ID of the balance transaction that describes the impact of this top-up on your account balance.
    /// May not be specified depending on status of top-up.
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Date the funds are expected to arrive in your Stripe account for payouts.
    /// This factors in delays like weekends or bank holidays.
    /// May not be specified depending on status of top-up.
    pub expected_availability_date: Option<stripe_types::Timestamp>,
    /// Error code explaining reason for top-up failure if available (see [the errors section](https://stripe.com/docs/api#errors) for a list of codes).
    pub failure_code: Option<String>,
    /// Message to user further explaining reason for top-up failure if available.
    pub failure_message: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::TopupId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The source field is deprecated. It might not always be present in the API response.
    pub source: Option<stripe_shared::Source>,
    /// Extra information about a top-up.
    /// This will appear on your source's bank statement.
    /// It must contain at least one letter.
    pub statement_descriptor: Option<String>,
    /// The status of the top-up is either `canceled`, `failed`, `pending`, `reversed`, or `succeeded`.
    pub status: TopupStatus,
    /// A string that identifies this top-up as part of a group.
    pub transfer_group: Option<String>,
}
#[doc(hidden)]
pub struct TopupBuilder {
    amount: Option<i64>,
    balance_transaction:
        Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    expected_availability_date: Option<Option<stripe_types::Timestamp>>,
    failure_code: Option<Option<String>>,
    failure_message: Option<Option<String>>,
    id: Option<stripe_shared::TopupId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    source: Option<Option<stripe_shared::Source>>,
    statement_descriptor: Option<Option<String>>,
    status: Option<TopupStatus>,
    transfer_group: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for Topup {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Topup>,
        builder: TopupBuilder,
    }

    impl Visitor for Place<Topup> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TopupBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TopupBuilder {
        type Out = Topup;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "balance_transaction" => Deserialize::begin(&mut self.balance_transaction),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "expected_availability_date" => {
                    Deserialize::begin(&mut self.expected_availability_date)
                }
                "failure_code" => Deserialize::begin(&mut self.failure_code),
                "failure_message" => Deserialize::begin(&mut self.failure_message),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "source" => Deserialize::begin(&mut self.source),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                "status" => Deserialize::begin(&mut self.status),
                "transfer_group" => Deserialize::begin(&mut self.transfer_group),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                balance_transaction: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                expected_availability_date: Deserialize::default(),
                failure_code: Deserialize::default(),
                failure_message: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                source: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                status: Deserialize::default(),
                transfer_group: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(balance_transaction),
                Some(created),
                Some(currency),
                Some(description),
                Some(expected_availability_date),
                Some(failure_code),
                Some(failure_message),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(source),
                Some(statement_descriptor),
                Some(status),
                Some(transfer_group),
            ) = (
                self.amount,
                self.balance_transaction.take(),
                self.created,
                self.currency,
                self.description.take(),
                self.expected_availability_date,
                self.failure_code.take(),
                self.failure_message.take(),
                self.id.take(),
                self.livemode,
                self.metadata.take(),
                self.source.take(),
                self.statement_descriptor.take(),
                self.status,
                self.transfer_group.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                balance_transaction,
                created,
                currency,
                description,
                expected_availability_date,
                failure_code,
                failure_message,
                id,
                livemode,
                metadata,
                source,
                statement_descriptor,
                status,
                transfer_group,
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

    impl ObjectDeser for Topup {
        type Builder = TopupBuilder;
    }

    impl FromValueOpt for Topup {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TopupBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "balance_transaction" => b.balance_transaction = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "expected_availability_date" => {
                        b.expected_availability_date = FromValueOpt::from_value(v)
                    }
                    "failure_code" => b.failure_code = FromValueOpt::from_value(v),
                    "failure_message" => b.failure_message = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "source" => b.source = FromValueOpt::from_value(v),
                    "statement_descriptor" => b.statement_descriptor = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "transfer_group" => b.transfer_group = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Topup {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Topup", 16)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("balance_transaction", &self.balance_transaction)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("expected_availability_date", &self.expected_availability_date)?;
        s.serialize_field("failure_code", &self.failure_code)?;
        s.serialize_field("failure_message", &self.failure_message)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("source", &self.source)?;
        s.serialize_field("statement_descriptor", &self.statement_descriptor)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("transfer_group", &self.transfer_group)?;

        s.serialize_field("object", "topup")?;
        s.end()
    }
}
/// The status of the top-up is either `canceled`, `failed`, `pending`, `reversed`, or `succeeded`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TopupStatus {
    Canceled,
    Failed,
    Pending,
    Reversed,
    Succeeded,
}
impl TopupStatus {
    pub fn as_str(self) -> &'static str {
        use TopupStatus::*;
        match self {
            Canceled => "canceled",
            Failed => "failed",
            Pending => "pending",
            Reversed => "reversed",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TopupStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TopupStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "failed" => Ok(Failed),
            "pending" => Ok(Pending),
            "reversed" => Ok(Reversed),
            "succeeded" => Ok(Succeeded),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TopupStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TopupStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TopupStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TopupStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TopupStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TopupStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TopupStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TopupStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TopupStatus"))
    }
}
impl stripe_types::Object for Topup {
    type Id = stripe_shared::TopupId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TopupId);
