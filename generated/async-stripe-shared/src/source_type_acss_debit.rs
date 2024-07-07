#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeAcssDebit {
    pub bank_address_city: Option<String>,
    pub bank_address_line_1: Option<String>,
    pub bank_address_line_2: Option<String>,
    pub bank_address_postal_code: Option<String>,
    pub bank_name: Option<String>,
    pub category: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub routing_number: Option<String>,
}
#[doc(hidden)]
pub struct SourceTypeAcssDebitBuilder {
    bank_address_city: Option<Option<String>>,
    bank_address_line_1: Option<Option<String>>,
    bank_address_line_2: Option<Option<String>>,
    bank_address_postal_code: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    category: Option<Option<String>>,
    country: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    routing_number: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeAcssDebit>,
        builder: SourceTypeAcssDebitBuilder,
    }

    impl Visitor for Place<SourceTypeAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeAcssDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeAcssDebitBuilder {
        type Out = SourceTypeAcssDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_address_city" => Deserialize::begin(&mut self.bank_address_city),
                "bank_address_line_1" => Deserialize::begin(&mut self.bank_address_line_1),
                "bank_address_line_2" => Deserialize::begin(&mut self.bank_address_line_2),
                "bank_address_postal_code" => {
                    Deserialize::begin(&mut self.bank_address_postal_code)
                }
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "category" => Deserialize::begin(&mut self.category),
                "country" => Deserialize::begin(&mut self.country),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                "routing_number" => Deserialize::begin(&mut self.routing_number),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_address_city: Deserialize::default(),
                bank_address_line_1: Deserialize::default(),
                bank_address_line_2: Deserialize::default(),
                bank_address_postal_code: Deserialize::default(),
                bank_name: Deserialize::default(),
                category: Deserialize::default(),
                country: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                routing_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                bank_address_city: self.bank_address_city.take()?,
                bank_address_line_1: self.bank_address_line_1.take()?,
                bank_address_line_2: self.bank_address_line_2.take()?,
                bank_address_postal_code: self.bank_address_postal_code.take()?,
                bank_name: self.bank_name.take()?,
                category: self.category.take()?,
                country: self.country.take()?,
                fingerprint: self.fingerprint.take()?,
                last4: self.last4.take()?,
                routing_number: self.routing_number.take()?,
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

    impl ObjectDeser for SourceTypeAcssDebit {
        type Builder = SourceTypeAcssDebitBuilder;
    }

    impl FromValueOpt for SourceTypeAcssDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeAcssDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_address_city" => b.bank_address_city = Some(FromValueOpt::from_value(v)?),
                    "bank_address_line_1" => {
                        b.bank_address_line_1 = Some(FromValueOpt::from_value(v)?)
                    }
                    "bank_address_line_2" => {
                        b.bank_address_line_2 = Some(FromValueOpt::from_value(v)?)
                    }
                    "bank_address_postal_code" => {
                        b.bank_address_postal_code = Some(FromValueOpt::from_value(v)?)
                    }
                    "bank_name" => b.bank_name = Some(FromValueOpt::from_value(v)?),
                    "category" => b.category = Some(FromValueOpt::from_value(v)?),
                    "country" => b.country = Some(FromValueOpt::from_value(v)?),
                    "fingerprint" => b.fingerprint = Some(FromValueOpt::from_value(v)?),
                    "last4" => b.last4 = Some(FromValueOpt::from_value(v)?),
                    "routing_number" => b.routing_number = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
