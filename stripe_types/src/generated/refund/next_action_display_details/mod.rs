#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NextActionDisplayDetails {
    pub email_sent: stripe_types::refund::next_action_display_details::email_sent::EmailSent,
    /// The expiry timestamp.
    pub expires_at: stripe_types::Timestamp,
}
pub mod email_sent;
pub use email_sent::EmailSent;
