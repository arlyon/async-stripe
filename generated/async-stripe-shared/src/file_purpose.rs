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
    TerminalWifiCertificate,
    TerminalWifiPrivateKey,
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
            TerminalWifiCertificate => "terminal_wifi_certificate",
            TerminalWifiPrivateKey => "terminal_wifi_private_key",
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
            "terminal_wifi_certificate" => Ok(TerminalWifiCertificate),
            "terminal_wifi_private_key" => Ok(TerminalWifiPrivateKey),
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FilePurpose)).finish_non_exhaustive()
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
