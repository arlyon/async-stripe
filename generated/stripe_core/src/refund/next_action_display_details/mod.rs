#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct NextActionDisplayDetails {
    pub email_sent: stripe_core::refund::next_action_display_details::email_sent::EmailSent,
    /// The expiry timestamp.
    pub expires_at: stripe_types::Timestamp,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for NextActionDisplayDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod email_sent;
pub use email_sent::EmailSent;
