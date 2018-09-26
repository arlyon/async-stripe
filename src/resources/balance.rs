use serde_json as json;
use params::{List, Timestamp};
use resources::{Currency, PaymentSource};

#[derive(Debug, Deserialize)]
pub struct FeeDetails {
    pub amount: u64,
    pub application: String,
    pub currency: Currency,
    pub description: String,
    #[serde(rename = "type")]
    pub fee_type: String, // (application_fee, stripe_fee, or tax)
}

/// The resource representing a Stripe account balance.
///
/// For more details see https://stripe.com/docs/api#balance_object.
#[derive(Debug, Deserialize)]
pub struct Balance {
    pub object: String,
    pub available: Vec<json::Value>,
    pub connect_reserved: Vec<json::Value>,
    pub livemode: bool,
    pub pending: Vec<json::Value>,
}

/// The resource representing a Stripe balance transaction.
///
/// For more details see https://stripe.com/docs/api#balance_transaction_object.
#[derive(Debug, Deserialize)]
pub struct BalanceTransaction {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub available_on: Timestamp,
    pub created: Timestamp,
    pub currency: Currency,
    pub description: String,
    pub fee: u64,
    pub fee_details: List<FeeDetails>,
    pub net: u64,
    pub source: PaymentSource,
    pub status: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
}
