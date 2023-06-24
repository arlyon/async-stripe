/// This is an object representing a file hosted on Stripe's servers.
///
/// The file may have been uploaded by yourself using the [create file](https://stripe.com/docs/api#create_file) request (for example, when uploading dispute evidence) or it may have been created by Stripe (for example, the results of a [Sigma scheduled query](#scheduled_queries)).  Related guide: [File Upload Guide](https://stripe.com/docs/file-upload).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct File {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The time at which the file expires and is no longer available in epoch seconds.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// A filename for the file, suitable for saving to a filesystem.
    pub filename: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_core::file::FileId,
    /// A list of [file links](https://stripe.com/docs/api#file_links) that point at this file.
    #[serde(default)]
    pub links: stripe_types::List<Option<stripe_core::file_link::FileLink>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: FileObject,
    /// The [purpose](https://stripe.com/docs/file-upload#uploading-a-file) of the uploaded file.
    pub purpose: FilePurpose,
    /// The size in bytes of the file object.
    pub size: u64,
    /// A user friendly title for the document.
    pub title: Option<String>,
    /// The type of the file returned (e.g., `csv`, `pdf`, `jpg`, or `png`).
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// The URL from which the file can be downloaded using your live secret API key.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for File {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FileObject {
    File,
}

impl FileObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
        }
    }
}

impl std::str::FromStr for FileObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "file" => Ok(Self::File),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for FileObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FileObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for FileObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FileObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for FileObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FileObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<FileObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FileObject::from_str(s)?);
        Ok(())
    }
}
/// The [purpose](https://stripe.com/docs/file-upload#uploading-a-file) of the uploaded file.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for FilePurpose {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "account_requirement" => Ok(Self::AccountRequirement),
            "additional_verification" => Ok(Self::AdditionalVerification),
            "business_icon" => Ok(Self::BusinessIcon),
            "business_logo" => Ok(Self::BusinessLogo),
            "customer_signature" => Ok(Self::CustomerSignature),
            "dispute_evidence" => Ok(Self::DisputeEvidence),
            "document_provider_identity_document" => Ok(Self::DocumentProviderIdentityDocument),
            "finance_report_run" => Ok(Self::FinanceReportRun),
            "identity_document" => Ok(Self::IdentityDocument),
            "identity_document_downloadable" => Ok(Self::IdentityDocumentDownloadable),
            "pci_document" => Ok(Self::PciDocument),
            "selfie" => Ok(Self::Selfie),
            "sigma_scheduled_query" => Ok(Self::SigmaScheduledQuery),
            "tax_document_user_upload" => Ok(Self::TaxDocumentUserUpload),
            "terminal_reader_splashscreen" => Ok(Self::TerminalReaderSplashscreen),

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
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for FilePurpose"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FilePurpose {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<FilePurpose> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FilePurpose::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for File {
    type Id = stripe_core::file::FileId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FileId, "file_");
pub mod requests;
