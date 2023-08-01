/// To share the contents of a `File` object with non-Stripe users, you can
/// create a `FileLink`.
///
/// `FileLink`s contain a URL that can be used to retrieve the contents of the file without authentication.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FileLink {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Whether this link is already expired.
    pub expired: bool,
    /// Time at which the link expires.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The file object this link points to.
    pub file: stripe_types::Expandable<stripe_types::file::File>,
    /// Unique identifier for the object.
    pub id: stripe_types::file_link::FileLinkId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: FileLinkObject,
    /// The publicly accessible URL to download the file.
    pub url: Option<String>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FileLinkObject {
    FileLink,
}

impl FileLinkObject {
    pub fn as_str(self) -> &'static str {
        use FileLinkObject::*;
        match self {
            FileLink => "file_link",
        }
    }
}

impl std::str::FromStr for FileLinkObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FileLinkObject::*;
        match s {
            "file_link" => Ok(FileLink),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for FileLinkObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FileLinkObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for FileLinkObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FileLinkObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for FileLinkObject"))
    }
}
impl stripe_types::Object for FileLink {
    type Id = stripe_types::file_link::FileLinkId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FileLinkId, "link_");
