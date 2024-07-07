#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    pub jurisdiction: String,
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTaxBuilder {
    jurisdiction: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>,
        builder: TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTaxBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTaxBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTaxBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "jurisdiction" => Deserialize::begin(&mut self.jurisdiction),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { jurisdiction: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { jurisdiction: self.jurisdiction.take()? })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax {
        type Builder = TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTaxBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTaxBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "jurisdiction" => b.jurisdiction = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
