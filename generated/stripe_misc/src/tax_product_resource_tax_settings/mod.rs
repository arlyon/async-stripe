/// You can use Tax `Settings` to manage configurations used by Stripe Tax calculations.
///
/// Related guide: [Using the Settings API](https://stripe.com/docs/tax/settings-api).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceTaxSettings {
    pub defaults: stripe_misc::TaxProductResourceTaxSettingsDefaults,
    /// The place where your business is located.
    pub head_office: Option<stripe_misc::TaxProductResourceTaxSettingsHeadOffice>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The `active` status indicates you have all required settings to calculate tax.
    ///
    /// A status can transition out of `active` when new required settings are introduced.
    pub status: TaxProductResourceTaxSettingsStatus,
    pub status_details: stripe_misc::TaxProductResourceTaxSettingsStatusDetails,
}
/// The `active` status indicates you have all required settings to calculate tax.
///
/// A status can transition out of `active` when new required settings are introduced.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceTaxSettingsStatus {
    Active,
    Pending,
}

impl TaxProductResourceTaxSettingsStatus {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceTaxSettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxSettingsStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxSettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxProductResourceTaxSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceTaxSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceTaxSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxSettingsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TaxProductResourceTaxSettingsStatus")
        })
    }
}
#[cfg(feature = "tax_product_resource_tax_settings")]
mod requests;
#[cfg(feature = "tax_product_resource_tax_settings")]
pub use requests::*;
