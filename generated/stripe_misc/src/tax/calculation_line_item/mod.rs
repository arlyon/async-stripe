#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CalculationLineItem {
    /// The line item amount in integer cents.
    ///
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    /// The amount of tax calculated for this line item, in integer cents.
    pub amount_tax: i64,
    /// Unique identifier for the object.
    pub id: stripe_misc::tax::calculation_line_item::TaxCalculationLineItemId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CalculationLineItemObject,
    /// The ID of an existing [Product](https://stripe.com/docs/api/products/object).
    pub product: Option<String>,
    /// The number of units of the item being purchased.
    ///
    /// For reversals, this is the quantity reversed.
    pub quantity: u64,
    /// A custom identifier for this line item.
    pub reference: Option<String>,
    /// Specifies whether the `amount` includes taxes.
    ///
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: CalculationLineItemTaxBehavior,
    /// Detailed account of taxes relevant to this line item.
    pub tax_breakdown: Option<Vec<stripe_misc::line_item_tax_breakdown::LineItemTaxBreakdown>>,
    /// The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for this resource.
    pub tax_code: String,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CalculationLineItemObject {
    TaxCalculationLineItem,
}

impl CalculationLineItemObject {
    pub fn as_str(self) -> &'static str {
        use CalculationLineItemObject::*;
        match self {
            TaxCalculationLineItem => "tax.calculation_line_item",
        }
    }
}

impl std::str::FromStr for CalculationLineItemObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CalculationLineItemObject::*;
        match s {
            "tax.calculation_line_item" => Ok(TaxCalculationLineItem),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CalculationLineItemObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CalculationLineItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CalculationLineItemObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CalculationLineItemObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CalculationLineItemObject"))
    }
}
/// Specifies whether the `amount` includes taxes.
///
/// If `tax_behavior=inclusive`, then the amount includes taxes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CalculationLineItemTaxBehavior {
    Exclusive,
    Inclusive,
}

impl CalculationLineItemTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CalculationLineItemTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
        }
    }
}

impl std::str::FromStr for CalculationLineItemTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CalculationLineItemTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CalculationLineItemTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CalculationLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CalculationLineItemTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CalculationLineItemTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CalculationLineItemTaxBehavior")
        })
    }
}
impl stripe_types::Object for CalculationLineItem {
    type Id = stripe_misc::tax::calculation_line_item::TaxCalculationLineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TaxCalculationLineItemId);
