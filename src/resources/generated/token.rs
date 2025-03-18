// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{CustomerId, TokenId};
use crate::params::{Expand, Metadata, Object, Timestamp};
use crate::resources::{Address, BankAccount, Card, CompanyParams, PersonParams, TokenType};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Token".
///
/// For more details see <https://stripe.com/docs/api/tokens/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Token {
    /// Unique identifier for the object.
    pub id: TokenId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,

    /// IP address of the client that generates the token.
    pub client_ip: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Type of the token: `account`, `bank_account`, `card`, or `pii`.
    #[serde(rename = "type")]
    pub type_: TokenType,

    /// Determines if you have already used this token (you can only use tokens once).
    pub used: bool,
}

impl Token {
    /// Creates a single-use token that represents a bank account’s details.
    /// You can use this token with any API method in place of a bank account dictionary.
    ///
    /// You can only use this token once.
    /// To do so, attach it to a [Custom account](https://stripe.com/docs/api#accounts).
    pub fn create(client: &Client, params: CreateToken<'_>) -> Response<Token> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/tokens", &params)
    }

    /// Retrieves the token with the given ID.
    pub fn retrieve(client: &Client, id: &TokenId, expand: &[&str]) -> Response<Token> {
        client.get_query(&format!("/tokens/{}", id), Expand { expand })
    }
}

impl Object for Token {
    type Id = TokenId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "token"
    }
}

/// The parameters for `Token::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateToken<'a> {
    /// Information for the account this token represents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<CreateTokenAccount>,

    /// The card this token will represent.
    ///
    /// If you also pass in a customer, the card must be the ID of a card belonging to the customer.
    /// Otherwise, if you do not pass in a customer, this is a dictionary containing a user's credit card details, with the options described below.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateTokenCardUnion>,

    /// Create a token for the customer, which is owned by the application's account.
    ///
    /// You can only use this with an [OAuth access token](https://stripe.com/docs/connect/standard-accounts) or [Stripe-Account header](https://stripe.com/docs/connect/authentication).
    /// Learn more about [cloning saved payment methods](https://stripe.com/docs/connect/cloning-saved-payment-methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// The updated CVC value this token represents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_update: Option<CreateTokenCvcUpdate>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Information for the person this token represents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<CreateTokenPerson>,

    /// The PII this token represents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii: Option<CreateTokenPii>,
}

impl<'a> CreateToken<'a> {
    pub fn new() -> Self {
        CreateToken {
            account: Default::default(),
            card: Default::default(),
            customer: Default::default(),
            cvc_update: Default::default(),
            expand: Default::default(),
            person: Default::default(),
            pii: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenAccount {
    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<CreateTokenAccountBusinessType>,

    /// Information about the company or business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CompanyParams>,

    /// Information about the person represented by the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonParams>,

    /// Whether the user described by the data in the token has been shown [the Stripe Connected Account Agreement](https://stripe.com/docs/connect/account-tokens#stripe-connected-account-agreement).
    ///
    /// When creating an account token to create a new Connect account, this value must be `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenCvcUpdate {
    /// The CVC value, in string form.
    pub cvc: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPerson {
    /// Details on the legal guardian's acceptance of the required Stripe agreements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_tos_acceptances: Option<CreateTokenPersonAdditionalTosAcceptances>,

    /// The person's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// The Kana variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    /// The Kanji variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    /// The person's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreateTokenPersonDob>,

    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<CreateTokenPersonDocuments>,

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

    /// The person's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    /// The person's ID number, as appropriate for their country.
    ///
    /// For example, a social security number in the U.S., social insurance number in Canada, etc.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://stripe.com/docs/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,

    /// The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks.
    ///
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://stripe.com/docs/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<String>,

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
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The country where the person is a national.
    ///
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)), or "XX" if unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,

    /// The person's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<String>,

    /// The person's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<CreateTokenPersonRegisteredAddress>,

    /// The relationship that this person has with the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<CreateTokenPersonRelationship>,

    /// The last four digits of the person's Social Security number (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,

    /// The person's verification status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationParams>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPii {
    /// The `id_number` for the PII, in string form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonAdditionalTosAcceptances {
    /// Details on the legal guardian's acceptance of the main Stripe service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<CreateTokenPersonAdditionalTosAcceptancesAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,

    /// The month of birth, between 1 and 12.
    pub month: i64,

    /// The four-digit year of birth.
    pub year: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonDocuments {
    /// One or more documents that demonstrate proof that this person is authorized to represent the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_authorization: Option<CreateTokenPersonDocumentsCompanyAuthorization>,

    /// One or more documents showing the person's passport page with photo and personal data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<CreateTokenPersonDocumentsPassport>,

    /// One or more documents showing the person's visa required for living in the country where they are residing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<CreateTokenPersonDocumentsVisa>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonRegisteredAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonRelationship {
    /// Whether the person is a director of the account's legal entity.
    ///
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
    ///
    /// This is the person nominated by the business to provide information about themselves, and general information about the account.
    /// There can only be one representative at any given time.
    /// At the time the account is created, this person should be set to the person responsible for opening the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,

    /// The person's title (e.g., CEO, Support Engineer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonVerificationParams {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<VerificationDocumentParams>,

    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<VerificationDocumentParams>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonAdditionalTosAcceptancesAccount {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonDocumentsCompanyAuthorization {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonDocumentsPassport {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonDocumentsVisa {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VerificationDocumentParams {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,

    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}

/// An enum representing the possible values of an `CreateTokenAccount`'s `business_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateTokenAccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
}

impl CreateTokenAccountBusinessType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateTokenAccountBusinessType::Company => "company",
            CreateTokenAccountBusinessType::GovernmentEntity => "government_entity",
            CreateTokenAccountBusinessType::Individual => "individual",
            CreateTokenAccountBusinessType::NonProfit => "non_profit",
        }
    }
}

impl AsRef<str> for CreateTokenAccountBusinessType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTokenAccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateTokenAccountBusinessType {
    fn default() -> Self {
        Self::Company
    }
}

/// The card this token will represent.
///
/// If you also pass in a customer, the card must be the ID of a card belonging to the customer.
/// Otherwise, if you do not pass in a customer, this is a dictionary containing a user's credit card details, with the options described below.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum CreateTokenCardUnion {
    CreditCardSpecs(CreditCardSpecs),
    String(String),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreditCardSpecs {
    /// City / District / Suburb / Town / Village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,
    /// Billing address country, if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,
    /// Address line 1 (Street address / PO Box / Company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    /// Address line 2 (Apartment / Suite / Unit / Building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    /// State / County / Province / Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,
    /// Required in order to add the card to an account; in all other cases, this parameter is not used.
    ///
    /// When added to an account, the card (which must be a debit card) can be used as a transfer destination for funds in this currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Card security code.
    ///
    /// Highly recommended to always include this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: String,
    /// Two- or four-digit number representing the card's expiration year.
    pub exp_year: String,
    /// Cardholder's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The card number, as a string without any separators.
    pub number: String,
}
