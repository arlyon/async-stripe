#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Wallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<stripe_types::apple_pay::ApplePay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<stripe_types::google_pay::GooglePay>,
    /// The type of the card wallet, one of `apple_pay`, `google_pay`, or `link`.
    ///
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[serde(rename = "type")]
    pub type_: WalletType,
}
/// The type of the card wallet, one of `apple_pay`, `google_pay`, or `link`.
///
/// An additional hash is included on the Wallet subhash with a name matching this value.
/// It contains additional information specific to the card wallet type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WalletType {
    ApplePay,
    GooglePay,
    Link,
}

impl WalletType {
    pub fn as_str(self) -> &'static str {
        use WalletType::*;
        match self {
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            Link => "link",
        }
    }
}

impl std::str::FromStr for WalletType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use WalletType::*;
        match s {
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "link" => Ok(Link),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for WalletType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for WalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for WalletType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WalletType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for WalletType"))
    }
}
