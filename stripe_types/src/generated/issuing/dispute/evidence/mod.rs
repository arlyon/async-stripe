#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Evidence {
#[serde(skip_serializing_if = "Option::is_none")]
pub canceled: Option<stripe_types::issuing::dispute::evidence::canceled::Canceled>,
#[serde(skip_serializing_if = "Option::is_none")]
pub duplicate: Option<stripe_types::issuing::dispute::evidence::duplicate::Duplicate>,
#[serde(skip_serializing_if = "Option::is_none")]
pub fraudulent: Option<stripe_types::issuing::dispute::evidence::fraudulent::Fraudulent>,
#[serde(skip_serializing_if = "Option::is_none")]
pub merchandise_not_as_described: Option<stripe_types::issuing::dispute::evidence::merchandise_not_as_described::MerchandiseNotAsDescribed>,
#[serde(skip_serializing_if = "Option::is_none")]
pub not_received: Option<stripe_types::issuing::dispute::evidence::not_received::NotReceived>,
#[serde(skip_serializing_if = "Option::is_none")]
pub other: Option<stripe_types::issuing::dispute::evidence::other::Other>,
    /// The reason for filing the dispute.
    ///
    /// Its value will match the field containing the evidence.
pub reason: EvidenceReason,
#[serde(skip_serializing_if = "Option::is_none")]
pub service_not_as_described: Option<stripe_types::issuing::dispute::evidence::service_not_as_described::ServiceNotAsDescribed>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Evidence {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
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
        match self {
            Self::Canceled => "canceled",
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::MerchandiseNotAsDescribed => "merchandise_not_as_described",
            Self::NotReceived => "not_received",
            Self::Other => "other",
            Self::ServiceNotAsDescribed => "service_not_as_described",
        }
    }
}

impl std::str::FromStr for EvidenceReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "canceled" => Ok(Self::Canceled),
            "duplicate" => Ok(Self::Duplicate),
            "fraudulent" => Ok(Self::Fraudulent),
            "merchandise_not_as_described" => Ok(Self::MerchandiseNotAsDescribed),
            "not_received" => Ok(Self::NotReceived),
            "other" => Ok(Self::Other),
            "service_not_as_described" => Ok(Self::ServiceNotAsDescribed),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for EvidenceReason"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for EvidenceReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<EvidenceReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(EvidenceReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
pub mod canceled;
pub use canceled::Canceled;
pub mod duplicate;
pub use duplicate::Duplicate;
pub mod fraudulent;
pub use fraudulent::Fraudulent;
pub mod merchandise_not_as_described;
pub use merchandise_not_as_described::MerchandiseNotAsDescribed;
pub mod not_received;
pub use not_received::NotReceived;
pub mod other;
pub use other::Other;
pub mod service_not_as_described;
pub use service_not_as_described::ServiceNotAsDescribed;
