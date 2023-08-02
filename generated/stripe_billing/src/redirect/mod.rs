#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Redirect {
    /// The URL the customer will be redirected to after the flow is completed.
    pub return_url: String,
}
