#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Blik {
    /// Date at which the mandate expires.
pub expires_after: Option<stripe_types::Timestamp>,
#[serde(skip_serializing_if = "Option::is_none")]
pub off_session: Option<stripe_types::payment_intent::payment_method_options::blik_mandate_options_off_session_details::BlikMandateOptionsOffSessionDetails>,
    /// Type of the mandate.
#[serde(rename = "type")]
pub type_: Option<BlikType>,

}
/// Type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlikType {
    OffSession,
    OnSession,
}

impl BlikType {
    pub fn as_str(self) -> &'static str {
        use BlikType::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for BlikType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BlikType::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BlikType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BlikType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BlikType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BlikType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for BlikType"))
    }
}
