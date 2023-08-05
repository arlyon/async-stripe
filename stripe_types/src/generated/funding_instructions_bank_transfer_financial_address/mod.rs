/// FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FundingInstructionsBankTransferFinancialAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<stripe_types::FundingInstructionsBankTransferIbanRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<stripe_types::FundingInstructionsBankTransferSortCodeRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spei: Option<stripe_types::FundingInstructionsBankTransferSpeiRecord>,
    /// The payment networks supported by this FinancialAddress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks:
        Option<Vec<FundingInstructionsBankTransferFinancialAddressSupportedNetworks>>,
    /// The type of financial address.
    #[serde(rename = "type")]
    pub type_: FundingInstructionsBankTransferFinancialAddressType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zengin: Option<stripe_types::FundingInstructionsBankTransferZenginRecord>,
}
/// The payment networks supported by this FinancialAddress.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    Bacs,
    Fps,
    Sepa,
    Spei,
    Zengin,
}

impl FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        use FundingInstructionsBankTransferFinancialAddressSupportedNetworks::*;
        match self {
            Bacs => "bacs",
            Fps => "fps",
            Sepa => "sepa",
            Spei => "spei",
            Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsBankTransferFinancialAddressSupportedNetworks::*;
        match s {
            "bacs" => Ok(Bacs),
            "fps" => Ok(Fps),
            "sepa" => Ok(Sepa),
            "spei" => Ok(Spei),
            "zengin" => Ok(Zengin),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for FundingInstructionsBankTransferFinancialAddressSupportedNetworks
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for FundingInstructionsBankTransferFinancialAddressSupportedNetworks"))
    }
}
/// The type of financial address.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FundingInstructionsBankTransferFinancialAddressType {
    Iban,
    SortCode,
    Spei,
    Zengin,
}

impl FundingInstructionsBankTransferFinancialAddressType {
    pub fn as_str(self) -> &'static str {
        use FundingInstructionsBankTransferFinancialAddressType::*;
        match self {
            Iban => "iban",
            SortCode => "sort_code",
            Spei => "spei",
            Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for FundingInstructionsBankTransferFinancialAddressType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsBankTransferFinancialAddressType::*;
        match s {
            "iban" => Ok(Iban),
            "sort_code" => Ok(SortCode),
            "spei" => Ok(Spei),
            "zengin" => Ok(Zengin),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for FundingInstructionsBankTransferFinancialAddressType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FundingInstructionsBankTransferFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FundingInstructionsBankTransferFinancialAddressType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FundingInstructionsBankTransferFinancialAddressType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for FundingInstructionsBankTransferFinancialAddressType",
            )
        })
    }
}
