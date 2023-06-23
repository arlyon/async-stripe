#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct LoginLink {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: LoginLinkObject,
    /// The URL for the login link.
    pub url: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LoginLink {
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
pub enum LoginLinkObject {
    LoginLink,
}

impl LoginLinkObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LoginLink => "login_link",
        }
    }
}

impl AsRef<str> for LoginLinkObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LoginLinkObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod requests;
