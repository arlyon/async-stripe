#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionTaxIdCollection {
    /// Indicates whether tax ID collection is enabled for the session
    pub enabled: bool,
}
