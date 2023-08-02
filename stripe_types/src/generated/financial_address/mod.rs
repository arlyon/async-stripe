/// FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FinancialAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<stripe_types::iban::Iban>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<stripe_types::sort_code::SortCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spei: Option<stripe_types::spei::Spei>,
    /// The payment networks supported by this FinancialAddress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks: Option<Vec<FinancialAddressSupportedNetworks>>,
    /// The type of financial address.
    #[serde(rename = "type")]
    pub type_: FinancialAddressType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zengin: Option<stripe_types::zengin::Zengin>,
}
/// The payment networks supported by this FinancialAddress.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FinancialAddressSupportedNetworks {
    Bacs,
    Fps,
    Sepa,
    Spei,
    Zengin,
}

impl FinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        use FinancialAddressSupportedNetworks::*;
        match self {
            Bacs => "bacs",
            Fps => "fps",
            Sepa => "sepa",
            Spei => "spei",
            Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for FinancialAddressSupportedNetworks {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialAddressSupportedNetworks::*;
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
impl serde::Serialize for FinancialAddressSupportedNetworks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialAddressSupportedNetworks {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialAddressSupportedNetworks")
        })
    }
}
/// The type of financial address.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FinancialAddressType {
    Iban,
    SortCode,
    Spei,
    Zengin,
}

impl FinancialAddressType {
    pub fn as_str(self) -> &'static str {
        use FinancialAddressType::*;
        match self {
            Iban => "iban",
            SortCode => "sort_code",
            Spei => "spei",
            Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for FinancialAddressType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialAddressType::*;
        match s {
            "iban" => Ok(Iban),
            "sort_code" => Ok(SortCode),
            "spei" => Ok(Spei),
            "zengin" => Ok(Zengin),
            _ => Err(()),
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
impl serde::Serialize for FinancialAddressType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialAddressType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for FinancialAddressType"))
    }
}
