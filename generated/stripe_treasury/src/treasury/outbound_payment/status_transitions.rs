#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct StatusTransitions {
    /// Timestamp describing when an OutboundPayment changed status to `canceled`.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundPayment changed status to `failed`.
    pub failed_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundPayment changed status to `posted`.
    pub posted_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundPayment changed status to `returned`.
    pub returned_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for StatusTransitions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}