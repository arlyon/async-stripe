/// Zengin Records contain Japan bank account details per the Zengin format.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferZenginRecord {
    pub account_holder_address: stripe_shared::Address,
    /// The account holder name
    pub account_holder_name: Option<String>,
    /// The account number
    pub account_number: Option<String>,
    /// The bank account type. In Japan, this can only be `futsu` or `toza`.
    pub account_type: Option<String>,
    pub bank_address: stripe_shared::Address,
    /// The bank code of the account
    pub bank_code: Option<String>,
    /// The bank name of the account
    pub bank_name: Option<String>,
    /// The branch code of the account
    pub branch_code: Option<String>,
    /// The branch name of the account
    pub branch_name: Option<String>,
}
#[doc(hidden)]
pub struct FundingInstructionsBankTransferZenginRecordBuilder {
    account_holder_address: Option<stripe_shared::Address>,
    account_holder_name: Option<Option<String>>,
    account_number: Option<Option<String>>,
    account_type: Option<Option<String>>,
    bank_address: Option<stripe_shared::Address>,
    bank_code: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    branch_code: Option<Option<String>>,
    branch_name: Option<Option<String>>,
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

    impl Deserialize for FundingInstructionsBankTransferZenginRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferZenginRecord>,
        builder: FundingInstructionsBankTransferZenginRecordBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferZenginRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FundingInstructionsBankTransferZenginRecordBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferZenginRecordBuilder {
        type Out = FundingInstructionsBankTransferZenginRecord;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_address" => Deserialize::begin(&mut self.account_holder_address),
                "account_holder_name" => Deserialize::begin(&mut self.account_holder_name),
                "account_number" => Deserialize::begin(&mut self.account_number),
                "account_type" => Deserialize::begin(&mut self.account_type),
                "bank_address" => Deserialize::begin(&mut self.bank_address),
                "bank_code" => Deserialize::begin(&mut self.bank_code),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "branch_code" => Deserialize::begin(&mut self.branch_code),
                "branch_name" => Deserialize::begin(&mut self.branch_name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder_address: Deserialize::default(),
                account_holder_name: Deserialize::default(),
                account_number: Deserialize::default(),
                account_type: Deserialize::default(),
                bank_address: Deserialize::default(),
                bank_code: Deserialize::default(),
                bank_name: Deserialize::default(),
                branch_code: Deserialize::default(),
                branch_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_holder_address),
                Some(account_holder_name),
                Some(account_number),
                Some(account_type),
                Some(bank_address),
                Some(bank_code),
                Some(bank_name),
                Some(branch_code),
                Some(branch_name),
            ) = (
                self.account_holder_address.take(),
                self.account_holder_name.take(),
                self.account_number.take(),
                self.account_type.take(),
                self.bank_address.take(),
                self.bank_code.take(),
                self.bank_name.take(),
                self.branch_code.take(),
                self.branch_name.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_holder_address,
                account_holder_name,
                account_number,
                account_type,
                bank_address,
                bank_code,
                bank_name,
                branch_code,
                branch_name,
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

    impl ObjectDeser for FundingInstructionsBankTransferZenginRecord {
        type Builder = FundingInstructionsBankTransferZenginRecordBuilder;
    }

    impl FromValueOpt for FundingInstructionsBankTransferZenginRecord {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FundingInstructionsBankTransferZenginRecordBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_address" => {
                        b.account_holder_address = FromValueOpt::from_value(v)
                    }
                    "account_holder_name" => b.account_holder_name = FromValueOpt::from_value(v),
                    "account_number" => b.account_number = FromValueOpt::from_value(v),
                    "account_type" => b.account_type = FromValueOpt::from_value(v),
                    "bank_address" => b.bank_address = FromValueOpt::from_value(v),
                    "bank_code" => b.bank_code = FromValueOpt::from_value(v),
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "branch_code" => b.branch_code = FromValueOpt::from_value(v),
                    "branch_name" => b.branch_name = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
