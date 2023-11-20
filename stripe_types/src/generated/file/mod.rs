/// This object represents files hosted on Stripe's servers.
///
/// You can upload files with the [create file](https://stripe.com/docs/api#create_file) request (for example, when uploading dispute evidence).
/// Stripe also creates files independetly (for example, the results of a [Sigma scheduled query](#scheduled_queries)).  Related guide: [File upload guide](https://stripe.com/docs/file-upload).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct File {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The file expires and isn't available at this time in epoch seconds.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The suitable name for saving the file to a filesystem.
    pub filename: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_types::file::FileId,
    /// A list of [file links](https://stripe.com/docs/api#file_links) that point at this file.
    #[serde(default)]
    pub links: stripe_types::List<stripe_types::FileLink>,
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
/// The [purpose](https://stripe.com/docs/file-upload#uploading-a-file) of the uploaded file.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}

impl FilePurpose {
    pub fn as_str(self) -> &'static str {
        use FilePurpose::*;
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

impl std::str::FromStr for FilePurpose {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FilePurpose::*;
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

impl AsRef<str> for FilePurpose {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FilePurpose {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FilePurpose {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(FilePurpose::Unknown))
    }
}
impl stripe_types::Object for File {
    type Id = stripe_types::file::FileId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(FileId, "file_");
