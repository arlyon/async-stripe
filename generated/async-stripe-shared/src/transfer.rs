/// A `Transfer` object is created when you move funds between Stripe accounts as
/// part of Connect.
///
/// Before April 6, 2017, transfers also represented movement of funds from a
/// Stripe account to a card or bank account. This behavior has since been split
/// out into a [Payout](https://api.stripe.com#payout_object) object, with corresponding payout endpoints.
/// For more.
/// information, read about the
/// [transfer/payout split](https://docs.stripe.com/transfer-payout-split).
///
/// Related guide: [Creating separate charges and transfers](https://docs.stripe.com/connect/separate-charges-and-transfers).
///
/// For more details see <<https://stripe.com/docs/api/transfers/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Transfer {
    /// Amount in cents (or local equivalent) to be transferred.
    pub amount: i64,
    /// Amount in cents (or local equivalent) reversed (can be less than the amount attribute on the transfer if a partial reversal was issued).
    pub amount_reversed: i64,
    /// Balance transaction that describes the impact of this transfer on your account balance.
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// Time that this record of the transfer was first created.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// ID of the Stripe account the transfer was sent to.
    pub destination: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// If the destination is a Stripe account, this will be the ID of the payment that the destination account received for the transfer.
    pub destination_payment: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::TransferId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// A list of reversals that have been applied to the transfer.
    pub reversals: stripe_types::List<stripe_shared::TransferReversal>,
    /// Whether the transfer has been fully reversed.
    /// If the transfer is only partially reversed, this attribute will still be false.
    pub reversed: bool,
    /// ID of the charge that was used to fund the transfer.
    /// If null, the transfer was funded from the available balance.
    pub source_transaction: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// The source balance this transfer came from. One of `card`, `fpx`, or `bank_account`.
    pub source_type: Option<String>,
    /// A string that identifies this transaction as part of a group.
    /// See the [Connect documentation](https://docs.stripe.com/connect/separate-charges-and-transfers#transfer-options) for details.
    pub transfer_group: Option<String>,
}
#[doc(hidden)]
pub struct TransferBuilder {
    amount: Option<i64>,
    amount_reversed: Option<i64>,
    balance_transaction:
        Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    destination: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    destination_payment: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    id: Option<stripe_shared::TransferId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    reversals: Option<stripe_types::List<stripe_shared::TransferReversal>>,
    reversed: Option<bool>,
    source_transaction: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    source_type: Option<Option<String>>,
    transfer_group: Option<Option<String>>,
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

    impl Deserialize for Transfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Transfer>,
        builder: TransferBuilder,
    }

    impl Visitor for Place<Transfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TransferBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TransferBuilder {
        type Out = Transfer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_reversed" => Deserialize::begin(&mut self.amount_reversed),
                "balance_transaction" => Deserialize::begin(&mut self.balance_transaction),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "destination" => Deserialize::begin(&mut self.destination),
                "destination_payment" => Deserialize::begin(&mut self.destination_payment),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "reversals" => Deserialize::begin(&mut self.reversals),
                "reversed" => Deserialize::begin(&mut self.reversed),
                "source_transaction" => Deserialize::begin(&mut self.source_transaction),
                "source_type" => Deserialize::begin(&mut self.source_type),
                "transfer_group" => Deserialize::begin(&mut self.transfer_group),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_reversed: Deserialize::default(),
                balance_transaction: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                destination: Deserialize::default(),
                destination_payment: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                reversals: Deserialize::default(),
                reversed: Deserialize::default(),
                source_transaction: Deserialize::default(),
                source_type: Deserialize::default(),
                transfer_group: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(amount_reversed),
                Some(balance_transaction),
                Some(created),
                Some(currency),
                Some(description),
                Some(destination),
                Some(destination_payment),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(reversals),
                Some(reversed),
                Some(source_transaction),
                Some(source_type),
                Some(transfer_group),
            ) = (
                self.amount,
                self.amount_reversed,
                self.balance_transaction.take(),
                self.created,
                self.currency.take(),
                self.description.take(),
                self.destination.take(),
                self.destination_payment.take(),
                self.id.take(),
                self.livemode,
                self.metadata.take(),
                self.reversals.take(),
                self.reversed,
                self.source_transaction.take(),
                self.source_type.take(),
                self.transfer_group.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                amount_reversed,
                balance_transaction,
                created,
                currency,
                description,
                destination,
                destination_payment,
                id,
                livemode,
                metadata,
                reversals,
                reversed,
                source_transaction,
                source_type,
                transfer_group,
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

    impl ObjectDeser for Transfer {
        type Builder = TransferBuilder;
    }

    impl FromValueOpt for Transfer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TransferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_reversed" => b.amount_reversed = FromValueOpt::from_value(v),
                    "balance_transaction" => b.balance_transaction = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "destination" => b.destination = FromValueOpt::from_value(v),
                    "destination_payment" => b.destination_payment = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "reversals" => b.reversals = FromValueOpt::from_value(v),
                    "reversed" => b.reversed = FromValueOpt::from_value(v),
                    "source_transaction" => b.source_transaction = FromValueOpt::from_value(v),
                    "source_type" => b.source_type = FromValueOpt::from_value(v),
                    "transfer_group" => b.transfer_group = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Transfer {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Transfer", 17)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_reversed", &self.amount_reversed)?;
        s.serialize_field("balance_transaction", &self.balance_transaction)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("destination", &self.destination)?;
        s.serialize_field("destination_payment", &self.destination_payment)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("reversals", &self.reversals)?;
        s.serialize_field("reversed", &self.reversed)?;
        s.serialize_field("source_transaction", &self.source_transaction)?;
        s.serialize_field("source_type", &self.source_type)?;
        s.serialize_field("transfer_group", &self.transfer_group)?;

        s.serialize_field("object", "transfer")?;
        s.end()
    }
}
impl stripe_types::Object for Transfer {
    type Id = stripe_shared::TransferId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TransferId);
