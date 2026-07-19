/// For more details see <<https://stripe.com/docs/api/application_fees/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// ID of the corresponding charge on the platform account, if this fee was the result of a charge using the `destination` parameter.
    pub originating_transaction: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// Whether the fee has been fully refunded.
    /// If the fee is only partially refunded, this attribute will still be false.
    pub refunded: bool,
    /// A list of refunds that have been applied to the fee.
    pub refunds: stripe_types::List<stripe_shared::ApplicationFeeRefund>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ApplicationFee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ApplicationFee").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: ApplicationFeeBuilder {
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
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.builder.account),
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "amount_refunded" => Deserialize::begin(&mut self.builder.amount_refunded),
                "application" => Deserialize::begin(&mut self.builder.application),
                "balance_transaction" => Deserialize::begin(&mut self.builder.balance_transaction),
                "charge" => Deserialize::begin(&mut self.builder.charge),
                "created" => Deserialize::begin(&mut self.builder.created),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "fee_source" => Deserialize::begin(&mut self.builder.fee_source),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "originating_transaction" => {
                    Deserialize::begin(&mut self.builder.originating_transaction)
                }
                "refunded" => Deserialize::begin(&mut self.builder.refunded),
                "refunds" => Deserialize::begin(&mut self.builder.refunds),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.account.take(),
                self.builder.amount,
                self.builder.amount_refunded,
                self.builder.application.take(),
                self.builder.balance_transaction.take(),
                self.builder.charge.take(),
                self.builder.created,
                self.builder.currency.take(),
                self.builder.fee_source.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.originating_transaction.take(),
                self.builder.refunded,
                self.builder.refunds.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(ApplicationFee {
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
            });
            Ok(())
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
