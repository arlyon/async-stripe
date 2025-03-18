// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::FileId;
use crate::params::{Expand, List, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::FileLink;
use serde::{Deserialize, Serialize};

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

    /// The file expires and isn't available at this time in epoch seconds.
    pub expires_at: Option<Timestamp>,

    /// The suitable name for saving the file to a filesystem.
    pub filename: Option<String>,

    /// A list of [file links](https://stripe.com/docs/api#file_links) that point at this file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<List<FileLink>>,

    /// The [purpose](https://stripe.com/docs/file-upload#uploading-a-file) of the uploaded file.
    pub purpose: FilePurpose,

    /// The size of the file object in bytes.
    pub size: u64,

    /// A suitable title for the document.
    pub title: Option<String>,

    /// The returned file type (for example, `csv`, `pdf`, `jpg`, or `png`).
    #[serde(rename = "type")]
    pub type_: Option<String>,

    /// Use your live secret API key to download the file from this URL.
    pub url: Option<String>,
}

impl File {
    /// Returns a list of the files that your account has access to.
    ///
    /// Stripe sorts and returns the files by their creation dates, placing the most recently created files at the top.
    pub fn list(client: &Client, params: &ListFiles<'_>) -> Response<List<File>> {
        client.get_query("/files", params)
    }

    /// Retrieves the details of an existing file object.
    ///
    /// After you supply a unique file ID, Stripe returns the corresponding file object.
    /// Learn how to [access file contents](https://stripe.com/docs/file-upload#download-file-contents).
    pub fn retrieve(client: &Client, id: &FileId, expand: &[&str]) -> Response<File> {
        client.get_query(&format!("/files/{}", id), Expand { expand })
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

    /// Filter queries by the file purpose.
    ///
    /// If you don't provide a purpose, the queries return unfiltered files.
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
impl Paginable for ListFiles<'_> {
    type O = File;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
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
    TerminalReaderSplashscreen,
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
            FilePurpose::TerminalReaderSplashscreen => "terminal_reader_splashscreen",
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
