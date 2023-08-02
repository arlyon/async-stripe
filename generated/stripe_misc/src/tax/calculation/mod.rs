/// A Tax Calculation allows you to calculate the tax to collect from your customer.
///
/// Related guide: [Calculate tax in your custom payment flow](https://stripe.com/docs/tax/custom).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Calculation {
    /// Total after taxes.
    pub amount_total: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of an existing [Customer](https://stripe.com/docs/api/customers/object) used for the resource.
    pub customer: Option<String>,
    pub customer_details: stripe_misc::customer_details::CustomerDetails,
    /// Timestamp of date at which the tax calculation will expire.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the calculation.
    pub id: Option<stripe_misc::tax::calculation::TaxCalculationId>,
    /// The list of items the customer is purchasing.
    pub line_items:
        stripe_types::List<stripe_misc::tax::calculation_line_item::CalculationLineItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CalculationObject,
    /// The shipping cost details for the calculation.
    pub shipping_cost: Option<stripe_misc::tax::calculation_shipping_cost::CalculationShippingCost>,
    /// The amount of tax to be collected on top of the line item prices.
    pub tax_amount_exclusive: i64,
    /// The amount of tax already included in the line item prices.
    pub tax_amount_inclusive: i64,
    /// Breakdown of individual tax amounts that add up to the total.
    pub tax_breakdown: Vec<stripe_misc::tax_breakdown::TaxBreakdown>,
    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: stripe_types::Timestamp,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CalculationObject {
    TaxCalculation,
}

impl CalculationObject {
    pub fn as_str(self) -> &'static str {
        use CalculationObject::*;
        match self {
            TaxCalculation => "tax.calculation",
        }
    }
}

impl std::str::FromStr for CalculationObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CalculationObject::*;
        match s {
            "tax.calculation" => Ok(TaxCalculation),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CalculationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CalculationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CalculationObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CalculationObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CalculationObject"))
    }
}
impl stripe_types::Object for Calculation {
    type Id = Option<stripe_misc::tax::calculation::TaxCalculationId>;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TaxCalculationId);
pub mod requests;
