// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::resources::{Discount, TaxRate};

/// The resource representing a Stripe "QuotesResourceTotalDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct QuotesResourceTotalDetails {
    /// This is the sum of all the line item discounts.
    pub amount_discount: i64,

    /// This is the sum of all the line item shipping amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_shipping: Option<Box<i64>>,

    /// This is the sum of all the line item tax amounts.
    pub amount_tax: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Box<QuotesResourceTotalDetailsResourceBreakdown>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct QuotesResourceTotalDetailsResourceBreakdown {
    /// The aggregated line item discounts.
    pub discounts: Vec<LineItemsDiscountAmount>,

    /// The aggregated line item tax amounts by rate.
    pub taxes: Vec<LineItemsTaxAmount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LineItemsDiscountAmount {
    /// The amount discounted.
    pub amount: i64,

    pub discount: Discount,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LineItemsTaxAmount {
    /// Amount of tax applied for this rate.
    pub amount: i64,

    pub rate: TaxRate,
}
