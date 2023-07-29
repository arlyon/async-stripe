#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NextActionCardAwaitNotification {
    /// The time that payment will be attempted.
    ///
    /// If customer approval is required, they need to provide approval before this time.
    pub charge_attempt_at: Option<stripe_types::Timestamp>,
    /// For payments greater than INR 15000, the customer must provide explicit approval of the payment with their bank.
    ///
    /// For payments of lower amount, no customer action is required.
    pub customer_approval_required: Option<bool>,
}
