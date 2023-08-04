/// A Connection Token is used by the Stripe Terminal SDK to connect to a reader.
///
/// Related guide: [Fleet management](https://stripe.com/docs/terminal/fleet/locations).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
#[cfg(feature = "terminal_connection_token")]
mod requests;
#[cfg(feature = "terminal_connection_token")]
pub use requests::*;
