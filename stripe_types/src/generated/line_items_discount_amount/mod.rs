#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LineItemsDiscountAmount {
    /// The amount discounted.
    pub amount: i64,
    pub discount: stripe_types::Discount,
}
