#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
pub bank_transfer: stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer,

}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder {
    bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer>,

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
            builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder {
        type Out = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_transfer" => Deserialize::begin(&mut self.bank_transfer),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { bank_transfer: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bank_transfer),) = (self.bank_transfer.take(),) else {
                return None;
            };
            Some(Self::Out { bank_transfer })
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

    impl ObjectDeser for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
        type Builder =
            CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder;
    }

    impl FromValueOpt for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_transfer" => b.bank_transfer = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
