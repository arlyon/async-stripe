#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Redaction {
    /// Indicates whether this object and its related objects have been redacted or not.
    pub status: RedactionStatus,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Redaction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Indicates whether this object and its related objects have been redacted or not.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum RedactionStatus {
    Processing,
    Redacted,
}

impl RedactionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Processing => "processing",
            Self::Redacted => "redacted",
        }
    }
}

impl AsRef<str> for RedactionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RedactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
