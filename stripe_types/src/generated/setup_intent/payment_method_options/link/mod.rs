#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Link {
    /// Token used for persistent Link logins.
    pub persistent_token: Option<String>,
}
