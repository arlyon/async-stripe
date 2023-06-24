/// Toggle settings for enabling/disabling a feature.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
pub requested: bool,
    /// Whether the Feature is operational.
pub status: ToggleSettingsStatus,
    /// Additional details; includes at least one entry when the status is not `active`.
pub status_details: Vec<stripe_treasury::treasury::financial_account::toggle_settings::status_details::StatusDetails>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ToggleSettings {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
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
        match self {
            Self::Active => "active",
            Self::Pending => "pending",
            Self::Restricted => "restricted",
        }
    }
}

impl std::str::FromStr for ToggleSettingsStatus {
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ToggleSettingsStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ToggleSettingsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ToggleSettingsStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ToggleSettingsStatus::from_str(s)?);
        Ok(())
    }
}
pub mod status_details;
pub use status_details::StatusDetails;
