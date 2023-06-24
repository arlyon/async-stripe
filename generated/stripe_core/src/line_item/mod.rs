/// A line item.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
    pub discounts: Option<Vec<stripe_core::line_item::discount::Discount>>,
    /// Unique identifier for the object.
    pub id: stripe_core::line_item::ItemId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: LineItemObject,
    /// The price used to generate the line item.
    pub price: Option<stripe_core::price::Price>,
    /// The ID of the product for this line item.
    ///
    /// This will always be the same as `price.product`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<stripe_types::Expandable<stripe_core::product::Product>>,
    /// The quantity of products being purchased.
    pub quantity: Option<u64>,
    /// The taxes applied to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<stripe_core::line_item::tax::Tax>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LineItem {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LineItemObject {
    Item,
}

impl LineItemObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Item => "item",
        }
    }
}

impl std::str::FromStr for LineItemObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "item" => Ok(Self::Item),

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
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for LineItemObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LineItemObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<LineItemObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(LineItemObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for LineItem {
    type Id = stripe_core::line_item::ItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ItemId);
pub mod discount;
pub use discount::Discount;
pub mod tax;
pub use tax::Tax;
