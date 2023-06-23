#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SelfieCheckError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<SelfieCheckErrorCode>,
    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SelfieCheckError {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// A short machine-readable string giving the reason for the verification failure.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SelfieCheckErrorCode {
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
}

impl SelfieCheckErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelfieDocumentMissingPhoto => "selfie_document_missing_photo",
            Self::SelfieFaceMismatch => "selfie_face_mismatch",
            Self::SelfieManipulated => "selfie_manipulated",
            Self::SelfieUnverifiedOther => "selfie_unverified_other",
        }
    }
}

impl AsRef<str> for SelfieCheckErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SelfieCheckErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
