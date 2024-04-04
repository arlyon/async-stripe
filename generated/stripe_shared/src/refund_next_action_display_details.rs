#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RefundNextActionDisplayDetails {
    pub email_sent: stripe_shared::EmailSent,
    /// The expiry timestamp.
    pub expires_at: stripe_types::Timestamp,
}
