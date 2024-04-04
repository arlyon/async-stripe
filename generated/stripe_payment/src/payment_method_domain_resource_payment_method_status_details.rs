/// Contains additional details about the status of a payment method for a specific payment method domain.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDomainResourcePaymentMethodStatusDetails {
    /// The error message associated with the status of the payment method on the domain.
    pub error_message: String,
}
