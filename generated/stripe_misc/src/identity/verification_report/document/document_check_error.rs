#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DocumentCheckError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<DocumentCheckErrorCode>,
    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DocumentCheckError {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// A short machine-readable string giving the reason for the verification failure.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DocumentCheckErrorCode {
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
}

impl DocumentCheckErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DocumentExpired => "document_expired",
            Self::DocumentTypeNotSupported => "document_type_not_supported",
            Self::DocumentUnverifiedOther => "document_unverified_other",
        }
    }
}

impl AsRef<str> for DocumentCheckErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DocumentCheckErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
