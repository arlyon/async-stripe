#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance {
    /// The [Balance Transaction](https://docs.stripe.com/api/balance_transactions/object) that corresponds to funds transferred to your Stripe balance.
    pub balance_transaction: stripe_types::Expandable<stripe_shared::BalanceTransaction>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalanceBuilder {
    balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
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
            builder: CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalanceBuilder { balance_transaction: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "balance_transaction" => Deserialize::begin(&mut self.builder.balance_transaction),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(balance_transaction),) = (self.builder.balance_transaction.take(),) else {
                return Ok(());
            };
            *self.out =
                Some(CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance {
                    balance_transaction,
                });
            Ok(())
        }
    }
};
