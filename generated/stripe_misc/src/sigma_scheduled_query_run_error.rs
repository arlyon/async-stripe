#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SigmaScheduledQueryRunError {
    /// Information about the run failure.
    pub message: String,
}
