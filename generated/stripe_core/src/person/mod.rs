/// This is an object representing a person associated with a Stripe account.
///
/// A platform cannot access a Standard or Express account's persons after the account starts onboarding, such as after generating an account link for the account.
/// See the [Standard onboarding](https://stripe.com/docs/connect/standard-accounts) or [Express onboarding documentation](https://stripe.com/docs/connect/express-accounts) for information about platform pre-filling and account onboarding steps.
///
/// Related guide: [Handling Identity Verification with the API](https://stripe.com/docs/connect/identity-verification-api#person-information).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Person {
    /// The account the person is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types::address::Address>,
    /// The Kana variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<stripe_core::person::japan_address::JapanAddress>,
    /// The Kanji variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<stripe_core::person::japan_address::JapanAddress>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<stripe_core::person::date_of_birth::DateOfBirth>,
    /// The person's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The person's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The Kana variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,
    /// The Kanji variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,
    /// A list of alternate names or aliases that the person is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,
    /// Information about the upcoming new requirements for this person, including what information needs to be collected, and by when.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<stripe_core::person::future_requirements::FutureRequirements>,
    /// The person's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_core::person::PersonId,
    /// Whether the person's `id_number` was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_provided: Option<bool>,
    /// Whether the person's `id_number_secondary` was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary_provided: Option<bool>,
    /// The person's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The Kana variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,
    /// The Kanji variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,
    /// The person's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<stripe_types::Metadata>,
    /// The country where the person is a national.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: PersonObject,
    /// The person's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<PersonPoliticalExposure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<stripe_types::address::Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<stripe_core::person::relationship::Relationship>,
    /// Information about the requirements for this person, including what information needs to be collected, and by when.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<stripe_core::person::requirements::Requirements>,
    /// Whether the last four digits of the person's Social Security number have been provided (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<stripe_core::person::verification::Verification>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Person {
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
pub enum PersonObject {
    Person,
}

impl PersonObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Person => "person",
        }
    }
}

impl AsRef<str> for PersonObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PersonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PersonPoliticalExposure {
    Existing,
    None,
}

impl PersonPoliticalExposure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Existing => "existing",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PersonPoliticalExposure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PersonPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for Person {
    type Id = stripe_core::person::PersonId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(PersonId, "person_");
pub mod deleted;
pub mod requests;
pub use deleted::DeletedPerson;
pub mod date_of_birth;
pub use date_of_birth::DateOfBirth;
pub mod japan_address;
pub use japan_address::JapanAddress;
pub mod verification;
pub use verification::Verification;
pub mod verification_document;
pub use verification_document::VerificationDocument;
pub mod future_requirements;
pub use future_requirements::FutureRequirements;
pub mod relationship;
pub use relationship::Relationship;
pub mod requirements;
pub use requirements::Requirements;
