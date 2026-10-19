#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum JurisdictionLevel {
    City,
    Country,
    County,
    District,
    Multiple,
    State,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl JurisdictionLevel {
    pub fn as_str(&self) -> &str {
        use JurisdictionLevel::*;
        match self {
            City => "city",
            Country => "country",
            County => "county",
            District => "district",
            Multiple => "multiple",
            State => "state",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for JurisdictionLevel {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use JurisdictionLevel::*;
        match s {
            "city" => Ok(City),
            "country" => Ok(Country),
            "county" => Ok(County),
            "district" => Ok(District),
            "multiple" => Ok(Multiple),
            "state" => Ok(State),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "JurisdictionLevel");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for JurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for JurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for JurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(JurisdictionLevel)).finish_non_exhaustive()
    }
}
impl serde::Serialize for JurisdictionLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for JurisdictionLevel {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<JurisdictionLevel> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(JurisdictionLevel::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(JurisdictionLevel);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for JurisdictionLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
