#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EphemeralKey {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Time at which the key will expire.
    ///
    /// Measured in seconds since the Unix epoch.
    pub expires: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::ephemeral_key::EphemeralKeyId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: EphemeralKeyObject,
    /// The key's secret.
    ///
    /// You can use this value to make authorized requests to the Stripe API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EphemeralKeyObject {
    EphemeralKey,
}

impl EphemeralKeyObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralKey => "ephemeral_key",
        }
    }
}

impl std::str::FromStr for EphemeralKeyObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ephemeral_key" => Ok(Self::EphemeralKey),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for EphemeralKeyObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for EphemeralKeyObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for EphemeralKeyObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EphemeralKeyObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for EphemeralKeyObject"))
    }
}
impl stripe_types::Object for EphemeralKey {
    type Id = stripe_misc::ephemeral_key::EphemeralKeyId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(EphemeralKeyId, "ephkey_");
