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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for BankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eu_bank_transfer" => Ok(Self::EuBankTransfer),
            "jp_bank_transfer" => Ok(Self::JpBankTransfer),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BankTransferType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BankTransferType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<BankTransferType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BankTransferType::from_str(s)?);
        Ok(())
    }
}
pub mod financial_address;
pub use financial_address::FinancialAddress;
