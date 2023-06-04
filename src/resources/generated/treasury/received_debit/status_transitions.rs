#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct StatusTransitions {
    /// Timestamp describing when the DebitReversal changed status to `completed`.
    pub completed_at: Option<crate::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for StatusTransitions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
