/// Represents a reader action to process a payment intent.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ProcessPaymentIntentAction {
    /// Most recent PaymentIntent processed by the reader.
    pub payment_intent: stripe_types::Expandable<stripe_types::payment_intent::PaymentIntent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_config: Option<stripe_terminal::process_config::ProcessConfig>,
}
