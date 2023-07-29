#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Link {
    /// Account owner's email address.
    pub email: Option<String>,
    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,
}
