use params::{List, Metadata, Timestamp};
use resources::Sku;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PackageDimensions {
    pub height: f64,
    pub length: f64,
    pub weight: f64,
    pub width: f64,
}

/// The resource representing a Stripe product.
///
/// For more details see https://stripe.com/docs/api#product_object.
#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub id: String,
    pub object: String,
    pub active: Option<bool>,
    pub attributes: Vec<String>,
    pub caption: Option<String>,
    pub created: Timestamp,
    pub deactivate_on: Vec<String>,
    pub description: Option<String>,
    pub images: Vec<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub name: String,
    pub package_dimensions: Option<PackageDimensions>,
    pub shippable: Option<bool>,
    pub skus: List<Sku>,
    pub updated: Timestamp,
    pub url: Option<String>,
}
