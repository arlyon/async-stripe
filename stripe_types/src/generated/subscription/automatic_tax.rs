#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AutomaticTax {
    /// Whether Stripe automatically computes tax on this subscription.
    pub enabled: bool,
}
