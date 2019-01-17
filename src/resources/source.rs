use crate::config::{Client, Response};
use crate::ids::TokenId;
use crate::params::{Identifiable, Metadata, Timestamp};
use crate::resources::{AchCreditTransfer, Address, CardShort, Currency};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OwnerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}

/// The resource representing a Stripe verification code.
///
/// For more details see [https://stripe.com/docs/api#source_object-code_verification](https://stripe.com/docs/api#source_object-code_verification).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeVerification {
    pub attempts_remaining: i64,
    pub status: VerificationStatus,
}

/// An enum representing the possible values of a `CodeVerification`'s `status` field.
///
/// For more details see [https://stripe.com/docs/api#source_object-code_verification-status](https://stripe.com/docs/api#source_object-code_verification-status)
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Pending,
    Succeeded,
    Failed,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

/// The resource representing a Stripe verified owner’s address.
///
/// For more details see [https://stripe.com/docs/api#source_object-owner-verified_address](https://stripe.com/docs/api#source_object-owner-verified_address).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifiedAddress {
    pub city: Address,
    pub country: String,
    pub line1: String,
    pub line2: String,
    pub postal_code: String,
    pub state: String,
}

/// The resource representing a Stripe information related to the receiver flow.
///
/// For more details see [https://stripe.com/docs/api#source_object-receiver](https://stripe.com/docs/api#source_object-receiver).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Receiver {
    pub address: String,
    pub amount_charged: i64,
    pub amount_received: i64,
    pub amount_returned: i64,
}

/// The resource representing a Stripe information related to the redirect flow.
///
/// For more details see [https://stripe.com/docs/api#source_object-redirect](https://stripe.com/docs/api#source_object-redirect).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Redirect {
    pub failure_reason: Option<String>,
    pub return_url: String,
    pub status: String,
    pub url: String,
}

/// The resource representing a Stripe information about the owner.
///
/// For more details see [https://stripe.com/docs/api#source_object-owner](https://stripe.com/docs/api#source_object-owner).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Owner {
    pub address: Option<Address>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub verified_address: Option<VerifiedAddress>,
    pub verified_email: Option<String>,
    pub verified_name: Option<String>,
    pub verified_phone: Option<String>,
}

/// An enum representing the possible values of a `Source`'s `type` field.
///
/// For more details see [https://stripe.com/docs/api#source_object-type](https://stripe.com/docs/api#source_object-type)
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    AchCreditTransfer,
    AchDebit,
    Alipay,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Multibanco,
    P24,
    PaperCheck,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    ThreeDSecure,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

/// An enum representing the possible values of a `Source`'s `status` field.
///
/// For more details see [https://stripe.com/docs/api#source_object-status](https://stripe.com/docs/api#source_object-status)
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceStatus {
    Canceled,
    Chargeable,
    Consumed,
    Failed,
    Pending,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

/// An enum representing the possible values of a `Source`'s `flow` field.
///
/// For more details see [https://stripe.com/docs/api#source_object-flow](https://stripe.com/docs/api#source_object-flow)
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceFlow {
    Redirect,
    Receiver,
    CodeVerification,
    None,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

/// An enum representing the possible values of a `Source`'s `usage` field.
///
/// For more details see [https://stripe.com/docs/api#source_object-usage](https://stripe.com/docs/api#source_object-usage)
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceUsage {
    Reusable,
    SingleUse,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedirectParams<'a> {
    return_url: &'a str,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SourceParams<'a> {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<SourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<SourceFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<OwnerParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<RedirectParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<TokenId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<SourceUsage>,
}

/// The resource representing a Stripe source.
///
/// For more details see [https://stripe.com/docs/api#sources](https://stripe.com/docs/api#sources).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Source {
    pub id: String,
    pub ach_credit_transfer: Option<AchCreditTransfer>,
    pub amount: Option<i64>,
    pub card: Option<CardShort>,
    pub client_secret: Option<String>,
    pub code_verification: Option<CodeVerification>,
    pub created: Timestamp,
    pub currency: Option<Currency>,
    pub flow: Option<SourceFlow>,
    pub livemode: bool,
    pub metadata: Option<Metadata>,
    pub owner: Option<Owner>,
    pub receiver: Option<Receiver>,
    pub redirect: Option<Redirect>,
    pub statement_descriptor: Option<String>,
    pub status: Option<SourceStatus>,
    #[serde(rename = "type")]
    pub source_type: Option<SourceType>,
    pub usage: Option<SourceUsage>,
}

impl Source {
    pub fn create(client: &Client, params: SourceParams<'_>) -> Response<Source> {
        client.post_form("/sources", params)
    }

    pub fn get(client: &Client, source_id: &str) -> Response<Source> {
        client.get(&format!("/sources/{}", source_id))
    }

    pub fn update(client: &Client, source_id: &str, params: SourceParams<'_>) -> Response<Source> {
        client.post_form(&format!("/source/{}", source_id), params)
    }
}

impl Identifiable for Source {
    fn id(&self) -> &str {
        &self.id
    }
}
