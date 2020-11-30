// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Expandable, Object};
use crate::resources::{Currency, Sku};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "OrderItem".
///
/// For more details see [https://stripe.com/docs/api/order_items/object](https://stripe.com/docs/api/order_items/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderItem {
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the line item.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Description of the line item, meant to be displayable to the user (e.g., `"Express shipping"`).
    pub description: String,

    /// The ID of the associated object for this line item.
    ///
    /// Expandable if not null (e.g., expandable to a SKU).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Expandable<Sku>>,

    /// A positive integer representing the number of instances of `parent` that are included in this order item.
    ///
    /// Applicable/present only if `type` is `sku`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The type of line item.
    ///
    /// One of `sku`, `tax`, `shipping`, or `discount`.
    #[serde(rename = "type")]
    pub type_: String,
}

impl Object for OrderItem {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "order_item"
    }
}
