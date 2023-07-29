/// Represents a reader action to process a payment intent.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ProcessPaymentIntentAction {
    /// Most recent PaymentIntent processed by the reader.
pub payment_intent: stripe_types::Expandable<stripe_types::payment_intent::PaymentIntent>,
#[serde(skip_serializing_if = "Option::is_none")]
pub process_config: Option<stripe_terminal::terminal::reader::reader_action::process_payment_intent_action::process_config::ProcessConfig>,

}
pub mod process_config;
pub use process_config::ProcessConfig;
pub mod tipping_config;
pub use tipping_config::TippingConfig;
