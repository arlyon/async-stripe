use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct DeleteEphemeralKeyBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl DeleteEphemeralKeyBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Invalidates a short-lived API key for a given resource.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct DeleteEphemeralKey {
    inner: DeleteEphemeralKeyBuilder,
    key: stripe_misc::EphemeralKeyId,
}
impl DeleteEphemeralKey {
    /// Construct a new `DeleteEphemeralKey`.
    pub fn new(key: impl Into<stripe_misc::EphemeralKeyId>) -> Self {
        Self { key: key.into(), inner: DeleteEphemeralKeyBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl DeleteEphemeralKey {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for DeleteEphemeralKey {
    type Output = stripe_misc::EphemeralKey;

    fn build(&self) -> RequestBuilder {
        let key = &self.key;
        RequestBuilder::new(StripeMethod::Delete, format!("/ephemeral_keys/{key}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateEphemeralKeyBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuing_card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_session: Option<String>,
}
impl CreateEphemeralKeyBuilder {
    fn new() -> Self {
        Self {
            customer: None,
            expand: None,
            issuing_card: None,
            nonce: None,
            verification_session: None,
        }
    }
}
/// Creates a short-lived API key for a given resource.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateEphemeralKey {
    inner: CreateEphemeralKeyBuilder,
}
impl CreateEphemeralKey {
    /// Construct a new `CreateEphemeralKey`.
    pub fn new() -> Self {
        Self { inner: CreateEphemeralKeyBuilder::new() }
    }
    /// The ID of the Customer you'd like to modify using the resulting ephemeral key.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The ID of the Issuing Card you'd like to access using the resulting ephemeral key.
    pub fn issuing_card(mut self, issuing_card: impl Into<String>) -> Self {
        self.inner.issuing_card = Some(issuing_card.into());
        self
    }
    /// A single-use token, created by Stripe.js, used for creating ephemeral keys for Issuing Cards without exchanging sensitive information.
    pub fn nonce(mut self, nonce: impl Into<String>) -> Self {
        self.inner.nonce = Some(nonce.into());
        self
    }
    /// The ID of the Identity VerificationSession you'd like to access using the resulting ephemeral key
    pub fn verification_session(mut self, verification_session: impl Into<String>) -> Self {
        self.inner.verification_session = Some(verification_session.into());
        self
    }
}
impl Default for CreateEphemeralKey {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateEphemeralKey {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateEphemeralKey {
    type Output = stripe_misc::EphemeralKey;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/ephemeral_keys").form(&self.inner)
    }
}
