/// Represents a reader action to process a setup intent.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalReaderReaderResourceProcessSetupIntentAction {
    /// ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions.
    ///
    /// Only present if it was possible to generate a card PaymentMethod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_card: Option<String>,
    /// Most recent SetupIntent processed by the reader.
    pub setup_intent: stripe_types::Expandable<stripe_types::SetupIntent>,
}
