/// Represents a reader action to process a payment intent.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ProcessPaymentIntentAction {
    /// Most recent PaymentIntent processed by the reader.
pub payment_intent: crate::Expandable<crate::payment_intent::PaymentIntent>,
#[serde(skip_serializing_if = "Option::is_none")]
pub process_config: Option<crate::terminal::reader::reader_action::process_payment_intent_action::process_config::ProcessConfig>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ProcessPaymentIntentAction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod process_config;
pub use process_config::ProcessConfig;
pub mod tipping_config;
pub use tipping_config::TippingConfig;
