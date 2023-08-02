/// Result from a selfie check.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Selfie {
    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the identity document used in this check.
    pub document: Option<String>,
    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
    pub error: Option<stripe_misc::selfie_check_error::SelfieCheckError>,
    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the selfie used in this check.
    pub selfie: Option<String>,
    /// Status of this `selfie` check.
    pub status: SelfieStatus,
}
/// Status of this `selfie` check.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SelfieStatus {
    Unverified,
    Verified,
}

impl SelfieStatus {
    pub fn as_str(self) -> &'static str {
        use SelfieStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for SelfieStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SelfieStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SelfieStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SelfieStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SelfieStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SelfieStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SelfieStatus"))
    }
}
