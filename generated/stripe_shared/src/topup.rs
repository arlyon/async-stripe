/// To top up your Stripe balance, you create a top-up object. You can retrieve
/// individual top-ups, as well as list all top-ups. Top-ups are identified by a
/// unique, random ID.
///
/// Related guide: [Topping up your platform account](https://stripe.com/docs/connect/top-ups)
///
/// For more details see <<https://stripe.com/docs/api/topups/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
pub struct TopupBuilder {
    amount: Option<i64>,
    balance_transaction: Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
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

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
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
                "expected_availability_date" => Deserialize::begin(&mut self.expected_availability_date),
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
            let amount = self.amount.take()?;
            let balance_transaction = self.balance_transaction.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let description = self.description.take()?;
            let expected_availability_date = self.expected_availability_date.take()?;
            let failure_code = self.failure_code.take()?;
            let failure_message = self.failure_message.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let source = self.source.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;
            let status = self.status.take()?;
            let transfer_group = self.transfer_group.take()?;

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
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "balance_transaction" => b.balance_transaction = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "expected_availability_date" => b.expected_availability_date = Some(FromValueOpt::from_value(v)?),
                    "failure_code" => b.failure_code = Some(FromValueOpt::from_value(v)?),
                    "failure_message" => b.failure_message = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "source" => b.source = Some(FromValueOpt::from_value(v)?),
                    "statement_descriptor" => b.statement_descriptor = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "transfer_group" => b.transfer_group = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TopupStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "failed" => Ok(Failed),
            "pending" => Ok(Pending),
            "reversed" => Ok(Reversed),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
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
impl serde::Serialize for TopupStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TopupStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TopupStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TopupStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TopupStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TopupStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(TopupStatus);
impl stripe_types::Object for Topup {
    type Id = stripe_shared::TopupId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TopupId, "tu_");
