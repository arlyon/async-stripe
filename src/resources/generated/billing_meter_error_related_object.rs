// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{BillingMeterErrorRelatedObjectId};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BillingMeterResourceRelatedObject".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterErrorRelatedObject {
    /// Unique identifier for the object.
    pub id: BillingMeterErrorRelatedObjectId,

    /// The url of the meter object.
    pub url: String,
}
