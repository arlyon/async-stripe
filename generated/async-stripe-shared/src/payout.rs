/// A `Payout` object is created when you receive funds from Stripe, or when you
/// initiate a payout to either a bank account or debit card of a [connected
/// Stripe account](/docs/connect/bank-debit-card-payouts). You can retrieve individual payouts,
/// and list all payouts. Payouts are made on [varying
/// schedules](/docs/connect/manage-payout-schedule), depending on your country and
/// industry.
///
/// Related guide: [Receiving payouts](https://docs.stripe.com/payouts)
///
/// For more details see <<https://stripe.com/docs/api/payouts/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Payout {
    /// The amount (in cents (or local equivalent)) that transfers to your bank account or debit card.
    pub amount: i64,
    /// The application fee (if any) for the payout.
    /// [See the Connect documentation](https://docs.stripe.com/connect/instant-payouts#monetization-and-fees) for details.
    pub application_fee: Option<stripe_types::Expandable<stripe_shared::ApplicationFee>>,
    /// The amount of the application fee (if any) requested for the payout.
    /// [See the Connect documentation](https://docs.stripe.com/connect/instant-payouts#monetization-and-fees) for details.
    pub application_fee_amount: Option<i64>,
    /// Date that you can expect the payout to arrive in the bank.
    /// This factors in delays to account for weekends or bank holidays.
    pub arrival_date: stripe_types::Timestamp,
    /// Returns `true` if the payout is created by an [automated payout schedule](https://docs.stripe.com/payouts#payout-schedule) and `false` if it's [requested manually](https://stripe.com/docs/payouts#manual-payouts).
    pub automatic: bool,
    /// ID of the balance transaction that describes the impact of this payout on your account balance.
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// ID of the bank account or card the payout is sent to.
    pub destination: Option<stripe_types::Expandable<stripe_shared::ExternalAccount>>,
    /// If the payout fails or cancels, this is the ID of the balance transaction that reverses the initial balance transaction and returns the funds from the failed payout back in your balance.
    pub failure_balance_transaction:
        Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// Error code that provides a reason for a payout failure, if available.
    /// View our [list of failure codes](https://docs.stripe.com/api#payout_failures).
    pub failure_code: Option<String>,
    /// Message that provides the reason for a payout failure, if available.
    pub failure_message: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::PayoutId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The method used to send this payout, which can be `standard` or `instant`.
    /// `instant` is supported for payouts to debit cards and bank accounts in certain countries.
    /// Learn more about [bank support for Instant Payouts](https://stripe.com/docs/payouts/instant-payouts-banks).
    pub method: String,
    /// If the payout reverses another, this is the ID of the original payout.
    pub original_payout: Option<stripe_types::Expandable<stripe_shared::Payout>>,
    /// ID of the v2 FinancialAccount the funds are sent to.
    pub payout_method: Option<String>,
    /// If `completed`, you can use the [Balance Transactions API](https://docs.stripe.com/api/balance_transactions/list#balance_transaction_list-payout) to list all balance transactions that are paid out in this payout.
    pub reconciliation_status: PayoutReconciliationStatus,
    /// If the payout reverses, this is the ID of the payout that reverses this payout.
    pub reversed_by: Option<stripe_types::Expandable<stripe_shared::Payout>>,
    /// The source balance this payout came from, which can be one of the following: `card`, `fpx`, or `bank_account`.
    pub source_type: String,
    /// Extra information about a payout that displays on the user's bank statement.
    pub statement_descriptor: Option<String>,
    /// Current status of the payout: `paid`, `pending`, `in_transit`, `canceled` or `failed`.
    /// A payout is `pending` until it's submitted to the bank, when it becomes `in_transit`.
    /// The status changes to `paid` if the transaction succeeds, or to `failed` or `canceled` (within 5 business days).
    /// Some payouts that fail might initially show as `paid`, then change to `failed`.
    pub status: String,
    /// A value that generates from the beneficiary's bank that allows users to track payouts with their bank.
    /// Banks might call this a "reference number" or something similar.
    pub trace_id: Option<stripe_shared::PayoutsTraceId>,
    /// Can be `bank_account` or `card`.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: PayoutType,
}
#[doc(hidden)]
pub struct PayoutBuilder {
    amount: Option<i64>,
    application_fee: Option<Option<stripe_types::Expandable<stripe_shared::ApplicationFee>>>,
    application_fee_amount: Option<Option<i64>>,
    arrival_date: Option<stripe_types::Timestamp>,
    automatic: Option<bool>,
    balance_transaction:
        Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    destination: Option<Option<stripe_types::Expandable<stripe_shared::ExternalAccount>>>,
    failure_balance_transaction:
        Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    failure_code: Option<Option<String>>,
    failure_message: Option<Option<String>>,
    id: Option<stripe_shared::PayoutId>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    method: Option<String>,
    original_payout: Option<Option<stripe_types::Expandable<stripe_shared::Payout>>>,
    payout_method: Option<Option<String>>,
    reconciliation_status: Option<PayoutReconciliationStatus>,
    reversed_by: Option<Option<stripe_types::Expandable<stripe_shared::Payout>>>,
    source_type: Option<String>,
    statement_descriptor: Option<Option<String>>,
    status: Option<String>,
    trace_id: Option<Option<stripe_shared::PayoutsTraceId>>,
    type_: Option<PayoutType>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Payout {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Payout>,
        builder: PayoutBuilder,
    }

