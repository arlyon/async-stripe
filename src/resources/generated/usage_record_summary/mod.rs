#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct UsageRecordSummary {
    /// Unique identifier for the object.
    pub id: crate::usage_record_summary::UsageRecordSummaryId,
    /// The invoice in which this usage period has been billed for.
    pub invoice: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: UsageRecordSummaryObject,
    pub period: crate::usage_record_summary::period::Period,
    /// The ID of the subscription item this summary is describing.
    pub subscription_item: String,
    /// The total usage within this usage period.
    pub total_usage: i64,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for UsageRecordSummary {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum UsageRecordSummaryObject {
    UsageRecordSummary,
}

impl UsageRecordSummaryObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsageRecordSummary => "usage_record_summary",
        }
    }
}

impl AsRef<str> for UsageRecordSummaryObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UsageRecordSummaryObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for UsageRecordSummary {
    type Id = crate::usage_record_summary::UsageRecordSummaryId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(UsageRecordSummaryId, "urs_");
pub mod period;
pub mod requests;
pub use period::Period;
