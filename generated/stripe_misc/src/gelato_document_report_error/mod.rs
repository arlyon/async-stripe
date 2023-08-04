#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GelatoDocumentReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<GelatoDocumentReportErrorCode>,
    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
/// A short machine-readable string giving the reason for the verification failure.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoDocumentReportErrorCode {
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
}

impl GelatoDocumentReportErrorCode {
    pub fn as_str(self) -> &'static str {
        use GelatoDocumentReportErrorCode::*;
        match self {
            DocumentExpired => "document_expired",
            DocumentTypeNotSupported => "document_type_not_supported",
            DocumentUnverifiedOther => "document_unverified_other",
        }
    }
}

impl std::str::FromStr for GelatoDocumentReportErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoDocumentReportErrorCode::*;
        match s {
            "document_expired" => Ok(DocumentExpired),
            "document_type_not_supported" => Ok(DocumentTypeNotSupported),
            "document_unverified_other" => Ok(DocumentUnverifiedOther),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for GelatoDocumentReportErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoDocumentReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoDocumentReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoDocumentReportErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoDocumentReportErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for GelatoDocumentReportErrorCode")
        })
    }
}
