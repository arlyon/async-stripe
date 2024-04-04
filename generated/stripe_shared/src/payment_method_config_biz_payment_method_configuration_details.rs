#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodConfigBizPaymentMethodConfigurationDetails {
    /// ID of the payment method configuration used.
    pub id: String,
    /// ID of the parent payment method configuration used.
    pub parent: Option<String>,
}
