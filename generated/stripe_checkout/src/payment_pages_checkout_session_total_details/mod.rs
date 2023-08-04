#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionTotalDetails {
    /// This is the sum of all the discounts.
    pub amount_discount: i64,
    /// This is the sum of all the shipping amounts.
    pub amount_shipping: Option<i64>,
    /// This is the sum of all the tax amounts.
    pub amount_tax: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown:
        Option<stripe_checkout::PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown>,
}
