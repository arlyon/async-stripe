#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Redirect {
    /// The URL the customer will be redirected to after the purchase is complete.
    pub url: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Redirect {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
