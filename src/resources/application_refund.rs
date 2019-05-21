use crate::params::{Metadata, Paginate, Timestamp};
use crate::resources::Currency;
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe application fee refund.
///
/// For more details see https://stripe.com/docs/api#fee_refunds.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationFeeRefund {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub balance_transaction: Option<String>,
    pub created: Timestamp,
    pub currency: Currency,
    pub fee: String,
    pub metadata: Metadata,
}

impl Paginate for ApplicationFeeRefund {
    fn cursor(&self) -> &str {
        &self.id
    }
}
