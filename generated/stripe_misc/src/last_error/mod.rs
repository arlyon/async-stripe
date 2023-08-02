/// Shows last VerificationSession error.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LastError {
    /// A short machine-readable string giving the reason for the verification or user-session failure.
    pub code: Option<LastErrorCode>,
    /// A message that explains the reason for verification or user-session failure.
    pub reason: Option<String>,
}
/// A short machine-readable string giving the reason for the verification or user-session failure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
        use LastErrorCode::*;
        match self {
            Abandoned => "abandoned",
            ConsentDeclined => "consent_declined",
            CountryNotSupported => "country_not_supported",
            DeviceNotSupported => "device_not_supported",
            DocumentExpired => "document_expired",
            DocumentTypeNotSupported => "document_type_not_supported",
            DocumentUnverifiedOther => "document_unverified_other",
            IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            IdNumberMismatch => "id_number_mismatch",
            IdNumberUnverifiedOther => "id_number_unverified_other",
            SelfieDocumentMissingPhoto => "selfie_document_missing_photo",
            SelfieFaceMismatch => "selfie_face_mismatch",
            SelfieManipulated => "selfie_manipulated",
            SelfieUnverifiedOther => "selfie_unverified_other",
            UnderSupportedAge => "under_supported_age",
        }
    }
}

impl std::str::FromStr for LastErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LastErrorCode::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "consent_declined" => Ok(ConsentDeclined),
            "country_not_supported" => Ok(CountryNotSupported),
            "device_not_supported" => Ok(DeviceNotSupported),
            "document_expired" => Ok(DocumentExpired),
            "document_type_not_supported" => Ok(DocumentTypeNotSupported),
            "document_unverified_other" => Ok(DocumentUnverifiedOther),
            "id_number_insufficient_document_data" => Ok(IdNumberInsufficientDocumentData),
            "id_number_mismatch" => Ok(IdNumberMismatch),
            "id_number_unverified_other" => Ok(IdNumberUnverifiedOther),
            "selfie_document_missing_photo" => Ok(SelfieDocumentMissingPhoto),
            "selfie_face_mismatch" => Ok(SelfieFaceMismatch),
            "selfie_manipulated" => Ok(SelfieManipulated),
            "selfie_unverified_other" => Ok(SelfieUnverifiedOther),
            "under_supported_age" => Ok(UnderSupportedAge),
            _ => Err(()),
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
impl serde::Serialize for LastErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LastErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for LastErrorCode"))
    }
}
