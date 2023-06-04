/// Represents a reader action to process a setup intent.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ProcessSetupIntentAction {
    /// ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions.
    ///
    /// Only present if it was possible to generate a card PaymentMethod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_card: Option<String>,
    /// Most recent SetupIntent processed by the reader.
    pub setup_intent: crate::Expandable<crate::setup_intent::SetupIntent>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ProcessSetupIntentAction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
