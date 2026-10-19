#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AlertType {
    UsageThreshold,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AlertType {
    pub fn as_str(&self) -> &str {
        use AlertType::*;
        match self {
            UsageThreshold => "usage_threshold",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AlertType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AlertType::*;
        match s {
            "usage_threshold" => Ok(UsageThreshold),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "AlertType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AlertType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for AlertType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AlertType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AlertType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AlertType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AlertType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AlertType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
