/// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PoliticalExposure {
    Existing,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PoliticalExposure {
    pub fn as_str(&self) -> &str {
        use PoliticalExposure::*;
        match self {
            Existing => "existing",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PoliticalExposure {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PoliticalExposure::*;
        match s {
            "existing" => Ok(Existing),
            "none" => Ok(None),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PoliticalExposure");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PoliticalExposure)).finish_non_exhaustive()
    }
}
impl serde::Serialize for PoliticalExposure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PoliticalExposure {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PoliticalExposure> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PoliticalExposure::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PoliticalExposure);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PoliticalExposure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
