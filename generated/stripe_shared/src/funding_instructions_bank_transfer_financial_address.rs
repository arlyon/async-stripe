/// FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FundingInstructionsBankTransferFinancialAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<stripe_shared::FundingInstructionsBankTransferAbaRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<stripe_shared::FundingInstructionsBankTransferIbanRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<stripe_shared::FundingInstructionsBankTransferSortCodeRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spei: Option<stripe_shared::FundingInstructionsBankTransferSpeiRecord>,
    /// The payment networks supported by this FinancialAddress
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks:
        Option<Vec<FundingInstructionsBankTransferFinancialAddressSupportedNetworks>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift: Option<stripe_shared::FundingInstructionsBankTransferSwiftRecord>,
    /// The type of financial address
    #[serde(rename = "type")]
    pub type_: FundingInstructionsBankTransferFinancialAddressType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zengin: Option<stripe_shared::FundingInstructionsBankTransferZenginRecord>,
}
/// The payment networks supported by this FinancialAddress
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    Ach,
    Bacs,
    DomesticWireUs,
    Fps,
    Sepa,
    Spei,
    Swift,
    Zengin,
}
impl FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        use FundingInstructionsBankTransferFinancialAddressSupportedNetworks::*;
        match self {
            Ach => "ach",
            Bacs => "bacs",
            DomesticWireUs => "domestic_wire_us",
            Fps => "fps",
            Sepa => "sepa",
            Spei => "spei",
            Swift => "swift",
            Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsBankTransferFinancialAddressSupportedNetworks::*;
        match s {
            "ach" => Ok(Ach),
            "bacs" => Ok(Bacs),
            "domestic_wire_us" => Ok(DomesticWireUs),
            "fps" => Ok(Fps),
            "sepa" => Ok(Sepa),
            "spei" => Ok(Spei),
            "swift" => Ok(Swift),
            "zengin" => Ok(Zengin),
            _ => Err(()),
        }
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
/// The type of financial address
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FundingInstructionsBankTransferFinancialAddressType {
    Aba,
    Iban,
    SortCode,
    Spei,
    Swift,
    Zengin,
}
impl FundingInstructionsBankTransferFinancialAddressType {
    pub fn as_str(self) -> &'static str {
        use FundingInstructionsBankTransferFinancialAddressType::*;
        match self {
            Aba => "aba",
            Iban => "iban",
            SortCode => "sort_code",
            Spei => "spei",
            Swift => "swift",
            Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for FundingInstructionsBankTransferFinancialAddressType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsBankTransferFinancialAddressType::*;
        match s {
            "aba" => Ok(Aba),
            "iban" => Ok(Iban),
            "sort_code" => Ok(SortCode),
            "spei" => Ok(Spei),
            "swift" => Ok(Swift),
            "zengin" => Ok(Zengin),
            _ => Err(()),
        }
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
