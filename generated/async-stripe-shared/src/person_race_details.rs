#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PersonRaceDetails {
    /// The persons race.
    pub race: Option<Vec<PersonRaceDetailsRace>>,
    /// Please specify your race, when other is selected.
    pub race_other: Option<String>,
}
#[doc(hidden)]
pub struct PersonRaceDetailsBuilder {
    race: Option<Option<Vec<PersonRaceDetailsRace>>>,
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
            Self { race: Deserialize::default(), race_other: Deserialize::default() }
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
            *self.out = self.builder.take_out();
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PersonRaceDetailsRace {
    AfricanAmerican,
    AmericanIndianOrAlaskaNative,
    Asian,
    AsianIndian,
    BlackOrAfricanAmerican,
    Chinese,
    Ethiopian,
    Filipino,
    GuamanianOrChamorro,
    Haitian,
    Jamaican,
    Japanese,
    Korean,
    NativeHawaiian,
    NativeHawaiianOrOtherPacificIslander,
    Nigerian,
    OtherAsian,
    OtherBlackOrAfricanAmerican,
    OtherPacificIslander,
    PreferNotToAnswer,
    Samoan,
    Somali,
    Vietnamese,
    White,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PersonRaceDetailsRace {
    pub fn as_str(&self) -> &str {
        use PersonRaceDetailsRace::*;
        match self {
            AfricanAmerican => "african_american",
            AmericanIndianOrAlaskaNative => "american_indian_or_alaska_native",
            Asian => "asian",
            AsianIndian => "asian_indian",
            BlackOrAfricanAmerican => "black_or_african_american",
            Chinese => "chinese",
            Ethiopian => "ethiopian",
            Filipino => "filipino",
            GuamanianOrChamorro => "guamanian_or_chamorro",
            Haitian => "haitian",
            Jamaican => "jamaican",
            Japanese => "japanese",
            Korean => "korean",
            NativeHawaiian => "native_hawaiian",
            NativeHawaiianOrOtherPacificIslander => "native_hawaiian_or_other_pacific_islander",
            Nigerian => "nigerian",
            OtherAsian => "other_asian",
            OtherBlackOrAfricanAmerican => "other_black_or_african_american",
            OtherPacificIslander => "other_pacific_islander",
            PreferNotToAnswer => "prefer_not_to_answer",
            Samoan => "samoan",
            Somali => "somali",
            Vietnamese => "vietnamese",
            White => "white",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PersonRaceDetailsRace {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PersonRaceDetailsRace::*;
        match s {
            "african_american" => Ok(AfricanAmerican),
            "american_indian_or_alaska_native" => Ok(AmericanIndianOrAlaskaNative),
            "asian" => Ok(Asian),
            "asian_indian" => Ok(AsianIndian),
            "black_or_african_american" => Ok(BlackOrAfricanAmerican),
            "chinese" => Ok(Chinese),
            "ethiopian" => Ok(Ethiopian),
            "filipino" => Ok(Filipino),
            "guamanian_or_chamorro" => Ok(GuamanianOrChamorro),
            "haitian" => Ok(Haitian),
            "jamaican" => Ok(Jamaican),
            "japanese" => Ok(Japanese),
            "korean" => Ok(Korean),
            "native_hawaiian" => Ok(NativeHawaiian),
            "native_hawaiian_or_other_pacific_islander" => Ok(NativeHawaiianOrOtherPacificIslander),
            "nigerian" => Ok(Nigerian),
            "other_asian" => Ok(OtherAsian),
            "other_black_or_african_american" => Ok(OtherBlackOrAfricanAmerican),
            "other_pacific_islander" => Ok(OtherPacificIslander),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "samoan" => Ok(Samoan),
            "somali" => Ok(Somali),
            "vietnamese" => Ok(Vietnamese),
            "white" => Ok(White),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for PersonRaceDetailsRace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PersonRaceDetailsRace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PersonRaceDetailsRace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PersonRaceDetailsRace {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PersonRaceDetailsRace> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PersonRaceDetailsRace::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PersonRaceDetailsRace);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PersonRaceDetailsRace {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
