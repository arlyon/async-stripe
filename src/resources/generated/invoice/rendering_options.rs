#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct RenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    pub amount_tax_display: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RenderingOptions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
