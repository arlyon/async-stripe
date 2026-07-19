#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PersonEthnicityDetails {
    /// The persons ethnicity
    pub ethnicity: Option<Vec<PersonEthnicityDetailsEthnicity>>,
    /// Please specify your origin, when other is selected.
    pub ethnicity_other: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonEthnicityDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonEthnicityDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PersonEthnicityDetailsBuilder {
    ethnicity: Option<Option<Vec<PersonEthnicityDetailsEthnicity>>>,
    ethnicity_other: Option<Option<String>>,
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

    impl Deserialize for PersonEthnicityDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonEthnicityDetails>,
        builder: PersonEthnicityDetailsBuilder,
    }

    impl Visitor for Place<PersonEthnicityDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PersonEthnicityDetailsBuilder {
                    ethnicity: Deserialize::default(),
                    ethnicity_other: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ethnicity" => Deserialize::begin(&mut self.builder.ethnicity),
                "ethnicity_other" => Deserialize::begin(&mut self.builder.ethnicity_other),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(ethnicity), Some(ethnicity_other)) =
                (self.builder.ethnicity.take(), self.builder.ethnicity_other.take())
            else {
                return Ok(());
            };
            *self.out = Some(PersonEthnicityDetails { ethnicity, ethnicity_other });
            Ok(())
        }
    }
};
/// The persons ethnicity
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PersonEthnicityDetailsEthnicity {
    Cuban,
    HispanicOrLatino,
    Mexican,
    NotHispanicOrLatino,
    OtherHispanicOrLatino,
    PreferNotToAnswer,
    PuertoRican,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PersonEthnicityDetailsEthnicity {
    pub fn as_str(&self) -> &str {
        use PersonEthnicityDetailsEthnicity::*;
        match self {
            Cuban => "cuban",
            HispanicOrLatino => "hispanic_or_latino",
            Mexican => "mexican",
            NotHispanicOrLatino => "not_hispanic_or_latino",
            OtherHispanicOrLatino => "other_hispanic_or_latino",
            PreferNotToAnswer => "prefer_not_to_answer",
            PuertoRican => "puerto_rican",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PersonEthnicityDetailsEthnicity {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PersonEthnicityDetailsEthnicity::*;
        match s {
            "cuban" => Ok(Cuban),
            "hispanic_or_latino" => Ok(HispanicOrLatino),
            "mexican" => Ok(Mexican),
            "not_hispanic_or_latino" => Ok(NotHispanicOrLatino),
            "other_hispanic_or_latino" => Ok(OtherHispanicOrLatino),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "puerto_rican" => Ok(PuertoRican),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PersonEthnicityDetailsEthnicity"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PersonEthnicityDetailsEthnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PersonEthnicityDetailsEthnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonEthnicityDetailsEthnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PersonEthnicityDetailsEthnicity)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PersonEthnicityDetailsEthnicity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PersonEthnicityDetailsEthnicity {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PersonEthnicityDetailsEthnicity> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PersonEthnicityDetailsEthnicity::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PersonEthnicityDetailsEthnicity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
