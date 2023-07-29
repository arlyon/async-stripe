#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BalanceSettings {
    /// The configuration for how funds that land in the customer cash balance are reconciled.
    pub reconciliation_mode: BalanceSettingsReconciliationMode,
}
/// The configuration for how funds that land in the customer cash balance are reconciled.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BalanceSettingsReconciliationMode {
    Automatic,
    Manual,
}

impl BalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr for BalanceSettingsReconciliationMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for BalanceSettingsReconciliationMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BalanceSettingsReconciliationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BalanceSettingsReconciliationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BalanceSettingsReconciliationMode")
        })
    }
}
