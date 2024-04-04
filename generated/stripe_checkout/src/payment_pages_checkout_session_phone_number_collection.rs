#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionPhoneNumberCollection {
    /// Indicates whether phone number collection is enabled for the session
    pub enabled: bool,
}
