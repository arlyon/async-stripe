/// A Tax `Registration` lets us know that your business is registered to collect tax on payments within a region, enabling you to [automatically collect tax](https://stripe.com/docs/tax).
///
/// Stripe doesn't register on your behalf with the relevant authorities when you create a Tax `Registration` object.
///
/// For more information on how to register to collect tax, see [our guide](https://stripe.com/docs/tax/registering).  Related guide: [Using the Registrations API](https://stripe.com/docs/tax/registrations-api).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductRegistrationsResourceTaxRegistration {
    /// Time at which the registration becomes active.
    ///
    /// Measured in seconds since the Unix epoch.
    pub active_from: stripe_types::Timestamp,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    pub country_options: stripe_misc::TaxProductRegistrationsResourceCountryOptions,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// If set, the registration stops being active at this time.
    ///
    /// If not set, the registration will be active indefinitely.
    /// Measured in seconds since the Unix epoch.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_misc::tax_product_registrations_resource_tax_registration::TaxRegistrationId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The status of the registration.
    ///
    /// This field is present for convenience and can be deduced from `active_from` and `expires_at`.
    pub status: TaxProductRegistrationsResourceTaxRegistrationStatus,
}
/// The status of the registration.
///
/// This field is present for convenience and can be deduced from `active_from` and `expires_at`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceTaxRegistrationStatus {
    Active,
    Expired,
    Scheduled,
}

impl TaxProductRegistrationsResourceTaxRegistrationStatus {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceTaxRegistrationStatus::*;
        match self {
            Active => "active",
            Expired => "expired",
            Scheduled => "scheduled",
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceTaxRegistrationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceTaxRegistrationStatus::*;
        match s {
            "active" => Ok(Active),
            "expired" => Ok(Expired),
            "scheduled" => Ok(Scheduled),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxProductRegistrationsResourceTaxRegistrationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductRegistrationsResourceTaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceTaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductRegistrationsResourceTaxRegistrationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductRegistrationsResourceTaxRegistrationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductRegistrationsResourceTaxRegistrationStatus",
            )
        })
    }
}
impl stripe_types::Object for TaxProductRegistrationsResourceTaxRegistration {
    type Id = stripe_misc::tax_product_registrations_resource_tax_registration::TaxRegistrationId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(TaxRegistrationId);
#[cfg(feature = "tax_product_registrations_resource_tax_registration")]
mod requests;
#[cfg(feature = "tax_product_registrations_resource_tax_registration")]
pub use requests::*;
