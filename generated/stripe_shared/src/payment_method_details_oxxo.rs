#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsOxxo {
    /// OXXO reference number
    pub number: Option<String>,
}
