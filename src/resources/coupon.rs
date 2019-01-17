use crate::params::{Identifiable, Metadata, Timestamp};
use crate::resources::Currency;
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe coupon.
///
/// For more details see https://stripe.com/docs/api#coupon_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Coupon {
    pub id: String,
    pub object: String,
    pub amount_off: Option<u64>,
    pub created: Timestamp,
    pub currency: Option<Currency>,
    pub duration: String, // (forever, once, repeating)
    pub duration_in_months: Option<u64>,
    pub livemode: bool,
    pub max_redemptions: Option<u64>,
    pub metadata: Metadata,
    pub percent_off: u64, // eg. 50 => 50%
    pub redeem_by: Timestamp,
    pub redeemed: u64,
    pub valid: bool,
    pub deleted: bool,
}

impl Identifiable for Coupon {
    fn id(&self) -> &str {
        &self.id
    }
}
