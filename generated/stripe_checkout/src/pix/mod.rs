#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Pix {
    /// The number of seconds after which Pix payment will expire.
    pub expires_after_seconds: Option<i64>,
}
