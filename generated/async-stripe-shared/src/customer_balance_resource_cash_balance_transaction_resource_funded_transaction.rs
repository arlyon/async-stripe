#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
pub bank_transfer: stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder {
    bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer>,

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

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction>,
        builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder,
    }

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder {
                        bank_transfer: Deserialize::default(),
                    },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_transfer" => Deserialize::begin(&mut self.builder.bank_transfer),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(bank_transfer),) = (self.builder.bank_transfer.take(),) else {
                return Ok(());
            };
            *self.out =
                Some(CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
                    bank_transfer,
                });
            Ok(())
        }
    }
};
