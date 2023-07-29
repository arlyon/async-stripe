#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct StatusTransitions {
    /// The time that the quote was accepted.
    ///
    /// Measured in seconds since Unix epoch.
    pub accepted_at: Option<stripe_types::Timestamp>,
    /// The time that the quote was canceled.
    ///
    /// Measured in seconds since Unix epoch.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// The time that the quote was finalized.
    ///
    /// Measured in seconds since Unix epoch.
    pub finalized_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for StatusTransitions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
