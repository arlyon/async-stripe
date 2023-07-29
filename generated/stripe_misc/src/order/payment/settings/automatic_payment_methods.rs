#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AutomaticPaymentMethods {
    /// Whether this Order has been opted into managing payment method types via the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    pub enabled: bool,
}
