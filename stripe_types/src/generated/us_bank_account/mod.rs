#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct UsBankAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<stripe_types::financial_connections::FinancialConnections>,
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<UsBankAccountVerificationMethod>,
}
/// Bank account verification method.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use UsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for UsBankAccountVerificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UsBankAccountVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UsBankAccountVerificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UsBankAccountVerificationMethod")
        })
    }
}
