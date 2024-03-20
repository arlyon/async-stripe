/// An issuing token object is created when an issued card is added to a digital wallet.
/// As a [card issuer](https://stripe.com/docs/issuing), you can [view and manage these tokens](https://stripe.com/docs/issuing/controls/token-management) through Stripe.
///
/// For more details see <<https://stripe.com/docs/api/issuing/tokens/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingToken {
    /// Card associated with this token.
    pub card: stripe_types::Expandable<stripe_shared::IssuingCard>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The hashed ID derived from the device ID from the card network associated with the token
    pub device_fingerprint: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingTokenId,
    /// The last four digits of the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The token service provider / card network associated with the token.
    pub network: IssuingTokenNetwork,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_data: Option<stripe_shared::IssuingNetworkTokenNetworkData>,
    /// Time at which the token was last updated by the card network.
    /// Measured in seconds since the Unix epoch.
    pub network_updated_at: stripe_types::Timestamp,
    /// The usage state of the token.
    pub status: stripe_shared::IssuingTokenStatus,
    /// The digital wallet for this token, if one was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_provider: Option<IssuingTokenWalletProvider>,
}
/// The token service provider / card network associated with the token.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingTokenNetwork {
    Mastercard,
    Visa,
}
impl IssuingTokenNetwork {
    pub fn as_str(self) -> &'static str {
        use IssuingTokenNetwork::*;
        match self {
            Mastercard => "mastercard",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for IssuingTokenNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTokenNetwork::*;
        match s {
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingTokenNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTokenNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingTokenNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingTokenNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingTokenNetwork"))
    }
}
/// The digital wallet for this token, if one was used.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingTokenWalletProvider {
    ApplePay,
    GooglePay,
    SamsungPay,
}
impl IssuingTokenWalletProvider {
    pub fn as_str(self) -> &'static str {
        use IssuingTokenWalletProvider::*;
        match self {
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            SamsungPay => "samsung_pay",
        }
    }
}

impl std::str::FromStr for IssuingTokenWalletProvider {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTokenWalletProvider::*;
        match s {
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "samsung_pay" => Ok(SamsungPay),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingTokenWalletProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTokenWalletProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingTokenWalletProvider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingTokenWalletProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingTokenWalletProvider"))
    }
}
impl stripe_types::Object for IssuingToken {
    type Id = stripe_shared::IssuingTokenId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(IssuingTokenId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingTokenStatus {
    Active,
    Deleted,
    Requested,
    Suspended,
}
impl IssuingTokenStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingTokenStatus::*;
        match self {
            Active => "active",
            Deleted => "deleted",
            Requested => "requested",
            Suspended => "suspended",
        }
    }
}

impl std::str::FromStr for IssuingTokenStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTokenStatus::*;
        match s {
            "active" => Ok(Active),
            "deleted" => Ok(Deleted),
            "requested" => Ok(Requested),
            "suspended" => Ok(Suspended),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingTokenStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingTokenStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingTokenStatus"))
    }
}
