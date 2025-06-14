/// This is an object representing a person associated with a Stripe account.
///
/// A platform can only access a subset of data in a person for an account where [account.controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`, which includes Standard and Express accounts, after creating an Account Link or Account Session to start Connect onboarding.
///
/// See the [Standard onboarding](/connect/standard-accounts) or [Express onboarding](/connect/express-accounts) documentation for information about prefilling information and account onboarding steps.
/// Learn more about [handling identity verification with the API](/connect/handling-api-verification#person-information).
///
/// For more details see <<https://stripe.com/docs/api/persons/object>>.
#[derive(Clone, Debug)]
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
    /// Information about the [upcoming new requirements for this person](https://stripe.com/docs/connect/custom-accounts/future-requirements), including what information needs to be collected, and by when.
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
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
            Ok(Box::new(Builder { out: &mut self.out, builder: PersonBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PersonBuilder {
        type Out = Person;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "additional_tos_acceptances" => {
                    Deserialize::begin(&mut self.additional_tos_acceptances)
                }
                "address" => Deserialize::begin(&mut self.address),
                "address_kana" => Deserialize::begin(&mut self.address_kana),
                "address_kanji" => Deserialize::begin(&mut self.address_kanji),
                "created" => Deserialize::begin(&mut self.created),
                "dob" => Deserialize::begin(&mut self.dob),
                "email" => Deserialize::begin(&mut self.email),
                "first_name" => Deserialize::begin(&mut self.first_name),
                "first_name_kana" => Deserialize::begin(&mut self.first_name_kana),
                "first_name_kanji" => Deserialize::begin(&mut self.first_name_kanji),
                "full_name_aliases" => Deserialize::begin(&mut self.full_name_aliases),
                "future_requirements" => Deserialize::begin(&mut self.future_requirements),
                "gender" => Deserialize::begin(&mut self.gender),
                "id" => Deserialize::begin(&mut self.id),
                "id_number_provided" => Deserialize::begin(&mut self.id_number_provided),
                "id_number_secondary_provided" => {
                    Deserialize::begin(&mut self.id_number_secondary_provided)
                }
                "last_name" => Deserialize::begin(&mut self.last_name),
                "last_name_kana" => Deserialize::begin(&mut self.last_name_kana),
                "last_name_kanji" => Deserialize::begin(&mut self.last_name_kanji),
                "maiden_name" => Deserialize::begin(&mut self.maiden_name),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "nationality" => Deserialize::begin(&mut self.nationality),
                "phone" => Deserialize::begin(&mut self.phone),
                "political_exposure" => Deserialize::begin(&mut self.political_exposure),
                "registered_address" => Deserialize::begin(&mut self.registered_address),
                "relationship" => Deserialize::begin(&mut self.relationship),
                "requirements" => Deserialize::begin(&mut self.requirements),
                "ssn_last_4_provided" => Deserialize::begin(&mut self.ssn_last_4_provided),
                "us_cfpb_data" => Deserialize::begin(&mut self.us_cfpb_data),
                "verification" => Deserialize::begin(&mut self.verification),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.account.take(),
                self.additional_tos_acceptances.take(),
                self.address.take(),
                self.address_kana.take(),
                self.address_kanji.take(),
                self.created,
                self.dob,
                self.email.take(),
                self.first_name.take(),
                self.first_name_kana.take(),
                self.first_name_kanji.take(),
                self.full_name_aliases.take(),
                self.future_requirements.take(),
                self.gender.take(),
                self.id.take(),
                self.id_number_provided,
                self.id_number_secondary_provided,
                self.last_name.take(),
                self.last_name_kana.take(),
                self.last_name_kanji.take(),
                self.maiden_name.take(),
                self.metadata.take(),
                self.nationality.take(),
                self.phone.take(),
                self.political_exposure,
                self.registered_address.take(),
                self.relationship.take(),
                self.requirements.take(),
                self.ssn_last_4_provided,
                self.us_cfpb_data.take(),
                self.verification.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
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
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for Person {
        type Builder = PersonBuilder;
    }

    impl FromValueOpt for Person {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PersonBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = FromValueOpt::from_value(v),
                    "additional_tos_acceptances" => {
                        b.additional_tos_acceptances = FromValueOpt::from_value(v)
                    }
                    "address" => b.address = FromValueOpt::from_value(v),
                    "address_kana" => b.address_kana = FromValueOpt::from_value(v),
                    "address_kanji" => b.address_kanji = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "dob" => b.dob = FromValueOpt::from_value(v),
                    "email" => b.email = FromValueOpt::from_value(v),
                    "first_name" => b.first_name = FromValueOpt::from_value(v),
                    "first_name_kana" => b.first_name_kana = FromValueOpt::from_value(v),
                    "first_name_kanji" => b.first_name_kanji = FromValueOpt::from_value(v),
                    "full_name_aliases" => b.full_name_aliases = FromValueOpt::from_value(v),
                    "future_requirements" => b.future_requirements = FromValueOpt::from_value(v),
                    "gender" => b.gender = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "id_number_provided" => b.id_number_provided = FromValueOpt::from_value(v),
                    "id_number_secondary_provided" => {
                        b.id_number_secondary_provided = FromValueOpt::from_value(v)
                    }
                    "last_name" => b.last_name = FromValueOpt::from_value(v),
                    "last_name_kana" => b.last_name_kana = FromValueOpt::from_value(v),
                    "last_name_kanji" => b.last_name_kanji = FromValueOpt::from_value(v),
                    "maiden_name" => b.maiden_name = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "nationality" => b.nationality = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    "political_exposure" => b.political_exposure = FromValueOpt::from_value(v),
                    "registered_address" => b.registered_address = FromValueOpt::from_value(v),
                    "relationship" => b.relationship = FromValueOpt::from_value(v),
                    "requirements" => b.requirements = FromValueOpt::from_value(v),
                    "ssn_last_4_provided" => b.ssn_last_4_provided = FromValueOpt::from_value(v),
                    "us_cfpb_data" => b.us_cfpb_data = FromValueOpt::from_value(v),
                    "verification" => b.verification = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PersonPoliticalExposure::*;
        match s {
            "existing" => Ok(Existing),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
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
impl miniserde::Deserialize for PersonPoliticalExposure {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PersonPoliticalExposure> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PersonPoliticalExposure::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PersonPoliticalExposure);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PersonPoliticalExposure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PersonPoliticalExposure"))
    }
}
