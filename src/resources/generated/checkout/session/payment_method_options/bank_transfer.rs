#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BankTransfer {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<crate::payment_intent::payment_method_options::bank_transfer::eu_bank_transfer::EuBankTransfer>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[serde(skip_serializing_if = "Option::is_none")]
pub requested_address_types: Option<Vec<BankTransferRequestedAddressTypes>>,
    /// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
pub type_: Option<BankTransferType>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BankTransfer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// List of address types that should be returned in the financial_addresses response.
///
/// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum BankTransferRequestedAddressTypes {
    Iban,
    Sepa,
    SortCode,
    Spei,
    Zengin,
}

impl BankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iban => "iban",
            Self::Sepa => "sepa",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for BankTransferRequestedAddressTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum BankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl BankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl AsRef<str> for BankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
