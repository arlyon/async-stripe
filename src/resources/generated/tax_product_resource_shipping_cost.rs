// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxProductResourceShippingCost".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceShippingCost {
    /// The shipping amount in integer cents.
    ///
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,

    /// The amount of tax calculated for shipping, in integer cents.
    pub amount_tax: i64,

    /// The ID of an existing [ShippingRate](https://stripe.com/docs/api/shipping_rates/object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,

    /// Specifies whether the `amount` includes taxes.
    ///
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: TaxProductResourceShippingCostTaxBehavior,

    /// The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for shipping.
    pub tax_code: String,
}

/// An enum representing the possible values of an `TaxProductResourceShippingCost`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceShippingCostTaxBehavior {
    Exclusive,
    Inclusive,
}

impl TaxProductResourceShippingCostTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceShippingCostTaxBehavior::Exclusive => "exclusive",
            TaxProductResourceShippingCostTaxBehavior::Inclusive => "inclusive",
        }
    }
}

impl AsRef<str> for TaxProductResourceShippingCostTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceShippingCostTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
