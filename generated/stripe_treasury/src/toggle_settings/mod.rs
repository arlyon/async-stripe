/// Toggle settings for enabling/disabling a feature.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
    /// Whether the Feature is operational.
    pub status: ToggleSettingsStatus,
    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details: Vec<stripe_treasury::status_details::StatusDetails>,
}
/// Whether the Feature is operational.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

impl ToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        use ToggleSettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
            Restricted => "restricted",
        }
    }
}

impl std::str::FromStr for ToggleSettingsStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ToggleSettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            "restricted" => Ok(Restricted),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ToggleSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ToggleSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ToggleSettingsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ToggleSettingsStatus"))
    }
}
