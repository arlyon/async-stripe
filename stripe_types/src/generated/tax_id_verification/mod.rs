#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxIdVerification {
    /// Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`.
    pub status: TaxIdVerificationStatus,
    /// Verified address.
    pub verified_address: Option<String>,
    /// Verified name.
    pub verified_name: Option<String>,
}
/// Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxIdVerificationStatus {
    Pending,
    Unavailable,
    Unverified,
    Verified,
}

impl TaxIdVerificationStatus {
    pub fn as_str(self) -> &'static str {
        use TaxIdVerificationStatus::*;
        match self {
            Pending => "pending",
            Unavailable => "unavailable",
            Unverified => "unverified",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for TaxIdVerificationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxIdVerificationStatus::*;
        match s {
            "pending" => Ok(Pending),
            "unavailable" => Ok(Unavailable),
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxIdVerificationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxIdVerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxIdVerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxIdVerificationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxIdVerificationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxIdVerificationStatus"))
    }
}
