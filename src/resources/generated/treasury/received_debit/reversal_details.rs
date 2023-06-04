#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ReversalDetails {
    /// Time before which a ReceivedDebit can be reversed.
    pub deadline: Option<crate::Timestamp>,
    /// Set if a ReceivedDebit can't be reversed.
    pub restricted_reason: Option<ReversalDetailsRestrictedReason>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReversalDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Set if a ReceivedDebit can't be reversed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
}

impl ReversalDetailsRestrictedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlreadyReversed => "already_reversed",
            Self::DeadlinePassed => "deadline_passed",
            Self::NetworkRestricted => "network_restricted",
            Self::Other => "other",
            Self::SourceFlowRestricted => "source_flow_restricted",
        }
    }
}

impl AsRef<str> for ReversalDetailsRestrictedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
