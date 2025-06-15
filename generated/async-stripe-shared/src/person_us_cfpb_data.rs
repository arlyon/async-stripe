#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct PersonUsCfpbDataBuilder {
    ethnicity_details: Option<Option<stripe_shared::PersonEthnicityDetails>>,
    race_details: Option<Option<stripe_shared::PersonRaceDetails>>,
    self_identified_gender: Option<Option<String>>,
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
                builder: PersonUsCfpbDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PersonUsCfpbDataBuilder {
        type Out = PersonUsCfpbData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ethnicity_details" => Deserialize::begin(&mut self.ethnicity_details),
                "race_details" => Deserialize::begin(&mut self.race_details),
                "self_identified_gender" => Deserialize::begin(&mut self.self_identified_gender),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                ethnicity_details: Deserialize::default(),
                race_details: Deserialize::default(),
                self_identified_gender: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(ethnicity_details), Some(race_details), Some(self_identified_gender)) = (
                self.ethnicity_details.take(),
                self.race_details.take(),
                self.self_identified_gender.take(),
            ) else {
                return None;
            };
            Some(Self::Out { ethnicity_details, race_details, self_identified_gender })
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

    impl ObjectDeser for PersonUsCfpbData {
        type Builder = PersonUsCfpbDataBuilder;
    }

    impl FromValueOpt for PersonUsCfpbData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PersonUsCfpbDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ethnicity_details" => b.ethnicity_details = FromValueOpt::from_value(v),
                    "race_details" => b.race_details = FromValueOpt::from_value(v),
                    "self_identified_gender" => {
                        b.self_identified_gender = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
