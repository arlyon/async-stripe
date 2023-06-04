/// To share the contents of a `File` object with non-Stripe users, you can
/// create a `FileLink`.
///
/// `FileLink`s contain a URL that can be used to retrieve the contents of the file without authentication.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FileLink {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// Whether this link is already expired.
    pub expired: bool,
    /// Time at which the link expires.
    pub expires_at: Option<crate::Timestamp>,
    /// The file object this link points to.
    pub file: crate::Expandable<crate::file::File>,
    /// Unique identifier for the object.
    pub id: crate::file_link::FileLinkId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::Metadata,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: FileLinkObject,
    /// The publicly accessible URL to download the file.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FileLink {
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
pub enum FileLinkObject {
    FileLink,
}

impl FileLinkObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FileLink => "file_link",
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
impl crate::Object for FileLink {
    type Id = crate::file_link::FileLinkId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(FileLinkId, "link_");
pub mod requests;
