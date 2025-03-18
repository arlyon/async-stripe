// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::TaxCalculationLineItemId;
use crate::params::Object;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxProductResourceTaxCalculationLineItem".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxCalculationLineItem {
    /// Unique identifier for the object.
    pub id: TaxCalculationLineItemId,

    /// The line item amount in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,

    /// The amount of tax calculated for this line item, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_tax: i64,

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
    pub tax_behavior: TaxCalculationLineItemTaxBehavior,

    /// Detailed account of taxes relevant to this line item.
    pub tax_breakdown: Option<Vec<TaxProductResourceLineItemTaxBreakdown>>,

    /// The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for this resource.
    pub tax_code: String,
}

impl Object for TaxCalculationLineItem {
    type Id = TaxCalculationLineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax.calculation_line_item"
    }
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

/// An enum representing the possible values of an `TaxCalculationLineItem`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxCalculationLineItemTaxBehavior {
    Exclusive,
    Inclusive,
}

impl TaxCalculationLineItemTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxCalculationLineItemTaxBehavior::Exclusive => "exclusive",
            TaxCalculationLineItemTaxBehavior::Inclusive => "inclusive",
        }
    }
}

impl AsRef<str> for TaxCalculationLineItemTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxCalculationLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxCalculationLineItemTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
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
