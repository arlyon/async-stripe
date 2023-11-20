#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DiscountsResourceDiscountAmount {
    /// The amount, in cents (or local equivalent), of the discount.
    pub amount: i64,
    /// The discount that was applied to get this discount amount.
    pub discount: stripe_types::Expandable<stripe_types::Discount>,
}
