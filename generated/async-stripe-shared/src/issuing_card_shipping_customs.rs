#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardShippingCustoms {
    /// A registration number used for customs in Europe.
    /// See [<https://www.gov.uk/eori>](https://www.gov.uk/eori) for the UK and [<https://ec.europa.eu/taxation_customs/business/customs-procedures-import-and-export/customs-procedures/economic-operators-registration-and-identification-number-eori_en>](https://ec.europa.eu/taxation_customs/business/customs-procedures-import-and-export/customs-procedures/economic-operators-registration-and-identification-number-eori_en) for the EU.
    pub eori_number: Option<String>,
}
#[doc(hidden)]
pub struct IssuingCardShippingCustomsBuilder {
    eori_number: Option<Option<String>>,
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

    impl Deserialize for IssuingCardShippingCustoms {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardShippingCustoms>,
        builder: IssuingCardShippingCustomsBuilder,
    }

    impl Visitor for Place<IssuingCardShippingCustoms> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardShippingCustomsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardShippingCustomsBuilder {
        type Out = IssuingCardShippingCustoms;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "eori_number" => Deserialize::begin(&mut self.eori_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { eori_number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(eori_number),) = (self.eori_number.take(),) else {
                return None;
            };
            Some(Self::Out { eori_number })
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

    impl ObjectDeser for IssuingCardShippingCustoms {
        type Builder = IssuingCardShippingCustomsBuilder;
    }

    impl FromValueOpt for IssuingCardShippingCustoms {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardShippingCustomsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "eori_number" => b.eori_number = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
