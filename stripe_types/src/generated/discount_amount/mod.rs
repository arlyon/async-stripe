#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DiscountAmount {
    /// The amount, in %s, of the discount.
    pub amount: i64,
    /// The discount that was applied to get this discount amount.
    pub discount: stripe_types::Expandable<stripe_types::discount::Discount>,
}
