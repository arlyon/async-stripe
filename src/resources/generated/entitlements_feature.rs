// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{EntitlementsFeatureId};
use crate::params::{Metadata, Object};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Feature".
///
/// For more details see <https://stripe.com/docs/api/entitlements/feature/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EntitlementsFeature {
    /// Unique identifier for the object.
    pub id: EntitlementsFeatureId,

    /// Inactive features cannot be attached to new products and will not be returned from the features list endpoint.
    pub active: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// A unique key you provide as your own system identifier.
    ///
    /// This may be up to 80 characters.
    pub lookup_key: String,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The feature's name, for your own purpose, not meant to be displayable to the customer.
    pub name: String,
}

impl Object for EntitlementsFeature {
    type Id = EntitlementsFeatureId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "entitlements.feature"
    }
}
