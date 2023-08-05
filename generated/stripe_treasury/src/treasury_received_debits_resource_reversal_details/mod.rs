#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TreasuryReceivedDebitsResourceReversalDetails {
    /// Time before which a ReceivedDebit can be reversed.
    pub deadline: Option<stripe_types::Timestamp>,
    /// Set if a ReceivedDebit can't be reversed.
    pub restricted_reason: Option<TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason>,
}
/// Set if a ReceivedDebit can't be reversed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
}

impl TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason::*;
        match self {
            AlreadyReversed => "already_reversed",
            DeadlinePassed => "deadline_passed",
            NetworkRestricted => "network_restricted",
            Other => "other",
            SourceFlowRestricted => "source_flow_restricted",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason::*;
        match s {
            "already_reversed" => Ok(AlreadyReversed),
            "deadline_passed" => Ok(DeadlinePassed),
            "network_restricted" => Ok(NetworkRestricted),
            "other" => Ok(Other),
            "source_flow_restricted" => Ok(SourceFlowRestricted),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason"))
    }
}
