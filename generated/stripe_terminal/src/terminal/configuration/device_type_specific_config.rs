#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeviceTypeSpecificConfig {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<stripe_types::Expandable<stripe_core::file::File>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeviceTypeSpecificConfig {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
