#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AcssDebit {
    /// Currency supported by the bank account.
    pub currency: Option<AcssDebitCurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<
        crate::setup_intent::payment_method_options::acss_debit::mandate_options::MandateOptions,
    >,
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<AcssDebitVerificationMethod>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AcssDebit {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Currency supported by the bank account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AcssDebitCurrency {
    Cad,
    Usd,
}

impl AcssDebitCurrency {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cad => "cad",
            Self::Usd => "usd",
        }
    }
}

impl AsRef<str> for AcssDebitCurrency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl AcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for AcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod mandate_options;
pub use mandate_options::MandateOptions;
