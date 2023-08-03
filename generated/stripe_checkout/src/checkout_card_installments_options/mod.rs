#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutCardInstallmentsOptions {
    /// Indicates if installments are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
