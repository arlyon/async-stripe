#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountLinkCollect {
    CurrentlyDue,
    EventuallyDue,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountLinkCollect {
    pub fn as_str(&self) -> &str {
        use CreateAccountLinkCollect::*;
        match self {
            CurrentlyDue => "currently_due",
            EventuallyDue => "eventually_due",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountLinkCollect {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkCollect::*;
        match s {
            "currently_due" => Ok(CurrentlyDue),
            "eventually_due" => Ok(EventuallyDue),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreateAccountLinkCollect");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountLinkCollect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountLinkCollect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountLinkCollect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountLinkCollect)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountLinkCollect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountLinkCollect {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
