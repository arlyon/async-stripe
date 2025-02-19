// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::DiscountId;
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{Coupon, Customer, PromotionCode};
use serde::{Deserialize, Serialize};

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
    pub checkout_session: Option<String>,

    pub coupon: Coupon,

    /// The ID of the customer associated with this discount.
    pub customer: Option<Expandable<Customer>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// If the coupon has a duration of `repeating`, the date that this discount will end.
    ///
    /// If the coupon has a duration of `once` or `forever`, this attribute will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Timestamp>,

    /// The invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice.
    pub invoice: Option<String>,

    /// The invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item.
    pub invoice_item: Option<String>,

    /// The promotion code applied to create this discount.
    pub promotion_code: Option<Expandable<PromotionCode>>,

    /// Date that the coupon was applied.
    pub start: Timestamp,

    /// The subscription that this coupon is applied to, if it is applied to a particular subscription.
    pub subscription: Option<String>,
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
