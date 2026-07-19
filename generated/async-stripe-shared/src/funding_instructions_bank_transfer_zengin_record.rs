/// Zengin Records contain Japan bank account details per the Zengin format.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FundingInstructionsBankTransferZenginRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FundingInstructionsBankTransferZenginRecord").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: FundingInstructionsBankTransferZenginRecordBuilder {
                    account_holder_address: Deserialize::default(),
                    account_holder_name: Deserialize::default(),
                    account_number: Deserialize::default(),
                    account_type: Deserialize::default(),
                    bank_address: Deserialize::default(),
                    bank_code: Deserialize::default(),
                    bank_name: Deserialize::default(),
                    branch_code: Deserialize::default(),
                    branch_name: Deserialize::default(),
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
                "bank_code" => Deserialize::begin(&mut self.builder.bank_code),
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "branch_code" => Deserialize::begin(&mut self.builder.branch_code),
                "branch_name" => Deserialize::begin(&mut self.builder.branch_name),
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
                Some(bank_code),
                Some(bank_name),
                Some(branch_code),
                Some(branch_name),
            ) = (
                self.builder.account_holder_address.take(),
                self.builder.account_holder_name.take(),
                self.builder.account_number.take(),
                self.builder.account_type.take(),
                self.builder.bank_address.take(),
                self.builder.bank_code.take(),
                self.builder.bank_name.take(),
                self.builder.branch_code.take(),
                self.builder.branch_name.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(FundingInstructionsBankTransferZenginRecord {
                account_holder_address,
                account_holder_name,
                account_number,
                account_type,
                bank_address,
                bank_code,
                bank_name,
                branch_code,
                branch_name,
            });
            Ok(())
        }
    }
};
