#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard {
    /// Two-letter CA province code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub province: String,
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder {
    province: Option<String>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard>,
        builder: TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "province" => Deserialize::begin(&mut self.province),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { province: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(province),) = (self.province.take(),) else {
                return None;
            };
            Some(Self::Out { province })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard {
        type Builder = TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "province" => b.province = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
