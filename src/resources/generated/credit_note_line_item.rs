// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{CreditNoteLineItemId};
use crate::params::{Expandable, Object};
use crate::resources::{BillingCreditBalanceTransaction, Discount, TaxRate};
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

    /// The pretax credit amounts (ex: discount, credit grants, etc) for this line item.
    pub pretax_credit_amounts: Vec<CreditNotesPretaxCreditAmount>,

    /// The number of units of product being credited.
    pub quantity: Option<u64>,

    /// The tax rates which apply to the line item.
    pub tax_rates: Vec<TaxRate>,

    /// The tax information of the line item.
    pub taxes: Option<Vec<BillingBillResourceInvoicingTaxesTax>>,

    /// The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    ///
    /// When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
    #[serde(rename = "type")]
    pub type_: CreditNoteLineItemType,

    /// The cost of each unit of product being credited.
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
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
pub struct CreditNotesPretaxCreditAmount {

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
    pub type_: CreditNotesPretaxCreditAmountType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DiscountsResourceDiscountAmount {

    /// The amount, in cents (or local equivalent), of the discount.
    pub amount: i64,

    /// The discount that was applied to get this discount amount.
    pub discount: Expandable<Discount>,
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

/// An enum representing the possible values of an `CreditNotesPretaxCreditAmount`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreditNotesPretaxCreditAmountType {
    CreditBalanceTransaction,
    Discount,
}

impl CreditNotesPretaxCreditAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreditNotesPretaxCreditAmountType::CreditBalanceTransaction => "credit_balance_transaction",
            CreditNotesPretaxCreditAmountType::Discount => "discount",
        }
    }
}

impl AsRef<str> for CreditNotesPretaxCreditAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNotesPretaxCreditAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreditNotesPretaxCreditAmountType {
    fn default() -> Self {
        Self::CreditBalanceTransaction
    }
}
