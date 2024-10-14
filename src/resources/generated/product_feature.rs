// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{ProductFeatureId};
use crate::params::{Object};
use crate::resources::{EntitlementsFeature};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ProductFeature".
///
/// For more details see <https://stripe.com/docs/api/product-feature/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProductFeature {
    /// Unique identifier for the object.
    pub id: ProductFeatureId,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_feature: Option<EntitlementsFeature>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,
}

impl Object for ProductFeature {
    type Id = ProductFeatureId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "product_feature"
    }
}
