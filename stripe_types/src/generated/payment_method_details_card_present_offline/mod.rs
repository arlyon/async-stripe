#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsCardPresentOffline {
    /// Time at which the payment was collected while offline.
    pub stored_at: Option<stripe_types::Timestamp>,
}