    impl Visitor for Place<Payout> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PayoutBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PayoutBuilder {
        type Out = Payout;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "application_fee" => Deserialize::begin(&mut self.application_fee),
                "application_fee_amount" => Deserialize::begin(&mut self.application_fee_amount),
                "arrival_date" => Deserialize::begin(&mut self.arrival_date),
                "automatic" => Deserialize::begin(&mut self.automatic),
                "balance_transaction" => Deserialize::begin(&mut self.balance_transaction),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "destination" => Deserialize::begin(&mut self.destination),
                "failure_balance_transaction" => {
                    Deserialize::begin(&mut self.failure_balance_transaction)
                }
                "failure_code" => Deserialize::begin(&mut self.failure_code),
                "failure_message" => Deserialize::begin(&mut self.failure_message),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "method" => Deserialize::begin(&mut self.method),
                "original_payout" => Deserialize::begin(&mut self.original_payout),
                "payout_method" => Deserialize::begin(&mut self.payout_method),
                "reconciliation_status" => Deserialize::begin(&mut self.reconciliation_status),
                "reversed_by" => Deserialize::begin(&mut self.reversed_by),
                "source_type" => Deserialize::begin(&mut self.source_type),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                "status" => Deserialize::begin(&mut self.status),
                "trace_id" => Deserialize::begin(&mut self.trace_id),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                application_fee: Deserialize::default(),
                application_fee_amount: Deserialize::default(),
                arrival_date: Deserialize::default(),
                automatic: Deserialize::default(),
                balance_transaction: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                destination: Deserialize::default(),
                failure_balance_transaction: Deserialize::default(),
                failure_code: Deserialize::default(),
                failure_message: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                method: Deserialize::default(),
                original_payout: Deserialize::default(),
                payout_method: Deserialize::default(),
                reconciliation_status: Deserialize::default(),
                reversed_by: Deserialize::default(),
                source_type: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                status: Deserialize::default(),
                trace_id: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(application_fee),
                Some(application_fee_amount),
                Some(arrival_date),
                Some(automatic),
                Some(balance_transaction),
                Some(created),
                Some(currency),
                Some(description),
                Some(destination),
                Some(failure_balance_transaction),
                Some(failure_code),
                Some(failure_message),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(method),
                Some(original_payout),
                Some(payout_method),
                Some(reconciliation_status),
                Some(reversed_by),
                Some(source_type),
                Some(statement_descriptor),
                Some(status),
                Some(trace_id),
                Some(type_),
            ) = (
                self.amount,
                self.application_fee.take(),
                self.application_fee_amount,
                self.arrival_date,
                self.automatic,
                self.balance_transaction.take(),
                self.created,
                self.currency.take(),
                self.description.take(),
                self.destination.take(),
                self.failure_balance_transaction.take(),
                self.failure_code.take(),
                self.failure_message.take(),
                self.id.take(),
                self.livemode,
                self.metadata.take(),
                self.method.take(),
                self.original_payout.take(),
                self.payout_method.take(),
                self.reconciliation_status.take(),
                self.reversed_by.take(),
                self.source_type.take(),
                self.statement_descriptor.take(),
                self.status.take(),
                self.trace_id.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                application_fee,
                application_fee_amount,
                arrival_date,
                automatic,
                balance_transaction,
                created,
                currency,
                description,
                destination,
                failure_balance_transaction,
                failure_code,
                failure_message,
                id,
                livemode,
                metadata,
                method,
                original_payout,
                payout_method,
                reconciliation_status,
                reversed_by,
                source_type,
                statement_descriptor,
                status,
                trace_id,
                type_,
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

    impl ObjectDeser for Payout {
        type Builder = PayoutBuilder;
    }

    impl FromValueOpt for Payout {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PayoutBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "application_fee" => b.application_fee = FromValueOpt::from_value(v),
                    "application_fee_amount" => {
                        b.application_fee_amount = FromValueOpt::from_value(v)
                    }
                    "arrival_date" => b.arrival_date = FromValueOpt::from_value(v),
                    "automatic" => b.automatic = FromValueOpt::from_value(v),
                    "balance_transaction" => b.balance_transaction = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "destination" => b.destination = FromValueOpt::from_value(v),
                    "failure_balance_transaction" => {
                        b.failure_balance_transaction = FromValueOpt::from_value(v)
                    }
                    "failure_code" => b.failure_code = FromValueOpt::from_value(v),
                    "failure_message" => b.failure_message = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "method" => b.method = FromValueOpt::from_value(v),
                    "original_payout" => b.original_payout = FromValueOpt::from_value(v),
                    "payout_method" => b.payout_method = FromValueOpt::from_value(v),
                    "reconciliation_status" => {
                        b.reconciliation_status = FromValueOpt::from_value(v)
                    }
                    "reversed_by" => b.reversed_by = FromValueOpt::from_value(v),
                    "source_type" => b.source_type = FromValueOpt::from_value(v),
                    "statement_descriptor" => b.statement_descriptor = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "trace_id" => b.trace_id = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Payout {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Payout", 27)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("application_fee", &self.application_fee)?;
        s.serialize_field("application_fee_amount", &self.application_fee_amount)?;
        s.serialize_field("arrival_date", &self.arrival_date)?;
        s.serialize_field("automatic", &self.automatic)?;
        s.serialize_field("balance_transaction", &self.balance_transaction)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("destination", &self.destination)?;
        s.serialize_field("failure_balance_transaction", &self.failure_balance_transaction)?;
        s.serialize_field("failure_code", &self.failure_code)?;
        s.serialize_field("failure_message", &self.failure_message)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("method", &self.method)?;
        s.serialize_field("original_payout", &self.original_payout)?;
        s.serialize_field("payout_method", &self.payout_method)?;
        s.serialize_field("reconciliation_status", &self.reconciliation_status)?;
        s.serialize_field("reversed_by", &self.reversed_by)?;
        s.serialize_field("source_type", &self.source_type)?;
        s.serialize_field("statement_descriptor", &self.statement_descriptor)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("trace_id", &self.trace_id)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "payout")?;
        s.end()
    }
}
/// If `completed`, you can use the [Balance Transactions API](https://docs.stripe.com/api/balance_transactions/list#balance_transaction_list-payout) to list all balance transactions that are paid out in this payout.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PayoutReconciliationStatus {
    Completed,
    InProgress,
    NotApplicable,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PayoutReconciliationStatus {
    pub fn as_str(&self) -> &str {
        use PayoutReconciliationStatus::*;
        match self {
            Completed => "completed",
            InProgress => "in_progress",
            NotApplicable => "not_applicable",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PayoutReconciliationStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PayoutReconciliationStatus::*;
        match s {
            "completed" => Ok(Completed),
            "in_progress" => Ok(InProgress),
            "not_applicable" => Ok(NotApplicable),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PayoutReconciliationStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PayoutReconciliationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PayoutReconciliationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PayoutReconciliationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PayoutReconciliationStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PayoutReconciliationStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PayoutReconciliationStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PayoutReconciliationStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PayoutReconciliationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Can be `bank_account` or `card`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PayoutType {
    BankAccount,
    Card,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PayoutType {
    pub fn as_str(&self) -> &str {
        use PayoutType::*;
        match self {
            BankAccount => "bank_account",
            Card => "card",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PayoutType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PayoutType::*;
        match s {
            "bank_account" => Ok(BankAccount),
            "card" => Ok(Card),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PayoutType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PayoutType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PayoutType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PayoutType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PayoutType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PayoutType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PayoutType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PayoutType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PayoutType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for Payout {
    type Id = stripe_shared::PayoutId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PayoutId);
