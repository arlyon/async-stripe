/// [Stripe Connect](https://stripe.com/docs/connect) platforms can reverse transfers made to a
/// connected account, either entirely or partially, and can also specify whether
/// to refund any related application fees. Transfer reversals add to the
/// platform's balance and subtract from the destination account's balance.
///
/// Reversing a transfer that was made for a [destination
/// charge](/docs/connect/destination-charges) is allowed only up to the amount of
/// the charge. It is possible to reverse a
/// [transfer_group](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options)
/// transfer only if the destination account has enough balance to cover the
/// reversal.
///
/// Related guide: [Reversing transfers](https://stripe.com/docs/connect/separate-charges-and-transfers#reversing-transfers).
///
/// For more details see <<https://stripe.com/docs/api/transfer_reversals/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TransferReversal {
    /// Amount, in cents (or local equivalent).
    pub amount: i64,
    /// Balance transaction that describes the impact on your account balance.
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Linked payment refund for the transfer reversal.
    pub destination_payment_refund: Option<stripe_types::Expandable<stripe_shared::Refund>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::TransferReversalId,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// ID of the refund responsible for the transfer reversal.
    pub source_refund: Option<stripe_types::Expandable<stripe_shared::Refund>>,
    /// ID of the transfer that was reversed.
    pub transfer: stripe_types::Expandable<stripe_shared::Transfer>,
}
#[cfg(feature = "min-ser")]
pub struct TransferReversalBuilder {
    amount: Option<i64>,
    balance_transaction: Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    destination_payment_refund: Option<Option<stripe_types::Expandable<stripe_shared::Refund>>>,
    id: Option<stripe_shared::TransferReversalId>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    source_refund: Option<Option<stripe_types::Expandable<stripe_shared::Refund>>>,
    transfer: Option<stripe_types::Expandable<stripe_shared::Transfer>>,
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

    impl Deserialize for TransferReversal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TransferReversal>,
        builder: TransferReversalBuilder,
    }

    impl Visitor for Place<TransferReversal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TransferReversalBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TransferReversalBuilder {
        type Out = TransferReversal;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "balance_transaction" => Deserialize::begin(&mut self.balance_transaction),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "destination_payment_refund" => Deserialize::begin(&mut self.destination_payment_refund),
                "id" => Deserialize::begin(&mut self.id),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "source_refund" => Deserialize::begin(&mut self.source_refund),
                "transfer" => Deserialize::begin(&mut self.transfer),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                balance_transaction: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                destination_payment_refund: Deserialize::default(),
                id: Deserialize::default(),
                metadata: Deserialize::default(),
                source_refund: Deserialize::default(),
                transfer: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let balance_transaction = self.balance_transaction.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let destination_payment_refund = self.destination_payment_refund.take()?;
            let id = self.id.take()?;
            let metadata = self.metadata.take()?;
            let source_refund = self.source_refund.take()?;
            let transfer = self.transfer.take()?;

            Some(Self::Out { amount, balance_transaction, created, currency, destination_payment_refund, id, metadata, source_refund, transfer })
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

    impl ObjectDeser for TransferReversal {
        type Builder = TransferReversalBuilder;
    }

    impl FromValueOpt for TransferReversal {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TransferReversalBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "balance_transaction" => b.balance_transaction = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "destination_payment_refund" => b.destination_payment_refund = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "source_refund" => b.source_refund = Some(FromValueOpt::from_value(v)?),
                    "transfer" => b.transfer = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
impl stripe_types::Object for TransferReversal {
    type Id = stripe_shared::TransferReversalId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TransferReversalId, "trr_");
