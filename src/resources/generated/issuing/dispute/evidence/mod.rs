#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Evidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<crate::issuing::dispute::evidence::canceled::Canceled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<crate::issuing::dispute::evidence::duplicate::Duplicate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<crate::issuing::dispute::evidence::fraudulent::Fraudulent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described: Option<
        crate::issuing::dispute::evidence::merchandise_not_as_described::MerchandiseNotAsDescribed,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<crate::issuing::dispute::evidence::not_received::NotReceived>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<crate::issuing::dispute::evidence::other::Other>,
    /// The reason for filing the dispute.
    ///
    /// Its value will match the field containing the evidence.
    pub reason: EvidenceReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described:
        Option<crate::issuing::dispute::evidence::service_not_as_described::ServiceNotAsDescribed>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
