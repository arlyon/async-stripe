#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MandateBlik {
    /// Date at which the mandate expires.
    pub expires_after: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<stripe_types::MandateOptionsOffSessionDetailsBlik>,
    /// Type of the mandate.
    #[serde(rename = "type")]
    pub type_: Option<MandateBlikType>,
}
/// Type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateBlikType {
    OffSession,
    OnSession,
}

impl MandateBlikType {
    pub fn as_str(self) -> &'static str {
        use MandateBlikType::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for MandateBlikType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateBlikType::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for MandateBlikType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateBlikType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateBlikType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for MandateBlikType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateBlikType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for MandateBlikType"))
    }
}
