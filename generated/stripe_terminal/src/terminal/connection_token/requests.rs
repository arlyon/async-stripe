use stripe::{Client, Response};

impl stripe_terminal::terminal::connection_token::ConnectionToken {
    /// To connect to a reader the Stripe Terminal SDK needs to retrieve a short-lived connection token from Stripe, proxied through your server.
    ///
    /// On your backend, add an endpoint that creates and returns a connection token.
    pub fn create(
        client: &Client,
        params: CreateConnectionToken,
    ) -> Response<stripe_terminal::terminal::connection_token::ConnectionToken> {
        client.send_form("/terminal/connection_tokens", params, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConnectionToken<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The id of the location that this connection token is scoped to.
    ///
    /// If specified the connection token will only be usable with readers assigned to that location, otherwise the connection token will be usable with all readers.
    /// Note that location scoping only applies to internet-connected readers.
    /// For more details, see [the docs on scoping connection tokens](https://stripe.com/docs/terminal/fleet/locations#connection-tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<&'a str>,
}
impl<'a> CreateConnectionToken<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
