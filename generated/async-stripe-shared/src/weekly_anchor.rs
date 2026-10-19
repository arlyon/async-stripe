/// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
/// Required and applicable only if `interval` is `weekly`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum WeeklyAnchor {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl WeeklyAnchor {
    pub fn as_str(&self) -> &str {
        use WeeklyAnchor::*;
        match self {
            Friday => "friday",
            Monday => "monday",
            Saturday => "saturday",
            Sunday => "sunday",
            Thursday => "thursday",
            Tuesday => "tuesday",
            Wednesday => "wednesday",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for WeeklyAnchor {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use WeeklyAnchor::*;
        match s {
            "friday" => Ok(Friday),
            "monday" => Ok(Monday),
            "saturday" => Ok(Saturday),
            "sunday" => Ok(Sunday),
            "thursday" => Ok(Thursday),
            "tuesday" => Ok(Tuesday),
            "wednesday" => Ok(Wednesday),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "WeeklyAnchor");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for WeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for WeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for WeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(WeeklyAnchor)).finish_non_exhaustive()
    }
}
impl serde::Serialize for WeeklyAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for WeeklyAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
