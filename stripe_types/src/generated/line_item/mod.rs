/// A line item.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LineItem {
    /// Total discount amount applied.
    ///
    /// If no discounts were applied, defaults to 0.
    pub amount_discount: i64,
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total tax amount applied.
    ///
    /// If no tax was applied, defaults to 0.
    pub amount_tax: i64,
    /// Total after discounts and taxes.
    pub amount_total: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Defaults to product name.
    pub description: String,
    /// The discounts applied to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<stripe_types::LineItemsDiscountAmount>>,
    /// Unique identifier for the object.
    pub id: stripe_types::line_item::ItemId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: LineItemObject,
    /// The price used to generate the line item.
    pub price: Option<stripe_types::Price>,
    /// The quantity of products being purchased.
    pub quantity: Option<u64>,
    /// The taxes applied to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<stripe_types::LineItemsTaxAmount>>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LineItemObject {
    Item,
}

impl LineItemObject {
    pub fn as_str(self) -> &'static str {
        use LineItemObject::*;
        match self {
            Item => "item",
        }
    }
}

impl std::str::FromStr for LineItemObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LineItemObject::*;
        match s {
            "item" => Ok(Item),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for LineItemObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LineItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for LineItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for LineItemObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LineItemObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for LineItemObject"))
    }
}
impl stripe_types::Object for LineItem {
    type Id = stripe_types::line_item::ItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ItemId);
