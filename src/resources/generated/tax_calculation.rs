// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::TaxCalculationId;
use crate::params::{List, Object, Timestamp};
use crate::resources::{Currency, TaxCalculationLineItem, TaxProductResourceCustomerDetails};
use serde::{Deserialize, Serialize};

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
    pub customer: Option<String>,

    pub customer_details: TaxProductResourceCustomerDetails,

    /// Timestamp of date at which the tax calculation will expire.
    pub expires_at: Option<Timestamp>,

    /// The list of items the customer is purchasing.
    pub line_items: Option<List<TaxCalculationLineItem>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The shipping cost details for the calculation.
    pub shipping_cost: Option<TaxProductResourceTaxCalculationShippingCost>,

    /// The amount of tax to be collected on top of the line item prices.
    pub tax_amount_exclusive: i64,

    /// The amount of tax already included in the line item prices.
    pub tax_amount_inclusive: i64,

    /// Breakdown of individual tax amounts that add up to the total.
    pub tax_breakdown: Vec<TaxProductResourceTaxBreakdown>,

    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: Timestamp,
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxBreakdown {
    /// The amount of tax, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,

    /// Specifies whether the tax amount is included in the line item amount.
    pub inclusive: bool,

    pub tax_rate_details: TaxProductResourceTaxRateDetails,

    /// The reasoning behind this tax, for example, if the product is tax exempt.
    ///
    /// We might extend the possible values for this field to support new tax rules.
    pub taxability_reason: TaxProductResourceTaxBreakdownTaxabilityReason,

    /// The amount on which tax is calculated, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub taxable_amount: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxCalculationShippingCost {
    /// The shipping amount in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,

    /// The amount of tax calculated for shipping, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_tax: i64,

    /// The ID of an existing [ShippingRate](https://stripe.com/docs/api/shipping_rates/object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,

    /// Specifies whether the `amount` includes taxes.
    ///
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: TaxProductResourceTaxCalculationShippingCostTaxBehavior,

    /// Detailed account of taxes relevant to shipping cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_breakdown: Option<Vec<TaxProductResourceLineItemTaxBreakdown>>,

    /// The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for shipping.
    pub tax_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceLineItemTaxBreakdown {
    /// The amount of tax, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,

    pub jurisdiction: TaxProductResourceJurisdiction,

    /// Indicates whether the jurisdiction was determined by the origin (merchant's address) or destination (customer's address).
    pub sourcing: TaxProductResourceLineItemTaxBreakdownSourcing,

    /// Details regarding the rate for this tax.
    ///
    /// This field will be `null` when the tax is not imposed, for example if the product is exempt from tax.
    pub tax_rate_details: Option<TaxProductResourceLineItemTaxRateDetails>,

    /// The reasoning behind this tax, for example, if the product is tax exempt.
    ///
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: TaxProductResourceLineItemTaxBreakdownTaxabilityReason,

    /// The amount on which tax is calculated, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub taxable_amount: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceJurisdiction {
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,

    /// A human-readable name for the jurisdiction imposing the tax.
    pub display_name: String,

    /// Indicates the level of the jurisdiction imposing the tax.
    pub level: TaxProductResourceJurisdictionLevel,

    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceLineItemTaxRateDetails {
    /// A localized display name for tax type, intended to be human-readable.
    ///
    /// For example, "Local Sales and Use Tax", "Value-added tax (VAT)", or "Umsatzsteuer (USt.)".
    pub display_name: String,

    /// The tax rate percentage as a string.
    ///
    /// For example, 8.5% is represented as "8.5".
    pub percentage_decimal: String,

    /// The tax type, such as `vat` or `sales_tax`.
    pub tax_type: TaxProductResourceLineItemTaxRateDetailsTaxType,
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

/// An enum representing the possible values of an `TaxProductResourceJurisdiction`'s `level` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceJurisdictionLevel {
    City,
    Country,
    County,
    District,
    State,
}

impl TaxProductResourceJurisdictionLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceJurisdictionLevel::City => "city",
            TaxProductResourceJurisdictionLevel::Country => "country",
            TaxProductResourceJurisdictionLevel::County => "county",
            TaxProductResourceJurisdictionLevel::District => "district",
            TaxProductResourceJurisdictionLevel::State => "state",
        }
    }
}

impl AsRef<str> for TaxProductResourceJurisdictionLevel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceJurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceJurisdictionLevel {
    fn default() -> Self {
        Self::City
    }
}

/// An enum representing the possible values of an `TaxProductResourceLineItemTaxBreakdown`'s `sourcing` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceLineItemTaxBreakdownSourcing {
    Destination,
    Origin,
}

impl TaxProductResourceLineItemTaxBreakdownSourcing {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceLineItemTaxBreakdownSourcing::Destination => "destination",
            TaxProductResourceLineItemTaxBreakdownSourcing::Origin => "origin",
        }
    }
}

