// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{CustomerId, TokenId};
use crate::params::{Expand, Object, Timestamp};
use crate::resources::{BankAccount, BusinessType, Card, CompanyParams, PersonParams};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Token".
///
/// For more details see [https://stripe.com/docs/api/tokens/object](https://stripe.com/docs/api/tokens/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Debug, Serialize)]
pub struct CreateToken<'a> {
    /// Information for the account this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<CreateTokenAccount>,

    /// The customer (owned by the application's account) for which to create a token.
    ///
    /// For use only with [Stripe Connect](https://stripe.com/docs/connect).
    /// Also, this can be used only with an [OAuth access token](https://stripe.com/docs/connect/standard-accounts) or [Stripe-Account header](https://stripe.com/docs/connect/authentication).
    /// For more details, see [Shared Customers](https://stripe.com/docs/connect/shared-customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The PII this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii: Option<CreateTokenPii>,
}

impl<'a> CreateToken<'a> {
    pub fn new() -> Self {
        CreateToken {
            account: Default::default(),
            customer: Default::default(),
            expand: Default::default(),
            pii: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CompanyParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPii {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
}

/// An enum representing the possible values of an `Token`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Account,
    BankAccount,
    Card,
    Pii,
}
