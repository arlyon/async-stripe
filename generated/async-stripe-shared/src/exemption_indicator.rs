/// The exemption requested via 3DS and accepted by the issuer at authentication time.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ExemptionIndicator {
    LowRisk,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ExemptionIndicator {
    pub fn as_str(&self) -> &str {
        use ExemptionIndicator::*;
        match self {
            LowRisk => "low_risk",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ExemptionIndicator {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ExemptionIndicator::*;
        match s {
            "low_risk" => Ok(LowRisk),
            "none" => Ok(None),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ExemptionIndicator");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ExemptionIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ExemptionIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ExemptionIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ExemptionIndicator)).finish_non_exhaustive()
    }
}
impl serde::Serialize for ExemptionIndicator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ExemptionIndicator {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ExemptionIndicator> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ExemptionIndicator::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ExemptionIndicator);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ExemptionIndicator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
