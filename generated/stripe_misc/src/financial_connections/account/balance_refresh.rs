#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BalanceRefresh {
    /// The time at which the last refresh attempt was initiated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub last_attempted_at: stripe_types::Timestamp,
    /// The status of the last refresh attempt.
    pub status: BalanceRefreshStatus,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BalanceRefresh {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The status of the last refresh attempt.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum BalanceRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}

impl BalanceRefreshStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Pending => "pending",
            Self::Succeeded => "succeeded",
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
