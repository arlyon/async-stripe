use stripe::{Client, Response};

impl stripe_core::file::File {
    /// Returns a list of the files that your account has access to.
    ///
    /// The files are returned sorted by creation date, with the most recently created files appearing first.
    pub fn list(
        client: &Client,
        params: ListFile,
    ) -> Response<stripe_types::List<stripe_core::file::File>> {
        client.get_query("/files", params)
    }
    /// Retrieves the details of an existing file object.
    ///
    /// Supply the unique file ID from a file, and Stripe will return the corresponding file object.
    /// To access file contents, see the [File Upload Guide](https://stripe.com/docs/file-upload#download-file-contents).
    pub fn retrieve(
        client: &Client,
        file: &stripe_core::file::FileId,
        params: RetrieveFile,
    ) -> Response<stripe_core::file::File> {
        client.get_query(&format!("/files/{file}", file = file), params)
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListFile<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The file purpose to filter queries by.
    ///
    /// If none is provided, files will not be filtered by purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<ListFilePurpose>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListFile<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The file purpose to filter queries by.
///
/// If none is provided, files will not be filtered by purpose.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListFilePurpose {
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

impl ListFilePurpose {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountRequirement => "account_requirement",
            Self::AdditionalVerification => "additional_verification",
            Self::BusinessIcon => "business_icon",
            Self::BusinessLogo => "business_logo",
            Self::CustomerSignature => "customer_signature",
            Self::DisputeEvidence => "dispute_evidence",
            Self::DocumentProviderIdentityDocument => "document_provider_identity_document",
            Self::FinanceReportRun => "finance_report_run",
            Self::IdentityDocument => "identity_document",
            Self::IdentityDocumentDownloadable => "identity_document_downloadable",
            Self::PciDocument => "pci_document",
            Self::Selfie => "selfie",
            Self::SigmaScheduledQuery => "sigma_scheduled_query",
            Self::TaxDocumentUserUpload => "tax_document_user_upload",
            Self::TerminalReaderSplashscreen => "terminal_reader_splashscreen",
        }
    }
}

impl AsRef<str> for ListFilePurpose {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListFilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveFile<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFile<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
