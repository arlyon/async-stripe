/// Sort Code Records contain U.K. bank account details per the sort code format.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferSortCodeRecord {
    /// The name of the person or business that owns the bank account
    pub account_holder_name: String,
    /// The account number
    pub account_number: String,
    /// The six-digit sort code
    pub sort_code: String,
}
#[doc(hidden)]
pub struct FundingInstructionsBankTransferSortCodeRecordBuilder {
    account_holder_name: Option<String>,
    account_number: Option<String>,
    sort_code: Option<String>,
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

    impl Deserialize for FundingInstructionsBankTransferSortCodeRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferSortCodeRecord>,
        builder: FundingInstructionsBankTransferSortCodeRecordBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferSortCodeRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FundingInstructionsBankTransferSortCodeRecordBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferSortCodeRecordBuilder {
        type Out = FundingInstructionsBankTransferSortCodeRecord;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_name" => Deserialize::begin(&mut self.account_holder_name),
                "account_number" => Deserialize::begin(&mut self.account_number),
                "sort_code" => Deserialize::begin(&mut self.sort_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder_name: Deserialize::default(),
                account_number: Deserialize::default(),
                sort_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(account_holder_name), Some(account_number), Some(sort_code)) = (
                self.account_holder_name.take(),
                self.account_number.take(),
                self.sort_code.take(),
            ) else {
                return None;
            };
            Some(Self::Out { account_holder_name, account_number, sort_code })
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

    impl ObjectDeser for FundingInstructionsBankTransferSortCodeRecord {
        type Builder = FundingInstructionsBankTransferSortCodeRecordBuilder;
    }

    impl FromValueOpt for FundingInstructionsBankTransferSortCodeRecord {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FundingInstructionsBankTransferSortCodeRecordBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_name" => b.account_holder_name = FromValueOpt::from_value(v),
                    "account_number" => b.account_number = FromValueOpt::from_value(v),
                    "sort_code" => b.sort_code = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
