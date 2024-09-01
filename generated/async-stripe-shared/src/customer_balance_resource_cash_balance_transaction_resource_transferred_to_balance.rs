#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance {
    /// The [Balance Transaction](https://stripe.com/docs/api/balance_transactions/object) that corresponds to funds transferred to your Stripe balance.
    pub balance_transaction: stripe_types::Expandable<stripe_shared::BalanceTransaction>,
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalanceBuilder {
    balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance,
        >,
        builder: CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalanceBuilder,
    }

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalanceBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalanceBuilder
    {
        type Out = CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "balance_transaction" => Deserialize::begin(&mut self.balance_transaction),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { balance_transaction: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(balance_transaction),) = (self.balance_transaction.take(),) else {
                return None;
            };
            Some(Self::Out { balance_transaction })
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

    impl ObjectDeser for CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance {
        type Builder =
            CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalanceBuilder;
    }

    impl FromValueOpt for CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "balance_transaction" => b.balance_transaction = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
