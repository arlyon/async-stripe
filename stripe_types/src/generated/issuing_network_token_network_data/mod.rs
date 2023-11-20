#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingNetworkTokenNetworkData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<stripe_types::IssuingNetworkTokenDevice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastercard: Option<stripe_types::IssuingNetworkTokenMastercard>,
    /// The network that the token is associated with.
    ///
    /// An additional hash is included with a name matching this value, containing tokenization data specific to the card network.
    #[serde(rename = "type")]
    pub type_: IssuingNetworkTokenNetworkDataType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<stripe_types::IssuingNetworkTokenVisa>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_provider: Option<stripe_types::IssuingNetworkTokenWalletProvider>,
}
/// The network that the token is associated with.
///
/// An additional hash is included with a name matching this value, containing tokenization data specific to the card network.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenNetworkDataType {
    Mastercard,
    Visa,
}

impl IssuingNetworkTokenNetworkDataType {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenNetworkDataType::*;
        match self {
            Mastercard => "mastercard",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenNetworkDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenNetworkDataType::*;
        match s {
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingNetworkTokenNetworkDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingNetworkTokenNetworkDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenNetworkDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingNetworkTokenNetworkDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenNetworkDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingNetworkTokenNetworkDataType")
        })
    }
}
