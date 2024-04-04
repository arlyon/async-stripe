#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GelatoIdNumberReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<GelatoIdNumberReportErrorCode>,
    /// A human-readable message giving the reason for the failure.
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
/// A short machine-readable string giving the reason for the verification failure.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoIdNumberReportErrorCode {
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
}
impl GelatoIdNumberReportErrorCode {
    pub fn as_str(self) -> &'static str {
        use GelatoIdNumberReportErrorCode::*;
        match self {
            IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            IdNumberMismatch => "id_number_mismatch",
            IdNumberUnverifiedOther => "id_number_unverified_other",
        }
    }
}

impl std::str::FromStr for GelatoIdNumberReportErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoIdNumberReportErrorCode::*;
        match s {
            "id_number_insufficient_document_data" => Ok(IdNumberInsufficientDocumentData),
            "id_number_mismatch" => Ok(IdNumberMismatch),
            "id_number_unverified_other" => Ok(IdNumberUnverifiedOther),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for GelatoIdNumberReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoIdNumberReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoIdNumberReportErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoIdNumberReportErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for GelatoIdNumberReportErrorCode")
        })
    }
}
