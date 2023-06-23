#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct StatusTransitions {
    /// Timestamp describing when the Transaction changed status to `posted`.
    pub posted_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when the Transaction changed status to `void`.
    pub void_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for StatusTransitions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
