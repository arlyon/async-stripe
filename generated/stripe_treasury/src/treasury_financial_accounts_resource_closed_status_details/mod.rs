#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryFinancialAccountsResourceClosedStatusDetails {
    /// The array that contains reasons for a FinancialAccount closure.
    pub reasons: Vec<TreasuryFinancialAccountsResourceClosedStatusDetailsReasons>,
}
/// The array that contains reasons for a FinancialAccount closure.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    AccountRejected,
    ClosedByPlatform,
    Other,
}

impl TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceClosedStatusDetailsReasons::*;
        match self {
            AccountRejected => "account_rejected",
            ClosedByPlatform => "closed_by_platform",
            Other => "other",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceClosedStatusDetailsReasons::*;
        match s {
            "account_rejected" => Ok(AccountRejected),
            "closed_by_platform" => Ok(ClosedByPlatform),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons"))
    }
}
