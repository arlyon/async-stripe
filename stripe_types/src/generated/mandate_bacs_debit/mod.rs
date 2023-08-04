#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MandateBacsDebit {
    /// The status of the mandate on the Bacs network.
    ///
    /// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
    pub network_status: MandateBacsDebitNetworkStatus,
    /// The unique reference identifying the mandate on the Bacs network.
    pub reference: String,
    /// The URL that will contain the mandate that the customer has signed.
    pub url: String,
}
/// The status of the mandate on the Bacs network.
///
/// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateBacsDebitNetworkStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}

impl MandateBacsDebitNetworkStatus {
    pub fn as_str(self) -> &'static str {
        use MandateBacsDebitNetworkStatus::*;
        match self {
            Accepted => "accepted",
            Pending => "pending",
            Refused => "refused",
            Revoked => "revoked",
        }
    }
}

impl std::str::FromStr for MandateBacsDebitNetworkStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateBacsDebitNetworkStatus::*;
        match s {
            "accepted" => Ok(Accepted),
            "pending" => Ok(Pending),
            "refused" => Ok(Refused),
            "revoked" => Ok(Revoked),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for MandateBacsDebitNetworkStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateBacsDebitNetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateBacsDebitNetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for MandateBacsDebitNetworkStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateBacsDebitNetworkStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for MandateBacsDebitNetworkStatus")
        })
    }
}
