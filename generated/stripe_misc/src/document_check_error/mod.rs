#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DocumentCheckError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<DocumentCheckErrorCode>,
    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
/// A short machine-readable string giving the reason for the verification failure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DocumentCheckErrorCode {
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
}

impl DocumentCheckErrorCode {
    pub fn as_str(self) -> &'static str {
        use DocumentCheckErrorCode::*;
        match self {
            DocumentExpired => "document_expired",
            DocumentTypeNotSupported => "document_type_not_supported",
            DocumentUnverifiedOther => "document_unverified_other",
        }
    }
}

impl std::str::FromStr for DocumentCheckErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DocumentCheckErrorCode::*;
        match s {
            "document_expired" => Ok(DocumentExpired),
            "document_type_not_supported" => Ok(DocumentTypeNotSupported),
            "document_unverified_other" => Ok(DocumentUnverifiedOther),
            _ => Err(()),
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
impl serde::Serialize for DocumentCheckErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DocumentCheckErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DocumentCheckErrorCode"))
    }
}
