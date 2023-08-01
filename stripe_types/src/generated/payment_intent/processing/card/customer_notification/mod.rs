#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomerNotification {
    /// Whether customer approval has been requested for this payment.
    ///
    /// For payments greater than INR 15000 or mandate amount, the customer must provide explicit approval of the payment with their bank.
    pub approval_requested: Option<bool>,
    /// If customer approval is required, they need to provide approval before this time.
    pub completes_at: Option<stripe_types::Timestamp>,
}
