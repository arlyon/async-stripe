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
    pub attributes: std::collections::HashMap<String, String>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_product::sku::SkuId,
    /// The URL of an image for this SKU, meant to be displayable to the customer.
    pub image: Option<String>,
    pub inventory: stripe_product::sku::inventory::Inventory,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SkuObject,
    /// The dimensions of this SKU for shipping purposes.
    pub package_dimensions: Option<stripe_types::package_dimensions::PackageDimensions>,
    /// The cost of the item as a positive integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    pub price: i64,
    /// The ID of the product this SKU is associated with.
    ///
    /// The product must be currently active.
    pub product: stripe_types::Expandable<stripe_core::product::Product>,
    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for SkuObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sku" => Ok(Self::Sku),

            _ => Err(()),
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
impl serde::Serialize for SkuObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SkuObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SkuObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SkuObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<SkuObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SkuObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for Sku {
    type Id = stripe_product::sku::SkuId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(SkuId, "sku_");
pub mod deleted;
pub mod requests;
pub use deleted::DeletedSku;
pub mod inventory;
pub use inventory::Inventory;
