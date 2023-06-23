/// A discount represents the actual application of a [coupon](https://stripe.com/docs/api#coupons) or [promotion code](https://stripe.com/docs/api#promotion_codes).
/// It contains information about when the discount began, when it will end, and what it is applied to.
///
/// Related guide: [Applying Discounts to Subscriptions](https://stripe.com/docs/billing/subscriptions/discounts).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Discount {
    /// The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode.
    ///
    /// Will not be present for subscription mode.
    pub checkout_session: Option<String>,
    pub coupon: stripe_core::coupon::Coupon,
    /// The ID of the customer associated with this discount.
    pub customer: Option<stripe_types::Expandable<stripe_core::customer::Customer>>,
    /// If the coupon has a duration of `repeating`, the date that this discount will end.
    ///
    /// If the coupon has a duration of `once` or `forever`, this attribute will be null.
    pub end: Option<stripe_types::Timestamp>,
    /// The ID of the discount object.
    ///
    /// Discounts cannot be fetched by ID.
    /// Use `expand[]=discounts` in API calls to expand discount IDs in an array.
    pub id: stripe_core::discount::DiscountId,
    /// The invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice.
    pub invoice: Option<String>,
    /// The invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item.
    pub invoice_item: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DiscountObject,
    /// The promotion code applied to create this discount.
    pub promotion_code:
        Option<stripe_types::Expandable<stripe_core::promotion_code::PromotionCode>>,
    /// Date that the coupon was applied.
    pub start: stripe_types::Timestamp,
    /// The subscription that this coupon is applied to, if it is applied to a particular subscription.
    pub subscription: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Discount {
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
pub enum DiscountObject {
    Discount,
}

impl DiscountObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Discount => "discount",
        }
    }
}

impl AsRef<str> for DiscountObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DiscountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for Discount {
    type Id = stripe_core::discount::DiscountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(DiscountId, "di_");
pub mod deleted;
pub use deleted::DeletedDiscount;
