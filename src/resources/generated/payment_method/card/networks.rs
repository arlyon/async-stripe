#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Networks {
    /// All available networks for the card.
    pub available: Vec<String>,
    /// The preferred network for the card.
    pub preferred: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Networks {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
