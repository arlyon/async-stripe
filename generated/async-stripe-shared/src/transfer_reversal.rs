/// [Stripe Connect](https://docs.stripe.com/connect) platforms can reverse transfers made to a
/// connected account, either entirely or partially, and can also specify whether
/// to refund any related application fees. Transfer reversals add to the
/// platform's balance and subtract from the destination account's balance.
///
/// Reversing a transfer that was made for a [destination
/// charge](/docs/connect/destination-charges) is allowed only up to the amount of
/// the charge. It is possible to reverse a
/// [transfer_group](https://docs.stripe.com/connect/separate-charges-and-transfers#transfer-options)
/// transfer only if the destination account has enough balance to cover the
/// reversal.
///
/// Related guide: [Reverse transfers](https://docs.stripe.com/connect/separate-charges-and-transfers#reverse-transfers).
///
/// For more details see <<https://stripe.com/docs/api/transfer_reversals/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
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
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// ID of the refund responsible for the transfer reversal.
    pub source_refund: Option<stripe_types::Expandable<stripe_shared::Refund>>,
    /// ID of the transfer that was reversed.
    pub transfer: stripe_types::Expandable<stripe_shared::Transfer>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TransferReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TransferReversal").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TransferReversalBuilder {
    amount: Option<i64>,
    balance_transaction:
        Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    destination_payment_refund: Option<Option<stripe_types::Expandable<stripe_shared::Refund>>>,
    id: Option<stripe_shared::TransferReversalId>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    source_refund: Option<Option<stripe_types::Expandable<stripe_shared::Refund>>>,
    transfer: Option<stripe_types::Expandable<stripe_shared::Transfer>>,
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
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TransferReversalBuilder {
                    amount: Deserialize::default(),
                    balance_transaction: Deserialize::default(),
                    created: Deserialize::default(),
                    currency: Deserialize::default(),
                    destination_payment_refund: Deserialize::default(),
                    id: Deserialize::default(),
                    metadata: Deserialize::default(),
                    source_refund: Deserialize::default(),
                    transfer: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "balance_transaction" => Deserialize::begin(&mut self.builder.balance_transaction),
                "created" => Deserialize::begin(&mut self.builder.created),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "destination_payment_refund" => {
                    Deserialize::begin(&mut self.builder.destination_payment_refund)
                }
                "id" => Deserialize::begin(&mut self.builder.id),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "source_refund" => Deserialize::begin(&mut self.builder.source_refund),
                "transfer" => Deserialize::begin(&mut self.builder.transfer),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(balance_transaction),
                Some(created),
                Some(currency),
                Some(destination_payment_refund),
                Some(id),
                Some(metadata),
                Some(source_refund),
                Some(transfer),
            ) = (
                self.builder.amount,
                self.builder.balance_transaction.take(),
                self.builder.created,
                self.builder.currency.take(),
                self.builder.destination_payment_refund.take(),
                self.builder.id.take(),
                self.builder.metadata.take(),
                self.builder.source_refund.take(),
                self.builder.transfer.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TransferReversal {
                amount,
                balance_transaction,
                created,
                currency,
                destination_payment_refund,
                id,
                metadata,
                source_refund,
                transfer,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TransferReversal {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TransferReversal", 10)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("balance_transaction", &self.balance_transaction)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("destination_payment_refund", &self.destination_payment_refund)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("source_refund", &self.source_refund)?;
        s.serialize_field("transfer", &self.transfer)?;

        s.serialize_field("object", "transfer_reversal")?;
        s.end()
    }
}
impl stripe_types::Object for TransferReversal {
    type Id = stripe_shared::TransferReversalId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TransferReversalId);
