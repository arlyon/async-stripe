/// Tax details contains information about the data that was used to calculate taxes.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductTaxDetailsResourceTaxDetails {
    /// The ID of a tax location with type `performance`, representing where the performance takes place.
    pub performance_location: Option<String>,
    /// A [tax code](https://docs.stripe.com/tax/tax-categories) ID.
    pub tax_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductTaxDetailsResourceTaxDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductTaxDetailsResourceTaxDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductTaxDetailsResourceTaxDetailsBuilder {
    performance_location: Option<Option<String>>,
    tax_code: Option<Option<String>>,
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

    impl Deserialize for TaxProductTaxDetailsResourceTaxDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductTaxDetailsResourceTaxDetails>,
        builder: TaxProductTaxDetailsResourceTaxDetailsBuilder,
    }

    impl Visitor for Place<TaxProductTaxDetailsResourceTaxDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductTaxDetailsResourceTaxDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductTaxDetailsResourceTaxDetailsBuilder {
        type Out = TaxProductTaxDetailsResourceTaxDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "performance_location" => Deserialize::begin(&mut self.performance_location),
                "tax_code" => Deserialize::begin(&mut self.tax_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { performance_location: Some(None), tax_code: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(performance_location), Some(tax_code)) =
                (self.performance_location.take(), self.tax_code.take())
            else {
                return None;
            };
            Some(Self::Out { performance_location, tax_code })
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

    impl ObjectDeser for TaxProductTaxDetailsResourceTaxDetails {
        type Builder = TaxProductTaxDetailsResourceTaxDetailsBuilder;
    }

    impl FromValueOpt for TaxProductTaxDetailsResourceTaxDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductTaxDetailsResourceTaxDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "performance_location" => b.performance_location = FromValueOpt::from_value(v),
                    "tax_code" => b.tax_code = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
