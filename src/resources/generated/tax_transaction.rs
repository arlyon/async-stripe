// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{TaxTransactionId};
use crate::params::{List, Metadata, Object, Timestamp};
use crate::resources::{Currency, TaxProductResourceCustomerDetails, TaxProductResourceShipFromDetails, TaxTransactionLineItem};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxProductResourceTaxTransaction".
///
/// For more details see <https://stripe.com/docs/api/tax/transactions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxTransaction {
    /// Unique identifier for the transaction.
    pub id: TaxTransactionId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of an existing [Customer](https://stripe.com/docs/api/customers/object) used for the resource.
    pub customer: Option<String>,

    pub customer_details: TaxProductResourceCustomerDetails,

    /// The tax collected or refunded, by line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<List<TaxTransactionLineItem>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// The Unix timestamp representing when the tax liability is assumed or reduced.
    pub posted_at: Timestamp,

    /// A custom unique identifier, such as 'myOrder_123'.
    pub reference: String,

    /// If `type=reversal`, contains information about what was reversed.
    pub reversal: Option<TaxProductResourceTaxTransactionResourceReversal>,

    /// The details of the ship from location, such as the address.
    pub ship_from_details: Option<TaxProductResourceShipFromDetails>,

    /// The shipping cost details for the transaction.
    pub shipping_cost: Option<TaxProductResourceTaxTransactionShippingCost>,

    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: Timestamp,

    /// If `reversal`, this transaction reverses an earlier transaction.
    #[serde(rename = "type")]
    pub type_: TaxTransactionType,
}

impl Object for TaxTransaction {
    type Id = TaxTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax.transaction"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxTransactionResourceReversal {

    /// The `id` of the reversed `Transaction` object.
    pub original_transaction: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxTransactionShippingCost {

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
    pub tax_behavior: TaxProductResourceTaxTransactionShippingCostTaxBehavior,

    /// Detailed account of taxes relevant to shipping cost.
    ///
    /// (It is not populated for the transaction resource object and will be removed in the next API version.).
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

    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2), without country prefix.
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
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::CustomerExempt => "customer_exempt",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::NotCollecting => "not_collecting",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::NotSubjectToTax => "not_subject_to_tax",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::NotSupported => "not_supported",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::PortionProductExempt => "portion_product_exempt",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::PortionReducedRated => "portion_reduced_rated",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::PortionStandardRated => "portion_standard_rated",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ProductExempt => "product_exempt",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ProductExemptHoliday => "product_exempt_holiday",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ProportionallyRated => "proportionally_rated",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ReducedRated => "reduced_rated",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ReverseCharge => "reverse_charge",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::StandardRated => "standard_rated",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::TaxableBasisReduced => "taxable_basis_reduced",
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
    RetailDeliveryFee,
    Rst,
    SalesTax,
    ServiceTax,
    Vat,
}

impl TaxProductResourceLineItemTaxRateDetailsTaxType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceLineItemTaxRateDetailsTaxType::AmusementTax => "amusement_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::CommunicationsTax => "communications_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Gst => "gst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Hst => "hst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Igst => "igst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Jct => "jct",
            TaxProductResourceLineItemTaxRateDetailsTaxType::LeaseTax => "lease_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Pst => "pst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Qst => "qst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::RetailDeliveryFee => "retail_delivery_fee",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Rst => "rst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::SalesTax => "sales_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::ServiceTax => "service_tax",
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

/// An enum representing the possible values of an `TaxProductResourceTaxTransactionShippingCost`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    Exclusive,
    Inclusive,
}

impl TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceTaxTransactionShippingCostTaxBehavior::Exclusive => "exclusive",
            TaxProductResourceTaxTransactionShippingCostTaxBehavior::Inclusive => "inclusive",
        }
    }
}

impl AsRef<str> for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `TaxTransaction`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxTransactionType {
    Reversal,
    Transaction,
}

impl TaxTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxTransactionType::Reversal => "reversal",
            TaxTransactionType::Transaction => "transaction",
        }
    }
}

impl AsRef<str> for TaxTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxTransactionType {
    fn default() -> Self {
        Self::Reversal
    }
}
