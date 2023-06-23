#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct EuBankTransfer {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: EuBankTransferCountry,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for EuBankTransfer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The desired country code of the bank account information.
///
/// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum EuBankTransferCountry {
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "NL")]
    Nl,
}

impl EuBankTransferCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "DE",
            Self::Es => "ES",
            Self::Fr => "FR",
            Self::Ie => "IE",
            Self::Nl => "NL",
        }
    }
}

impl AsRef<str> for EuBankTransferCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for EuBankTransferCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
