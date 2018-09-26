use error::Error;
use client::Client;
use resources::{Address, Card, Currency};
use params::{Metadata, Timestamp};

#[derive(Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct CodeVerification {
    pub attempts_remaining: i64,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifiedAddress {
    pub city: Address,
    pub country: String,
    pub line1: String,
    pub line2: String,
    pub postal_code: String,
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Receiver {
    pub address: String,
    pub amount_charged: i64,
    pub amount_receive: i64,
    pub amount_returned: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Redirect {
    pub failure_reason: String,
    pub return_url: String,
    pub status: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Owner {
    pub address: Address,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub verified_address: VerifiedAddress,
    pub verified_email: String,
    pub verified_name: String,
    pub verified_phone: String,
    pub receiver: Option<Receiver>,
    pub redirect: Option<Redirect>,
}

#[derive(Serialize)]
pub struct RedirectParams<'a> {
    return_url: &'a str,
}

#[derive(Default, Serialize)]
pub struct SourceParams<'a> {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<&'a str>, // (redirect, receiver, code_verification, none)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<OwnerParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<RedirectParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<&'a str>, // (reusable, single-use)
}

#[derive(Debug, Deserialize)]
#[serde(tag = "object")]
pub enum PaymentSource {
    // BitcoinReceiver(...),
    #[serde(rename = "card")]
    Card(Card),
    #[serde(rename = "source")]
    Source(Source),
}

/// The resource representing a Stripe source.
///
/// For more details see https://stripe.com/docs/api#sources.
#[derive(Debug, Deserialize)]
pub struct Source {
    pub id: String,
    pub object: String, // source
    pub amount: i64,
    pub client_secret: String,
    pub code_verification: Option<CodeVerification>,
    pub created: Timestamp,
    pub currency: Currency,
    pub flow: String,
    pub livemode: bool,
    pub metadata: Metadata,
    pub owner: Owner,
    pub statement_descriptor: String,
    pub status: String,
    #[serde(rename = "type")]
    pub source_type: String, // (ach_credit_transfer, card, alipay etc.)
    pub usage: String, // (reusable, single-use)
}

impl PaymentSource {
    pub fn create(client: &Client, params: SourceParams) -> Result<PaymentSource, Error> {
        client.post("/sources", params)
    }

    pub fn get(client: &Client, source_id: &str) -> Result<PaymentSource, Error> {
        client.get(&format!("/sources/{}", source_id))
    }

    pub fn update(client: &Client, source_id: &str, params: SourceParams) -> Result<PaymentSource, Error> {
        client.post(&format!("/source/{}", source_id), params)
    }

    /// Attaches a source to a customer, does not change default Source for the Customer
    ///
    /// For more details see https://stripe.com/docs/api#attach_source.
    pub fn attach_source(client: &Client, customer_id: &str, source: &str) -> Result<PaymentSource, Error> {
        client.post(&format!("/customers/{}/sources", customer_id), source)
    }

    /// Detaches a source from a customer
    ///
    /// For more details see https://stripe.com/docs/api#detach_source.
    pub fn detach_source(client: &Client, customer_id: &str, source_id: &str) -> Result<PaymentSource, Error> {
        client.delete(&format!("/customers/{}/sources/{}", customer_id, source_id))
    }
}
