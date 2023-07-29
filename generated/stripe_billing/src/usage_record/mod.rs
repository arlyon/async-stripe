/// Usage records allow you to report customer usage and metrics to Stripe for
/// metered billing of subscription prices.
///
/// Related guide: [Metered Billing](https://stripe.com/docs/billing/subscriptions/metered-billing).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UsageRecord {
    /// Unique identifier for the object.
    pub id: stripe_billing::usage_record::UsageRecordId,
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
    pub timestamp: stripe_types::Timestamp,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UsageRecordObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "usage_record" => Ok(Self::UsageRecord),

            _ => Err(()),
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
impl serde::Serialize for UsageRecordObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UsageRecordObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for UsageRecordObject"))
    }
}
impl stripe_types::Object for UsageRecord {
    type Id = stripe_billing::usage_record::UsageRecordId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(UsageRecordId, "mbur_");
