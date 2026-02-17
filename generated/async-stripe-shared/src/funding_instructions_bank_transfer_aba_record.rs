/// ABA Records contain U.S. bank account details per the ABA format.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferAbaRecord {
    pub account_holder_address: stripe_shared::Address,
    /// The account holder name
    pub account_holder_name: String,
    /// The ABA account number
    pub account_number: String,
    /// The account type
    pub account_type: String,
    pub bank_address: stripe_shared::Address,
    /// The bank name
    pub bank_name: String,
    /// The ABA routing number
    pub routing_number: String,
}
#[doc(hidden)]
pub struct FundingInstructionsBankTransferAbaRecordBuilder {
    account_holder_address: Option<stripe_shared::Address>,
    account_holder_name: Option<String>,
    account_number: Option<String>,
    account_type: Option<String>,
    bank_address: Option<stripe_shared::Address>,
    bank_name: Option<String>,
    routing_number: Option<String>,
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

    impl Deserialize for FundingInstructionsBankTransferAbaRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferAbaRecord>,
        builder: FundingInstructionsBankTransferAbaRecordBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferAbaRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FundingInstructionsBankTransferAbaRecordBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferAbaRecordBuilder {
        type Out = FundingInstructionsBankTransferAbaRecord;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_address" => Deserialize::begin(&mut self.account_holder_address),
                "account_holder_name" => Deserialize::begin(&mut self.account_holder_name),
                "account_number" => Deserialize::begin(&mut self.account_number),
                "account_type" => Deserialize::begin(&mut self.account_type),
                "bank_address" => Deserialize::begin(&mut self.bank_address),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "routing_number" => Deserialize::begin(&mut self.routing_number),
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
                bank_name: Deserialize::default(),
                routing_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_holder_address),
                Some(account_holder_name),
                Some(account_number),
                Some(account_type),
                Some(bank_address),
                Some(bank_name),
                Some(routing_number),
            ) = (
                self.account_holder_address.take(),
                self.account_holder_name.take(),
                self.account_number.take(),
                self.account_type.take(),
                self.bank_address.take(),
                self.bank_name.take(),
                self.routing_number.take(),
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
                bank_name,
                routing_number,
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

    impl ObjectDeser for FundingInstructionsBankTransferAbaRecord {
        type Builder = FundingInstructionsBankTransferAbaRecordBuilder;
    }

    impl FromValueOpt for FundingInstructionsBankTransferAbaRecord {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FundingInstructionsBankTransferAbaRecordBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_address" => {
                        b.account_holder_address = FromValueOpt::from_value(v)
                    }
                    "account_holder_name" => b.account_holder_name = FromValueOpt::from_value(v),
                    "account_number" => b.account_number = FromValueOpt::from_value(v),
                    "account_type" => b.account_type = FromValueOpt::from_value(v),
                    "bank_address" => b.bank_address = FromValueOpt::from_value(v),
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "routing_number" => b.routing_number = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
