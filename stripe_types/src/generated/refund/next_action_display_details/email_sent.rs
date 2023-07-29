#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct EmailSent {
    /// The timestamp when the email was sent.
    pub email_sent_at: stripe_types::Timestamp,
    /// The recipient's email address.
    pub email_sent_to: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for EmailSent {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
