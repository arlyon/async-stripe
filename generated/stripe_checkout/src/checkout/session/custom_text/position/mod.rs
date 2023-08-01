#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Position {
    /// Text may be up to 1000 characters in length.
    pub message: String,
}
