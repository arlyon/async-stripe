#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsCanada {
    pub province_standard:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard>,
    /// Type of registration in Canada.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsCaType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsCanada {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductRegistrationsResourceCountryOptionsCanada")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsCanadaBuilder {
    province_standard: Option<
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard>,
    >,
    type_: Option<stripe_shared::CreateTaxRegistrationCountryOptionsCaType>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsCanada {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsCanada>,
        builder: TaxProductRegistrationsResourceCountryOptionsCanadaBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsCanada> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductRegistrationsResourceCountryOptionsCanadaBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsCanadaBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsCanada;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "province_standard" => Deserialize::begin(&mut self.province_standard),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { province_standard: Some(None), type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(province_standard), Some(type_)) =
                (self.province_standard.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { province_standard, type_ })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsCanada {
        type Builder = TaxProductRegistrationsResourceCountryOptionsCanadaBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsCanada {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductRegistrationsResourceCountryOptionsCanadaBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "province_standard" => b.province_standard = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of registration in Canada.
pub type TaxProductRegistrationsResourceCountryOptionsCanadaType =
    stripe_shared::CreateTaxRegistrationCountryOptionsCaType;
