use crate::params::{Identifiable, List, Timestamp};
use crate::resources::{Currency, OrderItem};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe order return.
///
/// For more details see https://stripe.com/docs/api#order_return_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderReturn {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub created: Timestamp,
    pub currency: Currency,
    pub items: List<OrderItem>,
    pub livemode: bool,
    pub order: String,
    pub refund: String,
}

impl Identifiable for OrderReturn {
    fn id(&self) -> &str {
        &self.id
    }
}
