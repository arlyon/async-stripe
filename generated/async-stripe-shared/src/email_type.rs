/// Type of email to send to the customer, one of `credit_note` or `none` and the default is `credit_note`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum EmailType {
    CreditNote,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl EmailType {
    pub fn as_str(&self) -> &str {
        use EmailType::*;
        match self {
            CreditNote => "credit_note",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for EmailType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EmailType::*;
        match s {
            "credit_note" => Ok(CreditNote),
            "none" => Ok(None),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "EmailType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for EmailType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for EmailType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for EmailType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(EmailType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for EmailType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for EmailType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
