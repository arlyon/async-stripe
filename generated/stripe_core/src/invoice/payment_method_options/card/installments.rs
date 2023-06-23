#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Installments {
    /// Whether Installments are enabled for this Invoice.
    pub enabled: Option<bool>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Installments {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
