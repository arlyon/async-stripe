use crate::params::{Identifiable, List, Metadata, Timestamp};
use crate::resources::{Currency, ShippingDetails};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StatusTransitions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfilled: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<Timestamp>,
}

/// The resource representing a Stripe order item.
///
/// For more details see https://stripe.com/docs/api#order_item_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderItem {
    pub object: String,
    pub amount: u64,
    pub currency: Currency,
    pub description: String,
    pub parent: Option<String>,
    pub quantity: Option<u64>,
    #[serde(rename = "type")]
    pub item_type: String,
}

/// The resource representing a Stripe order.
///
/// For more details see https://stripe.com/docs/api#order_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub amount_returned: u64,
    pub application: String,
    pub application_fee: u64,
    pub charge: Option<String>,
    pub created: Timestamp,
    pub currency: Currency,
    pub customer: String,
    pub email: String,
    pub external_coupon_code: String,
    pub items: List<OrderItem>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub returns: List<serde_json::Value>,
    pub selected_shipping_method: Option<String>,
    pub shipping: Option<ShippingDetails>,
    pub shipping_methods: List<serde_json::Value>,
    pub status: String, // (created, paid, canceled, fulfilled, returned)
    pub status_transitions: StatusTransitions,
    pub udpated: Timestamp,
    pub upstream_id: Option<String>,
}

impl Identifiable for Order {
    fn id(&self) -> &str {
        &self.id
    }
}
