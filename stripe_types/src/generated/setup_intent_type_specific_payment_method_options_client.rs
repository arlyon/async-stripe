#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SetupIntentTypeSpecificPaymentMethodOptionsClient {
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod>,
}
/// Bank account verification method.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "instant" => Ok(Self::Instant),
            "microdeposits" => Ok(Self::Microdeposits),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod"))
    }
}
