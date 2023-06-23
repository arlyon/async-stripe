#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BankTransfer {
    /// The country of the bank account to fund.
    pub country: String,
    /// A list of financial addresses that can be used to fund a particular balance.
    pub financial_addresses:
        Vec<stripe_types::funding_instructions::bank_transfer::financial_address::FinancialAddress>,
    /// The bank_transfer type.
    #[serde(rename = "type")]
    pub type_: BankTransferType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BankTransfer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The bank_transfer type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum BankTransferType {
    EuBankTransfer,
    JpBankTransfer,
}

impl BankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
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
pub mod financial_address;
pub use financial_address::FinancialAddress;
