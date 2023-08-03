#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QuotesResourceTotalDetailsResourceBreakdown {
    /// The aggregated discounts.
    pub discounts: Vec<stripe_types::LineItemsDiscountAmount>,
    /// The aggregated tax amounts by rate.
    pub taxes: Vec<stripe_types::LineItemsTaxAmount>,
}
