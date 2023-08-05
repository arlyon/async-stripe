#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceTaxCalculationLineItem {
    /// The line item amount in integer cents.
    ///
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    /// The amount of tax calculated for this line item, in integer cents.
    pub amount_tax: i64,
    /// Unique identifier for the object.
    pub id: stripe_misc::tax_product_resource_tax_calculation_line_item::TaxCalculationLineItemId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
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
    pub tax_behavior: TaxProductResourceTaxCalculationLineItemTaxBehavior,
    /// Detailed account of taxes relevant to this line item.
    pub tax_breakdown: Option<Vec<stripe_misc::TaxProductResourceLineItemTaxBreakdown>>,
    /// The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for this resource.
    pub tax_code: String,
}
/// Specifies whether the `amount` includes taxes.
///
/// If `tax_behavior=inclusive`, then the amount includes taxes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceTaxCalculationLineItemTaxBehavior {
    Exclusive,
    Inclusive,
}

impl TaxProductResourceTaxCalculationLineItemTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceTaxCalculationLineItemTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxCalculationLineItemTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxCalculationLineItemTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxProductResourceTaxCalculationLineItemTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceTaxCalculationLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxCalculationLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceTaxCalculationLineItemTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxCalculationLineItemTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductResourceTaxCalculationLineItemTaxBehavior",
            )
        })
    }
}
impl stripe_types::Object for TaxProductResourceTaxCalculationLineItem {
    type Id = stripe_misc::tax_product_resource_tax_calculation_line_item::TaxCalculationLineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TaxCalculationLineItemId);
