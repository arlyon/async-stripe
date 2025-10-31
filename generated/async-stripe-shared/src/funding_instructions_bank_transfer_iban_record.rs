/// Iban Records contain E.U. bank account details per the SEPA format.
#[derive(Clone, Debug)]
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
                builder: FundingInstructionsBankTransferIbanRecordBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferIbanRecordBuilder {
        type Out = FundingInstructionsBankTransferIbanRecord;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_address" => Deserialize::begin(&mut self.account_holder_address),
                "account_holder_name" => Deserialize::begin(&mut self.account_holder_name),
                "bank_address" => Deserialize::begin(&mut self.bank_address),
                "bic" => Deserialize::begin(&mut self.bic),
                "country" => Deserialize::begin(&mut self.country),
                "iban" => Deserialize::begin(&mut self.iban),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder_address: Deserialize::default(),
                account_holder_name: Deserialize::default(),
                bank_address: Deserialize::default(),
                bic: Deserialize::default(),
                country: Deserialize::default(),
                iban: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_holder_address),
                Some(account_holder_name),
                Some(bank_address),
                Some(bic),
                Some(country),
                Some(iban),
            ) = (
                self.account_holder_address.take(),
                self.account_holder_name.take(),
                self.bank_address.take(),
                self.bic.take(),
                self.country.take(),
                self.iban.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_holder_address,
                account_holder_name,
                bank_address,
                bic,
                country,
                iban,
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

    impl ObjectDeser for FundingInstructionsBankTransferIbanRecord {
        type Builder = FundingInstructionsBankTransferIbanRecordBuilder;
    }

    impl FromValueOpt for FundingInstructionsBankTransferIbanRecord {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FundingInstructionsBankTransferIbanRecordBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_address" => {
                        b.account_holder_address = FromValueOpt::from_value(v)
                    }
                    "account_holder_name" => b.account_holder_name = FromValueOpt::from_value(v),
                    "bank_address" => b.bank_address = FromValueOpt::from_value(v),
                    "bic" => b.bic = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "iban" => b.iban = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
