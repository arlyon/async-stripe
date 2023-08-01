#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AppliesTo {
    /// A list of product IDs this coupon applies to.
    pub products: Vec<String>,
}
