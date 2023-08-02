#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BalanceRefresh {
    /// The time at which the last refresh attempt was initiated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub last_attempted_at: stripe_types::Timestamp,
    /// The status of the last refresh attempt.
    pub status: BalanceRefreshStatus,
}
/// The status of the last refresh attempt.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BalanceRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}

impl BalanceRefreshStatus {
    pub fn as_str(self) -> &'static str {
        use BalanceRefreshStatus::*;
        match self {
            Failed => "failed",
            Pending => "pending",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for BalanceRefreshStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BalanceRefreshStatus::*;
        match s {
            "failed" => Ok(Failed),
            "pending" => Ok(Pending),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BalanceRefreshStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BalanceRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BalanceRefreshStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BalanceRefreshStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BalanceRefreshStatus"))
    }
}
