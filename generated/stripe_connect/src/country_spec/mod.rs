/// Stripe needs to collect certain pieces of information about each account
/// created.
///
/// These requirements can differ depending on the account's country.
/// The Country Specs API makes these rules available to your integration.  You can also view the information from this API call as [an online guide](/docs/connect/required-verification-information).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CountrySpec {
    /// The default currency for this country.
    ///
    /// This applies to both payment methods and bank accounts.
    pub default_currency: stripe_types::Currency,
    /// Unique identifier for the object.
    ///
    /// Represented as the ISO country code for this country.
    pub id: stripe_connect::country_spec::CountrySpecId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CountrySpecObject,
    /// Currencies that can be accepted in the specific country (for transfers).
    pub supported_bank_account_currencies: std::collections::HashMap<String, Vec<String>>,
    /// Currencies that can be accepted in the specified country (for payments).
    pub supported_payment_currencies: Vec<String>,
    /// Payment methods available in the specified country.
    ///
    /// You may need to enable some payment methods (e.g., [ACH](https://stripe.com/docs/ach)) on your account before they appear in this list.
    /// The `stripe` payment method refers to [charging through your platform](https://stripe.com/docs/connect/destination-charges).
    pub supported_payment_methods: Vec<String>,
    /// Countries that can accept transfers from the specified country.
    pub supported_transfer_countries: Vec<String>,
    pub verification_fields: stripe_connect::country_spec::verification_fields::VerificationFields,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CountrySpec {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CountrySpecObject {
    CountrySpec,
}

impl CountrySpecObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CountrySpec => "country_spec",
        }
    }
}

impl AsRef<str> for CountrySpecObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CountrySpecObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for CountrySpec {
    type Id = stripe_connect::country_spec::CountrySpecId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(CountrySpecId);
pub mod requests;
pub mod verification_fields;
pub use verification_fields::VerificationFields;
