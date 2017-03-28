use params::{Metadata, Timestamp};
use resources::Currency;

#[derive(Debug, Deserialize)]
pub struct Refund {
    pub id: String,
    pub amount: u64,
    pub balance_transaction: String,
    pub charge: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub description: String,
    pub metadata: Metadata,
    pub reason: Option<String>, // (duplicate, fraudulent, requested_by_customer)
    pub receipt_number: Option<String>,
    pub status: String, // (succeeded, pending, failed, cancelled)
}
