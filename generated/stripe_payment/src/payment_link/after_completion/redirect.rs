#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Redirect {
    /// The URL the customer will be redirected to after the purchase is complete.
    pub url: String,
}
