use error::Error;
use http;
use resources::{Address, Discount, Source, Subscription};
use params::{List, Metadata};

#[derive(Deserialize, Serialize)]
pub struct CustomerShippingDetails {
    pub address: Address,
    pub name: String,
    pub phone: String,
}

#[derive(Serialize)]
pub struct CustomerParams {
    #[serde(skip_serializing_if = "Option::is_none")] pub account_balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub business_vat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub coupon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub email: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub metadata: Option<Metadata>,
    // #[serde(skip_serializing_if = "Option::is_none")] pub shipping: Option<CustomerShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")] pub source: Option<String>,
}

#[derive(Deserialize)]
pub struct Customer {
    pub id: String,
    pub account_balance: i64,
    pub business_vat_id: Option<String>,
    pub created: u64,
    pub currency: String,
    pub default_source: String,
    pub delinquent: bool,
    pub desc: Option<String>,
    pub discount: Option<Discount>,
    pub email: String,
    pub livemode: bool,
    pub metadata: Metadata,
    pub shipping: Option<CustomerShippingDetails>,
    pub sources: List<Source>,
    pub subscriptions: List<Subscription>,
}

impl Customer {
    pub fn create(params: CustomerParams, key: &str) -> Result<Customer, Error> {
        return http::post("/customers", key, params);
    }

    pub fn get(customer_id: &str, key: &str) -> Result<Customer, Error> {
        return http::get(&format!("/customers/{}", customer_id), key);
    }

    pub fn update(customer_id: &str, params: CustomerParams, key: &str) -> Result<Customer, Error> {
        return http::post(&format!("/customers/{}", customer_id), key, params);
    }
}
