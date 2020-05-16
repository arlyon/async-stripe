// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::CheckoutSessionItemId;
use crate::params::Object;
use crate::resources::{Currency, Price, TaxRate};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "PaymentPagesCheckoutSessionLineItem".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutSessionItem {
    /// Unique identifier for the object.
    pub id: CheckoutSessionItemId,

    /// Total before any discounts or taxes is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_subtotal: Option<i64>,

    /// Total after discounts and taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_total: Option<i64>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Defaults to product name.
    pub description: String,

    pub price: Price,

    /// The quantity of products being purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The taxes applied to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<PaymentPagesCheckoutSessionLineItemResourceLineItemTax>>,
}

impl Object for CheckoutSessionItem {
    type Id = CheckoutSessionItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "item"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionLineItemResourceLineItemTax {
    /// Amount of tax for this line item.
    pub amount: i64,

    pub rate: TaxRate,
}
