use serde_derive::{Deserialize, Serialize};

// ======================================
// This file was automatically generated.
// ======================================
use crate::ids::CheckoutSessionItemId;
use crate::params::Object;
use crate::resources::{Currency, Discount, Price, TaxRate};
use crate::resources::{LineItemsDiscountAmount, LineItemsTaxAmount};

/// The resource representing a Stripe "LineItem".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutSessionItem {
    /// Unique identifier for the object.
    pub id: CheckoutSessionItemId,

    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,

    /// Total after discounts and taxes.
    pub amount_total: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Defaults to product name.
    pub description: String,

    /// The discounts applied to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Box<Vec<LineItemsDiscountAmount>>>,

    /// The price used to generate the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<Price>>,

    /// The quantity of products being purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Box<u64>>,

    /// The taxes applied to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Box<Vec<LineItemsTaxAmount>>>,
}

//automatically added back in service of CheckoutSessionItem with hash-7800975525549346558
impl Object for CheckoutSessionItem {
    type Id = CheckoutSessionItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "item"
    }
}
