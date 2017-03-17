use error::Error;
use client::Client;
use resources::{Address, CardParams, Currency, Deleted, Discount, Source, Subscription};
use params::{List, Metadata};

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomerShippingDetails {
    pub address: Address,
    pub name: String,
    pub phone: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum CustomerSource<'a> {
    Token(&'a str),
    Card(CardParams<'a>),
}

#[derive(Default, Serialize)]
pub struct CustomerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")] pub account_balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub business_vat_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")] pub shipping: Option<CustomerShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")] pub source: Option<CustomerSource<'a>>,
}

#[derive(Debug, Deserialize)]
pub struct Customer {
    pub id: String,
    pub account_balance: i64,
    pub business_vat_id: Option<String>,
    pub created: u64,
    pub currency: Option<Currency>,
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
    pub fn create(c: &Client, params: CustomerParams) -> Result<Customer, Error> {
        return c.post("/customers", params);
    }

    pub fn get(c: &Client, customer_id: &str) -> Result<Customer, Error> {
        return c.get(&format!("/customers/{}", customer_id));
    }

    pub fn update(c: &Client, customer_id: &str, params: CustomerParams) -> Result<Customer, Error> {
        return c.post(&format!("/customers/{}", customer_id), params);
    }

    pub fn delete(c: &Client, customer_id: &str) -> Result<Deleted, Error> {
        return c.delete(&format!("/customers/{}", customer_id));
    }
}
