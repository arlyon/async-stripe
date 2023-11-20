#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectEmbeddedBaseConfigClaim {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
}
