#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListFile<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filter queries by the file purpose.
    ///
    /// If you don't provide a purpose, the queries return unfiltered files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<ListFilePurpose>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListFile<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Filter queries by the file purpose.
///
/// If you don't provide a purpose, the queries return unfiltered files.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}

impl ListFilePurpose {
    pub fn as_str(self) -> &'static str {
        use ListFilePurpose::*;
        match self {
            AccountRequirement => "account_requirement",
            AdditionalVerification => "additional_verification",
            BusinessIcon => "business_icon",
            BusinessLogo => "business_logo",
            CustomerSignature => "customer_signature",
            DisputeEvidence => "dispute_evidence",
            DocumentProviderIdentityDocument => "document_provider_identity_document",
            FinanceReportRun => "finance_report_run",
            IdentityDocument => "identity_document",
            IdentityDocumentDownloadable => "identity_document_downloadable",
            PciDocument => "pci_document",
            Selfie => "selfie",
            SigmaScheduledQuery => "sigma_scheduled_query",
            TaxDocumentUserUpload => "tax_document_user_upload",
            TerminalReaderSplashscreen => "terminal_reader_splashscreen",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for ListFilePurpose {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListFilePurpose::*;
        match s {
            "account_requirement" => Ok(AccountRequirement),
            "additional_verification" => Ok(AdditionalVerification),
            "business_icon" => Ok(BusinessIcon),
            "business_logo" => Ok(BusinessLogo),
            "customer_signature" => Ok(CustomerSignature),
            "dispute_evidence" => Ok(DisputeEvidence),
            "document_provider_identity_document" => Ok(DocumentProviderIdentityDocument),
            "finance_report_run" => Ok(FinanceReportRun),
            "identity_document" => Ok(IdentityDocument),
            "identity_document_downloadable" => Ok(IdentityDocumentDownloadable),
            "pci_document" => Ok(PciDocument),
            "selfie" => Ok(Selfie),
            "sigma_scheduled_query" => Ok(SigmaScheduledQuery),
            "tax_document_user_upload" => Ok(TaxDocumentUserUpload),
            "terminal_reader_splashscreen" => Ok(TerminalReaderSplashscreen),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListFilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListFilePurpose {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ListFile<'a> {
    /// Returns a list of the files that your account has access to.
    ///
    /// Stripe sorts and returns the files by their creation dates, placing the most recently created files at the top.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::File>> {
        client.get_query("/files", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::File> {
        stripe::ListPaginator::from_params("/files", self)
    }
}
impl<'a> stripe::PaginationParams for ListFile<'a> {}
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
impl<'a> RetrieveFile<'a> {
    /// Retrieves the details of an existing file object.
    ///
    /// After you supply a unique file ID, Stripe returns the corresponding file object.
    /// Learn how to [access file contents](https://stripe.com/docs/file-upload#download-file-contents).
    pub fn send(
        &self,
        client: &stripe::Client,
        file: &stripe_types::file::FileId,
    ) -> stripe::Response<stripe_types::File> {
        client.get_query(&format!("/files/{file}", file = file), self)
    }
}
