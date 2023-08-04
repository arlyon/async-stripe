#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FundingInstructionsBankTransfer {
    /// The country of the bank account to fund.
    pub country: String,
    /// A list of financial addresses that can be used to fund a particular balance.
    pub financial_addresses: Vec<stripe_types::FundingInstructionsBankTransferFinancialAddress>,
    /// The bank_transfer type.
    #[serde(rename = "type")]
    pub type_: FundingInstructionsBankTransferType,
}
/// The bank_transfer type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FundingInstructionsBankTransferType {
    EuBankTransfer,
    JpBankTransfer,
}

impl FundingInstructionsBankTransferType {
    pub fn as_str(self) -> &'static str {
        use FundingInstructionsBankTransferType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
        }
    }
}

impl std::str::FromStr for FundingInstructionsBankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsBankTransferType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for FundingInstructionsBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FundingInstructionsBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FundingInstructionsBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FundingInstructionsBankTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FundingInstructionsBankTransferType")
        })
    }
}
