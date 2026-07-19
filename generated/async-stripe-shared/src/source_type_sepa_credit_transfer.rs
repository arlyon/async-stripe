#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeSepaCreditTransfer {
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub iban: Option<String>,
    pub refund_account_holder_address_city: Option<String>,
    pub refund_account_holder_address_country: Option<String>,
    pub refund_account_holder_address_line1: Option<String>,
    pub refund_account_holder_address_line2: Option<String>,
    pub refund_account_holder_address_postal_code: Option<String>,
    pub refund_account_holder_address_state: Option<String>,
    pub refund_account_holder_name: Option<String>,
    pub refund_iban: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeSepaCreditTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeSepaCreditTransfer").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTypeSepaCreditTransferBuilder {
    bank_name: Option<Option<String>>,
    bic: Option<Option<String>>,
    iban: Option<Option<String>>,
    refund_account_holder_address_city: Option<Option<String>>,
    refund_account_holder_address_country: Option<Option<String>>,
    refund_account_holder_address_line1: Option<Option<String>>,
    refund_account_holder_address_line2: Option<Option<String>>,
    refund_account_holder_address_postal_code: Option<Option<String>>,
    refund_account_holder_address_state: Option<Option<String>>,
    refund_account_holder_name: Option<Option<String>>,
    refund_iban: Option<Option<String>>,
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

    impl Deserialize for SourceTypeSepaCreditTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeSepaCreditTransfer>,
        builder: SourceTypeSepaCreditTransferBuilder,
    }

    impl Visitor for Place<SourceTypeSepaCreditTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeSepaCreditTransferBuilder {
                    bank_name: Deserialize::default(),
                    bic: Deserialize::default(),
                    iban: Deserialize::default(),
                    refund_account_holder_address_city: Deserialize::default(),
                    refund_account_holder_address_country: Deserialize::default(),
                    refund_account_holder_address_line1: Deserialize::default(),
                    refund_account_holder_address_line2: Deserialize::default(),
                    refund_account_holder_address_postal_code: Deserialize::default(),
                    refund_account_holder_address_state: Deserialize::default(),
                    refund_account_holder_name: Deserialize::default(),
                    refund_iban: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "bic" => Deserialize::begin(&mut self.builder.bic),
                "iban" => Deserialize::begin(&mut self.builder.iban),
                "refund_account_holder_address_city" => {
                    Deserialize::begin(&mut self.builder.refund_account_holder_address_city)
                }
                "refund_account_holder_address_country" => {
                    Deserialize::begin(&mut self.builder.refund_account_holder_address_country)
                }
                "refund_account_holder_address_line1" => {
                    Deserialize::begin(&mut self.builder.refund_account_holder_address_line1)
                }
                "refund_account_holder_address_line2" => {
                    Deserialize::begin(&mut self.builder.refund_account_holder_address_line2)
                }
                "refund_account_holder_address_postal_code" => {
                    Deserialize::begin(&mut self.builder.refund_account_holder_address_postal_code)
                }
                "refund_account_holder_address_state" => {
                    Deserialize::begin(&mut self.builder.refund_account_holder_address_state)
                }
                "refund_account_holder_name" => {
                    Deserialize::begin(&mut self.builder.refund_account_holder_name)
                }
                "refund_iban" => Deserialize::begin(&mut self.builder.refund_iban),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(bank_name),
                Some(bic),
                Some(iban),
                Some(refund_account_holder_address_city),
                Some(refund_account_holder_address_country),
                Some(refund_account_holder_address_line1),
                Some(refund_account_holder_address_line2),
                Some(refund_account_holder_address_postal_code),
                Some(refund_account_holder_address_state),
                Some(refund_account_holder_name),
                Some(refund_iban),
            ) = (
                self.builder.bank_name.take(),
                self.builder.bic.take(),
                self.builder.iban.take(),
                self.builder.refund_account_holder_address_city.take(),
                self.builder.refund_account_holder_address_country.take(),
                self.builder.refund_account_holder_address_line1.take(),
                self.builder.refund_account_holder_address_line2.take(),
                self.builder.refund_account_holder_address_postal_code.take(),
                self.builder.refund_account_holder_address_state.take(),
                self.builder.refund_account_holder_name.take(),
                self.builder.refund_iban.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SourceTypeSepaCreditTransfer {
                bank_name,
                bic,
                iban,
                refund_account_holder_address_city,
                refund_account_holder_address_country,
                refund_account_holder_address_line1,
                refund_account_holder_address_line2,
                refund_account_holder_address_postal_code,
                refund_account_holder_address_state,
                refund_account_holder_name,
                refund_iban,
            });
            Ok(())
        }
    }
};
