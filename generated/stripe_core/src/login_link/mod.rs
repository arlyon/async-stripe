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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for LoginLinkObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "login_link" => Ok(Self::LoginLink),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for LoginLinkObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LoginLinkObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<LoginLinkObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(LoginLinkObject::from_str(s)?);
        Ok(())
    }
}
pub mod requests;
