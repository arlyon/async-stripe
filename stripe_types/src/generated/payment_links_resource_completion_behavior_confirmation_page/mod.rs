#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceCompletionBehaviorConfirmationPage {
    /// The custom message that is displayed to the customer after the purchase is complete.
    pub custom_message: Option<String>,
}
