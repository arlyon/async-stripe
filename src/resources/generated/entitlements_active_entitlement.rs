// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{EntitlementsActiveEntitlementId};
use crate::params::{Object};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ActiveEntitlement".
///
/// For more details see <https://stripe.com/docs/api/entitlements/active-entitlement/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EntitlementsActiveEntitlement {
    /// Unique identifier for the object.
    pub id: EntitlementsActiveEntitlementId,

    /// The feature that the customer is entitled to.
    pub feature: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// A unique key you provide as your own system identifier.
    ///
    /// This may be up to 80 characters.
    pub lookup_key: String,
}

impl Object for EntitlementsActiveEntitlement {
    type Id = EntitlementsActiveEntitlementId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "entitlements.active_entitlement"
    }
}
