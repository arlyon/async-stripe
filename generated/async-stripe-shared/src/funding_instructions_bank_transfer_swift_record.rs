/// SWIFT Records contain U.S. bank account details per the SWIFT format.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferSwiftRecord {
    pub account_holder_address: stripe_shared::Address,
    /// The account holder name
    pub account_holder_name: String,
    /// The account number
    pub account_number: String,
    /// The account type
    pub account_type: String,
    pub bank_address: stripe_shared::Address,
    /// The bank name
    pub bank_name: String,
    /// The SWIFT code
    pub swift_code: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FundingInstructionsBankTransferSwiftRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FundingInstructionsBankTransferSwiftRecord").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct FundingInstructionsBankTransferSwiftRecordBuilder {
    account_holder_address: Option<stripe_shared::Address>,
    account_holder_name: Option<String>,
    account_number: Option<String>,
    account_type: Option<String>,
    bank_address: Option<stripe_shared::Address>,
    bank_name: Option<String>,
    swift_code: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: FundingInstructionsBankTransferSwiftRecordBuilder {
                    account_holder_address: Deserialize::default(),
                    account_holder_name: Deserialize::default(),
                    account_number: Deserialize::default(),
                    account_type: Deserialize::default(),
                    bank_address: Deserialize::default(),
                    bank_name: Deserialize::default(),
                    swift_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_address" => {
                    Deserialize::begin(&mut self.builder.account_holder_address)
                }
                "account_holder_name" => Deserialize::begin(&mut self.builder.account_holder_name),
                "account_number" => Deserialize::begin(&mut self.builder.account_number),
                "account_type" => Deserialize::begin(&mut self.builder.account_type),
                "bank_address" => Deserialize::begin(&mut self.builder.bank_address),
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "swift_code" => Deserialize::begin(&mut self.builder.swift_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account_holder_address),
                Some(account_holder_name),
                Some(account_number),
                Some(account_type),
                Some(bank_address),
                Some(bank_name),
                Some(swift_code),
            ) = (
                self.builder.account_holder_address.take(),
                self.builder.account_holder_name.take(),
                self.builder.account_number.take(),
                self.builder.account_type.take(),
                self.builder.bank_address.take(),
                self.builder.bank_name.take(),
                self.builder.swift_code.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(FundingInstructionsBankTransferSwiftRecord {
                account_holder_address,
                account_holder_name,
                account_number,
                account_type,
                bank_address,
                bank_name,
                swift_code,
            });
            Ok(())
        }
    }
};
