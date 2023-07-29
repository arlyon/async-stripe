#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SelfieCheckError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<SelfieCheckErrorCode>,
    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
/// A short machine-readable string giving the reason for the verification failure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for SelfieCheckErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "selfie_document_missing_photo" => Ok(Self::SelfieDocumentMissingPhoto),
            "selfie_face_mismatch" => Ok(Self::SelfieFaceMismatch),
            "selfie_manipulated" => Ok(Self::SelfieManipulated),
            "selfie_unverified_other" => Ok(Self::SelfieUnverifiedOther),

            _ => Err(()),
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
impl serde::Serialize for SelfieCheckErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SelfieCheckErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SelfieCheckErrorCode"))
    }
}
