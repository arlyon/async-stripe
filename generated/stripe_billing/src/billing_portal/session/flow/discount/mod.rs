#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Discount {
    /// The ID of the coupon to apply to this subscription update.
    pub coupon: Option<String>,
    /// The ID of a promotion code to apply to this subscription update.
    pub promotion_code: Option<String>,
}
