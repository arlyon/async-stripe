use crate::params::{Identifiable, List, Timestamp};
use crate::resources::{Currency, PaymentSource};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Balance {
    pub object: String,
    pub available: Vec<serde_json::Value>,
    pub connect_reserved: Vec<serde_json::Value>,
    pub livemode: bool,
    pub pending: Vec<serde_json::Value>,
}

/// The resource representing a Stripe balance transaction.
///
/// For more details see https://stripe.com/docs/api#balance_transaction_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl Identifiable for BalanceTransaction {
    fn id(&self) -> &str {
        &self.id
    }
}
