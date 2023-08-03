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
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TerminalConnectionTokenObject,
    /// Your application should pass this token to the Stripe Terminal SDK.
    pub secret: String,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalConnectionTokenObject {
    TerminalConnectionToken,
}

impl TerminalConnectionTokenObject {
    pub fn as_str(self) -> &'static str {
        use TerminalConnectionTokenObject::*;
        match self {
            TerminalConnectionToken => "terminal.connection_token",
        }
    }
}

impl std::str::FromStr for TerminalConnectionTokenObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalConnectionTokenObject::*;
        match s {
            "terminal.connection_token" => Ok(TerminalConnectionToken),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TerminalConnectionTokenObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalConnectionTokenObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalConnectionTokenObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TerminalConnectionTokenObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TerminalConnectionTokenObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TerminalConnectionTokenObject"))
    }
}
pub mod requests;
