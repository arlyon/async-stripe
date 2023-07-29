#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CurrencyOption {
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    pub minimum_amount: i64,
}
