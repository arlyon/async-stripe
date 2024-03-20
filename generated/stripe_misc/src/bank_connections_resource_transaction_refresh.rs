#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BankConnectionsResourceTransactionRefresh {
    /// Unique identifier for the object.
    pub id: stripe_misc::BankConnectionsResourceTransactionRefreshId,
    /// The time at which the last refresh attempt was initiated. Measured in seconds since the Unix epoch.
    pub last_attempted_at: stripe_types::Timestamp,
    /// Time at which the next transaction refresh can be initiated.
    /// This value will be `null` when `status` is `pending`.
    /// Measured in seconds since the Unix epoch.
    pub next_refresh_available_at: Option<stripe_types::Timestamp>,
    /// The status of the last refresh attempt.
    pub status: BankConnectionsResourceTransactionRefreshStatus,
}
/// The status of the last refresh attempt.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceTransactionRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}
impl BankConnectionsResourceTransactionRefreshStatus {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceTransactionRefreshStatus::*;
        match self {
            Failed => "failed",
            Pending => "pending",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceTransactionRefreshStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceTransactionRefreshStatus::*;
        match s {
            "failed" => Ok(Failed),
            "pending" => Ok(Pending),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceTransactionRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceTransactionRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceTransactionRefreshStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceTransactionRefreshStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for BankConnectionsResourceTransactionRefreshStatus",
            )
        })
    }
}
impl stripe_types::Object for BankConnectionsResourceTransactionRefresh {
    type Id = stripe_misc::BankConnectionsResourceTransactionRefreshId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(BankConnectionsResourceTransactionRefreshId);
