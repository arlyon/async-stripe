#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Evidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<stripe_types::canceled::Canceled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<stripe_types::duplicate::Duplicate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<stripe_types::fraudulent::Fraudulent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described:
        Option<stripe_types::merchandise_not_as_described::MerchandiseNotAsDescribed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<stripe_types::not_received::NotReceived>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<stripe_types::other::Other>,
    /// The reason for filing the dispute.
    ///
    /// Its value will match the field containing the evidence.
    pub reason: EvidenceReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described:
        Option<stripe_types::service_not_as_described::ServiceNotAsDescribed>,
}
/// The reason for filing the dispute.
///
/// Its value will match the field containing the evidence.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
}

impl EvidenceReason {
    pub fn as_str(self) -> &'static str {
        use EvidenceReason::*;
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

impl std::str::FromStr for EvidenceReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EvidenceReason::*;
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

impl AsRef<str> for EvidenceReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for EvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for EvidenceReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EvidenceReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for EvidenceReason"))
    }
}
