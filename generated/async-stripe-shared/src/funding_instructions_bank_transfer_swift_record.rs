/// SWIFT Records contain U.S. bank account details per the SWIFT format.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferSwiftRecord {
    /// The account number
    pub account_number: String,
    /// The bank name
    pub bank_name: String,
    /// The SWIFT code
    pub swift_code: String,
}
#[doc(hidden)]
pub struct FundingInstructionsBankTransferSwiftRecordBuilder {
    account_number: Option<String>,
    bank_name: Option<String>,
    swift_code: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FundingInstructionsBankTransferSwiftRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferSwiftRecord>,
        builder: FundingInstructionsBankTransferSwiftRecordBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferSwiftRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FundingInstructionsBankTransferSwiftRecordBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferSwiftRecordBuilder {
        type Out = FundingInstructionsBankTransferSwiftRecord;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_number" => Deserialize::begin(&mut self.account_number),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "swift_code" => Deserialize::begin(&mut self.swift_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_number: Deserialize::default(),
                bank_name: Deserialize::default(),
                swift_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                account_number: self.account_number.take()?,
                bank_name: self.bank_name.take()?,
                swift_code: self.swift_code.take()?,
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

    impl ObjectDeser for FundingInstructionsBankTransferSwiftRecord {
        type Builder = FundingInstructionsBankTransferSwiftRecordBuilder;
    }

    impl FromValueOpt for FundingInstructionsBankTransferSwiftRecord {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FundingInstructionsBankTransferSwiftRecordBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_number" => b.account_number = Some(FromValueOpt::from_value(v)?),
                    "bank_name" => b.bank_name = Some(FromValueOpt::from_value(v)?),
                    "swift_code" => b.swift_code = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
