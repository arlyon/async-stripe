use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes an existing person’s relationship to the account’s legal entity.
/// Any person with a relationship for an account can be deleted through the API, except if the person is the `account_opener`.
/// If your integration is using the `executive` parameter, you cannot delete the only verified `executive` on file.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeletePerson<'a> {
    account: &'a stripe_shared::AccountId,
    person: &'a str,
}
impl<'a> DeletePerson<'a> {
    /// Construct a new `DeletePerson`.
    pub fn new(account: &'a stripe_shared::AccountId, person: &'a str) -> Self {
        Self { account, person }
    }
}
impl DeletePerson<'_> {
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

impl StripeRequest for DeletePerson<'_> {
    type Output = stripe_shared::DeletedPerson;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        let person = self.person;
        RequestBuilder::new(StripeMethod::Delete, format!("/accounts/{account}/persons/{person}"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListAccountPersonBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relationship: Option<ListAccountPersonRelationship>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListAccountPersonBuilder<'a> {
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListAccountPersonRelationship {
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
pub struct ListAccountPerson<'a> {
    inner: ListAccountPersonBuilder<'a>,
    account: &'a stripe_shared::AccountId,
}
impl<'a> ListAccountPerson<'a> {
    /// Construct a new `ListAccountPerson`.
    pub fn new(account: &'a stripe_shared::AccountId) -> Self {
        Self { account, inner: ListAccountPersonBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// Filters on the list of people returned based on the person's relationship to the account's company.
    pub fn relationship(mut self, relationship: ListAccountPersonRelationship) -> Self {
        self.inner.relationship = Some(relationship);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl ListAccountPerson<'_> {
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
        let account = self.account;

        stripe_client_core::ListPaginator::new_list(
            format!("/accounts/{account}/persons"),
            self.inner,
        )
    }
}

impl StripeRequest for ListAccountPerson<'_> {
    type Output = stripe_types::List<stripe_shared::Person>;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        RequestBuilder::new(StripeMethod::Get, format!("/accounts/{account}/persons"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrievePersonBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePersonBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an existing person.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePerson<'a> {
    inner: RetrievePersonBuilder<'a>,
    account: &'a stripe_shared::AccountId,
    person: &'a str,
}
impl<'a> RetrievePerson<'a> {
    /// Construct a new `RetrievePerson`.
    pub fn new(account: &'a stripe_shared::AccountId, person: &'a str) -> Self {
        Self { account, person, inner: RetrievePersonBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrievePerson<'_> {
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

impl StripeRequest for RetrievePerson<'_> {
    type Output = stripe_shared::Person;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        let person = self.person;
        RequestBuilder::new(StripeMethod::Get, format!("/accounts/{account}/persons/{person}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateAccountPersonBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_tos_acceptances: Option<PersonAdditionalTosAcceptancesSpecs<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<AddressSpecs<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_kana: Option<CreateAccountPersonAddressKana<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_kanji: Option<CreateAccountPersonAddressKanji<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dob: Option<DateOfBirthSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documents: Option<PersonDocumentsSpecs<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name_kana: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name_kanji: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_name_aliases: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_number: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_number_secondary: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name_kana: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name_kanji: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maiden_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nationality: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    person_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    political_exposure: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registered_address: Option<AddressSpecs<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relationship: Option<RelationshipSpecs<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssn_last_4: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification: Option<PersonVerificationSpecs<'a>>,
}
impl<'a> CreateAccountPersonBuilder<'a> {
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
            verification: None,
        }
    }
}
/// The Kana variation of the person's address (Japan only).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountPersonAddressKana<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> CreateAccountPersonAddressKana<'a> {
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
impl<'a> Default for CreateAccountPersonAddressKana<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the person's address (Japan only).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountPersonAddressKanji<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> CreateAccountPersonAddressKanji<'a> {
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
impl<'a> Default for CreateAccountPersonAddressKanji<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Creates a new person.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAccountPerson<'a> {
    inner: CreateAccountPersonBuilder<'a>,
    account: &'a stripe_shared::AccountId,
}
impl<'a> CreateAccountPerson<'a> {
    /// Construct a new `CreateAccountPerson`.
    pub fn new(account: &'a stripe_shared::AccountId) -> Self {
        Self { account, inner: CreateAccountPersonBuilder::new() }
    }
    /// Details on the legal guardian's acceptance of the required Stripe agreements.
    pub fn additional_tos_acceptances(
        mut self,
        additional_tos_acceptances: PersonAdditionalTosAcceptancesSpecs<'a>,
    ) -> Self {
        self.inner.additional_tos_acceptances = Some(additional_tos_acceptances);
        self
    }
    /// The person's address.
    pub fn address(mut self, address: AddressSpecs<'a>) -> Self {
        self.inner.address = Some(address);
        self
    }
    /// The Kana variation of the person's address (Japan only).
    pub fn address_kana(mut self, address_kana: CreateAccountPersonAddressKana<'a>) -> Self {
        self.inner.address_kana = Some(address_kana);
        self
    }
    /// The Kanji variation of the person's address (Japan only).
    pub fn address_kanji(mut self, address_kanji: CreateAccountPersonAddressKanji<'a>) -> Self {
        self.inner.address_kanji = Some(address_kanji);
        self
    }
    /// The person's date of birth.
    pub fn dob(mut self, dob: DateOfBirthSpecs) -> Self {
        self.inner.dob = Some(dob);
        self
    }
    /// Documents that may be submitted to satisfy various informational requests.
    pub fn documents(mut self, documents: PersonDocumentsSpecs<'a>) -> Self {
        self.inner.documents = Some(documents);
        self
    }
    /// The person's email address.
    pub fn email(mut self, email: &'a str) -> Self {
        self.inner.email = Some(email);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The person's first name.
    pub fn first_name(mut self, first_name: &'a str) -> Self {
        self.inner.first_name = Some(first_name);
        self
    }
    /// The Kana variation of the person's first name (Japan only).
    pub fn first_name_kana(mut self, first_name_kana: &'a str) -> Self {
        self.inner.first_name_kana = Some(first_name_kana);
        self
    }
    /// The Kanji variation of the person's first name (Japan only).
    pub fn first_name_kanji(mut self, first_name_kanji: &'a str) -> Self {
        self.inner.first_name_kanji = Some(first_name_kanji);
        self
    }
    /// A list of alternate names or aliases that the person is known by.
    pub fn full_name_aliases(mut self, full_name_aliases: &'a [&'a str]) -> Self {
        self.inner.full_name_aliases = Some(full_name_aliases);
        self
    }
    /// The person's gender (International regulations require either "male" or "female").
    pub fn gender(mut self, gender: &'a str) -> Self {
        self.inner.gender = Some(gender);
        self
    }
    /// The person's ID number, as appropriate for their country.
    /// For example, a social security number in the U.S., social insurance number in Canada, etc.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    pub fn id_number(mut self, id_number: &'a str) -> Self {
        self.inner.id_number = Some(id_number);
        self
    }
    /// The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks.
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    pub fn id_number_secondary(mut self, id_number_secondary: &'a str) -> Self {
        self.inner.id_number_secondary = Some(id_number_secondary);
        self
    }
    /// The person's last name.
    pub fn last_name(mut self, last_name: &'a str) -> Self {
        self.inner.last_name = Some(last_name);
        self
    }
    /// The Kana variation of the person's last name (Japan only).
    pub fn last_name_kana(mut self, last_name_kana: &'a str) -> Self {
        self.inner.last_name_kana = Some(last_name_kana);
        self
    }
    /// The Kanji variation of the person's last name (Japan only).
    pub fn last_name_kanji(mut self, last_name_kanji: &'a str) -> Self {
        self.inner.last_name_kanji = Some(last_name_kanji);
        self
    }
    /// The person's maiden name.
    pub fn maiden_name(mut self, maiden_name: &'a str) -> Self {
        self.inner.maiden_name = Some(maiden_name);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The country where the person is a national.
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)), or "XX" if unavailable.
    pub fn nationality(mut self, nationality: &'a str) -> Self {
        self.inner.nationality = Some(nationality);
        self
    }
    /// A [person token](https://docs.stripe.com/connect/account-tokens), used to securely provide details to the person.
    pub fn person_token(mut self, person_token: &'a str) -> Self {
        self.inner.person_token = Some(person_token);
        self
    }
    /// The person's phone number.
    pub fn phone(mut self, phone: &'a str) -> Self {
        self.inner.phone = Some(phone);
        self
    }
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    pub fn political_exposure(mut self, political_exposure: &'a str) -> Self {
        self.inner.political_exposure = Some(political_exposure);
        self
    }
    /// The person's registered address.
    pub fn registered_address(mut self, registered_address: AddressSpecs<'a>) -> Self {
        self.inner.registered_address = Some(registered_address);
        self
    }
    /// The relationship that this person has with the account's legal entity.
    pub fn relationship(mut self, relationship: RelationshipSpecs<'a>) -> Self {
        self.inner.relationship = Some(relationship);
        self
    }
    /// The last four digits of the person's Social Security number (U.S. only).
    pub fn ssn_last_4(mut self, ssn_last_4: &'a str) -> Self {
        self.inner.ssn_last_4 = Some(ssn_last_4);
        self
    }
    /// The person's verification status.
    pub fn verification(mut self, verification: PersonVerificationSpecs<'a>) -> Self {
        self.inner.verification = Some(verification);
        self
    }
}
impl CreateAccountPerson<'_> {
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

impl StripeRequest for CreateAccountPerson<'_> {
    type Output = stripe_shared::Person;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        RequestBuilder::new(StripeMethod::Post, format!("/accounts/{account}/persons"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdatePersonBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_tos_acceptances: Option<PersonAdditionalTosAcceptancesSpecs<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<AddressSpecs<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_kana: Option<UpdatePersonAddressKana<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_kanji: Option<UpdatePersonAddressKanji<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dob: Option<DateOfBirthSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documents: Option<PersonDocumentsSpecs<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name_kana: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name_kanji: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_name_aliases: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_number: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_number_secondary: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name_kana: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name_kanji: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maiden_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nationality: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    person_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    political_exposure: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registered_address: Option<AddressSpecs<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relationship: Option<RelationshipSpecs<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssn_last_4: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification: Option<PersonVerificationSpecs<'a>>,
}
impl<'a> UpdatePersonBuilder<'a> {
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
            verification: None,
        }
    }
}
/// The Kana variation of the person's address (Japan only).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePersonAddressKana<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> UpdatePersonAddressKana<'a> {
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
impl<'a> Default for UpdatePersonAddressKana<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the person's address (Japan only).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePersonAddressKanji<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> UpdatePersonAddressKanji<'a> {
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
impl<'a> Default for UpdatePersonAddressKanji<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates an existing person.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePerson<'a> {
    inner: UpdatePersonBuilder<'a>,
    account: &'a stripe_shared::AccountId,
    person: &'a str,
}
impl<'a> UpdatePerson<'a> {
    /// Construct a new `UpdatePerson`.
    pub fn new(account: &'a stripe_shared::AccountId, person: &'a str) -> Self {
        Self { account, person, inner: UpdatePersonBuilder::new() }
    }
    /// Details on the legal guardian's acceptance of the required Stripe agreements.
    pub fn additional_tos_acceptances(
        mut self,
        additional_tos_acceptances: PersonAdditionalTosAcceptancesSpecs<'a>,
    ) -> Self {
        self.inner.additional_tos_acceptances = Some(additional_tos_acceptances);
        self
    }
    /// The person's address.
    pub fn address(mut self, address: AddressSpecs<'a>) -> Self {
        self.inner.address = Some(address);
        self
    }
    /// The Kana variation of the person's address (Japan only).
    pub fn address_kana(mut self, address_kana: UpdatePersonAddressKana<'a>) -> Self {
        self.inner.address_kana = Some(address_kana);
        self
    }
    /// The Kanji variation of the person's address (Japan only).
    pub fn address_kanji(mut self, address_kanji: UpdatePersonAddressKanji<'a>) -> Self {
        self.inner.address_kanji = Some(address_kanji);
        self
    }
    /// The person's date of birth.
    pub fn dob(mut self, dob: DateOfBirthSpecs) -> Self {
        self.inner.dob = Some(dob);
        self
    }
    /// Documents that may be submitted to satisfy various informational requests.
    pub fn documents(mut self, documents: PersonDocumentsSpecs<'a>) -> Self {
        self.inner.documents = Some(documents);
        self
    }
    /// The person's email address.
    pub fn email(mut self, email: &'a str) -> Self {
        self.inner.email = Some(email);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The person's first name.
    pub fn first_name(mut self, first_name: &'a str) -> Self {
        self.inner.first_name = Some(first_name);
        self
    }
    /// The Kana variation of the person's first name (Japan only).
    pub fn first_name_kana(mut self, first_name_kana: &'a str) -> Self {
        self.inner.first_name_kana = Some(first_name_kana);
        self
    }
    /// The Kanji variation of the person's first name (Japan only).
    pub fn first_name_kanji(mut self, first_name_kanji: &'a str) -> Self {
        self.inner.first_name_kanji = Some(first_name_kanji);
        self
    }
    /// A list of alternate names or aliases that the person is known by.
    pub fn full_name_aliases(mut self, full_name_aliases: &'a [&'a str]) -> Self {
        self.inner.full_name_aliases = Some(full_name_aliases);
        self
    }
    /// The person's gender (International regulations require either "male" or "female").
    pub fn gender(mut self, gender: &'a str) -> Self {
        self.inner.gender = Some(gender);
        self
    }
    /// The person's ID number, as appropriate for their country.
    /// For example, a social security number in the U.S., social insurance number in Canada, etc.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    pub fn id_number(mut self, id_number: &'a str) -> Self {
        self.inner.id_number = Some(id_number);
        self
    }
    /// The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks.
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    pub fn id_number_secondary(mut self, id_number_secondary: &'a str) -> Self {
        self.inner.id_number_secondary = Some(id_number_secondary);
        self
    }
    /// The person's last name.
    pub fn last_name(mut self, last_name: &'a str) -> Self {
        self.inner.last_name = Some(last_name);
        self
    }
    /// The Kana variation of the person's last name (Japan only).
    pub fn last_name_kana(mut self, last_name_kana: &'a str) -> Self {
        self.inner.last_name_kana = Some(last_name_kana);
        self
    }
    /// The Kanji variation of the person's last name (Japan only).
    pub fn last_name_kanji(mut self, last_name_kanji: &'a str) -> Self {
        self.inner.last_name_kanji = Some(last_name_kanji);
        self
    }
    /// The person's maiden name.
    pub fn maiden_name(mut self, maiden_name: &'a str) -> Self {
        self.inner.maiden_name = Some(maiden_name);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The country where the person is a national.
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)), or "XX" if unavailable.
    pub fn nationality(mut self, nationality: &'a str) -> Self {
        self.inner.nationality = Some(nationality);
        self
    }
    /// A [person token](https://docs.stripe.com/connect/account-tokens), used to securely provide details to the person.
    pub fn person_token(mut self, person_token: &'a str) -> Self {
        self.inner.person_token = Some(person_token);
        self
    }
    /// The person's phone number.
    pub fn phone(mut self, phone: &'a str) -> Self {
        self.inner.phone = Some(phone);
        self
    }
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    pub fn political_exposure(mut self, political_exposure: &'a str) -> Self {
        self.inner.political_exposure = Some(political_exposure);
        self
    }
    /// The person's registered address.
    pub fn registered_address(mut self, registered_address: AddressSpecs<'a>) -> Self {
        self.inner.registered_address = Some(registered_address);
        self
    }
    /// The relationship that this person has with the account's legal entity.
    pub fn relationship(mut self, relationship: RelationshipSpecs<'a>) -> Self {
        self.inner.relationship = Some(relationship);
        self
    }
    /// The last four digits of the person's Social Security number (U.S. only).
    pub fn ssn_last_4(mut self, ssn_last_4: &'a str) -> Self {
        self.inner.ssn_last_4 = Some(ssn_last_4);
        self
    }
    /// The person's verification status.
    pub fn verification(mut self, verification: PersonVerificationSpecs<'a>) -> Self {
        self.inner.verification = Some(verification);
        self
    }
}
impl UpdatePerson<'_> {
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

impl StripeRequest for UpdatePerson<'_> {
    type Output = stripe_shared::Person;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        let person = self.person;
        RequestBuilder::new(StripeMethod::Post, format!("/accounts/{account}/persons/{person}"))
            .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SettingsTermsOfServiceSpecs<'a> {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> SettingsTermsOfServiceSpecs<'a> {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl<'a> Default for SettingsTermsOfServiceSpecs<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AddressSpecs<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> AddressSpecs<'a> {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl<'a> Default for AddressSpecs<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DateOfBirthSpecs {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl DateOfBirthSpecs {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DocumentsParam<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> DocumentsParam<'a> {
    pub fn new() -> Self {
        Self { files: None }
    }
}
impl<'a> Default for DocumentsParam<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RelationshipSpecs<'a> {
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
    pub title: Option<&'a str>,
}
impl<'a> RelationshipSpecs<'a> {
    pub fn new() -> Self {
        Self {
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
impl<'a> Default for RelationshipSpecs<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PersonVerificationDocumentSpecs<'a> {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> PersonVerificationDocumentSpecs<'a> {
    pub fn new() -> Self {
        Self { back: None, front: None }
    }
}
impl<'a> Default for PersonVerificationDocumentSpecs<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PersonAdditionalTosAcceptancesSpecs<'a> {
    /// Details on the legal guardian's acceptance of the main Stripe service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<SettingsTermsOfServiceSpecs<'a>>,
}
impl<'a> PersonAdditionalTosAcceptancesSpecs<'a> {
    pub fn new() -> Self {
        Self { account: None }
    }
}
impl<'a> Default for PersonAdditionalTosAcceptancesSpecs<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PersonDocumentsSpecs<'a> {
    /// One or more documents that demonstrate proof that this person is authorized to represent the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_authorization: Option<DocumentsParam<'a>>,
    /// One or more documents showing the person's passport page with photo and personal data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<DocumentsParam<'a>>,
    /// One or more documents showing the person's visa required for living in the country where they are residing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<DocumentsParam<'a>>,
}
impl<'a> PersonDocumentsSpecs<'a> {
    pub fn new() -> Self {
        Self { company_authorization: None, passport: None, visa: None }
    }
}
impl<'a> Default for PersonDocumentsSpecs<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PersonVerificationSpecs<'a> {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<PersonVerificationDocumentSpecs<'a>>,
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocumentSpecs<'a>>,
}
impl<'a> PersonVerificationSpecs<'a> {
    pub fn new() -> Self {
        Self { additional_document: None, document: None }
    }
}
impl<'a> Default for PersonVerificationSpecs<'a> {
    fn default() -> Self {
        Self::new()
    }
}
