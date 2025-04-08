// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::UsageRecordId;
use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "UsageRecord".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UsageRecord {
    /// Unique identifier for the object.
    pub id: UsageRecordId,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The usage quantity for the specified date.
    pub quantity: u64,

    /// The ID of the subscription item this usage record contains data for.
    pub subscription_item: String,

    /// The timestamp when this usage occurred.
    pub timestamp: Timestamp,
}

impl Object for UsageRecord {
    type Id = UsageRecordId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "usage_record"
    }
}
