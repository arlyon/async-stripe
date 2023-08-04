/// Result from a selfie check.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GelatoSelfieReport {
    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the identity document used in this check.
    pub document: Option<String>,
    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoSelfieReportError>,
    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the selfie used in this check.
    pub selfie: Option<String>,
    /// Status of this `selfie` check.
    pub status: GelatoSelfieReportStatus,
}
/// Status of this `selfie` check.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoSelfieReportStatus {
    Unverified,
    Verified,
}

impl GelatoSelfieReportStatus {
    pub fn as_str(self) -> &'static str {
        use GelatoSelfieReportStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for GelatoSelfieReportStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoSelfieReportStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for GelatoSelfieReportStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoSelfieReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoSelfieReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoSelfieReportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoSelfieReportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GelatoSelfieReportStatus"))
    }
}
