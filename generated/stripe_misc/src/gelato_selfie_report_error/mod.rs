#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GelatoSelfieReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<GelatoSelfieReportErrorCode>,
    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
/// A short machine-readable string giving the reason for the verification failure.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoSelfieReportErrorCode {
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
}

impl GelatoSelfieReportErrorCode {
    pub fn as_str(self) -> &'static str {
        use GelatoSelfieReportErrorCode::*;
        match self {
            SelfieDocumentMissingPhoto => "selfie_document_missing_photo",
            SelfieFaceMismatch => "selfie_face_mismatch",
            SelfieManipulated => "selfie_manipulated",
            SelfieUnverifiedOther => "selfie_unverified_other",
        }
    }
}

impl std::str::FromStr for GelatoSelfieReportErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoSelfieReportErrorCode::*;
        match s {
            "selfie_document_missing_photo" => Ok(SelfieDocumentMissingPhoto),
            "selfie_face_mismatch" => Ok(SelfieFaceMismatch),
            "selfie_manipulated" => Ok(SelfieManipulated),
            "selfie_unverified_other" => Ok(SelfieUnverifiedOther),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for GelatoSelfieReportErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoSelfieReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoSelfieReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoSelfieReportErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoSelfieReportErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GelatoSelfieReportErrorCode"))
    }
}
