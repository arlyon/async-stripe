/// Toggle settings for enabling/disabling an ACH specific feature.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AchToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
pub requested: bool,
    /// Whether the Feature is operational.
pub status: AchToggleSettingsStatus,
    /// Additional details; includes at least one entry when the status is not `active`.
pub status_details: Vec<stripe_treasury::treasury::financial_account::toggle_settings::status_details::StatusDetails>,

}
/// Whether the Feature is operational.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AchToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

impl AchToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Pending => "pending",
            Self::Restricted => "restricted",
        }
    }
}

impl std::str::FromStr for AchToggleSettingsStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "pending" => Ok(Self::Pending),
            "restricted" => Ok(Self::Restricted),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AchToggleSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AchToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AchToggleSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AchToggleSettingsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AchToggleSettingsStatus"))
    }
}
