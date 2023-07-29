#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Verification {
    /// Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`.
    pub status: VerificationStatus,
    /// Verified address.
    pub verified_address: Option<String>,
    /// Verified name.
    pub verified_name: Option<String>,
}
/// Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VerificationStatus {
    Pending,
    Unavailable,
    Unverified,
    Verified,
}

impl VerificationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Unavailable => "unavailable",
            Self::Unverified => "unverified",
            Self::Verified => "verified",
        }
    }
}

impl std::str::FromStr for VerificationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(Self::Pending),
            "unavailable" => Ok(Self::Unavailable),
            "unverified" => Ok(Self::Unverified),
            "verified" => Ok(Self::Verified),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for VerificationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for VerificationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerificationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for VerificationStatus"))
    }
}
