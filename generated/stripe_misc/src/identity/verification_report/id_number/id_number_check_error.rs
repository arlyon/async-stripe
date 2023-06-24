#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IdNumberCheckError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<IdNumberCheckErrorCode>,
    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IdNumberCheckError {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
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
        match self {
            Self::IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            Self::IdNumberMismatch => "id_number_mismatch",
            Self::IdNumberUnverifiedOther => "id_number_unverified_other",
        }
    }
}

impl std::str::FromStr for IdNumberCheckErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "id_number_insufficient_document_data" => Ok(Self::IdNumberInsufficientDocumentData),
            "id_number_mismatch" => Ok(Self::IdNumberMismatch),
            "id_number_unverified_other" => Ok(Self::IdNumberUnverifiedOther),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IdNumberCheckErrorCode"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IdNumberCheckErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<IdNumberCheckErrorCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IdNumberCheckErrorCode::from_str(s)?);
        Ok(())
    }
}
