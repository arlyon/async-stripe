// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::params::{Expand, Object};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TerminalConnectionToken".
///
/// For more details see <https://stripe.com/docs/api/terminal/connection_tokens/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConnectionToken {
    /// The id of the location that this connection token is scoped to.
    ///
    /// Note that location scoping only applies to internet-connected readers.
    /// For more details, see [the docs on scoping connection tokens](https://stripe.com/docs/terminal/fleet/locations#connection-tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Your application should pass this token to the Stripe Terminal SDK.
    pub secret: String,
}

impl TerminalConnectionToken {
    /// To connect to a reader the Stripe Terminal SDK needs to retrieve a short-lived connection token from Stripe, proxied through your server.
    ///
    /// On your backend, add an endpoint that creates and returns a connection token.
    pub fn create(
        client: &Client,
        params: CreateTerminalConnectionToken<'_>,
    ) -> Response<TerminalConnectionToken> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/terminal/connection_tokens", &params)
    }
}

impl Object for TerminalConnectionToken {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "terminal.connection_token"
    }
}

/// The parameters for `TerminalConnectionToken::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateTerminalConnectionToken<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The id of the location that this connection token is scoped to.
    ///
    /// If specified the connection token will only be usable with readers assigned to that location, otherwise the connection token will be usable with all readers.
    /// Note that location scoping only applies to internet-connected readers.
    /// For more details, see [the docs on scoping connection tokens](https://stripe.com/docs/terminal/fleet/locations#connection-tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<&'a str>,
}

impl<'a> CreateTerminalConnectionToken<'a> {
    pub fn new() -> Self {
        CreateTerminalConnectionToken { expand: Default::default(), location: Default::default() }
    }
}
