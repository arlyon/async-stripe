/// The type of customer acceptance information included with the Mandate.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    Online,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    pub fn as_str(&self) -> &str {
        use ConfirmSetupIntentClientKeyParamCustomerAcceptanceType::*;
        match self {
            Online => "online",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentClientKeyParamCustomerAcceptanceType::*;
        match s {
            "online" => Ok(Online),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ConfirmSetupIntentClientKeyParamCustomerAcceptanceType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ConfirmSetupIntentClientKeyParamCustomerAcceptanceType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
