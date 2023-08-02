#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OwnershipRefresh {
    /// The time at which the last refresh attempt was initiated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub last_attempted_at: stripe_types::Timestamp,
    /// The status of the last refresh attempt.
    pub status: OwnershipRefreshStatus,
}
/// The status of the last refresh attempt.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OwnershipRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}

impl OwnershipRefreshStatus {
    pub fn as_str(self) -> &'static str {
        use OwnershipRefreshStatus::*;
        match self {
            Failed => "failed",
            Pending => "pending",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for OwnershipRefreshStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OwnershipRefreshStatus::*;
        match s {
            "failed" => Ok(Failed),
            "pending" => Ok(Pending),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for OwnershipRefreshStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OwnershipRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for OwnershipRefreshStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OwnershipRefreshStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for OwnershipRefreshStatus"))
    }
}
