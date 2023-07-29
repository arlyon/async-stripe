#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerNotification {
    /// Whether customer approval has been requested for this payment.
    ///
    /// For payments greater than INR 15000 or mandate amount, the customer must provide explicit approval of the payment with their bank.
    pub approval_requested: Option<bool>,
    /// If customer approval is required, they need to provide approval before this time.
    pub completes_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerNotification {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
