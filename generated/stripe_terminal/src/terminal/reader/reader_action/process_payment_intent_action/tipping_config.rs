/// Represents a per-transaction tipping configuration.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TippingConfig {
    /// Amount used to calculate tip suggestions on tipping selection screen for this transaction.
    ///
    /// Must be a positive integer in the smallest currency unit (e.g., 100 cents to represent $1.00 or 100 to represent ¥100, a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eligible: Option<i64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TippingConfig {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
