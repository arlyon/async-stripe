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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
