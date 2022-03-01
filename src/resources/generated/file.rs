// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::FileId;
use crate::params::{Expand, List, Object, RangeQuery, Timestamp};
use crate::resources::FileLink;

/// The resource representing a Stripe "File".
///
/// For more details see <https://stripe.com/docs/api/files/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct File {
    /// Unique identifier for the object.
    pub id: FileId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The time at which the file expires and is no longer available in epoch seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Box<Timestamp>>,

    /// A filename for the file, suitable for saving to a filesystem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<Box<String>>,

    /// A list of [file links](https://stripe.com/docs/api#file_links) that point at this file.
    #[serde(default)]
    pub links: List<FileLink>,

    /// The [purpose](https://stripe.com/docs/file-upload#uploading-a-file) of the uploaded file.
    pub purpose: FilePurpose,

    /// The size in bytes of the file object.
    pub size: u64,

    /// A user friendly title for the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<String>>,

    /// The type of the file returned (e.g., `csv`, `pdf`, `jpg`, or `png`).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Box<String>>,

    /// The URL from which the file can be downloaded using your live secret API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Box<String>>,
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
    AccountRequirement,
    AdditionalVerification,
    BusinessIcon,
    BusinessLogo,
    CustomerSignature,
    DisputeEvidence,
    DocumentProviderIdentityDocument,
    FinanceReportRun,
    IdentityDocument,
    IdentityDocumentDownloadable,
    PciDocument,
    Selfie,
    SigmaScheduledQuery,
    TaxDocumentUserUpload,
}

impl FilePurpose {
    pub fn as_str(self) -> &'static str {
        match self {
            FilePurpose::AccountRequirement => "account_requirement",
            FilePurpose::AdditionalVerification => "additional_verification",
            FilePurpose::BusinessIcon => "business_icon",
            FilePurpose::BusinessLogo => "business_logo",
            FilePurpose::CustomerSignature => "customer_signature",
            FilePurpose::DisputeEvidence => "dispute_evidence",
            FilePurpose::DocumentProviderIdentityDocument => "document_provider_identity_document",
            FilePurpose::FinanceReportRun => "finance_report_run",
            FilePurpose::IdentityDocument => "identity_document",
            FilePurpose::IdentityDocumentDownloadable => "identity_document_downloadable",
            FilePurpose::PciDocument => "pci_document",
            FilePurpose::Selfie => "selfie",
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
impl std::default::Default for FilePurpose {
    fn default() -> Self {
        Self::AccountRequirement
    }
}
