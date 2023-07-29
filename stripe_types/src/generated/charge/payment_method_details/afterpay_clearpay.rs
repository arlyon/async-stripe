#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AfterpayClearpay {
    /// Order identifier shown to the merchant in Afterpay’s online portal.
    pub reference: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AfterpayClearpay {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
