// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{CustomerId, TaxCalculationId};
use crate::params::{Expand, List, Object, Timestamp};
use crate::resources::{
    CreateTaxCalculationLineItem, Currency, TaxCalculationLineItem,
    TaxProductResourceCustomerDetails, TaxProductResourceShippingCost,
};

/// The resource representing a Stripe "TaxProductResourceTaxCalculation".
///
/// For more details see <https://stripe.com/docs/api/tax/calculations/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxCalculation {
    /// Unique identifier for the calculation.
    pub id: TaxCalculationId,

    /// Total after taxes.
    pub amount_total: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of an existing [Customer](https://stripe.com/docs/api/customers/object) used for the resource.
    pub customer: Option<CustomerId>,

    pub customer_details: TaxProductResourceCustomerDetails,

    /// Timestamp of date at which the tax calculation will expire.
    pub expires_at: Option<Timestamp>,

    /// The list of items the customer is purchasing.
    pub line_items: List<TaxCalculationLineItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The shipping cost details for the calculation.
    pub shipping_cost: Option<TaxProductResourceShippingCost>,

    /// The amount of tax to be collected on top of the line item prices.
    pub tax_amount_exclusive: i64,

    /// The amount of tax already included in the line item prices.
    pub tax_amount_inclusive: i64,

    /// Breakdown of individual tax amounts that add up to the total.
    pub tax_breakdown: Vec<TaxProductResourceTaxBreakdown>,

    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: Timestamp,
}
impl TaxCalculation {
    /// Creates an item to be added to a draft invoice (up to 250 items per invoice).
    ///
    /// If no invoice is specified, the item will be on the next invoice created for the customer specified.
    pub fn create(client: &Client, params: CreateTaxCalculation<'_>) -> Response<TaxCalculation> {
        client.post_form("/tax/calculations", &params)
    }
}

impl Object for TaxCalculation {
    type Id = TaxCalculationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax.calculation"
    }
}

/// The parameters for `TaxCalculation::create`
///
/// For more details see <https://stripe.com/docs/api/tax/calculations/object>
#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateTaxCalculation<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<&'a str>,

    /// The ID of an existing [Customer](https://stripe.com/docs/api/customers/object) used for the resource.
    pub customer: Option<CustomerId>,

    pub customer_details: Option<TaxProductResourceCustomerDetails>,

    /// The list of items the customer is purchasing.
    pub line_items: List<CreateTaxCalculationLineItem>,

    /// The shipping cost details for the calculation.
    pub shipping_cost: Option<TaxProductResourceShippingCost>,

    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: Option<Timestamp>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxBreakdown {
    /// The amount of tax, in integer cents.
    pub amount: i64,

    /// Specifies whether the tax amount is included in the line item amount.
    pub inclusive: bool,

    pub tax_rate_details: TaxProductResourceTaxRateDetails,

    /// The amount on which tax is calculated, in integer cents.
    pub taxable_amount: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxRateDetails {
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,

    /// The tax rate percentage as a string.
    ///
    /// For example, 8.5% is represented as `"8.5"`.
    pub percentage_decimal: String,

    /// State, county, province, or region.
    pub state: Option<String>,

    /// The tax type, such as `vat` or `sales_tax`.
    pub tax_type: Option<TaxProductResourceTaxRateDetailsTaxType>,
}

/// An enum representing the possible values of an `TaxProductResourceTaxRateDetails`'s `tax_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceTaxRateDetailsTaxType {
    Gst,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    Pst,
    Qst,
    Rst,
    SalesTax,
    Vat,
}

impl TaxProductResourceTaxRateDetailsTaxType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceTaxRateDetailsTaxType::Gst => "gst",
            TaxProductResourceTaxRateDetailsTaxType::Hst => "hst",
            TaxProductResourceTaxRateDetailsTaxType::Igst => "igst",
            TaxProductResourceTaxRateDetailsTaxType::Jct => "jct",
            TaxProductResourceTaxRateDetailsTaxType::LeaseTax => "lease_tax",
            TaxProductResourceTaxRateDetailsTaxType::Pst => "pst",
            TaxProductResourceTaxRateDetailsTaxType::Qst => "qst",
            TaxProductResourceTaxRateDetailsTaxType::Rst => "rst",
            TaxProductResourceTaxRateDetailsTaxType::SalesTax => "sales_tax",
            TaxProductResourceTaxRateDetailsTaxType::Vat => "vat",
        }
    }
}

impl AsRef<str> for TaxProductResourceTaxRateDetailsTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceTaxRateDetailsTaxType {
    fn default() -> Self {
        Self::Gst
    }
}
