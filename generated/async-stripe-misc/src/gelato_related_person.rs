#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoRelatedPerson {
    /// Token referencing the associated Account of the related Person resource.
    pub account: String,
    /// Token referencing the related Person resource.
    pub person: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoRelatedPerson {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoRelatedPerson").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoRelatedPersonBuilder {
    account: Option<String>,
    person: Option<String>,
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

    impl Deserialize for GelatoRelatedPerson {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoRelatedPerson>,
        builder: GelatoRelatedPersonBuilder,
    }

    impl Visitor for Place<GelatoRelatedPerson> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoRelatedPersonBuilder {
                    account: Deserialize::default(),
                    person: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.builder.account),
                "person" => Deserialize::begin(&mut self.builder.person),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(account), Some(person)) =
                (self.builder.account.take(), self.builder.person.take())
            else {
                return Ok(());
            };
            *self.out = Some(GelatoRelatedPerson { account, person });
            Ok(())
        }
    }
};
