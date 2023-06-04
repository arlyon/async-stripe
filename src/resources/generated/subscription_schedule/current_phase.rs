#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CurrentPhase {
    /// The end of this phase of the subscription schedule.
    pub end_date: crate::Timestamp,
    /// The start of this phase of the subscription schedule.
    pub start_date: crate::Timestamp,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CurrentPhase {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
