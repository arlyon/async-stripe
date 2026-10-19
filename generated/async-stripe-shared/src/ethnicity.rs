/// The persons ethnicity
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Ethnicity {
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
impl Ethnicity {
    pub fn as_str(&self) -> &str {
        use Ethnicity::*;
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

impl std::str::FromStr for Ethnicity {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Ethnicity::*;
        match s {
            "cuban" => Ok(Cuban),
            "hispanic_or_latino" => Ok(HispanicOrLatino),
            "mexican" => Ok(Mexican),
            "not_hispanic_or_latino" => Ok(NotHispanicOrLatino),
            "other_hispanic_or_latino" => Ok(OtherHispanicOrLatino),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "puerto_rican" => Ok(PuertoRican),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "Ethnicity");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for Ethnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for Ethnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Ethnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(Ethnicity)).finish_non_exhaustive()
    }
}
impl serde::Serialize for Ethnicity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for Ethnicity {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<Ethnicity> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(Ethnicity::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(Ethnicity);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Ethnicity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
