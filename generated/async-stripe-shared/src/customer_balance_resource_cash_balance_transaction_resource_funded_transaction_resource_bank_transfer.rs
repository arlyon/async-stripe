#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {
pub eu_bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer>,
pub gb_bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer>,
pub jp_bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer>,
    /// The user-supplied reference field on the bank transfer.
pub reference: Option<String>,
        /// The funding method type used to fund the customer balance.
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
pub type_: stripe_shared::CreateFundingInstructionsCustomerBankTransferType,
pub us_bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder {
    eu_bank_transfer: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer>>,
gb_bank_transfer: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer>>,
jp_bank_transfer: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer>>,
reference: Option<Option<String>>,
type_: Option<stripe_shared::CreateFundingInstructionsCustomerBankTransferType>,
us_bank_transfer: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer>>,

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

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

    struct Builder<'a> {
    out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer>,
    builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder,
}

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder::deser_default(),
        }))
    }
}

    impl MapBuilder for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder {
    type Out = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "eu_bank_transfer" => Deserialize::begin(&mut self.eu_bank_transfer),
            "gb_bank_transfer" => Deserialize::begin(&mut self.gb_bank_transfer),
            "jp_bank_transfer" => Deserialize::begin(&mut self.jp_bank_transfer),
            "reference" => Deserialize::begin(&mut self.reference),
            "type" => Deserialize::begin(&mut self.type_),
            "us_bank_transfer" => Deserialize::begin(&mut self.us_bank_transfer),
            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self { eu_bank_transfer: Some(None),
gb_bank_transfer: Some(None),
jp_bank_transfer: Some(None),
reference: Some(None),
type_: None,
us_bank_transfer: Some(None),
 }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(eu_bank_transfer),
Some(gb_bank_transfer),
Some(jp_bank_transfer),
Some(reference),
Some(type_),
Some(us_bank_transfer),
) = (self.eu_bank_transfer.take(),
self.gb_bank_transfer.take(),
self.jp_bank_transfer.take(),
self.reference.take(),
self.type_.take(),
self.us_bank_transfer.take(),
) else {
            return None;
        };
        Some(Self::Out { eu_bank_transfer,gb_bank_transfer,jp_bank_transfer,reference,type_,us_bank_transfer })
    }
}

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {
    type Builder = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder;
}

    impl FromValueOpt for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
            "eu_bank_transfer" => b.eu_bank_transfer = FromValueOpt::from_value(v),
            "gb_bank_transfer" => b.gb_bank_transfer = FromValueOpt::from_value(v),
            "jp_bank_transfer" => b.jp_bank_transfer = FromValueOpt::from_value(v),
            "reference" => b.reference = FromValueOpt::from_value(v),
            "type" => b.type_ = FromValueOpt::from_value(v),
            "us_bank_transfer" => b.us_bank_transfer = FromValueOpt::from_value(v),
                _ => {}
            }
        }
        b.take_out()
    }
}
};
/// The funding method type used to fund the customer balance.
/// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
pub type CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType =
    stripe_shared::CreateFundingInstructionsCustomerBankTransferType;
