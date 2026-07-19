#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
    /// The [Balance Transaction](https://docs.stripe.com/api/balance_transactions/object) that corresponds to funds taken out of your Stripe balance.
    pub balance_transaction: stripe_types::Expandable<stripe_shared::BalanceTransaction>,
    /// The [Cash Balance Transaction](https://docs.stripe.com/api/cash_balance_transactions/object) that brought the customer balance negative, triggering the clawback of funds.
    pub linked_transaction: stripe_types::Expandable<stripe_shared::CustomerCashBalanceTransaction>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft")
            .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
            builder: CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraftBuilder { balance_transaction: Deserialize::default(),
linked_transaction: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "balance_transaction" => Deserialize::begin(&mut self.builder.balance_transaction),
                "linked_transaction" => Deserialize::begin(&mut self.builder.linked_transaction),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(balance_transaction), Some(linked_transaction)) =
                (self.builder.balance_transaction.take(), self.builder.linked_transaction.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
                    balance_transaction,
                    linked_transaction,
                });
            Ok(())
        }
    }
};
