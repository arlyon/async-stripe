#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Sofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Sofort {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
