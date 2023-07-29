#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SingleUse {
    /// On a single use mandate, the amount of the payment.
    pub amount: i64,
    /// On a single use mandate, the currency of the payment.
    pub currency: stripe_types::Currency,
}
