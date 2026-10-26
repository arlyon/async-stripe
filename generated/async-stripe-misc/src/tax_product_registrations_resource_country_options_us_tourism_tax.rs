#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsUsTourismTax {
    /// A [jurisdiction code](https://docs.stripe.com/tax/registering?type=tourism_tax#registration-types) representing the local jurisdiction.
    pub jurisdiction: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsUsTourismTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductRegistrationsResourceCountryOptionsUsTourismTax")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsTourismTaxBuilder {
    jurisdiction: Option<String>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsUsTourismTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsUsTourismTax>,
        builder: TaxProductRegistrationsResourceCountryOptionsUsTourismTaxBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsUsTourismTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TaxProductRegistrationsResourceCountryOptionsUsTourismTaxBuilder::deser_default(
                    ),
            }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsUsTourismTaxBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsUsTourismTax;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "jurisdiction" => Deserialize::begin(&mut self.jurisdiction),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { jurisdiction: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(jurisdiction),) = (self.jurisdiction.take(),) else {
                return None;
            };
            Some(Self::Out { jurisdiction })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsUsTourismTax {
        type Builder = TaxProductRegistrationsResourceCountryOptionsUsTourismTaxBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsUsTourismTax {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TaxProductRegistrationsResourceCountryOptionsUsTourismTaxBuilder::deser_default();
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
