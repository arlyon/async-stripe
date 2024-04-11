/// This is an object representing a person associated with a Stripe account.
///
/// A platform cannot access a Standard or Express account's persons after the account starts onboarding, such as after generating an account link for the account.
/// See the [Standard onboarding](https://stripe.com/docs/connect/standard-accounts) or [Express onboarding documentation](https://stripe.com/docs/connect/express-accounts) for information about platform prefilling and account onboarding steps.
///
/// Related guide: [Handling identity verification with the API](https://stripe.com/docs/connect/handling-api-verification#person-information).
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
    pub email: Option<String>,
    /// The person's first name.
    pub first_name: Option<String>,
    /// The Kana variation of the person's first name (Japan only).
    pub first_name_kana: Option<String>,
    /// The Kanji variation of the person's first name (Japan only).
    pub first_name_kanji: Option<String>,
    /// A list of alternate names or aliases that the person is known by.
    pub full_name_aliases: Option<Vec<String>>,
    /// Information about the [upcoming new requirements for this person](https://stripe.com/docs/connect/custom-accounts/future-requirements), including what information needs to be collected, and by when.
    pub future_requirements: Option<stripe_shared::PersonFutureRequirements>,
    /// The person's gender (International regulations require either "male" or "female").
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
    pub last_name: Option<String>,
    /// The Kana variation of the person's last name (Japan only).
    pub last_name_kana: Option<String>,
    /// The Kanji variation of the person's last name (Japan only).
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
    pub political_exposure: Option<PersonPoliticalExposure>,
    pub registered_address: Option<stripe_shared::Address>,
    pub relationship: Option<stripe_shared::PersonRelationship>,
    /// Information about the requirements for this person, including what information needs to be collected, and by when.
    pub requirements: Option<stripe_shared::PersonRequirements>,
    /// Whether the last four digits of the person's Social Security number have been provided (U.S. only).
    pub ssn_last_4_provided: Option<bool>,
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
    political_exposure: Option<Option<PersonPoliticalExposure>>,
    registered_address: Option<Option<stripe_shared::Address>>,
    relationship: Option<Option<stripe_shared::PersonRelationship>>,
    requirements: Option<Option<stripe_shared::PersonRequirements>>,
    ssn_last_4_provided: Option<Option<bool>>,
    verification: Option<Option<stripe_shared::LegalEntityPersonVerification>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
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
                verification: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                account: self.account.take()?,
                additional_tos_acceptances: self.additional_tos_acceptances.take()?,
                address: self.address.take()?,
                address_kana: self.address_kana.take()?,
                address_kanji: self.address_kanji.take()?,
                created: self.created?,
                dob: self.dob?,
                email: self.email.take()?,
                first_name: self.first_name.take()?,
                first_name_kana: self.first_name_kana.take()?,
                first_name_kanji: self.first_name_kanji.take()?,
                full_name_aliases: self.full_name_aliases.take()?,
                future_requirements: self.future_requirements.take()?,
                gender: self.gender.take()?,
                id: self.id.take()?,
                id_number_provided: self.id_number_provided?,
                id_number_secondary_provided: self.id_number_secondary_provided?,
                last_name: self.last_name.take()?,
                last_name_kana: self.last_name_kana.take()?,
                last_name_kanji: self.last_name_kanji.take()?,
                maiden_name: self.maiden_name.take()?,
                metadata: self.metadata.take()?,
                nationality: self.nationality.take()?,
                phone: self.phone.take()?,
                political_exposure: self.political_exposure?,
                registered_address: self.registered_address.take()?,
                relationship: self.relationship.take()?,
                requirements: self.requirements.take()?,
                ssn_last_4_provided: self.ssn_last_4_provided?,
                verification: self.verification.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
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
                    "account" => b.account = Some(FromValueOpt::from_value(v)?),
                    "additional_tos_acceptances" => {
                        b.additional_tos_acceptances = Some(FromValueOpt::from_value(v)?)
                    }
                    "address" => b.address = Some(FromValueOpt::from_value(v)?),
                    "address_kana" => b.address_kana = Some(FromValueOpt::from_value(v)?),
                    "address_kanji" => b.address_kanji = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "dob" => b.dob = Some(FromValueOpt::from_value(v)?),
                    "email" => b.email = Some(FromValueOpt::from_value(v)?),
                    "first_name" => b.first_name = Some(FromValueOpt::from_value(v)?),
                    "first_name_kana" => b.first_name_kana = Some(FromValueOpt::from_value(v)?),
                    "first_name_kanji" => b.first_name_kanji = Some(FromValueOpt::from_value(v)?),
                    "full_name_aliases" => b.full_name_aliases = Some(FromValueOpt::from_value(v)?),
                    "future_requirements" => {
                        b.future_requirements = Some(FromValueOpt::from_value(v)?)
                    }
                    "gender" => b.gender = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "id_number_provided" => {
                        b.id_number_provided = Some(FromValueOpt::from_value(v)?)
                    }
                    "id_number_secondary_provided" => {
                        b.id_number_secondary_provided = Some(FromValueOpt::from_value(v)?)
                    }
                    "last_name" => b.last_name = Some(FromValueOpt::from_value(v)?),
                    "last_name_kana" => b.last_name_kana = Some(FromValueOpt::from_value(v)?),
                    "last_name_kanji" => b.last_name_kanji = Some(FromValueOpt::from_value(v)?),
                    "maiden_name" => b.maiden_name = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "nationality" => b.nationality = Some(FromValueOpt::from_value(v)?),
                    "phone" => b.phone = Some(FromValueOpt::from_value(v)?),
                    "political_exposure" => {
                        b.political_exposure = Some(FromValueOpt::from_value(v)?)
                    }
                    "registered_address" => {
                        b.registered_address = Some(FromValueOpt::from_value(v)?)
                    }
                    "relationship" => b.relationship = Some(FromValueOpt::from_value(v)?),
                    "requirements" => b.requirements = Some(FromValueOpt::from_value(v)?),
                    "ssn_last_4_provided" => {
                        b.ssn_last_4_provided = Some(FromValueOpt::from_value(v)?)
                    }
                    "verification" => b.verification = Some(FromValueOpt::from_value(v)?),

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
        let mut s = s.serialize_struct("Person", 31)?;
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
        s.serialize_field("verification", &self.verification)?;

        s.serialize_field("object", "person")?;
        s.end()
    }
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
#[cfg(feature = "serialize")]
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
