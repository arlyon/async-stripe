#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DeviceTypeSpecificConfig {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<stripe_types::Expandable<stripe_types::file::File>>,
}
