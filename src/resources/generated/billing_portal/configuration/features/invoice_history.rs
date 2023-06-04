#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceHistory {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoiceHistory {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
