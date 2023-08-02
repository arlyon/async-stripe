#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NextActionDisplayDetails {
    pub email_sent: stripe_types::email_sent::EmailSent,
    /// The expiry timestamp.
    pub expires_at: stripe_types::Timestamp,
}
