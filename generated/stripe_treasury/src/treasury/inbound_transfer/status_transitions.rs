#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct StatusTransitions {
    /// Timestamp describing when an InboundTransfer changed status to `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an InboundTransfer changed status to `failed`.
    pub failed_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an InboundTransfer changed status to `succeeded`.
    pub succeeded_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for StatusTransitions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
