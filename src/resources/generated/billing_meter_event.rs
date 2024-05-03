// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BillingMeterEvent".
///
/// For more details see <https://stripe.com/docs/api/billing/meter-event/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterEvent {

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The name of the meter event.
    ///
    /// Corresponds with the `event_name` field on a meter.
    pub event_name: String,

    /// A unique identifier for the event.
    pub identifier: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The payload of the event.
    ///
    /// This contains the fields corresponding to a meter's `customer_mapping.event_payload_key` (default is `stripe_customer_id`) and `value_settings.event_payload_key` (default is `value`).
    /// Read more about the [payload](https://stripe.com/docs/billing/subscriptions/usage-based/recording-usage#payload-key-overrides).
    pub payload: String,

    /// The timestamp passed in when creating the event.
    ///
    /// Measured in seconds since the Unix epoch.
    pub timestamp: Timestamp,
}

impl Object for BillingMeterEvent {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "billing.meter_event"
    }
}
