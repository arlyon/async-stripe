#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Link {
    /// Two-letter ISO code representing the funding source (i.e.
    ///
    /// card, bank) country beneath the Link payment method. You could use this attribute to get a sense of the international breakdown of funding sources you've collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Account owner's email address.
    pub email: Option<String>,
    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,
}
