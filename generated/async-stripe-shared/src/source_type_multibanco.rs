#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeMultibanco {
    pub entity: Option<String>,
    pub reference: Option<String>,
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
pub struct SourceTypeMultibancoBuilder {
    entity: Option<Option<String>>,
    reference: Option<Option<String>>,
    refund_account_holder_address_city: Option<Option<String>>,
    refund_account_holder_address_country: Option<Option<String>>,
    refund_account_holder_address_line1: Option<Option<String>>,
    refund_account_holder_address_line2: Option<Option<String>>,
    refund_account_holder_address_postal_code: Option<Option<String>>,
    refund_account_holder_address_state: Option<Option<String>>,
    refund_account_holder_name: Option<Option<String>>,
    refund_iban: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeMultibanco {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeMultibanco>,
        builder: SourceTypeMultibancoBuilder,
    }

    impl Visitor for Place<SourceTypeMultibanco> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeMultibancoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeMultibancoBuilder {
        type Out = SourceTypeMultibanco;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "entity" => Deserialize::begin(&mut self.entity),
                "reference" => Deserialize::begin(&mut self.reference),
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
                entity: Deserialize::default(),
                reference: Deserialize::default(),
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
            Some(Self::Out {
                entity: self.entity.take()?,
                reference: self.reference.take()?,
                refund_account_holder_address_city: self
                    .refund_account_holder_address_city
                    .take()?,
                refund_account_holder_address_country: self
                    .refund_account_holder_address_country
                    .take()?,
                refund_account_holder_address_line1: self
                    .refund_account_holder_address_line1
                    .take()?,
                refund_account_holder_address_line2: self
                    .refund_account_holder_address_line2
                    .take()?,
                refund_account_holder_address_postal_code: self
                    .refund_account_holder_address_postal_code
                    .take()?,
                refund_account_holder_address_state: self
                    .refund_account_holder_address_state
                    .take()?,
                refund_account_holder_name: self.refund_account_holder_name.take()?,
                refund_iban: self.refund_iban.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SourceTypeMultibanco {
        type Builder = SourceTypeMultibancoBuilder;
    }

    impl FromValueOpt for SourceTypeMultibanco {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeMultibancoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "entity" => b.entity = Some(FromValueOpt::from_value(v)?),
                    "reference" => b.reference = Some(FromValueOpt::from_value(v)?),
                    "refund_account_holder_address_city" => {
                        b.refund_account_holder_address_city = Some(FromValueOpt::from_value(v)?)
                    }
                    "refund_account_holder_address_country" => {
                        b.refund_account_holder_address_country = Some(FromValueOpt::from_value(v)?)
                    }
                    "refund_account_holder_address_line1" => {
                        b.refund_account_holder_address_line1 = Some(FromValueOpt::from_value(v)?)
                    }
                    "refund_account_holder_address_line2" => {
                        b.refund_account_holder_address_line2 = Some(FromValueOpt::from_value(v)?)
                    }
                    "refund_account_holder_address_postal_code" => {
                        b.refund_account_holder_address_postal_code =
                            Some(FromValueOpt::from_value(v)?)
                    }
                    "refund_account_holder_address_state" => {
                        b.refund_account_holder_address_state = Some(FromValueOpt::from_value(v)?)
                    }
                    "refund_account_holder_name" => {
                        b.refund_account_holder_name = Some(FromValueOpt::from_value(v)?)
                    }
                    "refund_iban" => b.refund_iban = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
