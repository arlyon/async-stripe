#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardholderCompany {
    /// Whether the company's business ID number was provided.
    pub tax_id_provided: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardholderCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardholderCompany").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardholderCompanyBuilder {
    tax_id_provided: Option<bool>,
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

    impl Deserialize for IssuingCardholderCompany {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderCompany>,
        builder: IssuingCardholderCompanyBuilder,
    }

    impl Visitor for Place<IssuingCardholderCompany> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardholderCompanyBuilder {
                    tax_id_provided: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "tax_id_provided" => Deserialize::begin(&mut self.builder.tax_id_provided),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(tax_id_provided),) = (self.builder.tax_id_provided,) else {
                return Ok(());
            };
            *self.out = Some(IssuingCardholderCompany { tax_id_provided });
            Ok(())
        }
    }
};
