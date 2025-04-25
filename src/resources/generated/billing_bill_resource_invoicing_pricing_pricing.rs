// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BillingBillResourceInvoicingPricingPricing".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingBillResourceInvoicingPricingPricing {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_details: Option<BillingBillResourceInvoicingPricingPricingPriceDetails>,

    /// The type of the pricing details.
    #[serde(rename = "type")]
    pub type_: BillingBillResourceInvoicingPricingPricingType,

    /// The unit amount (in the `currency` specified) of the item which contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingBillResourceInvoicingPricingPricingPriceDetails {

    /// The ID of the price this item is associated with.
    pub price: String,

    /// The ID of the product this item is associated with.
    pub product: String,
}

/// An enum representing the possible values of an `BillingBillResourceInvoicingPricingPricing`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingBillResourceInvoicingPricingPricingType {
    PriceDetails,
}

impl BillingBillResourceInvoicingPricingPricingType {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingBillResourceInvoicingPricingPricingType::PriceDetails => "price_details",
        }
    }
}

impl AsRef<str> for BillingBillResourceInvoicingPricingPricingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingBillResourceInvoicingPricingPricingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingBillResourceInvoicingPricingPricingType {
    fn default() -> Self {
        Self::PriceDetails
    }
}
