/// The interval of debits permitted by the mandate.
/// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSourceMandateInterval {
    OneTime,
    Scheduled,
    Variable,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSourceMandateInterval {
    pub fn as_str(&self) -> &str {
        use CreateSourceMandateInterval::*;
        match self {
            OneTime => "one_time",
            Scheduled => "scheduled",
            Variable => "variable",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSourceMandateInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceMandateInterval::*;
        match s {
            "one_time" => Ok(OneTime),
            "scheduled" => Ok(Scheduled),
            "variable" => Ok(Variable),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSourceMandateInterval"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSourceMandateInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateSourceMandateInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSourceMandateInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateSourceMandateInterval)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateSourceMandateInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSourceMandateInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
