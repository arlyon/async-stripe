#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PersonEthnicityDetails {
    /// The persons ethnicity
    pub ethnicity: Option<Vec<PersonEthnicityDetailsEthnicity>>,
    /// Please specify your origin, when other is selected.
    pub ethnicity_other: Option<String>,
}
#[doc(hidden)]
pub struct PersonEthnicityDetailsBuilder {
    ethnicity: Option<Option<Vec<PersonEthnicityDetailsEthnicity>>>,
    ethnicity_other: Option<Option<String>>,
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
                builder: PersonEthnicityDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PersonEthnicityDetailsBuilder {
        type Out = PersonEthnicityDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ethnicity" => Deserialize::begin(&mut self.ethnicity),
                "ethnicity_other" => Deserialize::begin(&mut self.ethnicity_other),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { ethnicity: Deserialize::default(), ethnicity_other: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(ethnicity), Some(ethnicity_other)) =
                (self.ethnicity.take(), self.ethnicity_other.take())
            else {
                return None;
            };
            Some(Self::Out { ethnicity, ethnicity_other })
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

    impl ObjectDeser for PersonEthnicityDetails {
        type Builder = PersonEthnicityDetailsBuilder;
    }

    impl FromValueOpt for PersonEthnicityDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PersonEthnicityDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ethnicity" => b.ethnicity = FromValueOpt::from_value(v),
                    "ethnicity_other" => b.ethnicity_other = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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

impl std::fmt::Debug for PersonEthnicityDetailsEthnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PersonEthnicityDetailsEthnicity {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PersonEthnicityDetailsEthnicity> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PersonEthnicityDetailsEthnicity::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PersonEthnicityDetailsEthnicity);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PersonEthnicityDetailsEthnicity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
