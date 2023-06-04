/// Toggle settings for enabling/disabling a feature.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
    /// Whether the Feature is operational.
    pub status: ToggleSettingsStatus,
    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details:
        Vec<crate::treasury::financial_account::toggle_settings::status_details::StatusDetails>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ToggleSettings {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Whether the Feature is operational.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

impl ToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Pending => "pending",
            Self::Restricted => "restricted",
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
pub mod status_details;
pub use status_details::StatusDetails;
