use crate::params::{Identifiable, Metadata, Timestamp};
use crate::resources::{Currency, PackageDimensions};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Inventory {
    pub quantity: u64,
    #[serde(rename = "type")]
    pub inventory_type: String,
    pub value: Option<String>,
}

/// The resource representing a Stripe Sku.
///
/// For more details see https://stripe.com/docs/api#sku_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sku {
    pub id: String,
    pub object: String,
    pub active: bool,
    pub attributes: serde_json::Value,
    pub created: Timestamp,
    pub currency: Currency,
    pub image: Option<String>,
    pub inventory: Inventory,
    pub livemode: bool,
    pub metadata: Metadata,
    pub package_dimensions: Option<PackageDimensions>,
    pub price: u64,
    pub product: String,
    pub updated: Timestamp,
}

impl Identifiable for Sku {
    fn id(&self) -> &str {
        &self.id
    }
}
