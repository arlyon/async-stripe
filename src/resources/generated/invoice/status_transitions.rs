#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct StatusTransitions {
    /// The time that the invoice draft was finalized.
    pub finalized_at: Option<crate::Timestamp>,
    /// The time that the invoice was marked uncollectible.
    pub marked_uncollectible_at: Option<crate::Timestamp>,
    /// The time that the invoice was paid.
    pub paid_at: Option<crate::Timestamp>,
    /// The time that the invoice was voided.
    pub voided_at: Option<crate::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for StatusTransitions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
