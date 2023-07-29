#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Details {
    /// Additional fields which are only required for some users.
    pub additional: Vec<String>,
    /// Fields which every account must eventually provide.
    pub minimum: Vec<String>,
}
