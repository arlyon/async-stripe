use params::Timestamp;
use resources::{Currency, OrderItem};

/// The resource representing a Stripe order return.
///
/// For more details see https://stripe.com/docs/api/dotnet#order_return_object.
#[derive(Debug, Deserialize)]
pub struct OrderReturn {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub created: Timestamp,
    pub currency: Currency,
    pub items: OrderItem,
    pub livemode: bool,
    pub order: String,
    pub refund: String,
}
