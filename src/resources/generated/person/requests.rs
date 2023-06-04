use crate::{Client, Response};

impl crate::person::Person {
    /// Returns a list of people associated with the account’s legal entity.
    ///
    /// The people are returned sorted by creation date, with the most recent people appearing first.
    pub fn list(
        client: &Client,
        account: &crate::account::AccountId,
        params: ListPerson,
    ) -> Response<crate::List<crate::person::Person>> {
        client.get_query(&format!("/accounts/{account}/persons", account = account), params)
    }
    /// Retrieves an existing person.
    pub fn retrieve(
        client: &Client,
        account: &crate::account::AccountId,
        person: &crate::person::PersonId,
        params: RetrievePerson,
    ) -> Response<crate::person::Person> {
        client.get_query(
            &format!("/accounts/{account}/persons/{person}", account = account, person = person),
            params,
        )
    }
    /// Creates a new person.
    pub fn create(
        client: &Client,
        account: &crate::account::AccountId,
        params: CreatePerson,
    ) -> Response<crate::person::Person> {
        client.send_form(
            &format!("/accounts/{account}/persons", account = account),
            params,
            http_types::Method::Post,
        )
    }
    /// Updates an existing person.
    pub fn update(
        client: &Client,
        account: &crate::account::AccountId,
        person: &crate::person::PersonId,
        params: UpdatePerson,
    ) -> Response<crate::person::Person> {
        client.send_form(
            &format!("/accounts/{account}/persons/{person}", account = account, person = person),
            params,
            http_types::Method::Post,
        )
    }
    /// Deletes an existing person’s relationship to the account’s legal entity.
    ///
    /// Any person with a relationship for an account can be deleted through the API, except if the person is the `account_opener`.
    /// If your integration is using the `executive` parameter, you cannot delete the only verified `executive` on file.
    pub fn delete(
        client: &Client,
        account: &crate::account::AccountId,
        person: &crate::person::PersonId,
    ) -> Response<crate::person::DeletedPerson> {
        client.send(
            &format!("/accounts/{account}/persons/{person}", account = account, person = person),
            http_types::Method::Delete,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListPerson<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filters on the list of people returned based on the person's relationship to the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<ListPersonRelationship>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListPerson<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Filters on the list of people returned based on the person's relationship to the account's company.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListPersonRelationship {
    /// A filter on the list of people returned based on whether these people are directors of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    /// A filter on the list of people returned based on whether these people are executives of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    /// A filter on the list of people returned based on whether these people are owners of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    /// A filter on the list of people returned based on whether these people are the representative of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
}
impl ListPersonRelationship {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePerson<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePerson<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePerson<'a> {
    /// The person's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreatePersonAddress<'a>>,
    /// The Kana variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreatePersonAddressKana<'a>>,
    /// The Kanji variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreatePersonAddressKanji<'a>>,
    /// The person's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreatePersonDob>,
    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<CreatePersonDocuments<'a>>,
    /// The person's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The person's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// The Kana variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<&'a str>,
    /// The Kanji variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<&'a str>,
    /// A list of alternate names or aliases that the person is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<&'a [&'a str]>,
    /// The person's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<&'a str>,
    /// The person's ID number, as appropriate for their country.
    ///
    /// For example, a social security number in the U.S., social insurance number in Canada, etc.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<&'a str>,
    /// The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks.
    ///
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<&'a str>,
    /// The person's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    /// The Kana variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<&'a str>,
    /// The Kanji variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<&'a str>,
    /// The person's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// The country where the person is a national.
    ///
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)), or "XX" if unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<&'a str>,
    /// A [person token](https://stripe.com/docs/connect/account-tokens), used to securely provide details to the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_token: Option<&'a str>,
    /// The person's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<&'a str>,
    /// The person's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<CreatePersonRegisteredAddress<'a>>,
    /// The relationship that this person has with the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<CreatePersonRelationship<'a>>,
    /// The last four digits of the person's Social Security number (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<&'a str>,
    /// The person's verification status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<CreatePersonVerification<'a>>,
}
impl<'a> CreatePerson<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The person's address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePersonAddress<'a> {
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
impl<'a> CreatePersonAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kana variation of the person's address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePersonAddressKana<'a> {
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
impl<'a> CreatePersonAddressKana<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kanji variation of the person's address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePersonAddressKanji<'a> {
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
impl<'a> CreatePersonAddressKanji<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The person's date of birth.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePersonDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl CreatePersonDob {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
/// Documents that may be submitted to satisfy various informational requests.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePersonDocuments<'a> {
    /// One or more documents that demonstrate proof that this person is authorized to represent the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_authorization: Option<CreatePersonDocumentsCompanyAuthorization<'a>>,
    /// One or more documents showing the person's passport page with photo and personal data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<CreatePersonDocumentsPassport<'a>>,
    /// One or more documents showing the person's visa required for living in the country where they are residing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<CreatePersonDocumentsVisa<'a>>,
}
impl<'a> CreatePersonDocuments<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that demonstrate proof that this person is authorized to represent the company.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePersonDocumentsCompanyAuthorization<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> CreatePersonDocumentsCompanyAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents showing the person's passport page with photo and personal data.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePersonDocumentsPassport<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> CreatePersonDocumentsPassport<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents showing the person's visa required for living in the country where they are residing.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePersonDocumentsVisa<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> CreatePersonDocumentsVisa<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The person's registered address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePersonRegisteredAddress<'a> {
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
impl<'a> CreatePersonRegisteredAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The relationship that this person has with the account's legal entity.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePersonRelationship<'a> {
    /// Whether the person is a director of the account's legal entity.
    ///
    /// Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    /// Whether the person has significant responsibility to control, manage, or direct the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    /// Whether the person is an owner of the account’s legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    /// The percent owned by the person of the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<f64>,
    /// Whether the person is authorized as the primary representative of the account.
    ///
    /// This is the person nominated by the business to provide information about themselves, and general information about the account.
    /// There can only be one representative at any given time.
    /// At the time the account is created, this person should be set to the person responsible for opening the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
    /// The person's title (e.g., CEO, Support Engineer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
}
impl<'a> CreatePersonRelationship<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The person's verification status.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePersonVerification<'a> {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<CreatePersonVerificationAdditionalDocument<'a>>,
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<CreatePersonVerificationDocument<'a>>,
}
impl<'a> CreatePersonVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePersonVerificationAdditionalDocument<'a> {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> CreatePersonVerificationAdditionalDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An identifying document, either a passport or local ID card.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePersonVerificationDocument<'a> {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> CreatePersonVerificationDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePerson<'a> {
    /// The person's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdatePersonAddress<'a>>,
    /// The Kana variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<UpdatePersonAddressKana<'a>>,
    /// The Kanji variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<UpdatePersonAddressKanji<'a>>,
    /// The person's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<UpdatePersonDob>,
    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<UpdatePersonDocuments<'a>>,
    /// The person's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The person's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// The Kana variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<&'a str>,
    /// The Kanji variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<&'a str>,
    /// A list of alternate names or aliases that the person is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<&'a [&'a str]>,
    /// The person's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<&'a str>,
    /// The person's ID number, as appropriate for their country.
    ///
    /// For example, a social security number in the U.S., social insurance number in Canada, etc.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<&'a str>,
    /// The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks.
    ///
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<&'a str>,
    /// The person's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    /// The Kana variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<&'a str>,
    /// The Kanji variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<&'a str>,
    /// The person's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// The country where the person is a national.
    ///
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)), or "XX" if unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<&'a str>,
    /// A [person token](https://stripe.com/docs/connect/account-tokens), used to securely provide details to the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_token: Option<&'a str>,
    /// The person's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<&'a str>,
    /// The person's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<UpdatePersonRegisteredAddress<'a>>,
    /// The relationship that this person has with the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<UpdatePersonRelationship<'a>>,
    /// The last four digits of the person's Social Security number (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<&'a str>,
    /// The person's verification status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<UpdatePersonVerification<'a>>,
}
impl<'a> UpdatePerson<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The person's address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePersonAddress<'a> {
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
impl<'a> UpdatePersonAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kana variation of the person's address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
    }
}
/// The Kanji variation of the person's address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
    }
}
/// The person's date of birth.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePersonDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl UpdatePersonDob {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
/// Documents that may be submitted to satisfy various informational requests.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePersonDocuments<'a> {
    /// One or more documents that demonstrate proof that this person is authorized to represent the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_authorization: Option<UpdatePersonDocumentsCompanyAuthorization<'a>>,
    /// One or more documents showing the person's passport page with photo and personal data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<UpdatePersonDocumentsPassport<'a>>,
    /// One or more documents showing the person's visa required for living in the country where they are residing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<UpdatePersonDocumentsVisa<'a>>,
}
impl<'a> UpdatePersonDocuments<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that demonstrate proof that this person is authorized to represent the company.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePersonDocumentsCompanyAuthorization<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdatePersonDocumentsCompanyAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents showing the person's passport page with photo and personal data.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePersonDocumentsPassport<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdatePersonDocumentsPassport<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents showing the person's visa required for living in the country where they are residing.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePersonDocumentsVisa<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdatePersonDocumentsVisa<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The person's registered address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePersonRegisteredAddress<'a> {
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
impl<'a> UpdatePersonRegisteredAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The relationship that this person has with the account's legal entity.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePersonRelationship<'a> {
    /// Whether the person is a director of the account's legal entity.
    ///
    /// Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    /// Whether the person has significant responsibility to control, manage, or direct the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    /// Whether the person is an owner of the account’s legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    /// The percent owned by the person of the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<f64>,
    /// Whether the person is authorized as the primary representative of the account.
    ///
    /// This is the person nominated by the business to provide information about themselves, and general information about the account.
    /// There can only be one representative at any given time.
    /// At the time the account is created, this person should be set to the person responsible for opening the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
    /// The person's title (e.g., CEO, Support Engineer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
}
impl<'a> UpdatePersonRelationship<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The person's verification status.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePersonVerification<'a> {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<UpdatePersonVerificationAdditionalDocument<'a>>,
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<UpdatePersonVerificationDocument<'a>>,
}
impl<'a> UpdatePersonVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePersonVerificationAdditionalDocument<'a> {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> UpdatePersonVerificationAdditionalDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An identifying document, either a passport or local ID card.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePersonVerificationDocument<'a> {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> UpdatePersonVerificationDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
