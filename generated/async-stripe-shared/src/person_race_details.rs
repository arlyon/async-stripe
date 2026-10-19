#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PersonRaceDetails {
    /// The persons race.
    pub race: Option<Vec<stripe_shared::Race>>,
    /// Please specify your race, when other is selected.
    pub race_other: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonRaceDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonRaceDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PersonRaceDetailsBuilder {
    race: Option<Option<Vec<stripe_shared::Race>>>,
    race_other: Option<Option<String>>,
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

    impl Deserialize for PersonRaceDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonRaceDetails>,
        builder: PersonRaceDetailsBuilder,
    }

    impl Visitor for Place<PersonRaceDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PersonRaceDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PersonRaceDetailsBuilder {
        type Out = PersonRaceDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "race" => Deserialize::begin(&mut self.race),
                "race_other" => Deserialize::begin(&mut self.race_other),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { race: Some(None), race_other: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(race), Some(race_other)) = (self.race.take(), self.race_other.take()) else {
                return None;
            };
            Some(Self::Out { race, race_other })
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

    impl ObjectDeser for PersonRaceDetails {
        type Builder = PersonRaceDetailsBuilder;
    }

    impl FromValueOpt for PersonRaceDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PersonRaceDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "race" => b.race = FromValueOpt::from_value(v),
                    "race_other" => b.race_other = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The persons race.
pub type PersonRaceDetailsRace = stripe_shared::Race;
