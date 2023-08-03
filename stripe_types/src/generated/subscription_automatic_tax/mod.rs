#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionAutomaticTax {
    /// Whether Stripe automatically computes tax on this subscription.
    pub enabled: bool,
}
