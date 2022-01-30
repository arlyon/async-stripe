// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::RadarValueListItemId;
use crate::params::{Object, Timestamp};

/// The resource representing a Stripe "RadarListListItem".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RadarValueListItem {
    /// Unique identifier for the object.
    pub id: RadarValueListItemId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// The name or email address of the user who added this item to the value list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<String>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<Box<bool>>,

    /// The value of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<String>>,

    /// The identifier of the value list this item belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_list: Option<Box<String>>,
}

impl Object for RadarValueListItem {
    type Id = RadarValueListItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "radar.value_list_item"
    }
}
