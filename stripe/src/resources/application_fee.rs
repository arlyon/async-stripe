use crate::params::{Identifiable, List, Timestamp};
use crate::resources::{Currency, Refund};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe application fee.
///
/// For more details see https://stripe.com/docs/api#application_fees.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationFee {
    pub id: String,
    pub object: String,
    pub account: String,
    pub amount: u64,
    pub amount_refunded: i64,
    pub application: String,
    pub balance_transaction: String,
    pub charge: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub livemode: bool,
    pub originating_transaction: Option<String>,
    pub refunded: bool,
    pub refunds: List<Refund>,
}

impl Identifiable for ApplicationFee {
    fn id(&self) -> &str {
        &self.id
    }
}
