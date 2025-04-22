use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct CreateTerminalConnectionTokenBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
}
impl CreateTerminalConnectionTokenBuilder {
    fn new() -> Self {
        Self { expand: None, location: None }
    }
}
/// To connect to a reader the Stripe Terminal SDK needs to retrieve a short-lived connection token from Stripe, proxied through your server.
/// On your backend, add an endpoint that creates and returns a connection token.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalConnectionToken {
    inner: CreateTerminalConnectionTokenBuilder,
}
impl CreateTerminalConnectionToken {
    /// Construct a new `CreateTerminalConnectionToken`.
    pub fn new() -> Self {
        Self { inner: CreateTerminalConnectionTokenBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The id of the location that this connection token is scoped to.
    /// If specified the connection token will only be usable with readers assigned to that location, otherwise the connection token will be usable with all readers.
    /// Note that location scoping only applies to internet-connected readers.
    /// For more details, see [the docs on scoping connection tokens](https://docs.stripe.com/terminal/fleet/locations-and-zones?dashboard-or-api=api#connection-tokens).
    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.inner.location = Some(location.into());
        self
    }
}
impl Default for CreateTerminalConnectionToken {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateTerminalConnectionToken {
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

impl StripeRequest for CreateTerminalConnectionToken {
    type Output = stripe_terminal::TerminalConnectionToken;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/terminal/connection_tokens").form(&self.inner)
    }
}
