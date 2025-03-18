// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{FileId, FileLinkId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{File, Scheduled};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "FileLink".
///
/// For more details see <https://stripe.com/docs/api/file_links/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FileLink {
    /// Unique identifier for the object.
    pub id: FileLinkId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Returns if the link is already expired.
    pub expired: bool,

    /// Time that the link expires.
    pub expires_at: Option<Timestamp>,

    /// The file object this link points to.
    pub file: Expandable<File>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The publicly accessible URL to download the file.
    pub url: Option<String>,
}

impl FileLink {
    /// Returns a list of file links.
    pub fn list(client: &Client, params: &ListFileLinks<'_>) -> Response<List<FileLink>> {
        client.get_query("/file_links", params)
    }

    /// Creates a new file link object.
    pub fn create(client: &Client, params: CreateFileLink<'_>) -> Response<FileLink> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/file_links", &params)
    }

    /// Retrieves the file link with the given ID.
    pub fn retrieve(client: &Client, id: &FileLinkId, expand: &[&str]) -> Response<FileLink> {
        client.get_query(&format!("/file_links/{}", id), Expand { expand })
    }

    /// Updates an existing file link object.
    ///
    /// Expired links can no longer be updated.
    pub fn update(
        client: &Client,
        id: &FileLinkId,
        params: UpdateFileLink<'_>,
    ) -> Response<FileLink> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/file_links/{}", id), &params)
    }
}

impl Object for FileLink {
    type Id = FileLinkId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "file_link"
    }
}

/// The parameters for `FileLink::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateFileLink<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The link isn't usable after this future timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,

    /// The ID of the file.
    ///
    /// The file's `purpose` must be one of the following: `business_icon`, `business_logo`, `customer_signature`, `dispute_evidence`, `finance_report_run`, `identity_document_downloadable`, `pci_document`, `selfie`, `sigma_scheduled_query`, `tax_document_user_upload`, or `terminal_reader_splashscreen`.
    pub file: FileId,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl<'a> CreateFileLink<'a> {
    pub fn new(file: FileId) -> Self {
        CreateFileLink {
            expand: Default::default(),
            expires_at: Default::default(),
            file,
            metadata: Default::default(),
        }
    }
}

/// The parameters for `FileLink::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListFileLinks<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<FileLinkId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Filter links by their expiration status.
    ///
    /// By default, Stripe returns all links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,

    /// Only return links for the given file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileId>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<FileLinkId>,
}

impl<'a> ListFileLinks<'a> {
    pub fn new() -> Self {
        ListFileLinks {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            expired: Default::default(),
            file: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListFileLinks<'_> {
    type O = FileLink;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `FileLink::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateFileLink<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A future timestamp after which the link will no longer be usable, or `now` to expire the link immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Scheduled>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl<'a> UpdateFileLink<'a> {
    pub fn new() -> Self {
        UpdateFileLink {
            expand: Default::default(),
            expires_at: Default::default(),
            metadata: Default::default(),
        }
    }
}
