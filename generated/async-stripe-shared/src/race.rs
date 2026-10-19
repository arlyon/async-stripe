/// The persons race.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Race {
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
impl Race {
    pub fn as_str(&self) -> &str {
        use Race::*;
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

impl std::str::FromStr for Race {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Race::*;
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
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "Race");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(Race)).finish_non_exhaustive()
    }
}
impl serde::Serialize for Race {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for Race {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<Race> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(Race::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(Race);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Race {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
