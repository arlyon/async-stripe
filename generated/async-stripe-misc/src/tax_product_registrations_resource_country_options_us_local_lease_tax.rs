#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    pub jurisdiction: String,
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTaxBuilder {
    jurisdiction: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>,
        builder: TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTaxBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTaxBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTaxBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax;
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
            let (Some(jurisdiction),) = (self.jurisdiction.take(),) else {
                return None;
            };
            Some(Self::Out { jurisdiction })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax {
        type Builder = TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTaxBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTaxBuilder::deser_default(
                );
            for (k, v) in obj {
                match k.as_str() {
                    "jurisdiction" => b.jurisdiction = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
