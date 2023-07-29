#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ReversalDetails {
    /// Time before which a ReceivedCredit can be reversed.
    pub deadline: Option<stripe_types::Timestamp>,
    /// Set if a ReceivedCredit cannot be reversed.
    pub restricted_reason: Option<ReversalDetailsRestrictedReason>,
}
/// Set if a ReceivedCredit cannot be reversed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ReversalDetailsRestrictedReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "already_reversed" => Ok(Self::AlreadyReversed),
            "deadline_passed" => Ok(Self::DeadlinePassed),
            "network_restricted" => Ok(Self::NetworkRestricted),
            "other" => Ok(Self::Other),
            "source_flow_restricted" => Ok(Self::SourceFlowRestricted),

            _ => Err(()),
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
impl serde::Serialize for ReversalDetailsRestrictedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReversalDetailsRestrictedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ReversalDetailsRestrictedReason")
        })
    }
}
