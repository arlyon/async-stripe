#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VerificationSessionRedaction {
    /// Indicates whether this object and its related objects have been redacted or not.
    pub status: VerificationSessionRedactionStatus,
}
/// Indicates whether this object and its related objects have been redacted or not.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VerificationSessionRedactionStatus {
    Processing,
    Redacted,
}

impl VerificationSessionRedactionStatus {
    pub fn as_str(self) -> &'static str {
        use VerificationSessionRedactionStatus::*;
        match self {
            Processing => "processing",
            Redacted => "redacted",
        }
    }
}

impl std::str::FromStr for VerificationSessionRedactionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use VerificationSessionRedactionStatus::*;
        match s {
            "processing" => Ok(Processing),
            "redacted" => Ok(Redacted),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for VerificationSessionRedactionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationSessionRedactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for VerificationSessionRedactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for VerificationSessionRedactionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerificationSessionRedactionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for VerificationSessionRedactionStatus")
        })
    }
}
