#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodKlarna {
    /// The customer's date of birth, if provided.
    pub dob: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKlarnaDob>,
}
