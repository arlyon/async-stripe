#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingDisputeEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<stripe_shared::IssuingDisputeCanceledEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<stripe_shared::IssuingDisputeDuplicateEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<stripe_shared::IssuingDisputeFraudulentEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described:
        Option<stripe_shared::IssuingDisputeMerchandiseNotAsDescribedEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<stripe_shared::IssuingDisputeNotReceivedEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<stripe_shared::IssuingDisputeOtherEvidence>,
    /// The reason for filing the dispute. Its value will match the field containing the evidence.
    pub reason: IssuingDisputeEvidenceReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described:
        Option<stripe_shared::IssuingDisputeServiceNotAsDescribedEvidence>,
}
/// The reason for filing the dispute. Its value will match the field containing the evidence.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingDisputeEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
}
impl IssuingDisputeEvidenceReason {
    pub fn as_str(self) -> &'static str {
        use IssuingDisputeEvidenceReason::*;
        match self {
            Canceled => "canceled",
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            MerchandiseNotAsDescribed => "merchandise_not_as_described",
            NotReceived => "not_received",
            Other => "other",
            ServiceNotAsDescribed => "service_not_as_described",
        }
    }
}

impl std::str::FromStr for IssuingDisputeEvidenceReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeEvidenceReason::*;
        match s {
            "canceled" => Ok(Canceled),
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "merchandise_not_as_described" => Ok(MerchandiseNotAsDescribed),
            "not_received" => Ok(NotReceived),
            "other" => Ok(Other),
            "service_not_as_described" => Ok(ServiceNotAsDescribed),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingDisputeEvidenceReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingDisputeEvidenceReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingDisputeEvidenceReason"))
    }
}
