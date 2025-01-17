// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{List, Object};
use crate::resources::{EntitlementsActiveEntitlement};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ActiveEntitlementSummary".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EntitlementsActiveEntitlementSummary {

    /// The customer that is entitled to this feature.
    pub customer: String,

    /// The list of entitlements this customer has.
    pub entitlements: List<EntitlementsActiveEntitlement>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}

impl Object for EntitlementsActiveEntitlementSummary {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "entitlements.active_entitlement_summary"
    }
}
