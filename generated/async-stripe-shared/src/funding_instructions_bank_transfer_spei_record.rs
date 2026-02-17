/// SPEI Records contain Mexico bank account details per the SPEI format.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferSpeiRecord {
    pub account_holder_address: stripe_shared::Address,
    /// The account holder name
    pub account_holder_name: String,
    pub bank_address: stripe_shared::Address,
    /// The three-digit bank code
    pub bank_code: String,
    /// The short banking institution name
    pub bank_name: String,
    /// The CLABE number
    pub clabe: String,
}
#[doc(hidden)]
pub struct FundingInstructionsBankTransferSpeiRecordBuilder {
    account_holder_address: Option<stripe_shared::Address>,
    account_holder_name: Option<String>,
    bank_address: Option<stripe_shared::Address>,
    bank_code: Option<String>,
    bank_name: Option<String>,
    clabe: Option<String>,
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
                "account_holder_address" => Deserialize::begin(&mut self.account_holder_address),
                "account_holder_name" => Deserialize::begin(&mut self.account_holder_name),
                "bank_address" => Deserialize::begin(&mut self.bank_address),
                "bank_code" => Deserialize::begin(&mut self.bank_code),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "clabe" => Deserialize::begin(&mut self.clabe),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder_address: Deserialize::default(),
                account_holder_name: Deserialize::default(),
                bank_address: Deserialize::default(),
                bank_code: Deserialize::default(),
                bank_name: Deserialize::default(),
                clabe: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_holder_address),
                Some(account_holder_name),
                Some(bank_address),
                Some(bank_code),
                Some(bank_name),
                Some(clabe),
            ) = (
                self.account_holder_address.take(),
                self.account_holder_name.take(),
                self.bank_address.take(),
                self.bank_code.take(),
                self.bank_name.take(),
                self.clabe.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_holder_address,
                account_holder_name,
                bank_address,
                bank_code,
                bank_name,
                clabe,
            })
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
                    "account_holder_address" => {
                        b.account_holder_address = FromValueOpt::from_value(v)
                    }
                    "account_holder_name" => b.account_holder_name = FromValueOpt::from_value(v),
                    "bank_address" => b.bank_address = FromValueOpt::from_value(v),
                    "bank_code" => b.bank_code = FromValueOpt::from_value(v),
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "clabe" => b.clabe = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
