#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoProvidedDetails {
    /// Email of user being verified
    pub email: Option<String>,
    /// Phone number of user being verified
    pub phone: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoProvidedDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoProvidedDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoProvidedDetailsBuilder {
    email: Option<Option<String>>,
    phone: Option<Option<String>>,
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

    impl Deserialize for GelatoProvidedDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoProvidedDetails>,
        builder: GelatoProvidedDetailsBuilder,
    }

    impl Visitor for Place<GelatoProvidedDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoProvidedDetailsBuilder {
                    email: Deserialize::default(),
                    phone: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "email" => Deserialize::begin(&mut self.builder.email),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(email), Some(phone)) = (self.builder.email.take(), self.builder.phone.take())
            else {
                return Ok(());
            };
            *self.out = Some(GelatoProvidedDetails { email, phone });
            Ok(())
        }
    }
};
