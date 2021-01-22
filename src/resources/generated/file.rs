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

    /// A list of [file links](https://stripe.com/docs/api#file_links) that point at this file.
    #[serde(default)]
    pub links: List<FileLink>,

    /// The purpose of the file.
    ///
    /// Possible values are `additional_verification`, `business_icon`, `business_logo`, `customer_signature`, `dispute_evidence`, `finance_report_run`, `identity_document`, `pci_document`, `sigma_scheduled_query`, or `tax_document_user_upload`.
    pub purpose: FilePurpose,

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
    pub fn list(client: &Client, params: ListFiles<'_>) -> Response<List<File>> {
        client.get_query("/files", &params)
    }

    /// Retrieves the details of an existing file object.
    ///
    /// Supply the unique file ID from a file, and Stripe will return the corresponding file object.
    /// To access file contents, see the [File Upload Guide](https://stripe.com/docs/file-upload#download-file-contents).
    pub fn retrieve(client: &Client, id: &FileId, expand: &[&str]) -> Response<File> {
        client.get_query(&format!("/files/{}", id), &Expand { expand })
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
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListFiles<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<FileId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// The file purpose to filter queries by.
    ///
    /// If none is provided, files will not be filtered by purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<FilePurpose>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<FileId>,
}

impl<'a> ListFiles<'a> {
    pub fn new() -> Self {
        ListFiles {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            purpose: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `ListFiles`'s `purpose` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FilePurpose {
    AdditionalVerification,
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

impl FilePurpose {
    pub fn as_str(self) -> &'static str {
        match self {
            FilePurpose::AdditionalVerification => "additional_verification",
            FilePurpose::BusinessIcon => "business_icon",
            FilePurpose::BusinessLogo => "business_logo",
            FilePurpose::CustomerSignature => "customer_signature",
            FilePurpose::DisputeEvidence => "dispute_evidence",
            FilePurpose::FinanceReportRun => "finance_report_run",
            FilePurpose::IdentityDocument => "identity_document",
            FilePurpose::PciDocument => "pci_document",
            FilePurpose::SigmaScheduledQuery => "sigma_scheduled_query",
            FilePurpose::TaxDocumentUserUpload => "tax_document_user_upload",
        }
    }
}

impl AsRef<str> for FilePurpose {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
