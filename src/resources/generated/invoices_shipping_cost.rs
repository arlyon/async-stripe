// ======================================
// This file was automatically generated.
// ======================================

use crate::params::Expandable;
use crate::resources::{ShippingRate, TaxRate};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "InvoicesShippingCost".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesShippingCost {
    /// Total shipping cost before any taxes are applied.
    pub amount_subtotal: i64,

    /// Total tax amount applied due to shipping costs.
    ///
    /// If no tax was applied, defaults to 0.
    pub amount_tax: i64,

    /// Total shipping cost after taxes are applied.
    pub amount_total: i64,

    /// The ID of the ShippingRate for this invoice.
    pub shipping_rate: Option<Expandable<ShippingRate>>,

    /// The taxes applied to the shipping rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LineItemsTaxAmount {
    /// Amount of tax applied for this rate.
    pub amount: i64,

    pub rate: TaxRate,

    /// The reasoning behind this tax, for example, if the product is tax exempt.
    ///
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: Option<LineItemsTaxAmountTaxabilityReason>,

    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: Option<i64>,
}

/// An enum representing the possible values of an `LineItemsTaxAmount`'s `taxability_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LineItemsTaxAmountTaxabilityReason {
    CustomerExempt,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    ZeroRated,
}

impl LineItemsTaxAmountTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        match self {
            LineItemsTaxAmountTaxabilityReason::CustomerExempt => "customer_exempt",
            LineItemsTaxAmountTaxabilityReason::NotCollecting => "not_collecting",
            LineItemsTaxAmountTaxabilityReason::NotSubjectToTax => "not_subject_to_tax",
            LineItemsTaxAmountTaxabilityReason::NotSupported => "not_supported",
            LineItemsTaxAmountTaxabilityReason::PortionProductExempt => "portion_product_exempt",
            LineItemsTaxAmountTaxabilityReason::PortionReducedRated => "portion_reduced_rated",
            LineItemsTaxAmountTaxabilityReason::PortionStandardRated => "portion_standard_rated",
            LineItemsTaxAmountTaxabilityReason::ProductExempt => "product_exempt",
            LineItemsTaxAmountTaxabilityReason::ProductExemptHoliday => "product_exempt_holiday",
            LineItemsTaxAmountTaxabilityReason::ProportionallyRated => "proportionally_rated",
            LineItemsTaxAmountTaxabilityReason::ReducedRated => "reduced_rated",
            LineItemsTaxAmountTaxabilityReason::ReverseCharge => "reverse_charge",
            LineItemsTaxAmountTaxabilityReason::StandardRated => "standard_rated",
            LineItemsTaxAmountTaxabilityReason::TaxableBasisReduced => "taxable_basis_reduced",
            LineItemsTaxAmountTaxabilityReason::ZeroRated => "zero_rated",
        }
    }
}

impl AsRef<str> for LineItemsTaxAmountTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LineItemsTaxAmountTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for LineItemsTaxAmountTaxabilityReason {
    fn default() -> Self {
        Self::CustomerExempt
    }
}
