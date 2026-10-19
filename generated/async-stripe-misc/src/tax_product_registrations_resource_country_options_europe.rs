#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsEurope {
    pub igic: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsIgic>,
    pub standard: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEuStandard>,
    /// Type of registration in an EU country.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsEurope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductRegistrationsResourceCountryOptionsEurope")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsEuropeBuilder {
    igic: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsIgic>>,
    standard: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEuStandard>>,
    type_: Option<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsEurope {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
        builder: TaxProductRegistrationsResourceCountryOptionsEuropeBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsEurope> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductRegistrationsResourceCountryOptionsEuropeBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsEuropeBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsEurope;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "igic" => Deserialize::begin(&mut self.igic),
                "standard" => Deserialize::begin(&mut self.standard),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { igic: Some(None), standard: Some(None), type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(igic), Some(standard), Some(type_)) =
                (self.igic.take(), self.standard.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { igic, standard, type_ })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsEurope {
        type Builder = TaxProductRegistrationsResourceCountryOptionsEuropeBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsEurope {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductRegistrationsResourceCountryOptionsEuropeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "igic" => b.igic = FromValueOpt::from_value(v),
                    "standard" => b.standard = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of registration in an EU country.
pub type TaxProductRegistrationsResourceCountryOptionsEuropeType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
