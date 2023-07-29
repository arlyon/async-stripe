#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AutomaticPaymentMethodsPaymentIntent {
    /// Automatically calculates compatible payment methods.
    pub enabled: bool,
}
