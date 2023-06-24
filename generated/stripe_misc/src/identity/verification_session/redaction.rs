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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for RedactionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "processing" => Ok(Self::Processing),
            "redacted" => Ok(Self::Redacted),

            _ => Err(()),
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
impl serde::Serialize for RedactionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RedactionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for RedactionStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RedactionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<RedactionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RedactionStatus::from_str(s)?);
        Ok(())
    }
}
