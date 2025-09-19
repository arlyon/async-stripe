#[derive(Clone, Debug)]
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
                builder: SourceTypeSepaCreditTransferBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeSepaCreditTransferBuilder {
        type Out = SourceTypeSepaCreditTransfer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "bic" => Deserialize::begin(&mut self.bic),
                "iban" => Deserialize::begin(&mut self.iban),
                "refund_account_holder_address_city" => {
                    Deserialize::begin(&mut self.refund_account_holder_address_city)
                }
                "refund_account_holder_address_country" => {
                    Deserialize::begin(&mut self.refund_account_holder_address_country)
                }
                "refund_account_holder_address_line1" => {
                    Deserialize::begin(&mut self.refund_account_holder_address_line1)
                }
                "refund_account_holder_address_line2" => {
                    Deserialize::begin(&mut self.refund_account_holder_address_line2)
                }
                "refund_account_holder_address_postal_code" => {
                    Deserialize::begin(&mut self.refund_account_holder_address_postal_code)
                }
                "refund_account_holder_address_state" => {
                    Deserialize::begin(&mut self.refund_account_holder_address_state)
                }
                "refund_account_holder_name" => {
                    Deserialize::begin(&mut self.refund_account_holder_name)
                }
                "refund_iban" => Deserialize::begin(&mut self.refund_iban),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.bank_name.take(),
                self.bic.take(),
                self.iban.take(),
                self.refund_account_holder_address_city.take(),
                self.refund_account_holder_address_country.take(),
                self.refund_account_holder_address_line1.take(),
                self.refund_account_holder_address_line2.take(),
                self.refund_account_holder_address_postal_code.take(),
                self.refund_account_holder_address_state.take(),
                self.refund_account_holder_name.take(),
                self.refund_iban.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
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

    impl ObjectDeser for SourceTypeSepaCreditTransfer {
        type Builder = SourceTypeSepaCreditTransferBuilder;
    }

    impl FromValueOpt for SourceTypeSepaCreditTransfer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeSepaCreditTransferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "bic" => b.bic = FromValueOpt::from_value(v),
                    "iban" => b.iban = FromValueOpt::from_value(v),
                    "refund_account_holder_address_city" => {
                        b.refund_account_holder_address_city = FromValueOpt::from_value(v)
                    }
                    "refund_account_holder_address_country" => {
                        b.refund_account_holder_address_country = FromValueOpt::from_value(v)
                    }
                    "refund_account_holder_address_line1" => {
                        b.refund_account_holder_address_line1 = FromValueOpt::from_value(v)
                    }
                    "refund_account_holder_address_line2" => {
                        b.refund_account_holder_address_line2 = FromValueOpt::from_value(v)
                    }
                    "refund_account_holder_address_postal_code" => {
                        b.refund_account_holder_address_postal_code = FromValueOpt::from_value(v)
                    }
                    "refund_account_holder_address_state" => {
                        b.refund_account_holder_address_state = FromValueOpt::from_value(v)
                    }
                    "refund_account_holder_name" => {
                        b.refund_account_holder_name = FromValueOpt::from_value(v)
                    }
                    "refund_iban" => b.refund_iban = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
