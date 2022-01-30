// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::params::Object;

/// The resource representing a Stripe "TerminalConnectionToken".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConnectionToken {
    /// The id of the location that this connection token is scoped to.
    ///
    /// Note that location scoping only applies to internet-connected readers.
    /// For more details, see [the docs on scoping connection tokens](https://stripe.com/docs/terminal/fleet/locations#connection-tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<String>>,

    /// Your application should pass this token to the Stripe Terminal SDK.
    pub secret: String,
}

impl Object for TerminalConnectionToken {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "terminal.connection_token"
    }
}
