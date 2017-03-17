use error::Error;
use http;
use resources::{Address, Card};
use params::Metadata;

#[derive(Serialize)]
pub struct OwnerParams {
    #[serde(skip_serializing_if = "Option::is_none")] pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")] pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub phone: Option<String>,
}

#[derive(Serialize)]
pub struct RedirectParams {
    return_url: String,
}

#[derive(Default, Serialize)]
pub struct SourceParams {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")] pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub currency: Option<String>, // eg. "usd"
    #[serde(skip_serializing_if = "Option::is_none")] pub flow: Option<String>, // (redirect, receiver, code_verification, none)
    #[serde(skip_serializing_if = "Option::is_none")] pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")] pub owner: Option<OwnerParams>,
    #[serde(skip_serializing_if = "Option::is_none")] pub redirect: Option<RedirectParams>,
    #[serde(skip_serializing_if = "Option::is_none")] pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub usage: Option<String>, // (reusable, single-use)
}

#[derive(Debug, Deserialize)]
#[serde(tag = "object")]
pub enum Source {
    // BitcoinReceiver(...),

    #[serde(rename = "card")]
    Card(Card),
}

impl Source {
    pub fn create(params: SourceParams, key: &str) -> Result<Source, Error> {
        return http::post("/sources", key, params);
    }

    pub fn get(source_id: &str, key: &str) -> Result<Source, Error> {
        return http::get(&format!("/sources/{}", source_id), key);
    }

    pub fn update(source_id: &str, params: SourceParams, key: &str) -> Result<Source, Error> {
        return http::post(&format!("/source/{}", source_id), key, params);
    }
}
