/// Shows last VerificationSession error.
#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct LastError {
    /// A short machine-readable string giving the reason for the verification or user-session failure.
    pub code: Option<LastErrorCode>,
    /// A message that explains the reason for verification or user-session failure.
    pub reason: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LastError {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// A short machine-readable string giving the reason for the verification or user-session failure.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum LastErrorCode {
    Abandoned,
    ConsentDeclined,
    CountryNotSupported,
    DeviceNotSupported,
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
    UnderSupportedAge,
}

impl LastErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Abandoned => "abandoned",
            Self::ConsentDeclined => "consent_declined",
            Self::CountryNotSupported => "country_not_supported",
            Self::DeviceNotSupported => "device_not_supported",
            Self::DocumentExpired => "document_expired",
            Self::DocumentTypeNotSupported => "document_type_not_supported",
            Self::DocumentUnverifiedOther => "document_unverified_other",
            Self::IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            Self::IdNumberMismatch => "id_number_mismatch",
            Self::IdNumberUnverifiedOther => "id_number_unverified_other",
            Self::SelfieDocumentMissingPhoto => "selfie_document_missing_photo",
            Self::SelfieFaceMismatch => "selfie_face_mismatch",
            Self::SelfieManipulated => "selfie_manipulated",
            Self::SelfieUnverifiedOther => "selfie_unverified_other",
            Self::UnderSupportedAge => "under_supported_age",
        }
    }
}

impl AsRef<str> for LastErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LastErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
