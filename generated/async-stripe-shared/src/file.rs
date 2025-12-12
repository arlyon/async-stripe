/// This object represents files hosted on Stripe's servers. You can upload
/// files with the [create file](https://api.stripe.com#create_file) request
/// (for example, when uploading dispute evidence). Stripe also
/// creates files independently (for example, the results of a [Sigma scheduled
/// query](#scheduled_queries)).
///
/// Related guide: [File upload guide](https://docs.stripe.com/file-upload)
///
/// For more details see <<https://stripe.com/docs/api/files/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct File {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The file expires and isn't available at this time in epoch seconds.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The suitable name for saving the file to a filesystem.
    pub filename: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::FileId,
    /// A list of [file links](https://api.stripe.com#file_links) that point at this file.
    pub links: Option<stripe_types::List<stripe_shared::FileLink>>,
    /// The [purpose](https://docs.stripe.com/file-upload#uploading-a-file) of the uploaded file.
    pub purpose: stripe_shared::FilePurpose,
    /// The size of the file object in bytes.
    pub size: u64,
    /// A suitable title for the document.
    pub title: Option<String>,
    /// The returned file type (for example, `csv`, `pdf`, `jpg`, or `png`).
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: Option<String>,
    /// Use your live secret API key to download the file from this URL.
    pub url: Option<String>,
}
#[doc(hidden)]
pub struct FileBuilder {
    created: Option<stripe_types::Timestamp>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    filename: Option<Option<String>>,
    id: Option<stripe_shared::FileId>,
    links: Option<Option<stripe_types::List<stripe_shared::FileLink>>>,
    purpose: Option<stripe_shared::FilePurpose>,
    size: Option<u64>,
    title: Option<Option<String>>,
    type_: Option<Option<String>>,
    url: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for File {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<File>,
        builder: FileBuilder,
    }

    impl Visitor for Place<File> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FileBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FileBuilder {
        type Out = File;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "filename" => Deserialize::begin(&mut self.filename),
                "id" => Deserialize::begin(&mut self.id),
                "links" => Deserialize::begin(&mut self.links),
                "purpose" => Deserialize::begin(&mut self.purpose),
                "size" => Deserialize::begin(&mut self.size),
                "title" => Deserialize::begin(&mut self.title),
                "type" => Deserialize::begin(&mut self.type_),
                "url" => Deserialize::begin(&mut self.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                expires_at: Deserialize::default(),
                filename: Deserialize::default(),
                id: Deserialize::default(),
                links: Deserialize::default(),
                purpose: Deserialize::default(),
                size: Deserialize::default(),
                title: Deserialize::default(),
                type_: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(created),
                Some(expires_at),
                Some(filename),
                Some(id),
                Some(links),
                Some(purpose),
                Some(size),
                Some(title),
                Some(type_),
                Some(url),
            ) = (
                self.created,
                self.expires_at,
                self.filename.take(),
                self.id.take(),
                self.links.take(),
                self.purpose.take(),
                self.size,
                self.title.take(),
                self.type_.take(),
                self.url.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                created,
                expires_at,
                filename,
                id,
                links,
                purpose,
                size,
                title,
                type_,
                url,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for File {
        type Builder = FileBuilder;
    }

    impl FromValueOpt for File {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FileBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "filename" => b.filename = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "links" => b.links = FromValueOpt::from_value(v),
                    "purpose" => b.purpose = FromValueOpt::from_value(v),
                    "size" => b.size = FromValueOpt::from_value(v),
                    "title" => b.title = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for File {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("File", 11)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("filename", &self.filename)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("links", &self.links)?;
        s.serialize_field("purpose", &self.purpose)?;
        s.serialize_field("size", &self.size)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("url", &self.url)?;

        s.serialize_field("object", "file")?;
        s.end()
    }
}
impl stripe_types::Object for File {
    type Id = stripe_shared::FileId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(FileId);
#[derive(Clone, Eq, PartialEq)]
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
    FinancialAccountStatement,
    IdentityDocument,
    IdentityDocumentDownloadable,
    IssuingRegulatoryReporting,
    PciDocument,
    PlatformTermsOfService,
    Selfie,
    SigmaScheduledQuery,
    TaxDocumentUserUpload,
    TerminalAndroidApk,
    TerminalReaderSplashscreen,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FilePurpose {
    pub fn as_str(&self) -> &str {
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
            FinancialAccountStatement => "financial_account_statement",
            IdentityDocument => "identity_document",
            IdentityDocumentDownloadable => "identity_document_downloadable",
            IssuingRegulatoryReporting => "issuing_regulatory_reporting",
            PciDocument => "pci_document",
            PlatformTermsOfService => "platform_terms_of_service",
            Selfie => "selfie",
            SigmaScheduledQuery => "sigma_scheduled_query",
            TaxDocumentUserUpload => "tax_document_user_upload",
            TerminalAndroidApk => "terminal_android_apk",
            TerminalReaderSplashscreen => "terminal_reader_splashscreen",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FilePurpose {
    type Err = std::convert::Infallible;
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
            "financial_account_statement" => Ok(FinancialAccountStatement),
            "identity_document" => Ok(IdentityDocument),
            "identity_document_downloadable" => Ok(IdentityDocumentDownloadable),
            "issuing_regulatory_reporting" => Ok(IssuingRegulatoryReporting),
            "pci_document" => Ok(PciDocument),
            "platform_terms_of_service" => Ok(PlatformTermsOfService),
            "selfie" => Ok(Selfie),
            "sigma_scheduled_query" => Ok(SigmaScheduledQuery),
            "tax_document_user_upload" => Ok(TaxDocumentUserUpload),
            "terminal_android_apk" => Ok(TerminalAndroidApk),
            "terminal_reader_splashscreen" => Ok(TerminalReaderSplashscreen),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "FilePurpose");
                Ok(Unknown(v.to_owned()))
            }
        }
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
impl miniserde::Deserialize for FilePurpose {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FilePurpose> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FilePurpose::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FilePurpose);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FilePurpose {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
