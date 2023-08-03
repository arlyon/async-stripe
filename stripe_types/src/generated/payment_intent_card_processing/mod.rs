#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentCardProcessing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notification: Option<stripe_types::PaymentIntentProcessingCustomerNotification>,
}
