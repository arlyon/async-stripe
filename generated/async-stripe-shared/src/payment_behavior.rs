#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentBehavior {
    pub fn as_str(&self) -> &str {
        use PaymentBehavior::*;
        match self {
            AllowIncomplete => "allow_incomplete",
            DefaultIncomplete => "default_incomplete",
            ErrorIfIncomplete => "error_if_incomplete",
            PendingIfIncomplete => "pending_if_incomplete",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentBehavior::*;
        match s {
            "allow_incomplete" => Ok(AllowIncomplete),
            "default_incomplete" => Ok(DefaultIncomplete),
            "error_if_incomplete" => Ok(ErrorIfIncomplete),
            "pending_if_incomplete" => Ok(PendingIfIncomplete),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PaymentBehavior");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentBehavior)).finish_non_exhaustive()
    }
}
impl serde::Serialize for PaymentBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
