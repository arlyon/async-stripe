#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThresholdsResourceUsageAlertFilterType {
    Customer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThresholdsResourceUsageAlertFilterType {
    pub fn as_str(&self) -> &str {
        use ThresholdsResourceUsageAlertFilterType::*;
        match self {
            Customer => "customer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThresholdsResourceUsageAlertFilterType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThresholdsResourceUsageAlertFilterType::*;
        match s {
            "customer" => Ok(Customer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThresholdsResourceUsageAlertFilterType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThresholdsResourceUsageAlertFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThresholdsResourceUsageAlertFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThresholdsResourceUsageAlertFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThresholdsResourceUsageAlertFilterType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for ThresholdsResourceUsageAlertFilterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThresholdsResourceUsageAlertFilterType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThresholdsResourceUsageAlertFilterType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThresholdsResourceUsageAlertFilterType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThresholdsResourceUsageAlertFilterType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThresholdsResourceUsageAlertFilterType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
