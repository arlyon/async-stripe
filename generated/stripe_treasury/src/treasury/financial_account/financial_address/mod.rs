/// FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FinancialAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<stripe_treasury::treasury::financial_account::financial_address::aba::Aba>,
    /// The list of networks that the address supports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks: Option<Vec<FinancialAddressSupportedNetworks>>,
    /// The type of financial address.
    #[serde(rename = "type")]
    pub type_: FinancialAddressType,
}
/// The list of networks that the address supports.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FinancialAddressSupportedNetworks {
    Ach,
    UsDomesticWire,
}

impl FinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for FinancialAddressSupportedNetworks {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ach" => Ok(Self::Ach),
            "us_domestic_wire" => Ok(Self::UsDomesticWire),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialAddressSupportedNetworks")
        })
    }
}
/// The type of financial address.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FinancialAddressType {
    Aba,
}

impl FinancialAddressType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Aba => "aba",
        }
    }
}

impl std::str::FromStr for FinancialAddressType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "aba" => Ok(Self::Aba),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for FinancialAddressType"))
    }
}
pub mod aba;
pub use aba::Aba;
