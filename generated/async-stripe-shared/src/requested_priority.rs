#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RequestedPriority {
    Domestic,
    International,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RequestedPriority {
    pub fn as_str(&self) -> &str {
        use RequestedPriority::*;
        match self {
            Domestic => "domestic",
            International => "international",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for RequestedPriority {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RequestedPriority::*;
        match s {
            "domestic" => Ok(Domestic),
            "international" => Ok(International),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "RequestedPriority");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for RequestedPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for RequestedPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RequestedPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(RequestedPriority)).finish_non_exhaustive()
    }
}
impl serde::Serialize for RequestedPriority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for RequestedPriority {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<RequestedPriority> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RequestedPriority::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(RequestedPriority);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RequestedPriority {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
