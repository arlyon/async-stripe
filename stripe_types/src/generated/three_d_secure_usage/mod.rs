#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ThreeDSecureUsage {
    /// Whether 3D Secure is supported on this card.
    pub supported: bool,
}
