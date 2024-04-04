#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomerBalanceCustomerBalanceSettings {
    /// The configuration for how funds that land in the customer cash balance are reconciled.
    pub reconciliation_mode: CustomerBalanceCustomerBalanceSettingsReconciliationMode,
    /// A flag to indicate if reconciliation mode returned is the user's default or is specific to this customer cash balance.
    pub using_merchant_default: bool,
}
/// The configuration for how funds that land in the customer cash balance are reconciled.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
}
impl CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        use CustomerBalanceCustomerBalanceSettingsReconciliationMode::*;
        match self {
            Automatic => "automatic",
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerBalanceCustomerBalanceSettingsReconciliationMode::*;
        match s {
            "automatic" => Ok(Automatic),
            "manual" => Ok(Manual),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CustomerBalanceCustomerBalanceSettingsReconciliationMode",
            )
        })
    }
}
