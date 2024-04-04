#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomTextPosition {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}
