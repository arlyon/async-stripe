use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes an existing person’s relationship to the account’s legal entity.
/// Any person with a relationship for an account can be deleted through the API, except if the person is the `account_opener`.
/// If your integration is using the `executive` parameter, you cannot delete the only verified `executive` on file.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeletePerson {
    account: stripe_shared::AccountId,
    person: String,
}
impl DeletePerson {
    /// Construct a new `DeletePerson`.
    pub fn new(account: impl Into<stripe_shared::AccountId>, person: impl Into<String>) -> Self {
        Self { account: account.into(), person: person.into() }
    }
}
impl DeletePerson {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for DeletePerson {
    type Output = stripe_shared::DeletedPerson;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        let person = &self.person;
        RequestBuilder::new(StripeMethod::Delete, format!("/accounts/{account}/persons/{person}"))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListAccountPersonBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relationship: Option<ListAccountPersonRelationship>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListAccountPersonBuilder {
    fn new() -> Self {
        Self {
            ending_before: None,
            expand: None,
            limit: None,
            relationship: None,
            starting_after: None,
        }
    }
}
/// Filters on the list of people returned based on the person's relationship to the account's company.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct ListAccountPersonRelationship {
    /// A filter on the list of people returned based on whether these people are authorizers of the account's representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer: Option<bool>,
    /// A filter on the list of people returned based on whether these people are directors of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    /// A filter on the list of people returned based on whether these people are executives of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    /// A filter on the list of people returned based on whether these people are legal guardians of the account's representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_guardian: Option<bool>,
    /// A filter on the list of people returned based on whether these people are owners of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    /// A filter on the list of people returned based on whether these people are the representative of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
}
impl ListAccountPersonRelationship {
    pub fn new() -> Self {
        Self {
            authorizer: None,
            director: None,
            executive: None,
            legal_guardian: None,
            owner: None,
            representative: None,
        }
    }
}
impl Default for ListAccountPersonRelationship {
    fn default() -> Self {
        Self::new()
    }
}
/// Returns a list of people associated with the account’s legal entity.
/// The people are returned sorted by creation date, with the most recent people appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListAccountPerson {
    inner: ListAccountPersonBuilder,
    account: stripe_shared::AccountId,
}
impl ListAccountPerson {
    /// Construct a new `ListAccountPerson`.
    pub fn new(account: impl Into<stripe_shared::AccountId>) -> Self {
        Self { account: account.into(), inner: ListAccountPersonBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// Filters on the list of people returned based on the person's relationship to the account's company.
    pub fn relationship(mut self, relationship: impl Into<ListAccountPersonRelationship>) -> Self {
        self.inner.relationship = Some(relationship.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl ListAccountPerson {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Person>> {
        let account = &self.account;

        stripe_client_core::ListPaginator::new_list(
            format!("/accounts/{account}/persons"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListAccountPerson {
    type Output = stripe_types::List<stripe_shared::Person>;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(StripeMethod::Get, format!("/accounts/{account}/persons"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrievePersonBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrievePersonBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an existing person.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePerson {
    inner: RetrievePersonBuilder,
    account: stripe_shared::AccountId,
    person: String,
}
impl RetrievePerson {
    /// Construct a new `RetrievePerson`.
    pub fn new(account: impl Into<stripe_shared::AccountId>, person: impl Into<String>) -> Self {
        Self { account: account.into(), person: person.into(), inner: RetrievePersonBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrievePerson {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrievePerson {
    type Output = stripe_shared::Person;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        let person = &self.person;
        RequestBuilder::new(StripeMethod::Get, format!("/accounts/{account}/persons/{person}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateAccountPersonBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_tos_acceptances: Option<PersonAdditionalTosAcceptancesSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<CreateAccountPersonAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_kana: Option<CreateAccountPersonAddressKana>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_kanji: Option<CreateAccountPersonAddressKanji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dob: Option<DateOfBirthSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documents: Option<PersonDocumentsSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name_kana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name_kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_name_aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_number_secondary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name_kana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name_kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maiden_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nationality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    person_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    political_exposure: Option<stripe_shared::PersonPoliticalExposure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registered_address: Option<CreateAccountPersonRegisteredAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relationship: Option<RelationshipSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssn_last_4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    us_cfpb_data: Option<CreateAccountPersonUsCfpbData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification: Option<PersonVerificationSpecs>,
}
impl CreateAccountPersonBuilder {
    fn new() -> Self {
        Self {
            additional_tos_acceptances: None,
            address: None,
            address_kana: None,
            address_kanji: None,
            dob: None,
            documents: None,
            email: None,
            expand: None,
            first_name: None,
            first_name_kana: None,
            first_name_kanji: None,
            full_name_aliases: None,
            gender: None,
            id_number: None,
            id_number_secondary: None,
            last_name: None,
            last_name_kana: None,
            last_name_kanji: None,
            maiden_name: None,
            metadata: None,
            nationality: None,
            person_token: None,
            phone: None,
            political_exposure: None,
            registered_address: None,
            relationship: None,
            ssn_last_4: None,
            us_cfpb_data: None,
            verification: None,
        }
    }
}
/// The person's address.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateAccountPersonAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl CreateAccountPersonAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateAccountPersonAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the person's address (Japan only).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateAccountPersonAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl CreateAccountPersonAddressKana {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateAccountPersonAddressKana {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the person's address (Japan only).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateAccountPersonAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl CreateAccountPersonAddressKanji {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateAccountPersonAddressKanji {
    fn default() -> Self {
        Self::new()
    }
}
/// The person's registered address.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateAccountPersonRegisteredAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl CreateAccountPersonRegisteredAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateAccountPersonRegisteredAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// Demographic data related to the person.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateAccountPersonUsCfpbData {
    /// The persons ethnicity details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity_details: Option<CreateAccountPersonUsCfpbDataEthnicityDetails>,
    /// The persons race details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race_details: Option<CreateAccountPersonUsCfpbDataRaceDetails>,
    /// The persons self-identified gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_identified_gender: Option<String>,
}
impl CreateAccountPersonUsCfpbData {
    pub fn new() -> Self {
        Self { ethnicity_details: None, race_details: None, self_identified_gender: None }
    }
}
impl Default for CreateAccountPersonUsCfpbData {
    fn default() -> Self {
        Self::new()
    }
}
/// The persons ethnicity details
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateAccountPersonUsCfpbDataEthnicityDetails {
    /// The persons ethnicity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<Vec<CreateAccountPersonUsCfpbDataEthnicityDetailsEthnicity>>,
    /// Please specify your origin, when other is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity_other: Option<String>,
}
impl CreateAccountPersonUsCfpbDataEthnicityDetails {
    pub fn new() -> Self {
        Self { ethnicity: None, ethnicity_other: None }
    }
}
impl Default for CreateAccountPersonUsCfpbDataEthnicityDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The persons ethnicity
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountPersonUsCfpbDataEthnicityDetailsEthnicity {
    Cuban,
    HispanicOrLatino,
    Mexican,
    NotHispanicOrLatino,
    OtherHispanicOrLatino,
    PreferNotToAnswer,
    PuertoRican,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountPersonUsCfpbDataEthnicityDetailsEthnicity {
    pub fn as_str(&self) -> &str {
        use CreateAccountPersonUsCfpbDataEthnicityDetailsEthnicity::*;
        match self {
            Cuban => "cuban",
            HispanicOrLatino => "hispanic_or_latino",
            Mexican => "mexican",
            NotHispanicOrLatino => "not_hispanic_or_latino",
            OtherHispanicOrLatino => "other_hispanic_or_latino",
            PreferNotToAnswer => "prefer_not_to_answer",
            PuertoRican => "puerto_rican",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountPersonUsCfpbDataEthnicityDetailsEthnicity {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountPersonUsCfpbDataEthnicityDetailsEthnicity::*;
        match s {
            "cuban" => Ok(Cuban),
            "hispanic_or_latino" => Ok(HispanicOrLatino),
            "mexican" => Ok(Mexican),
            "not_hispanic_or_latino" => Ok(NotHispanicOrLatino),
            "other_hispanic_or_latino" => Ok(OtherHispanicOrLatino),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "puerto_rican" => Ok(PuertoRican),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountPersonUsCfpbDataEthnicityDetailsEthnicity"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountPersonUsCfpbDataEthnicityDetailsEthnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountPersonUsCfpbDataEthnicityDetailsEthnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateAccountPersonUsCfpbDataEthnicityDetailsEthnicity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountPersonUsCfpbDataEthnicityDetailsEthnicity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The persons race details
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateAccountPersonUsCfpbDataRaceDetails {
    /// The persons race.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race: Option<Vec<CreateAccountPersonUsCfpbDataRaceDetailsRace>>,
    /// Please specify your race, when other is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race_other: Option<String>,
}
impl CreateAccountPersonUsCfpbDataRaceDetails {
    pub fn new() -> Self {
        Self { race: None, race_other: None }
    }
}
impl Default for CreateAccountPersonUsCfpbDataRaceDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The persons race.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountPersonUsCfpbDataRaceDetailsRace {
    AfricanAmerican,
    AmericanIndianOrAlaskaNative,
    Asian,
    AsianIndian,
    BlackOrAfricanAmerican,
    Chinese,
    Ethiopian,
    Filipino,
    GuamanianOrChamorro,
    Haitian,
    Jamaican,
    Japanese,
    Korean,
    NativeHawaiian,
    NativeHawaiianOrOtherPacificIslander,
    Nigerian,
    OtherAsian,
    OtherBlackOrAfricanAmerican,
    OtherPacificIslander,
    PreferNotToAnswer,
    Samoan,
    Somali,
    Vietnamese,
    White,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountPersonUsCfpbDataRaceDetailsRace {
    pub fn as_str(&self) -> &str {
        use CreateAccountPersonUsCfpbDataRaceDetailsRace::*;
        match self {
            AfricanAmerican => "african_american",
            AmericanIndianOrAlaskaNative => "american_indian_or_alaska_native",
            Asian => "asian",
            AsianIndian => "asian_indian",
            BlackOrAfricanAmerican => "black_or_african_american",
            Chinese => "chinese",
            Ethiopian => "ethiopian",
            Filipino => "filipino",
            GuamanianOrChamorro => "guamanian_or_chamorro",
            Haitian => "haitian",
            Jamaican => "jamaican",
            Japanese => "japanese",
            Korean => "korean",
            NativeHawaiian => "native_hawaiian",
            NativeHawaiianOrOtherPacificIslander => "native_hawaiian_or_other_pacific_islander",
            Nigerian => "nigerian",
            OtherAsian => "other_asian",
            OtherBlackOrAfricanAmerican => "other_black_or_african_american",
            OtherPacificIslander => "other_pacific_islander",
            PreferNotToAnswer => "prefer_not_to_answer",
            Samoan => "samoan",
            Somali => "somali",
            Vietnamese => "vietnamese",
            White => "white",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountPersonUsCfpbDataRaceDetailsRace {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountPersonUsCfpbDataRaceDetailsRace::*;
        match s {
            "african_american" => Ok(AfricanAmerican),
            "american_indian_or_alaska_native" => Ok(AmericanIndianOrAlaskaNative),
            "asian" => Ok(Asian),
            "asian_indian" => Ok(AsianIndian),
            "black_or_african_american" => Ok(BlackOrAfricanAmerican),
            "chinese" => Ok(Chinese),
            "ethiopian" => Ok(Ethiopian),
            "filipino" => Ok(Filipino),
            "guamanian_or_chamorro" => Ok(GuamanianOrChamorro),
            "haitian" => Ok(Haitian),
            "jamaican" => Ok(Jamaican),
            "japanese" => Ok(Japanese),
            "korean" => Ok(Korean),
            "native_hawaiian" => Ok(NativeHawaiian),
            "native_hawaiian_or_other_pacific_islander" => Ok(NativeHawaiianOrOtherPacificIslander),
            "nigerian" => Ok(Nigerian),
            "other_asian" => Ok(OtherAsian),
            "other_black_or_african_american" => Ok(OtherBlackOrAfricanAmerican),
            "other_pacific_islander" => Ok(OtherPacificIslander),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "samoan" => Ok(Samoan),
            "somali" => Ok(Somali),
            "vietnamese" => Ok(Vietnamese),
            "white" => Ok(White),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountPersonUsCfpbDataRaceDetailsRace"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountPersonUsCfpbDataRaceDetailsRace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountPersonUsCfpbDataRaceDetailsRace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateAccountPersonUsCfpbDataRaceDetailsRace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountPersonUsCfpbDataRaceDetailsRace {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Creates a new person.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAccountPerson {
    inner: CreateAccountPersonBuilder,
    account: stripe_shared::AccountId,
}
impl CreateAccountPerson {
    /// Construct a new `CreateAccountPerson`.
    pub fn new(account: impl Into<stripe_shared::AccountId>) -> Self {
        Self { account: account.into(), inner: CreateAccountPersonBuilder::new() }
    }
    /// Details on the legal guardian's or authorizer's acceptance of the required Stripe agreements.
    pub fn additional_tos_acceptances(
        mut self,
        additional_tos_acceptances: impl Into<PersonAdditionalTosAcceptancesSpecs>,
    ) -> Self {
        self.inner.additional_tos_acceptances = Some(additional_tos_acceptances.into());
        self
    }
    /// The person's address.
    pub fn address(mut self, address: impl Into<CreateAccountPersonAddress>) -> Self {
        self.inner.address = Some(address.into());
        self
    }
    /// The Kana variation of the person's address (Japan only).
    pub fn address_kana(mut self, address_kana: impl Into<CreateAccountPersonAddressKana>) -> Self {
        self.inner.address_kana = Some(address_kana.into());
        self
    }
    /// The Kanji variation of the person's address (Japan only).
    pub fn address_kanji(
        mut self,
        address_kanji: impl Into<CreateAccountPersonAddressKanji>,
    ) -> Self {
        self.inner.address_kanji = Some(address_kanji.into());
        self
    }
    /// The person's date of birth.
    pub fn dob(mut self, dob: impl Into<DateOfBirthSpecs>) -> Self {
        self.inner.dob = Some(dob.into());
        self
    }
    /// Documents that may be submitted to satisfy various informational requests.
    pub fn documents(mut self, documents: impl Into<PersonDocumentsSpecs>) -> Self {
        self.inner.documents = Some(documents.into());
        self
    }
    /// The person's email address.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.inner.email = Some(email.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The person's first name.
    pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.inner.first_name = Some(first_name.into());
        self
    }
    /// The Kana variation of the person's first name (Japan only).
    pub fn first_name_kana(mut self, first_name_kana: impl Into<String>) -> Self {
        self.inner.first_name_kana = Some(first_name_kana.into());
        self
    }
    /// The Kanji variation of the person's first name (Japan only).
    pub fn first_name_kanji(mut self, first_name_kanji: impl Into<String>) -> Self {
        self.inner.first_name_kanji = Some(first_name_kanji.into());
        self
    }
    /// A list of alternate names or aliases that the person is known by.
    pub fn full_name_aliases(mut self, full_name_aliases: impl Into<Vec<String>>) -> Self {
        self.inner.full_name_aliases = Some(full_name_aliases.into());
        self
    }
    /// The person's gender (International regulations require either "male" or "female").
    pub fn gender(mut self, gender: impl Into<String>) -> Self {
        self.inner.gender = Some(gender.into());
        self
    }
    /// The person's ID number, as appropriate for their country.
    /// For example, a social security number in the U.S., social insurance number in Canada, etc.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    pub fn id_number(mut self, id_number: impl Into<String>) -> Self {
        self.inner.id_number = Some(id_number.into());
        self
    }
    /// The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks.
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    pub fn id_number_secondary(mut self, id_number_secondary: impl Into<String>) -> Self {
        self.inner.id_number_secondary = Some(id_number_secondary.into());
        self
    }
    /// The person's last name.
    pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.inner.last_name = Some(last_name.into());
        self
    }
    /// The Kana variation of the person's last name (Japan only).
    pub fn last_name_kana(mut self, last_name_kana: impl Into<String>) -> Self {
        self.inner.last_name_kana = Some(last_name_kana.into());
        self
    }
    /// The Kanji variation of the person's last name (Japan only).
    pub fn last_name_kanji(mut self, last_name_kanji: impl Into<String>) -> Self {
        self.inner.last_name_kanji = Some(last_name_kanji.into());
        self
    }
    /// The person's maiden name.
    pub fn maiden_name(mut self, maiden_name: impl Into<String>) -> Self {
        self.inner.maiden_name = Some(maiden_name.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The country where the person is a national.
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)), or "XX" if unavailable.
    pub fn nationality(mut self, nationality: impl Into<String>) -> Self {
        self.inner.nationality = Some(nationality.into());
        self
    }
    /// A [person token](https://docs.stripe.com/connect/account-tokens), used to securely provide details to the person.
    pub fn person_token(mut self, person_token: impl Into<String>) -> Self {
        self.inner.person_token = Some(person_token.into());
        self
    }
    /// The person's phone number.
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.inner.phone = Some(phone.into());
        self
    }
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    pub fn political_exposure(
        mut self,
        political_exposure: impl Into<stripe_shared::PersonPoliticalExposure>,
    ) -> Self {
        self.inner.political_exposure = Some(political_exposure.into());
        self
    }
    /// The person's registered address.
    pub fn registered_address(
        mut self,
        registered_address: impl Into<CreateAccountPersonRegisteredAddress>,
    ) -> Self {
        self.inner.registered_address = Some(registered_address.into());
        self
    }
    /// The relationship that this person has with the account's legal entity.
    pub fn relationship(mut self, relationship: impl Into<RelationshipSpecs>) -> Self {
        self.inner.relationship = Some(relationship.into());
        self
    }
    /// The last four digits of the person's Social Security number (U.S. only).
    pub fn ssn_last_4(mut self, ssn_last_4: impl Into<String>) -> Self {
        self.inner.ssn_last_4 = Some(ssn_last_4.into());
        self
    }
    /// Demographic data related to the person.
    pub fn us_cfpb_data(mut self, us_cfpb_data: impl Into<CreateAccountPersonUsCfpbData>) -> Self {
        self.inner.us_cfpb_data = Some(us_cfpb_data.into());
        self
    }
    /// The person's verification status.
    pub fn verification(mut self, verification: impl Into<PersonVerificationSpecs>) -> Self {
        self.inner.verification = Some(verification.into());
        self
    }
}
impl CreateAccountPerson {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateAccountPerson {
    type Output = stripe_shared::Person;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(StripeMethod::Post, format!("/accounts/{account}/persons"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdatePersonBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_tos_acceptances: Option<PersonAdditionalTosAcceptancesSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<UpdatePersonAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_kana: Option<UpdatePersonAddressKana>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_kanji: Option<UpdatePersonAddressKanji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dob: Option<DateOfBirthSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documents: Option<PersonDocumentsSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name_kana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name_kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_name_aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_number_secondary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name_kana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name_kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maiden_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nationality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    person_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    political_exposure: Option<stripe_shared::PersonPoliticalExposure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registered_address: Option<UpdatePersonRegisteredAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relationship: Option<RelationshipSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssn_last_4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    us_cfpb_data: Option<UpdatePersonUsCfpbData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification: Option<PersonVerificationSpecs>,
}
impl UpdatePersonBuilder {
    fn new() -> Self {
        Self {
            additional_tos_acceptances: None,
            address: None,
            address_kana: None,
            address_kanji: None,
            dob: None,
            documents: None,
            email: None,
            expand: None,
            first_name: None,
            first_name_kana: None,
            first_name_kanji: None,
            full_name_aliases: None,
            gender: None,
            id_number: None,
            id_number_secondary: None,
            last_name: None,
            last_name_kana: None,
            last_name_kanji: None,
            maiden_name: None,
            metadata: None,
            nationality: None,
            person_token: None,
            phone: None,
            political_exposure: None,
            registered_address: None,
            relationship: None,
            ssn_last_4: None,
            us_cfpb_data: None,
            verification: None,
        }
    }
}
/// The person's address.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePersonAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl UpdatePersonAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for UpdatePersonAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the person's address (Japan only).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePersonAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl UpdatePersonAddressKana {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for UpdatePersonAddressKana {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the person's address (Japan only).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePersonAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl UpdatePersonAddressKanji {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for UpdatePersonAddressKanji {
    fn default() -> Self {
        Self::new()
    }
}
/// The person's registered address.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePersonRegisteredAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl UpdatePersonRegisteredAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for UpdatePersonRegisteredAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// Demographic data related to the person.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePersonUsCfpbData {
    /// The persons ethnicity details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity_details: Option<UpdatePersonUsCfpbDataEthnicityDetails>,
    /// The persons race details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race_details: Option<UpdatePersonUsCfpbDataRaceDetails>,
    /// The persons self-identified gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_identified_gender: Option<String>,
}
impl UpdatePersonUsCfpbData {
    pub fn new() -> Self {
        Self { ethnicity_details: None, race_details: None, self_identified_gender: None }
    }
}
impl Default for UpdatePersonUsCfpbData {
    fn default() -> Self {
        Self::new()
    }
}
/// The persons ethnicity details
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePersonUsCfpbDataEthnicityDetails {
    /// The persons ethnicity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<Vec<UpdatePersonUsCfpbDataEthnicityDetailsEthnicity>>,
    /// Please specify your origin, when other is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity_other: Option<String>,
}
impl UpdatePersonUsCfpbDataEthnicityDetails {
    pub fn new() -> Self {
        Self { ethnicity: None, ethnicity_other: None }
    }
}
impl Default for UpdatePersonUsCfpbDataEthnicityDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The persons ethnicity
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePersonUsCfpbDataEthnicityDetailsEthnicity {
    Cuban,
    HispanicOrLatino,
    Mexican,
    NotHispanicOrLatino,
    OtherHispanicOrLatino,
    PreferNotToAnswer,
    PuertoRican,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePersonUsCfpbDataEthnicityDetailsEthnicity {
    pub fn as_str(&self) -> &str {
        use UpdatePersonUsCfpbDataEthnicityDetailsEthnicity::*;
        match self {
            Cuban => "cuban",
            HispanicOrLatino => "hispanic_or_latino",
            Mexican => "mexican",
            NotHispanicOrLatino => "not_hispanic_or_latino",
            OtherHispanicOrLatino => "other_hispanic_or_latino",
            PreferNotToAnswer => "prefer_not_to_answer",
            PuertoRican => "puerto_rican",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePersonUsCfpbDataEthnicityDetailsEthnicity {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePersonUsCfpbDataEthnicityDetailsEthnicity::*;
        match s {
            "cuban" => Ok(Cuban),
            "hispanic_or_latino" => Ok(HispanicOrLatino),
            "mexican" => Ok(Mexican),
            "not_hispanic_or_latino" => Ok(NotHispanicOrLatino),
            "other_hispanic_or_latino" => Ok(OtherHispanicOrLatino),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "puerto_rican" => Ok(PuertoRican),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePersonUsCfpbDataEthnicityDetailsEthnicity"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePersonUsCfpbDataEthnicityDetailsEthnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePersonUsCfpbDataEthnicityDetailsEthnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePersonUsCfpbDataEthnicityDetailsEthnicity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePersonUsCfpbDataEthnicityDetailsEthnicity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The persons race details
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePersonUsCfpbDataRaceDetails {
    /// The persons race.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race: Option<Vec<UpdatePersonUsCfpbDataRaceDetailsRace>>,
    /// Please specify your race, when other is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race_other: Option<String>,
}
impl UpdatePersonUsCfpbDataRaceDetails {
    pub fn new() -> Self {
        Self { race: None, race_other: None }
    }
}
impl Default for UpdatePersonUsCfpbDataRaceDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The persons race.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePersonUsCfpbDataRaceDetailsRace {
    AfricanAmerican,
    AmericanIndianOrAlaskaNative,
    Asian,
    AsianIndian,
    BlackOrAfricanAmerican,
    Chinese,
    Ethiopian,
    Filipino,
    GuamanianOrChamorro,
    Haitian,
    Jamaican,
    Japanese,
    Korean,
    NativeHawaiian,
    NativeHawaiianOrOtherPacificIslander,
    Nigerian,
    OtherAsian,
    OtherBlackOrAfricanAmerican,
    OtherPacificIslander,
    PreferNotToAnswer,
    Samoan,
    Somali,
    Vietnamese,
    White,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePersonUsCfpbDataRaceDetailsRace {
    pub fn as_str(&self) -> &str {
        use UpdatePersonUsCfpbDataRaceDetailsRace::*;
        match self {
            AfricanAmerican => "african_american",
            AmericanIndianOrAlaskaNative => "american_indian_or_alaska_native",
            Asian => "asian",
            AsianIndian => "asian_indian",
            BlackOrAfricanAmerican => "black_or_african_american",
            Chinese => "chinese",
            Ethiopian => "ethiopian",
            Filipino => "filipino",
            GuamanianOrChamorro => "guamanian_or_chamorro",
            Haitian => "haitian",
            Jamaican => "jamaican",
            Japanese => "japanese",
            Korean => "korean",
            NativeHawaiian => "native_hawaiian",
            NativeHawaiianOrOtherPacificIslander => "native_hawaiian_or_other_pacific_islander",
            Nigerian => "nigerian",
            OtherAsian => "other_asian",
            OtherBlackOrAfricanAmerican => "other_black_or_african_american",
            OtherPacificIslander => "other_pacific_islander",
            PreferNotToAnswer => "prefer_not_to_answer",
            Samoan => "samoan",
            Somali => "somali",
            Vietnamese => "vietnamese",
            White => "white",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePersonUsCfpbDataRaceDetailsRace {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePersonUsCfpbDataRaceDetailsRace::*;
        match s {
            "african_american" => Ok(AfricanAmerican),
            "american_indian_or_alaska_native" => Ok(AmericanIndianOrAlaskaNative),
            "asian" => Ok(Asian),
            "asian_indian" => Ok(AsianIndian),
            "black_or_african_american" => Ok(BlackOrAfricanAmerican),
            "chinese" => Ok(Chinese),
            "ethiopian" => Ok(Ethiopian),
            "filipino" => Ok(Filipino),
            "guamanian_or_chamorro" => Ok(GuamanianOrChamorro),
            "haitian" => Ok(Haitian),
            "jamaican" => Ok(Jamaican),
            "japanese" => Ok(Japanese),
            "korean" => Ok(Korean),
            "native_hawaiian" => Ok(NativeHawaiian),
            "native_hawaiian_or_other_pacific_islander" => Ok(NativeHawaiianOrOtherPacificIslander),
            "nigerian" => Ok(Nigerian),
            "other_asian" => Ok(OtherAsian),
            "other_black_or_african_american" => Ok(OtherBlackOrAfricanAmerican),
            "other_pacific_islander" => Ok(OtherPacificIslander),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "samoan" => Ok(Samoan),
            "somali" => Ok(Somali),
            "vietnamese" => Ok(Vietnamese),
            "white" => Ok(White),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePersonUsCfpbDataRaceDetailsRace"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePersonUsCfpbDataRaceDetailsRace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePersonUsCfpbDataRaceDetailsRace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePersonUsCfpbDataRaceDetailsRace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePersonUsCfpbDataRaceDetailsRace {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Updates an existing person.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePerson {
    inner: UpdatePersonBuilder,
    account: stripe_shared::AccountId,
    person: String,
}
impl UpdatePerson {
    /// Construct a new `UpdatePerson`.
    pub fn new(account: impl Into<stripe_shared::AccountId>, person: impl Into<String>) -> Self {
        Self { account: account.into(), person: person.into(), inner: UpdatePersonBuilder::new() }
    }
    /// Details on the legal guardian's or authorizer's acceptance of the required Stripe agreements.
    pub fn additional_tos_acceptances(
        mut self,
        additional_tos_acceptances: impl Into<PersonAdditionalTosAcceptancesSpecs>,
    ) -> Self {
        self.inner.additional_tos_acceptances = Some(additional_tos_acceptances.into());
        self
    }
    /// The person's address.
    pub fn address(mut self, address: impl Into<UpdatePersonAddress>) -> Self {
        self.inner.address = Some(address.into());
        self
    }
    /// The Kana variation of the person's address (Japan only).
    pub fn address_kana(mut self, address_kana: impl Into<UpdatePersonAddressKana>) -> Self {
        self.inner.address_kana = Some(address_kana.into());
        self
    }
    /// The Kanji variation of the person's address (Japan only).
    pub fn address_kanji(mut self, address_kanji: impl Into<UpdatePersonAddressKanji>) -> Self {
        self.inner.address_kanji = Some(address_kanji.into());
        self
    }
    /// The person's date of birth.
    pub fn dob(mut self, dob: impl Into<DateOfBirthSpecs>) -> Self {
        self.inner.dob = Some(dob.into());
        self
    }
    /// Documents that may be submitted to satisfy various informational requests.
    pub fn documents(mut self, documents: impl Into<PersonDocumentsSpecs>) -> Self {
        self.inner.documents = Some(documents.into());
        self
    }
    /// The person's email address.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.inner.email = Some(email.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The person's first name.
    pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.inner.first_name = Some(first_name.into());
        self
    }
    /// The Kana variation of the person's first name (Japan only).
    pub fn first_name_kana(mut self, first_name_kana: impl Into<String>) -> Self {
        self.inner.first_name_kana = Some(first_name_kana.into());
        self
    }
    /// The Kanji variation of the person's first name (Japan only).
    pub fn first_name_kanji(mut self, first_name_kanji: impl Into<String>) -> Self {
        self.inner.first_name_kanji = Some(first_name_kanji.into());
        self
    }
    /// A list of alternate names or aliases that the person is known by.
    pub fn full_name_aliases(mut self, full_name_aliases: impl Into<Vec<String>>) -> Self {
        self.inner.full_name_aliases = Some(full_name_aliases.into());
        self
    }
    /// The person's gender (International regulations require either "male" or "female").
    pub fn gender(mut self, gender: impl Into<String>) -> Self {
        self.inner.gender = Some(gender.into());
        self
    }
    /// The person's ID number, as appropriate for their country.
    /// For example, a social security number in the U.S., social insurance number in Canada, etc.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    pub fn id_number(mut self, id_number: impl Into<String>) -> Self {
        self.inner.id_number = Some(id_number.into());
        self
    }
    /// The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks.
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    pub fn id_number_secondary(mut self, id_number_secondary: impl Into<String>) -> Self {
        self.inner.id_number_secondary = Some(id_number_secondary.into());
        self
    }
    /// The person's last name.
    pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.inner.last_name = Some(last_name.into());
        self
    }
    /// The Kana variation of the person's last name (Japan only).
    pub fn last_name_kana(mut self, last_name_kana: impl Into<String>) -> Self {
        self.inner.last_name_kana = Some(last_name_kana.into());
        self
    }
    /// The Kanji variation of the person's last name (Japan only).
    pub fn last_name_kanji(mut self, last_name_kanji: impl Into<String>) -> Self {
        self.inner.last_name_kanji = Some(last_name_kanji.into());
        self
    }
    /// The person's maiden name.
    pub fn maiden_name(mut self, maiden_name: impl Into<String>) -> Self {
        self.inner.maiden_name = Some(maiden_name.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The country where the person is a national.
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)), or "XX" if unavailable.
    pub fn nationality(mut self, nationality: impl Into<String>) -> Self {
        self.inner.nationality = Some(nationality.into());
        self
    }
    /// A [person token](https://docs.stripe.com/connect/account-tokens), used to securely provide details to the person.
    pub fn person_token(mut self, person_token: impl Into<String>) -> Self {
        self.inner.person_token = Some(person_token.into());
        self
    }
    /// The person's phone number.
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.inner.phone = Some(phone.into());
        self
    }
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    pub fn political_exposure(
        mut self,
        political_exposure: impl Into<stripe_shared::PersonPoliticalExposure>,
    ) -> Self {
        self.inner.political_exposure = Some(political_exposure.into());
        self
    }
    /// The person's registered address.
    pub fn registered_address(
        mut self,
        registered_address: impl Into<UpdatePersonRegisteredAddress>,
    ) -> Self {
        self.inner.registered_address = Some(registered_address.into());
        self
    }
    /// The relationship that this person has with the account's legal entity.
    pub fn relationship(mut self, relationship: impl Into<RelationshipSpecs>) -> Self {
        self.inner.relationship = Some(relationship.into());
        self
    }
    /// The last four digits of the person's Social Security number (U.S. only).
    pub fn ssn_last_4(mut self, ssn_last_4: impl Into<String>) -> Self {
        self.inner.ssn_last_4 = Some(ssn_last_4.into());
        self
    }
    /// Demographic data related to the person.
    pub fn us_cfpb_data(mut self, us_cfpb_data: impl Into<UpdatePersonUsCfpbData>) -> Self {
        self.inner.us_cfpb_data = Some(us_cfpb_data.into());
        self
    }
    /// The person's verification status.
    pub fn verification(mut self, verification: impl Into<PersonVerificationSpecs>) -> Self {
        self.inner.verification = Some(verification.into());
        self
    }
}
impl UpdatePerson {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdatePerson {
    type Output = stripe_shared::Person;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        let person = &self.person;
        RequestBuilder::new(StripeMethod::Post, format!("/accounts/{account}/persons/{person}"))
            .form(&self.inner)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct SettingsTermsOfServiceSpecs {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl SettingsTermsOfServiceSpecs {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl Default for SettingsTermsOfServiceSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct DateOfBirthSpecs {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl DateOfBirthSpecs {
    pub fn new(day: impl Into<i64>, month: impl Into<i64>, year: impl Into<i64>) -> Self {
        Self { day: day.into(), month: month.into(), year: year.into() }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct DocumentsParam {
    /// One or more document ids returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}
impl DocumentsParam {
    pub fn new() -> Self {
        Self { files: None }
    }
}
impl Default for DocumentsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct RelationshipSpecs {
    /// Whether the person is the authorizer of the account's representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer: Option<bool>,
    /// Whether the person is a director of the account's legal entity.
    /// Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    /// Whether the person has significant responsibility to control, manage, or direct the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    /// Whether the person is the legal guardian of the account's representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_guardian: Option<bool>,
    /// Whether the person is an owner of the account’s legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    /// The percent owned by the person of the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<f64>,
    /// Whether the person is authorized as the primary representative of the account.
    /// This is the person nominated by the business to provide information about themselves, and general information about the account.
    /// There can only be one representative at any given time.
    /// At the time the account is created, this person should be set to the person responsible for opening the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
    /// The person's title (e.g., CEO, Support Engineer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl RelationshipSpecs {
    pub fn new() -> Self {
        Self {
            authorizer: None,
            director: None,
            executive: None,
            legal_guardian: None,
            owner: None,
            percent_ownership: None,
            representative: None,
            title: None,
        }
    }
}
impl Default for RelationshipSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PersonVerificationDocumentSpecs {
    /// The back of an ID returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `identity_document`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,
    /// The front of an ID returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `identity_document`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}
impl PersonVerificationDocumentSpecs {
    pub fn new() -> Self {
        Self { back: None, front: None }
    }
}
impl Default for PersonVerificationDocumentSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PersonAdditionalTosAcceptancesSpecs {
    /// Details on the legal guardian's acceptance of the main Stripe service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<SettingsTermsOfServiceSpecs>,
}
impl PersonAdditionalTosAcceptancesSpecs {
    pub fn new() -> Self {
        Self { account: None }
    }
}
impl Default for PersonAdditionalTosAcceptancesSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PersonDocumentsSpecs {
    /// One or more documents that demonstrate proof that this person is authorized to represent the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_authorization: Option<DocumentsParam>,
    /// One or more documents showing the person's passport page with photo and personal data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<DocumentsParam>,
    /// One or more documents showing the person's visa required for living in the country where they are residing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<DocumentsParam>,
}
impl PersonDocumentsSpecs {
    pub fn new() -> Self {
        Self { company_authorization: None, passport: None, visa: None }
    }
}
impl Default for PersonDocumentsSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PersonVerificationSpecs {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<PersonVerificationDocumentSpecs>,
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocumentSpecs>,
}
impl PersonVerificationSpecs {
    pub fn new() -> Self {
        Self { additional_document: None, document: None }
    }
}
impl Default for PersonVerificationSpecs {
    fn default() -> Self {
        Self::new()
    }
}
