#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxIdCollection {
    /// Indicates whether tax ID collection is enabled for the session.
    pub enabled: bool,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxIdCollection {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
