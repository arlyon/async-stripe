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

impl std::str::FromStr for LastErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "abandoned" => Ok(Self::Abandoned),
            "consent_declined" => Ok(Self::ConsentDeclined),
            "country_not_supported" => Ok(Self::CountryNotSupported),
            "device_not_supported" => Ok(Self::DeviceNotSupported),
            "document_expired" => Ok(Self::DocumentExpired),
            "document_type_not_supported" => Ok(Self::DocumentTypeNotSupported),
            "document_unverified_other" => Ok(Self::DocumentUnverifiedOther),
            "id_number_insufficient_document_data" => Ok(Self::IdNumberInsufficientDocumentData),
            "id_number_mismatch" => Ok(Self::IdNumberMismatch),
            "id_number_unverified_other" => Ok(Self::IdNumberUnverifiedOther),
            "selfie_document_missing_photo" => Ok(Self::SelfieDocumentMissingPhoto),
            "selfie_face_mismatch" => Ok(Self::SelfieFaceMismatch),
            "selfie_manipulated" => Ok(Self::SelfieManipulated),
            "selfie_unverified_other" => Ok(Self::SelfieUnverifiedOther),
            "under_supported_age" => Ok(Self::UnderSupportedAge),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for LastErrorCode"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LastErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<LastErrorCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(LastErrorCode::from_str(s)?);
        Ok(())
    }
}
