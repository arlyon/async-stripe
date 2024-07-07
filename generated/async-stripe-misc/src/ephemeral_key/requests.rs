use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct DeleteEphemeralKeyBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> DeleteEphemeralKeyBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Invalidates a short-lived API key for a given resource.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteEphemeralKey<'a> {
    inner: DeleteEphemeralKeyBuilder<'a>,
    key: &'a stripe_misc::EphemeralKeyId,
}
impl<'a> DeleteEphemeralKey<'a> {
    /// Construct a new `DeleteEphemeralKey`.
    pub fn new(key: &'a stripe_misc::EphemeralKeyId) -> Self {
        Self { key, inner: DeleteEphemeralKeyBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl DeleteEphemeralKey<'_> {
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

impl StripeRequest for DeleteEphemeralKey<'_> {
    type Output = stripe_misc::EphemeralKey;

    fn build(&self) -> RequestBuilder {
        let key = self.key;
        RequestBuilder::new(StripeMethod::Delete, format!("/ephemeral_keys/{key}"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateEphemeralKeyBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuing_card: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_session: Option<&'a str>,
}
impl<'a> CreateEphemeralKeyBuilder<'a> {
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
pub struct CreateEphemeralKey<'a> {
    inner: CreateEphemeralKeyBuilder<'a>,
}
impl<'a> CreateEphemeralKey<'a> {
    /// Construct a new `CreateEphemeralKey`.
    pub fn new() -> Self {
        Self { inner: CreateEphemeralKeyBuilder::new() }
    }
    /// The ID of the Customer you'd like to modify using the resulting ephemeral key.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The ID of the Issuing Card you'd like to access using the resulting ephemeral key.
    pub fn issuing_card(mut self, issuing_card: &'a str) -> Self {
        self.inner.issuing_card = Some(issuing_card);
        self
    }
    /// A single-use token, created by Stripe.js, used for creating ephemeral keys for Issuing Cards without exchanging sensitive information.
    pub fn nonce(mut self, nonce: &'a str) -> Self {
        self.inner.nonce = Some(nonce);
        self
    }
    /// The ID of the Identity VerificationSession you'd like to access using the resulting ephemeral key
    pub fn verification_session(mut self, verification_session: &'a str) -> Self {
        self.inner.verification_session = Some(verification_session);
        self
    }
}
impl<'a> Default for CreateEphemeralKey<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateEphemeralKey<'_> {
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

impl StripeRequest for CreateEphemeralKey<'_> {
    type Output = stripe_misc::EphemeralKey;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/ephemeral_keys").form(&self.inner)
    }
}
