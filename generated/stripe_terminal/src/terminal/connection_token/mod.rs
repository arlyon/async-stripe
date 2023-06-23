/// A Connection Token is used by the Stripe Terminal SDK to connect to a reader.
///
/// Related guide: [Fleet Management](https://stripe.com/docs/terminal/fleet/locations).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ConnectionToken {
    /// The id of the location that this connection token is scoped to.
    ///
    /// Note that location scoping only applies to internet-connected readers.
    /// For more details, see [the docs on scoping connection tokens](https://stripe.com/docs/terminal/fleet/locations#connection-tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ConnectionTokenObject,
    /// Your application should pass this token to the Stripe Terminal SDK.
    pub secret: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ConnectionToken {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ConnectionTokenObject {
    #[serde(rename = "terminal.connection_token")]
    TerminalConnectionToken,
}

impl ConnectionTokenObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TerminalConnectionToken => "terminal.connection_token",
        }
    }
}

impl AsRef<str> for ConnectionTokenObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConnectionTokenObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod requests;
