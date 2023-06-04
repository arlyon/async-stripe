#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Pix {
    /// Unique transaction id generated by BCB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transaction_id: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Pix {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
