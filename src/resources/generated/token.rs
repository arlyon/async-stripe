// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{CustomerId, TokenId};
use crate::params::{Expand, Metadata, Object, Timestamp};
use crate::resources::{Address, BankAccount, Card, CompanyParams, PersonParams, TokenType};

/// The resource representing a Stripe "Token".
///
/// For more details see <https://stripe.com/docs/api/tokens/object>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    /// Unique identifier for the object.
    pub id: TokenId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<Box<BankAccount>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Box<Card>>,

    /// IP address of the client that generated the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<Box<String>>,

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
    pub account: Option<Box<CreateTokenAccount>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateTokenCardInfo>,

    /// The customer (owned by the application's account) for which to create a token.
    ///
    /// This can be used only with an [OAuth access token](https://stripe.com/docs/connect/standard-accounts) or [Stripe-Account header](https://stripe.com/docs/connect/authentication).
    /// For more details, see [Cloning Saved Payment Methods](https://stripe.com/docs/connect/cloning-saved-payment-methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// The updated CVC value this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_update: Option<Box<CreateTokenCvcUpdate>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Information for the person this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<Box<CreateTokenPerson>>,

    /// The PII this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii: Option<Box<CreateTokenPii>>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<Box<CreateTokenAccountBusinessType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CompanyParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenCvcUpdate {
    pub cvc: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPerson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<Box<CreateTokenPersonDob>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Box<CreateTokenPersonDocuments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Box<Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<Box<String>>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Box<CreateTokenPersonRelationship>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<Box<PersonVerificationParams>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPii {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPersonDob {
    pub day: i64,

    pub month: i64,

    pub year: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPersonDocuments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_authorization: Option<Box<CreateTokenPersonDocumentsCompanyAuthorization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<Box<CreateTokenPersonDocumentsPassport>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<Box<CreateTokenPersonDocumentsVisa>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPersonRelationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<Box<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonVerificationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<Box<VerificationDocumentParams>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Box<VerificationDocumentParams>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPersonDocumentsCompanyAuthorization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPersonDocumentsPassport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPersonDocumentsVisa {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerificationDocumentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<Box<String>>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CreateTokenCardInfo {
    CreditCardSpecs(CreditCardSpecs),
    Alternate1(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
