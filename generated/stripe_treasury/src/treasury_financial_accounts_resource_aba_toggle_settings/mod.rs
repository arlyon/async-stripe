/// Toggle settings for enabling/disabling the ABA address feature.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryFinancialAccountsResourceAbaToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
    /// Whether the Feature is operational.
    pub status: TreasuryFinancialAccountsResourceAbaToggleSettingsStatus,
    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details: Vec<stripe_treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
}
/// Whether the Feature is operational.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

impl TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
            Restricted => "restricted",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            "restricted" => Ok(Restricted),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus"))
    }
}
