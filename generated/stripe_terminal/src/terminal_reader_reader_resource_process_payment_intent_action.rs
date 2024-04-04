/// Represents a reader action to process a payment intent
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalReaderReaderResourceProcessPaymentIntentAction {
    /// Most recent PaymentIntent processed by the reader.
    pub payment_intent: stripe_types::Expandable<stripe_shared::PaymentIntent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_config: Option<stripe_terminal::TerminalReaderReaderResourceProcessConfig>,
}
