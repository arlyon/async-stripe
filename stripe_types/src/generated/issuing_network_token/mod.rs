/// An issuing token object is created when an issued card is added to a digital wallet.
///
/// As a [card issuer](https://stripe.com/docs/issuing), you can [view and manage these tokens](https://stripe.com/docs/issuing/controls/token-management) through Stripe.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingNetworkToken {
    /// Card associated with this token.
    pub card: stripe_types::Expandable<stripe_types::IssuingCard>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The hashed ID derived from the device ID from the card network associated with the token.
    pub device_fingerprint: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_types::issuing_network_token::IssuingTokenId,
    /// The last four digits of the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The token service provider / card network associated with the token.
    pub network: IssuingNetworkTokenNetwork,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_data: Option<stripe_types::IssuingNetworkTokenNetworkData>,
    /// Time at which the token was last updated by the card network.
    ///
    /// Measured in seconds since the Unix epoch.
    pub network_updated_at: stripe_types::Timestamp,
    /// The usage state of the token.
    pub status: IssuingNetworkTokenStatus,
    /// The digital wallet for this token, if one was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_provider: Option<IssuingNetworkTokenWalletProvider>,
}
/// The token service provider / card network associated with the token.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenNetwork {
    Mastercard,
    Visa,
}

impl IssuingNetworkTokenNetwork {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenNetwork::*;
        match self {
            Mastercard => "mastercard",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenNetwork::*;
        match s {
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingNetworkTokenNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingNetworkTokenNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingNetworkTokenNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingNetworkTokenNetwork"))
    }
}
/// The usage state of the token.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenStatus {
    Active,
    Deleted,
    Requested,
    Suspended,
}

impl IssuingNetworkTokenStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenStatus::*;
        match self {
            Active => "active",
            Deleted => "deleted",
            Requested => "requested",
            Suspended => "suspended",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenStatus::*;
        match s {
            "active" => Ok(Active),
            "deleted" => Ok(Deleted),
            "requested" => Ok(Requested),
            "suspended" => Ok(Suspended),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingNetworkTokenStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingNetworkTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingNetworkTokenStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingNetworkTokenStatus"))
    }
}
/// The digital wallet for this token, if one was used.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenWalletProvider {
    ApplePay,
    GooglePay,
    SamsungPay,
}

impl IssuingNetworkTokenWalletProvider {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenWalletProvider::*;
        match self {
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            SamsungPay => "samsung_pay",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenWalletProvider {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenWalletProvider::*;
        match s {
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "samsung_pay" => Ok(SamsungPay),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingNetworkTokenWalletProvider {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingNetworkTokenWalletProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenWalletProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingNetworkTokenWalletProvider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenWalletProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingNetworkTokenWalletProvider")
        })
    }
}
impl stripe_types::Object for IssuingNetworkToken {
    type Id = stripe_types::issuing_network_token::IssuingTokenId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(IssuingTokenId);
