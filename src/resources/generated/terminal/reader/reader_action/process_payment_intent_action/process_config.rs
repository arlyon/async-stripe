/// Represents a per-transaction override of a reader configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ProcessConfig {
    /// Override showing a tipping selection screen on this transaction.
#[serde(skip_serializing_if = "Option::is_none")]
pub skip_tipping: Option<bool>,
#[serde(skip_serializing_if = "Option::is_none")]
pub tipping: Option<crate::terminal::reader::reader_action::process_payment_intent_action::tipping_config::TippingConfig>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ProcessConfig {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
