#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CountrySpecVerificationFields {
    pub company: stripe_connect::CountrySpecVerificationFieldDetails,
    pub individual: stripe_connect::CountrySpecVerificationFieldDetails,
}
#[doc(hidden)]
pub struct CountrySpecVerificationFieldsBuilder {
    company: Option<stripe_connect::CountrySpecVerificationFieldDetails>,
    individual: Option<stripe_connect::CountrySpecVerificationFieldDetails>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CountrySpecVerificationFields {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CountrySpecVerificationFields>,
        builder: CountrySpecVerificationFieldsBuilder,
    }

    impl Visitor for Place<CountrySpecVerificationFields> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CountrySpecVerificationFieldsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CountrySpecVerificationFieldsBuilder {
        type Out = CountrySpecVerificationFields;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "company" => Deserialize::begin(&mut self.company),
                "individual" => Deserialize::begin(&mut self.individual),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { company: Deserialize::default(), individual: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(company), Some(individual)) = (self.company.take(), self.individual.take())
            else {
                return None;
            };
            Some(Self::Out { company, individual })
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

    impl ObjectDeser for CountrySpecVerificationFields {
        type Builder = CountrySpecVerificationFieldsBuilder;
    }

    impl FromValueOpt for CountrySpecVerificationFields {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CountrySpecVerificationFieldsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "company" => b.company = FromValueOpt::from_value(v),
                    "individual" => b.individual = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
