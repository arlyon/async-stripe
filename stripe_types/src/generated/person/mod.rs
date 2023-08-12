/// This is an object representing a person associated with a Stripe account.
///
/// A platform cannot access a Standard or Express account's persons after the account starts onboarding, such as after generating an account link for the account.
/// See the [Standard onboarding](https://stripe.com/docs/connect/standard-accounts) or [Express onboarding documentation](https://stripe.com/docs/connect/express-accounts) for information about platform prefilling and account onboarding steps.
///
/// Related guide: [Handling identity verification with the API](https://stripe.com/docs/connect/identity-verification-api#person-information).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Person {
    /// The account the person is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types::Address>,
    /// The Kana variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<stripe_types::LegalEntityJapanAddress>,
    /// The Kanji variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<stripe_types::LegalEntityJapanAddress>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<stripe_types::LegalEntityDob>,
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
    pub future_requirements: Option<stripe_types::PersonFutureRequirements>,
    /// The person's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_types::person::PersonId,
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
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The country where the person is a national.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// The person's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<PersonPoliticalExposure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<stripe_types::Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<stripe_types::PersonRelationship>,
    /// Information about the requirements for this person, including what information needs to be collected, and by when.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<stripe_types::PersonRequirements>,
    /// Whether the last four digits of the person's Social Security number have been provided (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<stripe_types::LegalEntityPersonVerification>,
}
/// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PersonPoliticalExposure {
    Existing,
    None,
}

impl PersonPoliticalExposure {
    pub fn as_str(self) -> &'static str {
        use PersonPoliticalExposure::*;
        match self {
            Existing => "existing",
            None => "none",
        }
    }
}

impl std::str::FromStr for PersonPoliticalExposure {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PersonPoliticalExposure::*;
        match s {
            "existing" => Ok(Existing),
            "none" => Ok(None),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PersonPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PersonPoliticalExposure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PersonPoliticalExposure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PersonPoliticalExposure"))
    }
}
impl stripe_types::Object for Person {
    type Id = stripe_types::person::PersonId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(PersonId, "person_");
