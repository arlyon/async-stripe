/// Sort Code Records contain U.K. bank account details per the sort code format.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferSortCodeRecord {
    pub account_holder_address: stripe_shared::Address,
    /// The name of the person or business that owns the bank account
    pub account_holder_name: String,
    /// The account number
    pub account_number: String,
    pub bank_address: stripe_shared::Address,
    /// The six-digit sort code
    pub sort_code: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FundingInstructionsBankTransferSortCodeRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FundingInstructionsBankTransferSortCodeRecord").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct FundingInstructionsBankTransferSortCodeRecordBuilder {
    account_holder_address: Option<stripe_shared::Address>,
    account_holder_name: Option<String>,
    account_number: Option<String>,
    bank_address: Option<stripe_shared::Address>,
    sort_code: Option<String>,
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
                builder: FundingInstructionsBankTransferSortCodeRecordBuilder {
                    account_holder_address: Deserialize::default(),
                    account_holder_name: Deserialize::default(),
                    account_number: Deserialize::default(),
                    bank_address: Deserialize::default(),
                    sort_code: Deserialize::default(),
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
                "bank_address" => Deserialize::begin(&mut self.builder.bank_address),
                "sort_code" => Deserialize::begin(&mut self.builder.sort_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account_holder_address),
                Some(account_holder_name),
                Some(account_number),
                Some(bank_address),
                Some(sort_code),
            ) = (
                self.builder.account_holder_address.take(),
                self.builder.account_holder_name.take(),
                self.builder.account_number.take(),
                self.builder.bank_address.take(),
                self.builder.sort_code.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(FundingInstructionsBankTransferSortCodeRecord {
                account_holder_address,
                account_holder_name,
                account_number,
                bank_address,
                sort_code,
            });
            Ok(())
        }
    }
};
