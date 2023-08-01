#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EmailSent {
    /// The timestamp when the email was sent.
    pub email_sent_at: stripe_types::Timestamp,
    /// The recipient's email address.
    pub email_sent_to: String,
}
