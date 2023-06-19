// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "NotificationEventData".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotificationEventData {

    /// Object containing the names of the updated attributes and their values prior to the event (only included in events of type `*.updated`).
    ///
    /// If an array attribute has any updated elements, this object contains the entire array.
    /// In Stripe API versions 2017-04-06 or earlier, an updated array attribute in this object includes only the updated array elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<NotificationEventDataPreviousAttributes>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotificationEventDataPreviousAttributes {
}
