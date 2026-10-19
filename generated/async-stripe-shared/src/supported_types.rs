/// Specifies the type of mandates supported. Possible values are `india`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SupportedTypes {
    India,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SupportedTypes {
    pub fn as_str(&self) -> &str {
        use SupportedTypes::*;
        match self {
            India => "india",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SupportedTypes {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SupportedTypes::*;
        match s {
            "india" => Ok(India),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "SupportedTypes");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SupportedTypes)).finish_non_exhaustive()
    }
}
impl serde::Serialize for SupportedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SupportedTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SupportedTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SupportedTypes::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SupportedTypes);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SupportedTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
