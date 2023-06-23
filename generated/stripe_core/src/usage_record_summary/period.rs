#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Period {
    /// The end date of this usage period.
    ///
    /// All usage up to and including this point in time is included.
    pub end: Option<stripe_types::Timestamp>,
    /// The start date of this usage period.
    ///
    /// All usage after this point in time is included.
    pub start: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Period {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
