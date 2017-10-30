use params::{Timestamp, Metadata};
use resources::Currency;

/// The resource representing a Stripe application fee refund.
///
/// For more details see https://stripe.com/docs/api#fee_refunds.
#[derive(Debug, Deserialize)]
pub struct ApplicationFeeRefund {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub balance_transaction: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub fee: String,
    pub metadata: Metadata,
}
