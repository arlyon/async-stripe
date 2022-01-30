// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::DiscountId;
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{Coupon, Customer, PromotionCode};

/// The resource representing a Stripe "Discount".
///
/// For more details see <https://stripe.com/docs/api/discounts/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Discount {
    /// The ID of the discount object.
    ///
    /// Discounts cannot be fetched by ID.
    /// Use `expand[]=discounts` in API calls to expand discount IDs in an array.
    pub id: DiscountId,

    /// The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode.
    ///
    /// Will not be present for subscription mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_session: Option<Box<String>>,

    pub coupon: Coupon,

    /// The ID of the customer associated with this discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<Expandable<Customer>>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// If the coupon has a duration of `repeating`, the date that this discount will end.
    ///
    /// If the coupon has a duration of `once` or `forever`, this attribute will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Box<Timestamp>>,

    /// The invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Box<String>>,

    /// The invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<Box<String>>,

    /// The promotion code applied to create this discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<Box<Expandable<PromotionCode>>>,

    /// Date that the coupon was applied.
    pub start: Timestamp,

    /// The subscription that this coupon is applied to, if it is applied to a particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Box<String>>,
}

impl Object for Discount {
    type Id = DiscountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "discount"
    }
}
