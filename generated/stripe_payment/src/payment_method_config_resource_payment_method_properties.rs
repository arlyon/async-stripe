#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodConfigResourcePaymentMethodProperties {
    /// Whether this payment method may be offered at checkout.
    /// True if `display_preference` is `on` and the payment method's capability is active.
    pub available: bool,
    pub display_preference: stripe_payment::PaymentMethodConfigResourceDisplayPreference,
}
