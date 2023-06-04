/// Stores representations of [stock keeping units](http://en.wikipedia.org/wiki/Stock_keeping_unit).
/// SKUs describe specific product variations, taking into account any combination of: attributes,
/// currency, and cost.
///
/// For example, a product may be a T-shirt, whereas a specific SKU represents the `size: large`, `color: red` version of that shirt.  Can also be used to manage inventory.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Sku {
    /// Whether the SKU is available for purchase.
    pub active: bool,
    /// A dictionary of attributes and values for the attributes defined by the product.
    ///
    /// If, for example, a product's attributes are `["size", "gender"]`, a valid SKU has the following dictionary of attributes: `{"size": "Medium", "gender": "Unisex"}`.
    pub attributes: String,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// Unique identifier for the object.
    pub id: crate::sku::SkuId,
    /// The URL of an image for this SKU, meant to be displayable to the customer.
    pub image: Option<String>,
    pub inventory: crate::sku::inventory::Inventory,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::Metadata,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SkuObject,
    /// The dimensions of this SKU for shipping purposes.
    pub package_dimensions: Option<crate::package_dimensions::PackageDimensions>,
    /// The cost of the item as a positive integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    pub price: i64,
    /// The ID of the product this SKU is associated with.
    ///
    /// The product must be currently active.
    pub product: crate::Expandable<crate::product::Product>,
    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: crate::Timestamp,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Sku {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SkuObject {
    Sku,
}

impl SkuObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sku => "sku",
        }
    }
}

impl AsRef<str> for SkuObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SkuObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for Sku {
    type Id = crate::sku::SkuId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(SkuId, "sku_");
pub mod deleted;
pub mod requests;
pub use deleted::DeletedSku;
pub mod inventory;
pub use inventory::Inventory;