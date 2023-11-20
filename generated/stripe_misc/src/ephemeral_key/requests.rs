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
    /// A single-use token, created by Stripe.js, used for creating ephemeral keys for Issuing Cards without exchanging sensitive information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<&'a str>,
    /// The ID of the Identity VerificationSession you'd like to access using the resulting ephemeral key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_session: Option<&'a str>,
}
impl<'a> CreateEphemeralKey<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CreateEphemeralKey<'a> {
    /// Creates a short-lived API key for a given resource.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_misc::EphemeralKey> {
        client.send_form("/ephemeral_keys", self, http_types::Method::Post)
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
impl<'a> DeleteEphemeralKey<'a> {
    /// Invalidates a short-lived API key for a given resource.
    pub fn send(
        &self,
        client: &stripe::Client,
        key: &stripe_misc::ephemeral_key::EphemeralKeyId,
    ) -> stripe::Response<stripe_misc::EphemeralKey> {
        client.send_form(
            &format!("/ephemeral_keys/{key}", key = key),
            self,
            http_types::Method::Delete,
        )
    }
}
