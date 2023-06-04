#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BacsDebit {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The status of the mandate on the Bacs network.
///
/// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
