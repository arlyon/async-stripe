use params::{Metadata, Timestamp};
use resources::{Currency, PackageDimensions};
use serde_json as json;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Inventory {
    pub quantity: u64,
    #[serde(rename = "type")]
    pub inventory_type: String,
    pub value: Option<String>,
}

/// The resource representing a Stripe Sku.
///
/// For more details see https://stripe.com/docs/api/dotnet#sku_object.
#[derive(Debug, Deserialize)]
pub struct Sku {
    pub id: String,
    pub object: String,
    pub active: bool,
    pub attributes: json::Value,
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
