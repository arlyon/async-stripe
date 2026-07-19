/// Iban Records contain E.U. bank account details per the SEPA format.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferIbanRecord {
    pub account_holder_address: stripe_shared::Address,
    /// The name of the person or business that owns the bank account
    pub account_holder_name: String,
    pub bank_address: stripe_shared::Address,
    /// The BIC/SWIFT code of the account.
    pub bic: String,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// The IBAN of the account.
    pub iban: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FundingInstructionsBankTransferIbanRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FundingInstructionsBankTransferIbanRecord").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct FundingInstructionsBankTransferIbanRecordBuilder {
    account_holder_address: Option<stripe_shared::Address>,
    account_holder_name: Option<String>,
    bank_address: Option<stripe_shared::Address>,
    bic: Option<String>,
    country: Option<String>,
    iban: Option<String>,
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

    impl Deserialize for FundingInstructionsBankTransferIbanRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferIbanRecord>,
        builder: FundingInstructionsBankTransferIbanRecordBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferIbanRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FundingInstructionsBankTransferIbanRecordBuilder {
                    account_holder_address: Deserialize::default(),
                    account_holder_name: Deserialize::default(),
                    bank_address: Deserialize::default(),
                    bic: Deserialize::default(),
                    country: Deserialize::default(),
                    iban: Deserialize::default(),
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
                "bank_address" => Deserialize::begin(&mut self.builder.bank_address),
                "bic" => Deserialize::begin(&mut self.builder.bic),
                "country" => Deserialize::begin(&mut self.builder.country),
                "iban" => Deserialize::begin(&mut self.builder.iban),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account_holder_address),
                Some(account_holder_name),
                Some(bank_address),
                Some(bic),
                Some(country),
                Some(iban),
            ) = (
                self.builder.account_holder_address.take(),
                self.builder.account_holder_name.take(),
                self.builder.bank_address.take(),
                self.builder.bic.take(),
                self.builder.country.take(),
                self.builder.iban.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(FundingInstructionsBankTransferIbanRecord {
                account_holder_address,
                account_holder_name,
                bank_address,
                bic,
                country,
                iban,
            });
            Ok(())
        }
    }
};
