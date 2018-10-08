use params::{Identifiable, Metadata, Timestamp};
use resources::Currency;

/// The resource representing a Stripe refund.
///
/// For more details see https://stripe.com/docs/api#refunds.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Refund {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub balance_transaction: String,
    pub charge: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub failure_balance_transaction: Option<String>,
    pub failure_reason: Option<String>,
    pub metadata: Metadata,
    pub reason: Option<String>, // (duplicate, fraudulent, requested_by_customer)
    pub receipt_number: Option<String>,
    pub status: String, // (succeeded, pending, failed, cancelled)
}

impl Identifiable for Refund {
    fn id(&self) -> &str {
        &self.id
    }
}
