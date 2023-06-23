#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct HostedConfirmation {
    /// The custom message that is displayed to the customer after the purchase is complete.
    pub custom_message: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for HostedConfirmation {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
