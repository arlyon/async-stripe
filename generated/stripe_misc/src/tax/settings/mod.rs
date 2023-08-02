/// You can use Tax `Settings` to manage configurations used by Stripe Tax calculations.
///
/// Related guide: [Using the Settings API](https://stripe.com/docs/tax/settings-api).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub defaults: stripe_misc::defaults::Defaults,
    /// The place where your business is located.
    pub head_office: Option<stripe_misc::head_office::HeadOffice>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SettingsObject,
    /// The `active` status indicates you have all required settings to calculate tax.
    ///
    /// A status can transition out of `active` when new required settings are introduced.
    pub status: SettingsStatus,
    pub status_details: stripe_misc::status_details::StatusDetails,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SettingsObject {
    TaxSettings,
}

impl SettingsObject {
    pub fn as_str(self) -> &'static str {
        use SettingsObject::*;
        match self {
            TaxSettings => "tax.settings",
        }
    }
}

impl std::str::FromStr for SettingsObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SettingsObject::*;
        match s {
            "tax.settings" => Ok(TaxSettings),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SettingsObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SettingsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SettingsObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SettingsObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SettingsObject"))
    }
}
/// The `active` status indicates you have all required settings to calculate tax.
///
/// A status can transition out of `active` when new required settings are introduced.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SettingsStatus {
    Active,
    Pending,
}

impl SettingsStatus {
    pub fn as_str(self) -> &'static str {
        use SettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for SettingsStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SettingsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SettingsStatus"))
    }
}
pub mod requests;
