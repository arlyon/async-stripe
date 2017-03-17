use params::{Metadata, Timestamp};

#[derive(Debug, Deserialize)]
pub struct Coupon {
    pub id: String,
    pub amount_off: Option<u64>,
    pub created: Timestamp,
    pub currency: Option<String>, // eg. "usd"
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
