#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CountrySpecVerificationFieldDetails {
    /// Additional fields which are only required for some users.
    pub additional: Vec<String>,
    /// Fields which every account must eventually provide.
    pub minimum: Vec<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CountrySpecVerificationFieldDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CountrySpecVerificationFieldDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CountrySpecVerificationFieldDetailsBuilder {
    additional: Option<Vec<String>>,
    minimum: Option<Vec<String>>,
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

    impl Deserialize for CountrySpecVerificationFieldDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CountrySpecVerificationFieldDetails>,
        builder: CountrySpecVerificationFieldDetailsBuilder,
    }

    impl Visitor for Place<CountrySpecVerificationFieldDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CountrySpecVerificationFieldDetailsBuilder {
                    additional: Deserialize::default(),
                    minimum: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "additional" => Deserialize::begin(&mut self.builder.additional),
                "minimum" => Deserialize::begin(&mut self.builder.minimum),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(additional), Some(minimum)) =
                (self.builder.additional.take(), self.builder.minimum.take())
            else {
                return Ok(());
            };
            *self.out = Some(CountrySpecVerificationFieldDetails { additional, minimum });
            Ok(())
        }
    }
};
