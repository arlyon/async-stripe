#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    /// The aggregated discounts.
    pub discounts: Vec<stripe_shared::LineItemsDiscountAmount>,
    /// The aggregated tax amounts by rate.
    pub taxes: Vec<stripe_shared::LineItemsTaxAmount>,
}
