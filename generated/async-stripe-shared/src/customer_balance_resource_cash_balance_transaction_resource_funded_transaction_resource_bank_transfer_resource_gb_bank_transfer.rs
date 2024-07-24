#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer
{
    /// The last 4 digits of the account number of the sender of the funding.
    pub account_number_last4: Option<String>,
    /// The full name of the sender, as supplied by the sending bank.
    pub sender_name: Option<String>,
    /// The sort code of the bank of the sender of the funding
    pub sort_code: Option<String>,
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransferBuilder
{
    account_number_last4: Option<Option<String>>,
    sender_name: Option<Option<String>>,
    sort_code: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

    struct Builder<'a> {
    out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer>,
    builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransferBuilder,
}

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransferBuilder::deser_default(),
        }))
    }
}

    impl MapBuilder for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransferBuilder {
    type Out = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "account_number_last4" => Deserialize::begin(&mut self.account_number_last4),
"sender_name" => Deserialize::begin(&mut self.sender_name),
"sort_code" => Deserialize::begin(&mut self.sort_code),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            account_number_last4: Deserialize::default(),
sender_name: Deserialize::default(),
sort_code: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        Some(Self::Out { account_number_last4: self.account_number_last4.take()?,
sender_name: self.sender_name.take()?,
sort_code: self.sort_code.take()?,
 })
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

    impl ObjectDeser for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer {
    type Builder = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransferBuilder;
}

    impl FromValueOpt for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransferBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "account_number_last4" => b.account_number_last4 = Some(FromValueOpt::from_value(v)?),
"sender_name" => b.sender_name = Some(FromValueOpt::from_value(v)?),
"sort_code" => b.sort_code = Some(FromValueOpt::from_value(v)?),

                _ => {}
            }
        }
        b.take_out()
    }
}
};
