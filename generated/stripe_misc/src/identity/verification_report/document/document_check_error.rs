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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for DocumentCheckErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "document_expired" => Ok(Self::DocumentExpired),
            "document_type_not_supported" => Ok(Self::DocumentTypeNotSupported),
            "document_unverified_other" => Ok(Self::DocumentUnverifiedOther),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DocumentCheckErrorCode"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DocumentCheckErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<DocumentCheckErrorCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DocumentCheckErrorCode::from_str(s)?);
        Ok(())
    }
}