impl AsRef<str> for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn default() -> Self {
        Self::Destination
    }
}

/// An enum representing the possible values of an `TaxProductResourceLineItemTaxBreakdown`'s `taxability_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
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

impl TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::CustomerExempt => {
                "customer_exempt"
            }
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::NotCollecting => {
                "not_collecting"
            }
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::NotSubjectToTax => {
                "not_subject_to_tax"
            }
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::NotSupported => "not_supported",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::PortionProductExempt => {
                "portion_product_exempt"
            }
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::PortionReducedRated => {
                "portion_reduced_rated"
            }
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::PortionStandardRated => {
                "portion_standard_rated"
            }
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ProductExempt => {
                "product_exempt"
            }
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ProductExemptHoliday => {
                "product_exempt_holiday"
            }
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ProportionallyRated => {
                "proportionally_rated"
            }
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ReducedRated => "reduced_rated",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ReverseCharge => {
                "reverse_charge"
            }
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::StandardRated => {
                "standard_rated"
            }
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::TaxableBasisReduced => {
                "taxable_basis_reduced"
            }
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ZeroRated => "zero_rated",
        }
    }
}

impl AsRef<str> for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn default() -> Self {
        Self::CustomerExempt
    }
}

/// An enum representing the possible values of an `TaxProductResourceLineItemTaxRateDetails`'s `tax_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceLineItemTaxRateDetailsTaxType {
    AmusementTax,
    CommunicationsTax,
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

impl TaxProductResourceLineItemTaxRateDetailsTaxType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceLineItemTaxRateDetailsTaxType::AmusementTax => "amusement_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::CommunicationsTax => {
                "communications_tax"
            }
            TaxProductResourceLineItemTaxRateDetailsTaxType::Gst => "gst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Hst => "hst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Igst => "igst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Jct => "jct",
            TaxProductResourceLineItemTaxRateDetailsTaxType::LeaseTax => "lease_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Pst => "pst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Qst => "qst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Rst => "rst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::SalesTax => "sales_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Vat => "vat",
        }
    }
}

impl AsRef<str> for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn default() -> Self {
        Self::AmusementTax
    }
}

/// An enum representing the possible values of an `TaxProductResourceTaxBreakdown`'s `taxability_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceTaxBreakdownTaxabilityReason {
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

impl TaxProductResourceTaxBreakdownTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceTaxBreakdownTaxabilityReason::CustomerExempt => "customer_exempt",
            TaxProductResourceTaxBreakdownTaxabilityReason::NotCollecting => "not_collecting",
            TaxProductResourceTaxBreakdownTaxabilityReason::NotSubjectToTax => "not_subject_to_tax",
            TaxProductResourceTaxBreakdownTaxabilityReason::NotSupported => "not_supported",
            TaxProductResourceTaxBreakdownTaxabilityReason::PortionProductExempt => {
                "portion_product_exempt"
            }
            TaxProductResourceTaxBreakdownTaxabilityReason::PortionReducedRated => {
                "portion_reduced_rated"
            }
            TaxProductResourceTaxBreakdownTaxabilityReason::PortionStandardRated => {
                "portion_standard_rated"
            }
            TaxProductResourceTaxBreakdownTaxabilityReason::ProductExempt => "product_exempt",
            TaxProductResourceTaxBreakdownTaxabilityReason::ProductExemptHoliday => {
                "product_exempt_holiday"
            }
            TaxProductResourceTaxBreakdownTaxabilityReason::ProportionallyRated => {
                "proportionally_rated"
            }
            TaxProductResourceTaxBreakdownTaxabilityReason::ReducedRated => "reduced_rated",
            TaxProductResourceTaxBreakdownTaxabilityReason::ReverseCharge => "reverse_charge",
            TaxProductResourceTaxBreakdownTaxabilityReason::StandardRated => "standard_rated",
            TaxProductResourceTaxBreakdownTaxabilityReason::TaxableBasisReduced => {
                "taxable_basis_reduced"
            }
            TaxProductResourceTaxBreakdownTaxabilityReason::ZeroRated => "zero_rated",
        }
    }
}

impl AsRef<str> for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn default() -> Self {
        Self::CustomerExempt
    }
}

/// An enum representing the possible values of an `TaxProductResourceTaxCalculationShippingCost`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    Exclusive,
    Inclusive,
}

impl TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceTaxCalculationShippingCostTaxBehavior::Exclusive => "exclusive",
            TaxProductResourceTaxCalculationShippingCostTaxBehavior::Inclusive => "inclusive",
        }
    }
}

impl AsRef<str> for TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `TaxProductResourceTaxRateDetails`'s `tax_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceTaxRateDetailsTaxType {
    AmusementTax,
    CommunicationsTax,
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
            TaxProductResourceTaxRateDetailsTaxType::AmusementTax => "amusement_tax",
            TaxProductResourceTaxRateDetailsTaxType::CommunicationsTax => "communications_tax",
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
        Self::AmusementTax
    }
}
