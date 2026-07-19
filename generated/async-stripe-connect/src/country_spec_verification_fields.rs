#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CountrySpecVerificationFields {
    pub company: stripe_connect::CountrySpecVerificationFieldDetails,
    pub individual: stripe_connect::CountrySpecVerificationFieldDetails,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CountrySpecVerificationFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CountrySpecVerificationFields").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CountrySpecVerificationFieldsBuilder {
    company: Option<stripe_connect::CountrySpecVerificationFieldDetails>,
    individual: Option<stripe_connect::CountrySpecVerificationFieldDetails>,
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

    impl Deserialize for CountrySpecVerificationFields {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CountrySpecVerificationFields>,
        builder: CountrySpecVerificationFieldsBuilder,
    }

    impl Visitor for Place<CountrySpecVerificationFields> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CountrySpecVerificationFieldsBuilder {
                    company: Deserialize::default(),
                    individual: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "company" => Deserialize::begin(&mut self.builder.company),
                "individual" => Deserialize::begin(&mut self.builder.individual),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(company), Some(individual)) =
                (self.builder.company.take(), self.builder.individual.take())
            else {
                return Ok(());
            };
            *self.out = Some(CountrySpecVerificationFields { company, individual });
            Ok(())
        }
    }
};
