#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsLink {
    /// Two-letter ISO code representing the funding source country beneath the Link payment.
    /// You could use this attribute to get a sense of international fees.
    pub country: Option<String>,
}
