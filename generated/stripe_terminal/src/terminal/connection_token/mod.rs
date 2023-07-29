/// A Connection Token is used by the Stripe Terminal SDK to connect to a reader.
///
/// Related guide: [Fleet Management](https://stripe.com/docs/terminal/fleet/locations).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ConnectionTokenObject {
    TerminalConnectionToken,
}

impl ConnectionTokenObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TerminalConnectionToken => "terminal.connection_token",
        }
    }
}

impl std::str::FromStr for ConnectionTokenObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "terminal.connection_token" => Ok(Self::TerminalConnectionToken),

            _ => Err(()),
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
impl serde::Serialize for ConnectionTokenObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConnectionTokenObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ConnectionTokenObject"))
    }
}
