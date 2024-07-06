#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer
{
    /// The name of the bank of the sender of the funding.
    pub sender_bank: Option<String>,
    /// The name of the bank branch of the sender of the funding.
    pub sender_branch: Option<String>,
    /// The full name of the sender, as supplied by the sending bank.
    pub sender_name: Option<String>,
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransferBuilder
{
    sender_bank: Option<Option<String>>,
    sender_branch: Option<Option<String>>,
    sender_name: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

    struct Builder<'a> {
    out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer>,
    builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransferBuilder,
}

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransferBuilder::deser_default(),
        }))
    }
}

    impl MapBuilder for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransferBuilder {
    type Out = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "sender_bank" => Deserialize::begin(&mut self.sender_bank),
"sender_branch" => Deserialize::begin(&mut self.sender_branch),
"sender_name" => Deserialize::begin(&mut self.sender_name),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            sender_bank: Deserialize::default(),
sender_branch: Deserialize::default(),
sender_name: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        Some(Self::Out { sender_bank: self.sender_bank.take()?,
sender_branch: self.sender_branch.take()?,
sender_name: self.sender_name.take()?,
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

    impl ObjectDeser for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer {
    type Builder = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransferBuilder;
}

    impl FromValueOpt for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransferBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "sender_bank" => b.sender_bank = Some(FromValueOpt::from_value(v)?),
"sender_branch" => b.sender_branch = Some(FromValueOpt::from_value(v)?),
"sender_name" => b.sender_name = Some(FromValueOpt::from_value(v)?),

                _ => {}
            }
        }
        b.take_out()
    }
}
};
