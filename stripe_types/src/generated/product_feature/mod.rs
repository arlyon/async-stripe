#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ProductFeature {
    /// The feature's name.
    ///
    /// Up to 80 characters long.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
