// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::InvoiceLineItemId;
use crate::params::{Expandable, Metadata, Object};
use crate::resources::{
    Currency, Discount, InvoiceItem, Period, Plan, Price, Subscription, SubscriptionItem, TaxRate,
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "InvoiceLineItem".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceLineItem {
    /// Unique identifier for the object.
    pub id: InvoiceLineItemId,

    /// The amount, in cents (or local equivalent).
    pub amount: i64,

    /// The integer amount in cents (or local equivalent) representing the amount for this line item, excluding all tax and discounts.
    pub amount_excluding_tax: Option<i64>,

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
    pub discounts: Option<Vec<Expandable<Discount>>>,

    /// The ID of the [invoice item](https://stripe.com/docs/api/invoiceitems) associated with this line item if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<Expandable<InvoiceItem>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Note that for line items with `type=subscription` this will reflect the metadata of the subscription that caused the line item to be created.
    pub metadata: Metadata,

    pub period: Option<Period>,

    /// The plan of the subscription, if the line item is a subscription or a proration.
    pub plan: Option<Plan>,

    /// The price of the line item.
    pub price: Option<Price>,

    /// Whether this is a proration.
    pub proration: bool,

    /// Additional details for proration line items.
    pub proration_details: Option<InvoicesResourceLineItemsProrationDetails>,

    /// The quantity of the subscription, if the line item is a subscription or a proration.
    pub quantity: Option<u64>,

    /// The subscription that the invoice item pertains to, if any.
    pub subscription: Option<Expandable<Subscription>>,

    /// The subscription item that generated this line item.
    ///
    /// Left empty if the line item is not an explicit result of a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<Expandable<SubscriptionItem>>,

    /// The amount of tax calculated per tax rate for this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<Vec<TaxAmount>>,

    /// The tax rates which apply to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,

    /// A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
    #[serde(rename = "type")]
    pub type_: InvoiceLineItemType,

    /// The amount in cents (or local equivalent) representing the unit amount for this line item, excluding all tax and discounts.
    pub unit_amount_excluding_tax: Option<String>,
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
pub struct DiscountsResourceDiscountAmount {
    /// The amount, in cents (or local equivalent), of the discount.
    pub amount: i64,

    /// The discount that was applied to get this discount amount.
    pub discount: Expandable<Discount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxAmount {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,

    /// Whether this tax amount is inclusive or exclusive.
    pub inclusive: bool,

    /// The tax rate that was applied to get this tax amount.
    pub tax_rate: Expandable<TaxRate>,

    /// The reasoning behind this tax, for example, if the product is tax exempt.
    ///
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: Option<TaxAmountTaxabilityReason>,

    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesResourceLineItemsProrationDetails {
    /// For a credit proration `line_item`, the original debit line_items to which the credit proration applies.
    pub credited_items: Option<InvoicesResourceLineItemsCreditedItems>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesResourceLineItemsCreditedItems {
    /// Invoice containing the credited invoice line items.
    pub invoice: String,

    /// Credited invoice line items.
    pub invoice_line_items: Vec<String>,
}

/// An enum representing the possible values of an `InvoiceLineItem`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceLineItemType {
    #[serde(rename = "invoiceitem")]
    InvoiceItem,
    Subscription,
}

impl InvoiceLineItemType {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceLineItemType::InvoiceItem => "invoiceitem",
            InvoiceLineItemType::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for InvoiceLineItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoiceLineItemType {
    fn default() -> Self {
        Self::InvoiceItem
    }
}

/// An enum representing the possible values of an `TaxAmount`'s `taxability_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxAmountTaxabilityReason {
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

impl TaxAmountTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxAmountTaxabilityReason::CustomerExempt => "customer_exempt",
            TaxAmountTaxabilityReason::NotCollecting => "not_collecting",
            TaxAmountTaxabilityReason::NotSubjectToTax => "not_subject_to_tax",
            TaxAmountTaxabilityReason::NotSupported => "not_supported",
            TaxAmountTaxabilityReason::PortionProductExempt => "portion_product_exempt",
            TaxAmountTaxabilityReason::PortionReducedRated => "portion_reduced_rated",
            TaxAmountTaxabilityReason::PortionStandardRated => "portion_standard_rated",
            TaxAmountTaxabilityReason::ProductExempt => "product_exempt",
            TaxAmountTaxabilityReason::ProductExemptHoliday => "product_exempt_holiday",
            TaxAmountTaxabilityReason::ProportionallyRated => "proportionally_rated",
            TaxAmountTaxabilityReason::ReducedRated => "reduced_rated",
            TaxAmountTaxabilityReason::ReverseCharge => "reverse_charge",
            TaxAmountTaxabilityReason::StandardRated => "standard_rated",
            TaxAmountTaxabilityReason::TaxableBasisReduced => "taxable_basis_reduced",
            TaxAmountTaxabilityReason::ZeroRated => "zero_rated",
        }
    }
}

impl AsRef<str> for TaxAmountTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxAmountTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxAmountTaxabilityReason {
    fn default() -> Self {
        Self::CustomerExempt
    }
}
