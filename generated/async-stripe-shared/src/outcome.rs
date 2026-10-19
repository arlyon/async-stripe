/// The outcome of the reported payment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Outcome {
    Failed,
    Guaranteed,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl Outcome {
    pub fn as_str(&self) -> &str {
        use Outcome::*;
        match self {
            Failed => "failed",
            Guaranteed => "guaranteed",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for Outcome {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Outcome::*;
        match s {
            "failed" => Ok(Failed),
            "guaranteed" => Ok(Guaranteed),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "Outcome");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(Outcome)).finish_non_exhaustive()
    }
}
impl serde::Serialize for Outcome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Outcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
