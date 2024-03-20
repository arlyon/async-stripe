#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingNetworkTokenDevice {
    /// An obfuscated ID derived from the device ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,
    /// The IP address of the device at provisioning time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The geographic latitude/longitude coordinates of the device at provisioning time.
    /// The format is [+-]decimal/[+-]decimal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// The name of the device used for tokenization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The phone number of the device used for tokenization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The type of device used for tokenization.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<IssuingNetworkTokenDeviceType>,
}
/// The type of device used for tokenization.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenDeviceType {
    Other,
    Phone,
    Watch,
}
impl IssuingNetworkTokenDeviceType {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenDeviceType::*;
        match self {
            Other => "other",
            Phone => "phone",
            Watch => "watch",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenDeviceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenDeviceType::*;
        match s {
            "other" => Ok(Other),
            "phone" => Ok(Phone),
            "watch" => Ok(Watch),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingNetworkTokenDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingNetworkTokenDeviceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenDeviceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingNetworkTokenDeviceType")
        })
    }
}
