// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{CustomerId, TokenId};
use crate::params::{Expand, Metadata, Object, Timestamp};
use crate::resources::{Address, BankAccount, Card, CompanyParams, PersonParams, TokenType};

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

    /// IP address of the client that generated the token.
    #[serde(skip_serializing_if = "Option::is_none")]
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

    /// Whether this token has already been used (tokens can be used only once).
    pub used: bool,
}

impl Token {
    /// Creates a single-use token that represents a bank account’s details.
    /// This token can be used with any API method in place of a bank account dictionary.
    ///
    /// This token can be used only once, by attaching it to a [Custom account](https://stripe.com/docs/api#accounts).
    pub fn create(client: &Client, params: CreateToken<'_>) -> Response<Token> {
        client.post_form("/tokens", &params)
    }

    /// Retrieves the token with the given ID.
    pub fn retrieve(client: &Client, id: &TokenId, expand: &[&str]) -> Response<Token> {
        client.get_query(&format!("/tokens/{}", id), &Expand { expand })
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
    /// Information for the account this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<CreateTokenAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateTokenCardUnion>,

    /// The customer (owned by the application's account) for which to create a token.
    ///
    /// This can be used only with an [OAuth access token](https://stripe.com/docs/connect/standard-accounts) or [Stripe-Account header](https://stripe.com/docs/connect/authentication).
    /// For more details, see [Cloning Saved Payment Methods](https://stripe.com/docs/connect/cloning-saved-payment-methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// The updated CVC value this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_update: Option<CreateTokenCvcUpdate>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Information for the person this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<CreateTokenPerson>,

    /// The PII this token will represent.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<CreateTokenAccountBusinessType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CompanyParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenCvcUpdate {
    pub cvc: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPerson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreateTokenPersonDob>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<CreateTokenPersonDocuments>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<CreateTokenPersonRegisteredAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<CreateTokenPersonRelationship>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationParams>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPii {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonDob {
    pub day: i64,

    pub month: i64,

    pub year: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonDocuments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_authorization: Option<CreateTokenPersonDocumentsCompanyAuthorization>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<CreateTokenPersonDocumentsPassport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<CreateTokenPersonDocumentsVisa>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonRegisteredAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonRelationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonVerificationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<VerificationDocumentParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<VerificationDocumentParams>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonDocumentsCompanyAuthorization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonDocumentsPassport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTokenPersonDocumentsVisa {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VerificationDocumentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,

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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum CreateTokenCardUnion {
    CreditCardSpecs(CreditCardSpecs),
    String(String),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreditCardSpecs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    pub exp_month: String,
    pub exp_year: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub number: String,
}
