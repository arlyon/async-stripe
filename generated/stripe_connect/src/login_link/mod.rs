/// Login Links are single-use login link for an Express account to access their Stripe dashboard.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LoginLinkObject {
    LoginLink,
}

impl LoginLinkObject {
    pub fn as_str(self) -> &'static str {
        use LoginLinkObject::*;
        match self {
            LoginLink => "login_link",
        }
    }
}

impl std::str::FromStr for LoginLinkObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LoginLinkObject::*;
        match s {
            "login_link" => Ok(LoginLink),
            _ => Err(()),
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
impl serde::Serialize for LoginLinkObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LoginLinkObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for LoginLinkObject"))
    }
}
pub mod requests;
