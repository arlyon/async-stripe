// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{Coupon, Customer};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Discount".
///
/// For more details see [https://stripe.com/docs/api/discounts/object](https://stripe.com/docs/api/discounts/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Discount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<Coupon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// If the coupon has a duration of `repeating`, the date that this discount will end.
    ///
    /// If the coupon has a duration of `once` or `forever`, this attribute will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Timestamp>,

    /// Date that the coupon was applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Timestamp>,

    /// The subscription that this coupon is applied to, if it is applied to a particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
}

impl Object for Discount {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "discount"
    }
}
