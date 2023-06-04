#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct RunError {
    /// Information about the run failure.
    pub message: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RunError {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
