// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::resources::{Discount, TaxRate};

/// The resource representing a Stripe "QuotesResourceTotalDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct QuotesResourceTotalDetails {
    /// This is the sum of all the discounts.
    pub amount_discount: i64,

    /// This is the sum of all the shipping amounts.
    pub amount_shipping: Option<i64>,

    /// This is the sum of all the tax amounts.
    pub amount_tax: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<QuotesResourceTotalDetailsResourceBreakdown>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct QuotesResourceTotalDetailsResourceBreakdown {
    /// The aggregated discounts.
    pub discounts: Vec<LineItemsDiscountAmount>,

    /// The aggregated tax amounts by rate.
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
