#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupIntentTypeSpecificPaymentMethodOptionsClient {
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetupIntentTypeSpecificPaymentMethodOptionsClient {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Bank account verification method.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
