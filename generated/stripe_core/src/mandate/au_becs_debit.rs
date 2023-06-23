#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AuBecsDebit {
    /// The URL of the mandate.
    ///
    /// This URL generally contains sensitive information about the customer and should be shared with them exclusively.
    pub url: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AuBecsDebit {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
