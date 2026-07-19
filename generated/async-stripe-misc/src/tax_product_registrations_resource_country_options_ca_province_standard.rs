#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard {
    /// Two-letter CA province code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub province: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder {
    province: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TaxProductRegistrationsResourceCountryOptionsCaProvinceStandardBuilder {
                    province: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "province" => Deserialize::begin(&mut self.builder.province),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(province),) = (self.builder.province.take(),) else {
                return Ok(());
            };
            *self.out =
                Some(TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard { province });
            Ok(())
        }
    }
};
