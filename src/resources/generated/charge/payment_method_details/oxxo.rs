#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Oxxo {
    /// OXXO reference number.
    pub number: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Oxxo {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
