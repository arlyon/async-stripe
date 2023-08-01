#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Closed {
    /// The array that contains reasons for a FinancialAccount closure.
    pub reasons: Vec<ClosedReasons>,
}
/// The array that contains reasons for a FinancialAccount closure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClosedReasons {
    AccountRejected,
    ClosedByPlatform,
    Other,
}

impl ClosedReasons {
    pub fn as_str(self) -> &'static str {
        use ClosedReasons::*;
        match self {
            AccountRejected => "account_rejected",
            ClosedByPlatform => "closed_by_platform",
            Other => "other",
        }
    }
}

impl std::str::FromStr for ClosedReasons {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ClosedReasons::*;
        match s {
            "account_rejected" => Ok(AccountRejected),
            "closed_by_platform" => Ok(ClosedByPlatform),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ClosedReasons {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ClosedReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ClosedReasons {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ClosedReasons {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for ClosedReasons"))
    }
}
