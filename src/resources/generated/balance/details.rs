#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Details {
    /// Funds that are available for use.
    pub available: Vec<crate::balance::money::Money>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Details {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
