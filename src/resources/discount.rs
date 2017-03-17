use params::Timestamp;
use resources::Coupon;

#[derive(Debug, Deserialize)]
pub struct Discount {
    pub coupon: Coupon,
    pub customer: String,
    pub subscription: Option<String>,

    pub start: Timestamp,
    pub end: Option<Timestamp>,
}
