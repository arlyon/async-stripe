// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{InvoiceLineItemId};
use crate::params::{Expandable, Metadata, Object};
use crate::resources::{BillingBillResourceInvoicingLinesCommonProrationDetails, BillingBillResourceInvoicingPricingPricing, BillingCreditBalanceTransaction, Currency, Discount, Period, Subscription};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "InvoiceLineItem".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceLineItem {
    /// Unique identifier for the object.
    pub id: InvoiceLineItemId,

    /// The amount, in cents (or local equivalent).
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// The amount of discount calculated per discount for this line item.
    pub discount_amounts: Option<Vec<DiscountsResourceDiscountAmount>>,

    /// If true, discounts will apply to this line item.
    ///
    /// Always false for prorations.
    pub discountable: bool,

    /// The discounts applied to the invoice line item.
    ///
    /// Line item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Vec<Expandable<Discount>>,

    /// The ID of the invoice that contains this line item.
    pub invoice: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Note that for line items with `type=subscription`, `metadata` reflects the current metadata from the subscription associated with the line item, unless the invoice line was directly updated with different metadata after creation.
    pub metadata: Metadata,

    /// The parent that generated this invoice.
    pub parent: Option<BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent>,

    pub period: Option<Period>,

    /// Contains pretax credit amounts (ex: discount, credit grants, etc) that apply to this line item.
    pub pretax_credit_amounts: Option<Vec<InvoicesResourcePretaxCreditAmount>>,

    /// The pricing information of the line item.
    pub pricing: Option<BillingBillResourceInvoicingPricingPricing>,

    /// The quantity of the subscription, if the line item is a subscription or a proration.
    pub quantity: Option<u64>,

    pub subscription: Option<Expandable<Subscription>>,

    /// The tax information of the line item.
    pub taxes: Option<Vec<BillingBillResourceInvoicingTaxesTax>>,
}

impl Object for InvoiceLineItem {
    type Id = InvoiceLineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "line_item"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent {

    /// Details about the invoice item that generated this line item.
    pub invoice_item_details: Option<BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent>,

    /// Details about the subscription item that generated this line item.
    pub subscription_item_details: Option<BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent>,

    /// The type of parent that generated this line item.
    #[serde(rename = "type")]
    pub type_: BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent {

    /// The invoice item that generated this line item.
    pub invoice_item: String,

    /// Whether this is a proration.
    pub proration: bool,

    /// Additional details for proration line items.
    pub proration_details: Option<BillingBillResourceInvoicingLinesCommonProrationDetails>,

    /// The subscription that the invoice item belongs to.
    pub subscription: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent {

    /// The invoice item that generated this line item.
    pub invoice_item: Option<String>,

    /// Whether this is a proration.
    pub proration: bool,

    /// Additional details for proration line items.
    pub proration_details: Option<BillingBillResourceInvoicingLinesCommonProrationDetails>,

    /// The subscription that the subscription item belongs to.
    pub subscription: String,

    /// The subscription item that generated this line item.
    pub subscription_item: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingBillResourceInvoicingTaxesTax {

    /// The amount of the tax, in cents (or local equivalent).
    pub amount: i64,

    /// Whether this tax is inclusive or exclusive.
    pub tax_behavior: BillingBillResourceInvoicingTaxesTaxTaxBehavior,

    /// Additional details about the tax rate.
    ///
    /// Only present when `type` is `tax_rate_details`.
    pub tax_rate_details: Option<BillingBillResourceInvoicingTaxesTaxRateDetails>,

    /// The reasoning behind this tax, for example, if the product is tax exempt.
    ///
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: BillingBillResourceInvoicingTaxesTaxTaxabilityReason,

    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: Option<i64>,

    /// The type of tax information.
    #[serde(rename = "type")]
    pub type_: BillingBillResourceInvoicingTaxesTaxType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingBillResourceInvoicingTaxesTaxRateDetails {

    pub tax_rate: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DiscountsResourceDiscountAmount {

    /// The amount, in cents (or local equivalent), of the discount.
    pub amount: i64,

    /// The discount that was applied to get this discount amount.
    pub discount: Expandable<Discount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesResourcePretaxCreditAmount {

    /// The amount, in cents (or local equivalent), of the pretax credit amount.
    pub amount: i64,

    /// The credit balance transaction that was applied to get this pretax credit amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_balance_transaction: Option<Expandable<BillingCreditBalanceTransaction>>,

    /// The discount that was applied to get this pretax credit amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Expandable<Discount>>,

    /// Type of the pretax credit amount referenced.
    #[serde(rename = "type")]
    pub type_: InvoicesResourcePretaxCreditAmountType,
}

/// An enum representing the possible values of an `BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    InvoiceItemDetails,
    SubscriptionItemDetails,
}

impl BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::InvoiceItemDetails => "invoice_item_details",
            BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::SubscriptionItemDetails => "subscription_item_details",
        }
    }
}

impl AsRef<str> for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn default() -> Self {
        Self::InvoiceItemDetails
    }
}

/// An enum representing the possible values of an `BillingBillResourceInvoicingTaxesTax`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    Exclusive,
    Inclusive,
}

impl BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingBillResourceInvoicingTaxesTaxTaxBehavior::Exclusive => "exclusive",
            BillingBillResourceInvoicingTaxesTaxTaxBehavior::Inclusive => "inclusive",
        }
    }
}

impl AsRef<str> for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `BillingBillResourceInvoicingTaxesTax`'s `taxability_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    CustomerExempt,
    NotAvailable,
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

impl BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::CustomerExempt => "customer_exempt",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::NotAvailable => "not_available",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::NotCollecting => "not_collecting",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::NotSubjectToTax => "not_subject_to_tax",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::NotSupported => "not_supported",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::PortionProductExempt => "portion_product_exempt",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::PortionReducedRated => "portion_reduced_rated",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::PortionStandardRated => "portion_standard_rated",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ProductExempt => "product_exempt",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ProductExemptHoliday => "product_exempt_holiday",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ProportionallyRated => "proportionally_rated",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ReducedRated => "reduced_rated",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ReverseCharge => "reverse_charge",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::StandardRated => "standard_rated",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::TaxableBasisReduced => "taxable_basis_reduced",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ZeroRated => "zero_rated",
        }
    }
}

impl AsRef<str> for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn default() -> Self {
        Self::CustomerExempt
    }
}

/// An enum representing the possible values of an `BillingBillResourceInvoicingTaxesTax`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingBillResourceInvoicingTaxesTaxType {
    TaxRateDetails,
}

impl BillingBillResourceInvoicingTaxesTaxType {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingBillResourceInvoicingTaxesTaxType::TaxRateDetails => "tax_rate_details",
        }
    }
}

impl AsRef<str> for BillingBillResourceInvoicingTaxesTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingBillResourceInvoicingTaxesTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingBillResourceInvoicingTaxesTaxType {
    fn default() -> Self {
        Self::TaxRateDetails
    }
}

/// An enum representing the possible values of an `InvoicesResourcePretaxCreditAmount`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicesResourcePretaxCreditAmountType {
    CreditBalanceTransaction,
    Discount,
}

impl InvoicesResourcePretaxCreditAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicesResourcePretaxCreditAmountType::CreditBalanceTransaction => "credit_balance_transaction",
            InvoicesResourcePretaxCreditAmountType::Discount => "discount",
        }
    }
}

impl AsRef<str> for InvoicesResourcePretaxCreditAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicesResourcePretaxCreditAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoicesResourcePretaxCreditAmountType {
    fn default() -> Self {
        Self::CreditBalanceTransaction
    }
}
