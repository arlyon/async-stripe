
/// Creates a short-lived API key for a given resource.
pub fn create(client: &stripe::Client, params: CreateEphemeralKey) -> stripe::Response<stripe_misc::EphemeralKey> {
    client.send_form("/ephemeral_keys", params, http_types::Method::Post)
}
/// Invalidates a short-lived API key for a given resource.
pub fn delete(client: &stripe::Client, key: &stripe_misc::ephemeral_key::EphemeralKeyId, params: DeleteEphemeralKey) -> stripe::Response<stripe_misc::EphemeralKey> {
    client.send_form(&format!("/ephemeral_keys/{key}", key = key), params, http_types::Method::Delete)
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateEphemeralKey<'a> {
    /// The ID of the Customer you'd like to modify using the resulting ephemeral key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The ID of the Issuing Card you'd like to access using the resulting ephemeral key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<&'a str>,
    /// The ID of the Identity VerificationSession you'd like to access using the resulting ephemeral key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_session: Option<&'a str>,
}
impl<'a> CreateEphemeralKey<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteEphemeralKey<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> DeleteEphemeralKey<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
