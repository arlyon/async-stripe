use resources::Coupon;

#[derive(Deserialize)]
pub struct Discount {
    pub coupon: Coupon,
    pub customer: String,
    pub subscription: Option<String>,

    pub start: i64,       // unix timestamp
    pub end: Option<i64>, // unix timestamp
}
