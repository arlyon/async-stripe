#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodLink {
    /// Account owner's email address.
    pub email: Option<String>,
    /// [Deprecated] This is a legacy parameter that no longer has any function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,
}
