/// FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FinancialAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<crate::funding_instructions::bank_transfer::financial_address::iban::Iban>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code:
        Option<crate::funding_instructions::bank_transfer::financial_address::sort_code::SortCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spei: Option<crate::funding_instructions::bank_transfer::financial_address::spei::Spei>,
    /// The payment networks supported by this FinancialAddress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks: Option<Vec<FinancialAddressSupportedNetworks>>,
    /// The type of financial address.
    #[serde(rename = "type")]
    pub type_: FinancialAddressType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zengin:
        Option<crate::funding_instructions::bank_transfer::financial_address::zengin::Zengin>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialAddress {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The payment networks supported by this FinancialAddress.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FinancialAddressSupportedNetworks {
    Bacs,
    Fps,
    Sepa,
    Spei,
    Zengin,
}

impl FinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bacs => "bacs",
            Self::Fps => "fps",
            Self::Sepa => "sepa",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for FinancialAddressSupportedNetworks {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of financial address.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FinancialAddressType {
    Iban,
    SortCode,
    Spei,
    Zengin,
}

impl FinancialAddressType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iban => "iban",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for FinancialAddressType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod iban;
pub use iban::Iban;
pub mod sort_code;
pub use sort_code::SortCode;
pub mod spei;
pub use spei::Spei;
pub mod zengin;
pub use zengin::Zengin;
