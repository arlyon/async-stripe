/// Toggle settings for enabling/disabling a feature
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryFinancialAccountsResourceToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
    /// Whether the Feature is operational.
    pub status: TreasuryFinancialAccountsResourceToggleSettingsStatus,
    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details:
        Vec<stripe_treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
}
/// Whether the Feature is operational.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}
impl TreasuryFinancialAccountsResourceToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceToggleSettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
            Restricted => "restricted",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceToggleSettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            "restricted" => Ok(Restricted),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TreasuryFinancialAccountsResourceToggleSettingsStatus",
            )
        })
    }
}
