/// `Application Fee Refund` objects allow you to refund an application fee that
/// has previously been created but not yet refunded. Funds will be refunded to
/// the Stripe account from which the fee was originally collected.
///
/// Related guide: [Refunding application fees](https://stripe.com/docs/connect/destination-charges#refunding-app-fee).
///
/// For more details see <<https://stripe.com/docs/api/fee_refunds/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "min-ser")]
pub struct ApplicationFeeRefundBuilder {
    amount: Option<i64>,
    balance_transaction: Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    fee: Option<stripe_types::Expandable<stripe_shared::ApplicationFee>>,
    id: Option<stripe_shared::ApplicationFeeRefundId>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
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
            Ok(Box::new(Builder { out: &mut self.out, builder: ApplicationFeeRefundBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ApplicationFeeRefundBuilder {
        type Out = ApplicationFeeRefund;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "balance_transaction" => Deserialize::begin(&mut self.balance_transaction),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "fee" => Deserialize::begin(&mut self.fee),
                "id" => Deserialize::begin(&mut self.id),
                "metadata" => Deserialize::begin(&mut self.metadata),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                balance_transaction: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                fee: Deserialize::default(),
                id: Deserialize::default(),
                metadata: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let balance_transaction = self.balance_transaction.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let fee = self.fee.take()?;
            let id = self.id.take()?;
            let metadata = self.metadata.take()?;

            Some(Self::Out { amount, balance_transaction, created, currency, fee, id, metadata })
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

    impl ObjectDeser for ApplicationFeeRefund {
        type Builder = ApplicationFeeRefundBuilder;
    }

    impl FromValueOpt for ApplicationFeeRefund {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ApplicationFeeRefundBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "balance_transaction" => b.balance_transaction = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "fee" => b.fee = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
impl stripe_types::Object for ApplicationFeeRefund {
    type Id = stripe_shared::ApplicationFeeRefundId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ApplicationFeeRefundId, "fr_");
