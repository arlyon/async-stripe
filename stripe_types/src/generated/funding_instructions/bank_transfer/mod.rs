#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
/// The bank_transfer type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BankTransferType {
    EuBankTransfer,
    JpBankTransfer,
}

impl BankTransferType {
    pub fn as_str(self) -> &'static str {
        use BankTransferType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
        }
    }
}

impl std::str::FromStr for BankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankTransferType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
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
pub mod financial_address;
pub use financial_address::FinancialAddress;
