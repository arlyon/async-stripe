#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BankTransfer {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<crate::customer_cash_balance_transaction::funded::bank_transfer::eu_bank_transfer::EuBankTransfer>,
    /// The user-supplied reference field on the bank transfer.
pub reference: Option<String>,
    /// The funding method type used to fund the customer balance.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
pub type_: BankTransferType,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BankTransfer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The funding method type used to fund the customer balance.
///
/// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
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
pub mod eu_bank_transfer;
pub use eu_bank_transfer::EuBankTransfer;
