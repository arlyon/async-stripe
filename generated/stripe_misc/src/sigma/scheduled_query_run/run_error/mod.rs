#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RunError {
    /// Information about the run failure.
    pub message: String,
}
