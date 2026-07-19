/// `Application Fee Refund` objects allow you to refund an application fee that
/// has previously been created but not yet refunded. Funds will be refunded to
/// the Stripe account from which the fee was originally collected.
///
/// Related guide: [Refunding application fees](https://docs.stripe.com/connect/destination-charges#refunding-app-fee).
///
/// For more details see <<https://stripe.com/docs/api/fee_refunds/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ApplicationFeeRefund {
    /// Amount, in cents (or local equivalent).
    pub amount: i64,
    /// Balance transaction that describes the impact on your account balance.
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the application fee that was refunded.
    pub fee: stripe_types::Expandable<stripe_shared::ApplicationFee>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ApplicationFeeRefundId,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ApplicationFeeRefund {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ApplicationFeeRefund").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ApplicationFeeRefundBuilder {
    amount: Option<i64>,
    balance_transaction:
        Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    fee: Option<stripe_types::Expandable<stripe_shared::ApplicationFee>>,
    id: Option<stripe_shared::ApplicationFeeRefundId>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
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

    impl Deserialize for ApplicationFeeRefund {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ApplicationFeeRefund>,
        builder: ApplicationFeeRefundBuilder,
    }

    impl Visitor for Place<ApplicationFeeRefund> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ApplicationFeeRefundBuilder {
                    amount: Deserialize::default(),
                    balance_transaction: Deserialize::default(),
                    created: Deserialize::default(),
                    currency: Deserialize::default(),
                    fee: Deserialize::default(),
                    id: Deserialize::default(),
                    metadata: Deserialize::default(),
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
                "fee" => Deserialize::begin(&mut self.builder.fee),
                "id" => Deserialize::begin(&mut self.builder.id),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(balance_transaction),
                Some(created),
                Some(currency),
                Some(fee),
                Some(id),
                Some(metadata),
            ) = (
                self.builder.amount,
                self.builder.balance_transaction.take(),
                self.builder.created,
                self.builder.currency.take(),
                self.builder.fee.take(),
                self.builder.id.take(),
                self.builder.metadata.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(ApplicationFeeRefund {
                amount,
                balance_transaction,
                created,
                currency,
                fee,
                id,
                metadata,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ApplicationFeeRefund {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ApplicationFeeRefund", 8)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("balance_transaction", &self.balance_transaction)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("fee", &self.fee)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("metadata", &self.metadata)?;

        s.serialize_field("object", "fee_refund")?;
        s.end()
    }
}
impl stripe_types::Object for ApplicationFeeRefund {
    type Id = stripe_shared::ApplicationFeeRefundId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ApplicationFeeRefundId);
