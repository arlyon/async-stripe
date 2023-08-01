#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BankTransfer {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<stripe_core::customer_cash_balance_transaction::funded::bank_transfer::eu_bank_transfer::EuBankTransfer>,
#[serde(skip_serializing_if = "Option::is_none")]
pub gb_bank_transfer: Option<stripe_core::customer_cash_balance_transaction::funded::bank_transfer::gb_bank_transfer::GbBankTransfer>,
#[serde(skip_serializing_if = "Option::is_none")]
pub jp_bank_transfer: Option<stripe_core::customer_cash_balance_transaction::funded::bank_transfer::jp_bank_transfer::JpBankTransfer>,
    /// The user-supplied reference field on the bank transfer.
pub reference: Option<String>,
    /// The funding method type used to fund the customer balance.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
pub type_: BankTransferType,

}
/// The funding method type used to fund the customer balance.
///
/// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl BankTransferType {
    pub fn as_str(self) -> &'static str {
        use BankTransferType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            GbBankTransfer => "gb_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
            MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl std::str::FromStr for BankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankTransferType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "gb_bank_transfer" => Ok(GbBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            "mx_bank_transfer" => Ok(MxBankTransfer),
            _ => Err(()),
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
impl serde::Serialize for BankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BankTransferType"))
    }
}
pub mod eu_bank_transfer;
pub use eu_bank_transfer::EuBankTransfer;
pub mod gb_bank_transfer;
pub use gb_bank_transfer::GbBankTransfer;
pub mod jp_bank_transfer;
pub use jp_bank_transfer::JpBankTransfer;
