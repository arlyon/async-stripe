/// This is an object representing a person associated with a Stripe account.
///
/// A platform can only access a subset of data in a person for an account where [account.controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`, which includes Standard and Express accounts, after creating an Account Link or Account Session to start Connect onboarding.
///
/// See the [Standard onboarding](/connect/standard-accounts) or [Express onboarding](/connect/express-accounts) documentation for information about prefilling information and account onboarding steps.
/// Learn more about [handling identity verification with the API](/connect/handling-api-verification#person-information).
///
/// For more details see <<https://stripe.com/docs/api/persons/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Person {
    /// The account the person is associated with.
    pub account: Option<String>,
    pub additional_tos_acceptances: Option<stripe_shared::PersonAdditionalTosAcceptances>,
    pub address: Option<stripe_shared::Address>,
    /// The Kana variation of the person's address (Japan only).
    pub address_kana: Option<stripe_shared::LegalEntityJapanAddress>,
    /// The Kanji variation of the person's address (Japan only).
    pub address_kanji: Option<stripe_shared::LegalEntityJapanAddress>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub dob: Option<stripe_shared::LegalEntityDob>,
    /// The person's email address.
    /// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
    pub email: Option<String>,
    /// The person's first name.
    /// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
    pub first_name: Option<String>,
    /// The Kana variation of the person's first name (Japan only).
    /// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
    pub first_name_kana: Option<String>,
    /// The Kanji variation of the person's first name (Japan only).
    /// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
    pub first_name_kanji: Option<String>,
    /// A list of alternate names or aliases that the person is known by.
    /// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
    pub full_name_aliases: Option<Vec<String>>,
    /// Information about the [upcoming new requirements for this person](https://docs.stripe.com/connect/custom-accounts/future-requirements), including what information needs to be collected, and by when.
    pub future_requirements: Option<stripe_shared::PersonFutureRequirements>,
    /// The person's gender.
    pub gender: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::PersonId,
    /// Whether the person's `id_number` was provided.
    /// True if either the full ID number was provided or if only the required part of the ID number was provided (ex.
    /// last four of an individual's SSN for the US indicated by `ssn_last_4_provided`).
    pub id_number_provided: Option<bool>,
    /// Whether the person's `id_number_secondary` was provided.
    pub id_number_secondary_provided: Option<bool>,
    /// The person's last name.
    /// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
    pub last_name: Option<String>,
    /// The Kana variation of the person's last name (Japan only).
    /// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
    pub last_name_kana: Option<String>,
    /// The Kanji variation of the person's last name (Japan only).
    /// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
    pub last_name_kanji: Option<String>,
    /// The person's maiden name.
    pub maiden_name: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The country where the person is a national.
    pub nationality: Option<String>,
    /// The person's phone number.
    pub phone: Option<String>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    pub political_exposure: Option<stripe_shared::PersonPoliticalExposure>,
    pub registered_address: Option<stripe_shared::Address>,
    pub relationship: Option<stripe_shared::PersonRelationship>,
    /// Information about the requirements for this person, including what information needs to be collected, and by when.
    pub requirements: Option<stripe_shared::PersonRequirements>,
    /// Whether the last four digits of the person's Social Security number have been provided (U.S. only).
    pub ssn_last_4_provided: Option<bool>,
    /// Demographic data related to the person.
    pub us_cfpb_data: Option<stripe_shared::PersonUsCfpbData>,
    pub verification: Option<stripe_shared::LegalEntityPersonVerification>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Person").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PersonBuilder {
    account: Option<Option<String>>,
    additional_tos_acceptances: Option<Option<stripe_shared::PersonAdditionalTosAcceptances>>,
    address: Option<Option<stripe_shared::Address>>,
    address_kana: Option<Option<stripe_shared::LegalEntityJapanAddress>>,
    address_kanji: Option<Option<stripe_shared::LegalEntityJapanAddress>>,
    created: Option<stripe_types::Timestamp>,
    dob: Option<Option<stripe_shared::LegalEntityDob>>,
    email: Option<Option<String>>,
    first_name: Option<Option<String>>,
    first_name_kana: Option<Option<String>>,
    first_name_kanji: Option<Option<String>>,
    full_name_aliases: Option<Option<Vec<String>>>,
    future_requirements: Option<Option<stripe_shared::PersonFutureRequirements>>,
    gender: Option<Option<String>>,
    id: Option<stripe_shared::PersonId>,
    id_number_provided: Option<Option<bool>>,
    id_number_secondary_provided: Option<Option<bool>>,
    last_name: Option<Option<String>>,
    last_name_kana: Option<Option<String>>,
    last_name_kanji: Option<Option<String>>,
    maiden_name: Option<Option<String>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    nationality: Option<Option<String>>,
    phone: Option<Option<String>>,
    political_exposure: Option<Option<stripe_shared::PersonPoliticalExposure>>,
    registered_address: Option<Option<stripe_shared::Address>>,
    relationship: Option<Option<stripe_shared::PersonRelationship>>,
    requirements: Option<Option<stripe_shared::PersonRequirements>>,
    ssn_last_4_provided: Option<Option<bool>>,
    us_cfpb_data: Option<Option<stripe_shared::PersonUsCfpbData>>,
    verification: Option<Option<stripe_shared::LegalEntityPersonVerification>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for Person {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Person>,
        builder: PersonBuilder,
    }

    impl Visitor for Place<Person> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PersonBuilder {
                    account: Deserialize::default(),
                    additional_tos_acceptances: Deserialize::default(),
                    address: Deserialize::default(),
                    address_kana: Deserialize::default(),
                    address_kanji: Deserialize::default(),
                    created: Deserialize::default(),
                    dob: Deserialize::default(),
                    email: Deserialize::default(),
                    first_name: Deserialize::default(),
                    first_name_kana: Deserialize::default(),
                    first_name_kanji: Deserialize::default(),
                    full_name_aliases: Deserialize::default(),
                    future_requirements: Deserialize::default(),
                    gender: Deserialize::default(),
                    id: Deserialize::default(),
                    id_number_provided: Deserialize::default(),
                    id_number_secondary_provided: Deserialize::default(),
                    last_name: Deserialize::default(),
                    last_name_kana: Deserialize::default(),
                    last_name_kanji: Deserialize::default(),
                    maiden_name: Deserialize::default(),
                    metadata: Deserialize::default(),
                    nationality: Deserialize::default(),
                    phone: Deserialize::default(),
                    political_exposure: Deserialize::default(),
                    registered_address: Deserialize::default(),
                    relationship: Deserialize::default(),
                    requirements: Deserialize::default(),
                    ssn_last_4_provided: Deserialize::default(),
                    us_cfpb_data: Deserialize::default(),
                    verification: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.builder.account),
                "additional_tos_acceptances" => {
                    Deserialize::begin(&mut self.builder.additional_tos_acceptances)
                }
                "address" => Deserialize::begin(&mut self.builder.address),
                "address_kana" => Deserialize::begin(&mut self.builder.address_kana),
                "address_kanji" => Deserialize::begin(&mut self.builder.address_kanji),
                "created" => Deserialize::begin(&mut self.builder.created),
                "dob" => Deserialize::begin(&mut self.builder.dob),
                "email" => Deserialize::begin(&mut self.builder.email),
                "first_name" => Deserialize::begin(&mut self.builder.first_name),
                "first_name_kana" => Deserialize::begin(&mut self.builder.first_name_kana),
                "first_name_kanji" => Deserialize::begin(&mut self.builder.first_name_kanji),
                "full_name_aliases" => Deserialize::begin(&mut self.builder.full_name_aliases),
                "future_requirements" => Deserialize::begin(&mut self.builder.future_requirements),
                "gender" => Deserialize::begin(&mut self.builder.gender),
                "id" => Deserialize::begin(&mut self.builder.id),
                "id_number_provided" => Deserialize::begin(&mut self.builder.id_number_provided),
                "id_number_secondary_provided" => {
                    Deserialize::begin(&mut self.builder.id_number_secondary_provided)
                }
                "last_name" => Deserialize::begin(&mut self.builder.last_name),
                "last_name_kana" => Deserialize::begin(&mut self.builder.last_name_kana),
                "last_name_kanji" => Deserialize::begin(&mut self.builder.last_name_kanji),
                "maiden_name" => Deserialize::begin(&mut self.builder.maiden_name),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "nationality" => Deserialize::begin(&mut self.builder.nationality),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                "political_exposure" => Deserialize::begin(&mut self.builder.political_exposure),
                "registered_address" => Deserialize::begin(&mut self.builder.registered_address),
                "relationship" => Deserialize::begin(&mut self.builder.relationship),
                "requirements" => Deserialize::begin(&mut self.builder.requirements),
                "ssn_last_4_provided" => Deserialize::begin(&mut self.builder.ssn_last_4_provided),
                "us_cfpb_data" => Deserialize::begin(&mut self.builder.us_cfpb_data),
                "verification" => Deserialize::begin(&mut self.builder.verification),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account),
                Some(additional_tos_acceptances),
                Some(address),
                Some(address_kana),
                Some(address_kanji),
                Some(created),
                Some(dob),
                Some(email),
                Some(first_name),
                Some(first_name_kana),
                Some(first_name_kanji),
                Some(full_name_aliases),
                Some(future_requirements),
                Some(gender),
                Some(id),
                Some(id_number_provided),
                Some(id_number_secondary_provided),
                Some(last_name),
                Some(last_name_kana),
                Some(last_name_kanji),
                Some(maiden_name),
                Some(metadata),
                Some(nationality),
                Some(phone),
                Some(political_exposure),
                Some(registered_address),
                Some(relationship),
                Some(requirements),
                Some(ssn_last_4_provided),
                Some(us_cfpb_data),
                Some(verification),
            ) = (
                self.builder.account.take(),
                self.builder.additional_tos_acceptances.take(),
                self.builder.address.take(),
                self.builder.address_kana.take(),
                self.builder.address_kanji.take(),
                self.builder.created,
                self.builder.dob,
                self.builder.email.take(),
                self.builder.first_name.take(),
                self.builder.first_name_kana.take(),
                self.builder.first_name_kanji.take(),
                self.builder.full_name_aliases.take(),
                self.builder.future_requirements.take(),
                self.builder.gender.take(),
                self.builder.id.take(),
                self.builder.id_number_provided,
                self.builder.id_number_secondary_provided,
                self.builder.last_name.take(),
                self.builder.last_name_kana.take(),
                self.builder.last_name_kanji.take(),
                self.builder.maiden_name.take(),
                self.builder.metadata.take(),
                self.builder.nationality.take(),
                self.builder.phone.take(),
                self.builder.political_exposure.take(),
                self.builder.registered_address.take(),
                self.builder.relationship.take(),
                self.builder.requirements.take(),
                self.builder.ssn_last_4_provided,
                self.builder.us_cfpb_data.take(),
                self.builder.verification.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(Person {
                account,
                additional_tos_acceptances,
                address,
                address_kana,
                address_kanji,
                created,
                dob,
                email,
                first_name,
                first_name_kana,
                first_name_kanji,
                full_name_aliases,
                future_requirements,
                gender,
                id,
                id_number_provided,
                id_number_secondary_provided,
                last_name,
                last_name_kana,
                last_name_kanji,
                maiden_name,
                metadata,
                nationality,
                phone,
                political_exposure,
                registered_address,
                relationship,
                requirements,
                ssn_last_4_provided,
                us_cfpb_data,
                verification,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Person {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Person", 32)?;
        s.serialize_field("account", &self.account)?;
        s.serialize_field("additional_tos_acceptances", &self.additional_tos_acceptances)?;
        s.serialize_field("address", &self.address)?;
        s.serialize_field("address_kana", &self.address_kana)?;
        s.serialize_field("address_kanji", &self.address_kanji)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("dob", &self.dob)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("first_name", &self.first_name)?;
        s.serialize_field("first_name_kana", &self.first_name_kana)?;
        s.serialize_field("first_name_kanji", &self.first_name_kanji)?;
        s.serialize_field("full_name_aliases", &self.full_name_aliases)?;
        s.serialize_field("future_requirements", &self.future_requirements)?;
        s.serialize_field("gender", &self.gender)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("id_number_provided", &self.id_number_provided)?;
        s.serialize_field("id_number_secondary_provided", &self.id_number_secondary_provided)?;
        s.serialize_field("last_name", &self.last_name)?;
        s.serialize_field("last_name_kana", &self.last_name_kana)?;
        s.serialize_field("last_name_kanji", &self.last_name_kanji)?;
        s.serialize_field("maiden_name", &self.maiden_name)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("nationality", &self.nationality)?;
        s.serialize_field("phone", &self.phone)?;
        s.serialize_field("political_exposure", &self.political_exposure)?;
        s.serialize_field("registered_address", &self.registered_address)?;
        s.serialize_field("relationship", &self.relationship)?;
        s.serialize_field("requirements", &self.requirements)?;
        s.serialize_field("ssn_last_4_provided", &self.ssn_last_4_provided)?;
        s.serialize_field("us_cfpb_data", &self.us_cfpb_data)?;
        s.serialize_field("verification", &self.verification)?;

        s.serialize_field("object", "person")?;
        s.end()
    }
}
impl stripe_types::Object for Person {
    type Id = stripe_shared::PersonId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PersonId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PersonPoliticalExposure {
    Existing,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PersonPoliticalExposure {
    pub fn as_str(&self) -> &str {
        use PersonPoliticalExposure::*;
        match self {
            Existing => "existing",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PersonPoliticalExposure {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PersonPoliticalExposure::*;
        match s {
            "existing" => Ok(Existing),
            "none" => Ok(None),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PersonPoliticalExposure");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PersonPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PersonPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PersonPoliticalExposure)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for PersonPoliticalExposure {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PersonPoliticalExposure> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PersonPoliticalExposure::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PersonPoliticalExposure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
