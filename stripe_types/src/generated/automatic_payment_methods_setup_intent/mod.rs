#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AutomaticPaymentMethodsSetupIntent {
    /// Automatically calculates compatible payment methods.
    pub enabled: Option<bool>,
}
