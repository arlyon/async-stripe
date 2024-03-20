/// You can use Tax `Settings` to manage configurations used by Stripe Tax calculations.
///
/// Related guide: [Using the Settings API](https://stripe.com/docs/tax/settings-api)
///
/// For more details see <<https://stripe.com/docs/api/tax/settings/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxSettings {
    pub defaults: stripe_misc::TaxProductResourceTaxSettingsDefaults,
    /// The place where your business is located.
    pub head_office: Option<stripe_misc::TaxProductResourceTaxSettingsHeadOffice>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The `active` status indicates you have all required settings to calculate tax.
    /// A status can transition out of `active` when new required settings are introduced.
    pub status: TaxSettingsStatus,
    pub status_details: stripe_misc::TaxProductResourceTaxSettingsStatusDetails,
}
/// The `active` status indicates you have all required settings to calculate tax.
/// A status can transition out of `active` when new required settings are introduced.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxSettingsStatus {
    Active,
    Pending,
}
impl TaxSettingsStatus {
    pub fn as_str(self) -> &'static str {
        use TaxSettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for TaxSettingsStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxSettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TaxSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxSettingsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxSettingsStatus"))
    }
}
