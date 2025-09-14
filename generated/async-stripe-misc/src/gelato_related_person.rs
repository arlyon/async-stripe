#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoRelatedPerson {
    /// Token referencing the associated Account of the related Person resource.
    pub account: String,
    /// Token referencing the related Person resource.
    pub person: String,
}
#[doc(hidden)]
pub struct GelatoRelatedPersonBuilder {
    account: Option<String>,
    person: Option<String>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: GelatoRelatedPersonBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoRelatedPersonBuilder {
        type Out = GelatoRelatedPerson;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "person" => Deserialize::begin(&mut self.person),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { account: Deserialize::default(), person: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(account), Some(person)) = (self.account.take(), self.person.take()) else {
                return None;
            };
            Some(Self::Out { account, person })
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

    impl ObjectDeser for GelatoRelatedPerson {
        type Builder = GelatoRelatedPersonBuilder;
    }

    impl FromValueOpt for GelatoRelatedPerson {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoRelatedPersonBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = FromValueOpt::from_value(v),
                    "person" => b.person = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
