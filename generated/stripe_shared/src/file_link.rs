/// To share the contents of a `File` object with non-Stripe users, you can
/// create a `FileLink`. `FileLink`s contain a URL that you can use to
/// retrieve the contents of the file without authentication.
///
/// For more details see <<https://stripe.com/docs/api/file_links/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FileLink {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Returns if the link is already expired.
    pub expired: bool,
    /// Time that the link expires.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The file object this link points to.
    pub file: stripe_types::Expandable<stripe_shared::File>,
    /// Unique identifier for the object.
    pub id: stripe_shared::FileLinkId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The publicly accessible URL to download the file.
    pub url: Option<String>,
}
impl stripe_types::Object for FileLink {
    type Id = stripe_shared::FileLinkId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(FileLinkId, "link_");
