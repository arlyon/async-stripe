use crate::params::Timestamp;
use crate::resources::Coupon;
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe discount.
///
/// For more details see https://stripe.com/docs/api#discounts.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Discount {
    pub coupon: Coupon,
    pub customer: String,
    pub subscription: Option<String>,

    pub start: Timestamp,
    pub end: Option<Timestamp>,
}
