#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Link {
    /// [Deprecated] This is a legacy parameter that no longer has any function.
    pub persistent_token: Option<String>,
}
