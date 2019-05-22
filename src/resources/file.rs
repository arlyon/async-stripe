// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::FileId;
use crate::params::{Expand, List, Object, RangeQuery, Timestamp};
use crate::resources::FileLink;
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "File".
///
/// For more details see [https://stripe.com/docs/api/files/object](https://stripe.com/docs/api/files/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    /// Unique identifier for the object.
    pub id: FileId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// A filename for the file, suitable for saving to a filesystem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    #[serde(default)]
    pub links: List<FileLink>,

    /// The purpose of the file.
    ///
    /// Possible values are `business_icon`, `business_logo`, `customer_signature`, `dispute_evidence`, `finance_report_run`, `identity_document`, `pci_document`, `sigma_scheduled_query`, or `tax_document_user_upload`.
    pub purpose: String,

    /// The size in bytes of the file object.
    pub size: u64,

    /// A user friendly title for the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// The type of the file returned (e.g., `csv`, `pdf`, `jpg`, or `png`).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    /// The URL from which the file can be downloaded using your live secret API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl File {
    /// Returns a list of the files that your account has access to.
    ///
    /// The files are returned sorted by creation date, with the most recently created files appearing first.
    pub fn list(client: &Client, params: FileListParams<'_>) -> Response<List<File>> {
        client.get_query("/files", &params)
    }
}

impl Object for File {
    type Id = FileId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "file"
    }
}

/// The parameters for `File::list`.
#[derive(Clone, Debug, Serialize)]
pub struct FileListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a FileId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// The file purpose to filter queries by.
    ///
    /// If none is provided, files will not be filtered by purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose: Option<FilePurpose>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a FileId>,
}

impl<'a> FileListParams<'a> {
    pub fn new() -> Self {
        FileListParams {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            purpose: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `FileListParams`'s `purpose` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FilePurpose {
    BusinessIcon,
    BusinessLogo,
    CustomerSignature,
    DisputeEvidence,
    FinanceReportRun,
    IdentityDocument,
    PciDocument,
    SigmaScheduledQuery,
    TaxDocumentUserUpload,
}
