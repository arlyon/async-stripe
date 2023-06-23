/// A Promotion Code represents a customer-redeemable code for a [coupon](https://stripe.com/docs/api#coupons).
///
/// It can be used to create multiple codes for a single coupon.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PromotionCode {
    /// Whether the promotion code is currently active.
    ///
    /// A promotion code is only active if the coupon is also valid.
    pub active: bool,
    /// The customer-facing code.
    ///
    /// Regardless of case, this code must be unique across all active promotion codes for each customer.
    pub code: String,
    pub coupon: stripe_core::coupon::Coupon,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The customer that this promotion code can be used by.
    pub customer: Option<stripe_types::Expandable<stripe_core::customer::Customer>>,
    /// Date at which the promotion code can no longer be redeemed.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_core::promotion_code::PromotionCodeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Maximum number of times this promotion code can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<stripe_types::Metadata>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: PromotionCodeObject,
    pub restrictions: stripe_core::promotion_code::restrictions::Restrictions,
    /// Number of times this promotion code has been used.
    pub times_redeemed: i64,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PromotionCode {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PromotionCodeObject {
    PromotionCode,
}

impl PromotionCodeObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PromotionCode => "promotion_code",
        }
    }
}

impl AsRef<str> for PromotionCodeObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PromotionCodeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for PromotionCode {
    type Id = stripe_core::promotion_code::PromotionCodeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(PromotionCodeId, "promo_");
pub mod currency_option;
pub mod requests;
pub use currency_option::CurrencyOption;
pub mod restrictions;
pub use restrictions::Restrictions;
