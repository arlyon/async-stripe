// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::params::Expandable;
use crate::resources::{ShippingRate, TaxRate};

/// The resource representing a Stripe "InvoicesShippingCost".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesShippingCost {
    /// Total shipping cost before any taxes are applied.
    pub amount_subtotal: i64,

    /// Total tax amount applied due to shipping costs.
    ///
    /// If no tax was applied, defaults to 0.
    pub amount_tax: i64,

    /// Total shipping cost after taxes are applied.
    pub amount_total: i64,

    /// The ID of the ShippingRate for this invoice.
    pub shipping_rate: Option<Expandable<ShippingRate>>,

    /// The taxes applied to the shipping rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LineItemsTaxAmount {
    /// Amount of tax applied for this rate.
    pub amount: i64,

    pub rate: TaxRate,
}
