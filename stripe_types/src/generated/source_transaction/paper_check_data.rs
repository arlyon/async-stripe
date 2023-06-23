#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaperCheckData {
    /// Time at which the deposited funds will be available for use.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_at: Option<String>,
    /// Comma-separated list of invoice IDs associated with the paper check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoices: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaperCheckData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
