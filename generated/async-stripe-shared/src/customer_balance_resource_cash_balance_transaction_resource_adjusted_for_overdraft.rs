#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
    /// The [Balance Transaction](https://stripe.com/docs/api/balance_transactions/object) that corresponds to funds taken out of your Stripe balance.
    pub balance_transaction: stripe_types::Expandable<stripe_shared::BalanceTransaction>,
    /// The [Cash Balance Transaction](https://stripe.com/docs/api/cash_balance_transactions/object) that brought the customer balance negative, triggering the clawback of funds.
    pub linked_transaction: stripe_types::Expandable<stripe_shared::CustomerCashBalanceTransaction>,
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraftBuilder {
    balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    linked_transaction:
        Option<stripe_types::Expandable<stripe_shared::CustomerCashBalanceTransaction>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft,
        >,
        builder: CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraftBuilder,
    }

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraftBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraftBuilder
    {
        type Out = CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "balance_transaction" => Deserialize::begin(&mut self.balance_transaction),
                "linked_transaction" => Deserialize::begin(&mut self.linked_transaction),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                balance_transaction: Deserialize::default(),
                linked_transaction: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(balance_transaction), Some(linked_transaction)) =
                (self.balance_transaction.take(), self.linked_transaction.take())
            else {
                return None;
            };
            Some(Self::Out { balance_transaction, linked_transaction })
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

    impl ObjectDeser for CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
        type Builder =
            CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraftBuilder;
    }

    impl FromValueOpt for CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraftBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "balance_transaction" => b.balance_transaction = FromValueOpt::from_value(v),
                    "linked_transaction" => b.linked_transaction = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
