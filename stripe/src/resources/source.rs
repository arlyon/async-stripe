use client::Client;
use error::Error;
use ids::TokenId;
use params::{Identifiable, Metadata, Timestamp};
use resources::{Address, Card, Currency};

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeVerification {
    pub attempts_remaining: i64,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifiedAddress {
    pub city: Address,
    pub country: String,
    pub line1: String,
    pub line2: String,
    pub postal_code: String,
    pub state: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Receiver {
    pub address: String,
    pub amount_charged: i64,
    pub amount_receive: i64,
    pub amount_returned: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Redirect {
    pub failure_reason: Option<String>,
    pub return_url: String,
    pub status: String,
    pub url: String,
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedirectParams<'a> {
    return_url: &'a str,
}

#[derive(Debug, Default, Deserialize, Serialize)]
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
    pub token: Option<TokenId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<&'a str>, // (reusable, single-use)
}

/// The resource representing a Stripe source.
///
/// For more details see https://stripe.com/docs/api#sources.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Source {
    pub id: String,
    pub amount: Option<i64>,
    pub card: Option<Card>,
    pub client_secret: Option<String>,
    pub code_verification: Option<CodeVerification>,
    pub created: Timestamp,
    pub currency: Option<Currency>,
    pub flow: Option<String>,
    pub livemode: bool,
    pub metadata: Option<Metadata>,
    pub owner: Option<Owner>,
    pub receiver: Option<Receiver>,
    pub redirect: Option<Redirect>,
    pub statement_descriptor: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "type")]
    pub source_type: Option<String>, // (ach_credit_transfer, card, alipay etc.)
    pub usage: Option<String>, // (reusable, single-use)
}

impl Source {
    pub fn create(client: &Client, params: SourceParams) -> Result<Source, Error> {
        client.post("/sources", params)
    }

    pub fn get(client: &Client, source_id: &str) -> Result<Source, Error> {
        client.get(&format!("/sources/{}", source_id))
    }

    pub fn update(client: &Client, source_id: &str, params: SourceParams) -> Result<Source, Error> {
        client.post(&format!("/source/{}", source_id), params)
    }

    /// Attaches a source to a customer, does not change default Source for the Customer
    ///
    /// For more details see https://stripe.com/docs/api#attach_source.
    pub fn attach_source(
        client: &Client,
        customer_id: &str,
        source_id: &str,
    ) -> Result<Source, Error> {
        #[derive(Serialize)]
        struct AttachSource<'a> { source: &'a str }
        let params = AttachSource { source: source_id };
        client.post(&format!("/customers/{}/sources", customer_id), params)
    }

    /// Detaches a source from a customer
    ///
    /// For more details see https://stripe.com/docs/api#detach_source.
    pub fn detach_source(
        client: &Client,
        customer_id: &str,
        source_id: &str,
    ) -> Result<Source, Error> {
        client.delete(&format!("/customers/{}/sources/{}", customer_id, source_id))
    }
}

impl Identifiable for Source {
    fn id(&self) -> &str {
        &self.id
    }
}
