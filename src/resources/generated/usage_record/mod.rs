/// Usage records allow you to report customer usage and metrics to Stripe for
/// metered billing of subscription prices.
///
/// Related guide: [Metered Billing](https://stripe.com/docs/billing/subscriptions/metered-billing).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct UsageRecord {
    /// Unique identifier for the object.
    pub id: crate::usage_record::UsageRecordId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: UsageRecordObject,
    /// The usage quantity for the specified date.
    pub quantity: u64,
    /// The ID of the subscription item this usage record contains data for.
    pub subscription_item: String,
    /// The timestamp when this usage occurred.
    pub timestamp: crate::Timestamp,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for UsageRecord {
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
pub enum UsageRecordObject {
    UsageRecord,
}

impl UsageRecordObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsageRecord => "usage_record",
        }
    }
}

impl AsRef<str> for UsageRecordObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UsageRecordObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for UsageRecord {
    type Id = crate::usage_record::UsageRecordId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(UsageRecordId, "mbur_");
pub mod requests;
