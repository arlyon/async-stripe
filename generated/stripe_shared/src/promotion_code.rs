/// A Promotion Code represents a customer-redeemable code for a [coupon](https://stripe.com/docs/api#coupons).
/// It can be used to.
/// create multiple codes for a single coupon.
///
/// For more details see <<https://stripe.com/docs/api/promotion_codes/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PromotionCode {
    /// Whether the promotion code is currently active.
    /// A promotion code is only active if the coupon is also valid.
    pub active: bool,
    /// The customer-facing code.
    /// Regardless of case, this code must be unique across all active promotion codes for each customer.
    pub code: String,
    pub coupon: stripe_shared::Coupon,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The customer that this promotion code can be used by.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// Date at which the promotion code can no longer be redeemed.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_shared::PromotionCodeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Maximum number of times this promotion code can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub restrictions: stripe_shared::PromotionCodesResourceRestrictions,
    /// Number of times this promotion code has been used.
    pub times_redeemed: i64,
}
impl stripe_types::Object for PromotionCode {
    type Id = stripe_shared::PromotionCodeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PromotionCodeId, "promo_");
