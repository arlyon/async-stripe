#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsCardPresent {
    /// The ID of the Card PaymentMethod which was generated by this SetupAttempt.
    pub generated_card: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
}
