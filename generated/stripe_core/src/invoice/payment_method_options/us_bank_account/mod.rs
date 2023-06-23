#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct UsBankAccount {
#[serde(skip_serializing_if = "Option::is_none")]
pub financial_connections: Option<stripe_core::invoice::payment_method_options::us_bank_account::financial_connections::FinancialConnections>,
    /// Bank account verification method.
#[serde(skip_serializing_if = "Option::is_none")]
pub verification_method: Option<UsBankAccountVerificationMethod>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for UsBankAccount {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Bank account verification method.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum UsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
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
pub mod financial_connections;
pub use financial_connections::FinancialConnections;
