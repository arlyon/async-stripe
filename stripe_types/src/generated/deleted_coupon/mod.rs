#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedCoupon {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::coupon::CouponId,
}
impl stripe_types::Object for DeletedCoupon {
    type Id = stripe_types::coupon::CouponId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
