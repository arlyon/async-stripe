#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BacsDebit {
    /// The status of the mandate on the Bacs network.
    ///
    /// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
    pub network_status: BacsDebitNetworkStatus,
    /// The unique reference identifying the mandate on the Bacs network.
    pub reference: String,
    /// The URL that will contain the mandate that the customer has signed.
    pub url: String,
}
/// The status of the mandate on the Bacs network.
///
/// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BacsDebitNetworkStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}

impl BacsDebitNetworkStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Pending => "pending",
            Self::Refused => "refused",
            Self::Revoked => "revoked",
        }
    }
}

impl std::str::FromStr for BacsDebitNetworkStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "accepted" => Ok(Self::Accepted),
            "pending" => Ok(Self::Pending),
            "refused" => Ok(Self::Refused),
            "revoked" => Ok(Self::Revoked),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for BacsDebitNetworkStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BacsDebitNetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BacsDebitNetworkStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BacsDebitNetworkStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BacsDebitNetworkStatus"))
    }
}
