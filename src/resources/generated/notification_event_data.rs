// ======================================
// This file was automatically generated.
// ======================================


use serde::{Deserialize, Serialize};


/// The resource representing a Stripe "NotificationEventData".
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct NotificationEventData {
    /// Object containing the names of the attributes that have changed, and their previous values (sent along only with *.updated events).
#[serde(skip_serializing_if = "Option::is_none")]
pub previous_attributes: Option<NotificationEventDataPreviousAttributes>,
}






#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct NotificationEventDataPreviousAttributes {

}



