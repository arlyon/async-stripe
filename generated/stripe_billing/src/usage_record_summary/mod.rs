#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UsageRecordSummary {
    /// Unique identifier for the object.
    pub id: stripe_billing::usage_record_summary::UsageRecordSummaryId,
    /// The invoice in which this usage period has been billed for.
    pub invoice: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: UsageRecordSummaryObject,
    pub period: stripe_billing::usage_record_summary::period::Period,
    /// The ID of the subscription item this summary is describing.
    pub subscription_item: String,
    /// The total usage within this usage period.
    pub total_usage: i64,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UsageRecordSummaryObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "usage_record_summary" => Ok(Self::UsageRecordSummary),

            _ => Err(()),
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
impl serde::Serialize for UsageRecordSummaryObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UsageRecordSummaryObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for UsageRecordSummaryObject"))
    }
}
impl stripe_types::Object for UsageRecordSummary {
    type Id = stripe_billing::usage_record_summary::UsageRecordSummaryId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(UsageRecordSummaryId, "urs_" | "sis_");
pub mod period;
pub use period::Period;
