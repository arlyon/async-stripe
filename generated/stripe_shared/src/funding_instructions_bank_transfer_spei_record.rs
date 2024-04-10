/// SPEI Records contain Mexico bank account details per the SPEI format.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferSpeiRecord {
    /// The three-digit bank code
    pub bank_code: String,
    /// The short banking institution name
    pub bank_name: String,
    /// The CLABE number
    pub clabe: String,
}
#[doc(hidden)]
pub struct FundingInstructionsBankTransferSpeiRecordBuilder {
    bank_code: Option<String>,
    bank_name: Option<String>,
    clabe: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FundingInstructionsBankTransferSpeiRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferSpeiRecord>,
        builder: FundingInstructionsBankTransferSpeiRecordBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferSpeiRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FundingInstructionsBankTransferSpeiRecordBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferSpeiRecordBuilder {
        type Out = FundingInstructionsBankTransferSpeiRecord;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_code" => Deserialize::begin(&mut self.bank_code),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "clabe" => Deserialize::begin(&mut self.clabe),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_code: Deserialize::default(),
                bank_name: Deserialize::default(),
                clabe: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                bank_code: self.bank_code.take()?,
                bank_name: self.bank_name.take()?,
                clabe: self.clabe.take()?,
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

    impl ObjectDeser for FundingInstructionsBankTransferSpeiRecord {
        type Builder = FundingInstructionsBankTransferSpeiRecordBuilder;
    }

    impl FromValueOpt for FundingInstructionsBankTransferSpeiRecord {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FundingInstructionsBankTransferSpeiRecordBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_code" => b.bank_code = Some(FromValueOpt::from_value(v)?),
                    "bank_name" => b.bank_name = Some(FromValueOpt::from_value(v)?),
                    "clabe" => b.clabe = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
