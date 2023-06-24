#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Closed {
    /// The array that contains reasons for a FinancialAccount closure.
    pub reasons: Vec<ClosedReasons>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Closed {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The array that contains reasons for a FinancialAccount closure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClosedReasons {
    AccountRejected,
    ClosedByPlatform,
    Other,
}

impl ClosedReasons {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountRejected => "account_rejected",
            Self::ClosedByPlatform => "closed_by_platform",
            Self::Other => "other",
        }
    }
}

impl std::str::FromStr for ClosedReasons {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "account_rejected" => Ok(Self::AccountRejected),
            "closed_by_platform" => Ok(Self::ClosedByPlatform),
            "other" => Ok(Self::Other),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ClosedReasons {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ClosedReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ClosedReasons {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ClosedReasons {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ClosedReasons"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ClosedReasons {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ClosedReasons> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ClosedReasons::from_str(s)?);
        Ok(())
    }
}
