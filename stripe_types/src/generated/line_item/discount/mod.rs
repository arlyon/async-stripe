#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Discount {
    /// The amount discounted.
    pub amount: i64,
    pub discount: stripe_types::discount::Discount,
}
