#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PersonUsCfpbData {
    /// The persons ethnicity details
    pub ethnicity_details: Option<stripe_shared::PersonEthnicityDetails>,
    /// The persons race details
    pub race_details: Option<stripe_shared::PersonRaceDetails>,
    /// The persons self-identified gender
    pub self_identified_gender: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonUsCfpbData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonUsCfpbData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PersonUsCfpbDataBuilder {
    ethnicity_details: Option<Option<stripe_shared::PersonEthnicityDetails>>,
    race_details: Option<Option<stripe_shared::PersonRaceDetails>>,
    self_identified_gender: Option<Option<String>>,
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

    impl Deserialize for PersonUsCfpbData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonUsCfpbData>,
        builder: PersonUsCfpbDataBuilder,
    }

    impl Visitor for Place<PersonUsCfpbData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PersonUsCfpbDataBuilder {
                    ethnicity_details: Deserialize::default(),
                    race_details: Deserialize::default(),
                    self_identified_gender: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ethnicity_details" => Deserialize::begin(&mut self.builder.ethnicity_details),
                "race_details" => Deserialize::begin(&mut self.builder.race_details),
                "self_identified_gender" => {
                    Deserialize::begin(&mut self.builder.self_identified_gender)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(ethnicity_details), Some(race_details), Some(self_identified_gender)) = (
                self.builder.ethnicity_details.take(),
                self.builder.race_details.take(),
                self.builder.self_identified_gender.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(PersonUsCfpbData { ethnicity_details, race_details, self_identified_gender });
            Ok(())
        }
    }
};
