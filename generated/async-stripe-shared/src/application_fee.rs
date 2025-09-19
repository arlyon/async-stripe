/// For more details see <<https://stripe.com/docs/api/application_fees/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ApplicationFee {
    /// ID of the Stripe account this fee was taken from.
    pub account: stripe_types::Expandable<stripe_shared::Account>,
    /// Amount earned, in cents (or local equivalent).
    pub amount: i64,
    /// Amount in cents (or local equivalent) refunded (can be less than the amount attribute on the fee if a partial refund was issued).
    pub amount_refunded: i64,
    /// ID of the Connect application that earned the fee.
    pub application: stripe_types::Expandable<stripe_shared::Application>,
    /// Balance transaction that describes the impact of this collected application fee on your account balance (not including refunds).
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// ID of the charge that the application fee was taken from.
    pub charge: stripe_types::Expandable<stripe_shared::Charge>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Polymorphic source of the application fee.
    /// Includes the ID of the object the application fee was created from.
    pub fee_source: Option<stripe_shared::PlatformEarningFeeSource>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ApplicationFeeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// ID of the corresponding charge on the platform account, if this fee was the result of a charge using the `destination` parameter.
    pub originating_transaction: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// Whether the fee has been fully refunded.
    /// If the fee is only partially refunded, this attribute will still be false.
    pub refunded: bool,
    /// A list of refunds that have been applied to the fee.
    pub refunds: stripe_types::List<stripe_shared::ApplicationFeeRefund>,
}
#[doc(hidden)]
pub struct ApplicationFeeBuilder {
    account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    amount: Option<i64>,
    amount_refunded: Option<i64>,
    application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    balance_transaction:
        Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    fee_source: Option<Option<stripe_shared::PlatformEarningFeeSource>>,
    id: Option<stripe_shared::ApplicationFeeId>,
    livemode: Option<bool>,
    originating_transaction: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    refunded: Option<bool>,
    refunds: Option<stripe_types::List<stripe_shared::ApplicationFeeRefund>>,
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

    impl Deserialize for ApplicationFee {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ApplicationFee>,
        builder: ApplicationFeeBuilder,
    }

    impl Visitor for Place<ApplicationFee> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ApplicationFeeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ApplicationFeeBuilder {
        type Out = ApplicationFee;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_refunded" => Deserialize::begin(&mut self.amount_refunded),
                "application" => Deserialize::begin(&mut self.application),
                "balance_transaction" => Deserialize::begin(&mut self.balance_transaction),
                "charge" => Deserialize::begin(&mut self.charge),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "fee_source" => Deserialize::begin(&mut self.fee_source),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "originating_transaction" => Deserialize::begin(&mut self.originating_transaction),
                "refunded" => Deserialize::begin(&mut self.refunded),
                "refunds" => Deserialize::begin(&mut self.refunds),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account: Deserialize::default(),
                amount: Deserialize::default(),
                amount_refunded: Deserialize::default(),
                application: Deserialize::default(),
                balance_transaction: Deserialize::default(),
                charge: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                fee_source: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                originating_transaction: Deserialize::default(),
                refunded: Deserialize::default(),
                refunds: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account),
                Some(amount),
                Some(amount_refunded),
                Some(application),
                Some(balance_transaction),
                Some(charge),
                Some(created),
                Some(currency),
                Some(fee_source),
                Some(id),
                Some(livemode),
                Some(originating_transaction),
                Some(refunded),
                Some(refunds),
            ) = (
                self.account.take(),
                self.amount,
                self.amount_refunded,
                self.application.take(),
                self.balance_transaction.take(),
                self.charge.take(),
                self.created,
                self.currency.take(),
                self.fee_source.take(),
                self.id.take(),
                self.livemode,
                self.originating_transaction.take(),
                self.refunded,
                self.refunds.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account,
                amount,
                amount_refunded,
                application,
                balance_transaction,
                charge,
                created,
                currency,
                fee_source,
                id,
                livemode,
                originating_transaction,
                refunded,
                refunds,
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

    impl ObjectDeser for ApplicationFee {
        type Builder = ApplicationFeeBuilder;
    }

    impl FromValueOpt for ApplicationFee {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ApplicationFeeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = FromValueOpt::from_value(v),
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_refunded" => b.amount_refunded = FromValueOpt::from_value(v),
                    "application" => b.application = FromValueOpt::from_value(v),
                    "balance_transaction" => b.balance_transaction = FromValueOpt::from_value(v),
                    "charge" => b.charge = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "fee_source" => b.fee_source = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "originating_transaction" => {
                        b.originating_transaction = FromValueOpt::from_value(v)
                    }
                    "refunded" => b.refunded = FromValueOpt::from_value(v),
                    "refunds" => b.refunds = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ApplicationFee {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ApplicationFee", 15)?;
        s.serialize_field("account", &self.account)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_refunded", &self.amount_refunded)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("balance_transaction", &self.balance_transaction)?;
        s.serialize_field("charge", &self.charge)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("fee_source", &self.fee_source)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("originating_transaction", &self.originating_transaction)?;
        s.serialize_field("refunded", &self.refunded)?;
        s.serialize_field("refunds", &self.refunds)?;

        s.serialize_field("object", "application_fee")?;
        s.end()
    }
}
impl stripe_types::Object for ApplicationFee {
    type Id = stripe_shared::ApplicationFeeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ApplicationFeeId);
