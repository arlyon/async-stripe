#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IdNumberCheckError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<IdNumberCheckErrorCode>,
    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
/// A short machine-readable string giving the reason for the verification failure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IdNumberCheckErrorCode {
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
}

impl IdNumberCheckErrorCode {
    pub fn as_str(self) -> &'static str {
        use IdNumberCheckErrorCode::*;
        match self {
            IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            IdNumberMismatch => "id_number_mismatch",
            IdNumberUnverifiedOther => "id_number_unverified_other",
        }
    }
}

impl std::str::FromStr for IdNumberCheckErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IdNumberCheckErrorCode::*;
        match s {
            "id_number_insufficient_document_data" => Ok(IdNumberInsufficientDocumentData),
            "id_number_mismatch" => Ok(IdNumberMismatch),
            "id_number_unverified_other" => Ok(IdNumberUnverifiedOther),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IdNumberCheckErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IdNumberCheckErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for IdNumberCheckErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdNumberCheckErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IdNumberCheckErrorCode"))
    }
}
