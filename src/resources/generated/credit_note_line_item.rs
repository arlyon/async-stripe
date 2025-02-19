// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::CreditNoteLineItemId;
use crate::params::{Expandable, Object};
use crate::resources::{Discount, TaxRate};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "CreditNoteLineItem".
///
/// For more details see <https://stripe.com/docs/api/credit_notes/line_item>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreditNoteLineItem {
    /// Unique identifier for the object.
    pub id: CreditNoteLineItemId,

    /// The integer amount in cents (or local equivalent) representing the gross amount being credited for this line item, excluding (exclusive) tax and discounts.
    pub amount: i64,

    /// The integer amount in cents (or local equivalent) representing the amount being credited for this line item, excluding all tax and discounts.
    pub amount_excluding_tax: Option<i64>,

    /// Description of the item being credited.
    pub description: Option<String>,

    /// The integer amount in cents (or local equivalent) representing the discount being credited for this line item.
    pub discount_amount: i64,

    /// The amount of discount calculated per discount for this line item.
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,

    /// ID of the invoice line item being credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The number of units of product being credited.
    pub quantity: Option<u64>,

    /// The amount of tax calculated per tax rate for this line item.
    pub tax_amounts: Vec<CreditNoteTaxAmount>,

    /// The tax rates which apply to the line item.
    pub tax_rates: Vec<TaxRate>,

    /// The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    ///
    /// When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
    #[serde(rename = "type")]
    pub type_: CreditNoteLineItemType,

    /// The cost of each unit of product being credited.
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,

    /// The amount in cents (or local equivalent) representing the unit amount being credited for this line item, excluding all tax and discounts.
    pub unit_amount_excluding_tax: Option<String>,
}

impl Object for CreditNoteLineItem {
    type Id = CreditNoteLineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "credit_note_line_item"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreditNoteTaxAmount {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,

    /// Whether this tax amount is inclusive or exclusive.
    pub inclusive: bool,

    /// The tax rate that was applied to get this tax amount.
    pub tax_rate: Expandable<TaxRate>,

    /// The reasoning behind this tax, for example, if the product is tax exempt.
    ///
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: Option<CreditNoteTaxAmountTaxabilityReason>,

    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DiscountsResourceDiscountAmount {
    /// The amount, in cents (or local equivalent), of the discount.
    pub amount: i64,

    /// The discount that was applied to get this discount amount.
    pub discount: Expandable<Discount>,
}

/// An enum representing the possible values of an `CreditNoteLineItem`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteLineItemType {
    CustomLineItem,
    InvoiceLineItem,
}

impl CreditNoteLineItemType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreditNoteLineItemType::CustomLineItem => "custom_line_item",
            CreditNoteLineItemType::InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl AsRef<str> for CreditNoteLineItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreditNoteLineItemType {
    fn default() -> Self {
        Self::CustomLineItem
    }
}

/// An enum representing the possible values of an `CreditNoteTaxAmount`'s `taxability_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteTaxAmountTaxabilityReason {
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

impl CreditNoteTaxAmountTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        match self {
            CreditNoteTaxAmountTaxabilityReason::CustomerExempt => "customer_exempt",
            CreditNoteTaxAmountTaxabilityReason::NotCollecting => "not_collecting",
            CreditNoteTaxAmountTaxabilityReason::NotSubjectToTax => "not_subject_to_tax",
            CreditNoteTaxAmountTaxabilityReason::NotSupported => "not_supported",
            CreditNoteTaxAmountTaxabilityReason::PortionProductExempt => "portion_product_exempt",
            CreditNoteTaxAmountTaxabilityReason::PortionReducedRated => "portion_reduced_rated",
            CreditNoteTaxAmountTaxabilityReason::PortionStandardRated => "portion_standard_rated",
            CreditNoteTaxAmountTaxabilityReason::ProductExempt => "product_exempt",
            CreditNoteTaxAmountTaxabilityReason::ProductExemptHoliday => "product_exempt_holiday",
            CreditNoteTaxAmountTaxabilityReason::ProportionallyRated => "proportionally_rated",
            CreditNoteTaxAmountTaxabilityReason::ReducedRated => "reduced_rated",
            CreditNoteTaxAmountTaxabilityReason::ReverseCharge => "reverse_charge",
            CreditNoteTaxAmountTaxabilityReason::StandardRated => "standard_rated",
            CreditNoteTaxAmountTaxabilityReason::TaxableBasisReduced => "taxable_basis_reduced",
            CreditNoteTaxAmountTaxabilityReason::ZeroRated => "zero_rated",
        }
    }
}

impl AsRef<str> for CreditNoteTaxAmountTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteTaxAmountTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreditNoteTaxAmountTaxabilityReason {
    fn default() -> Self {
        Self::CustomerExempt
    }
}
