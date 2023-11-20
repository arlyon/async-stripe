#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MandateSingleUse {
    /// The amount of the payment on a single use mandate.
    pub amount: i64,
    /// The currency of the payment on a single use mandate.
    pub currency: stripe_types::Currency,
}
