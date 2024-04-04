#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BankConnectionsResourceBalanceRefresh {
    /// The time at which the last refresh attempt was initiated. Measured in seconds since the Unix epoch.
    pub last_attempted_at: stripe_types::Timestamp,
    /// Time at which the next balance refresh can be initiated.
    /// This value will be `null` when `status` is `pending`.
    /// Measured in seconds since the Unix epoch.
    pub next_refresh_available_at: Option<stripe_types::Timestamp>,
    /// The status of the last refresh attempt.
    pub status: BankConnectionsResourceBalanceRefreshStatus,
}
/// The status of the last refresh attempt.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceBalanceRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}
impl BankConnectionsResourceBalanceRefreshStatus {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceBalanceRefreshStatus::*;
        match self {
            Failed => "failed",
            Pending => "pending",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceBalanceRefreshStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceBalanceRefreshStatus::*;
        match s {
            "failed" => Ok(Failed),
            "pending" => Ok(Pending),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceBalanceRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceBalanceRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceBalanceRefreshStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceBalanceRefreshStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for BankConnectionsResourceBalanceRefreshStatus",
            )
        })
    }
}
