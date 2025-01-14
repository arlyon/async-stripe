// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{CreditNoteId, CustomerId, InvoiceId, RefundId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, Timestamp};
use crate::resources::{
    CreditNoteLineItem, Currency, Customer, CustomerBalanceTransaction, Discount, Invoice,
    InvoicesShippingCost, Refund, TaxRate,
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "CreditNote".
///
/// For more details see <https://stripe.com/docs/api/credit_notes/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreditNote {
    /// Unique identifier for the object.
    pub id: CreditNoteId,

    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, including tax.
    pub amount: i64,

    /// This is the sum of all the shipping amounts.
    pub amount_shipping: i64,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// ID of the customer.
    pub customer: Expandable<Customer>,

    /// Customer balance transaction related to this credit note.
    pub customer_balance_transaction: Option<Expandable<CustomerBalanceTransaction>>,

    /// The integer amount in cents (or local equivalent) representing the total amount of discount that was credited.
    pub discount_amount: i64,

    /// The aggregate amounts calculated per discount for all line items.
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,

    /// The date when this credit note is in effect.
    ///
    /// Same as `created` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the credit note PDF.
    pub effective_at: Option<Timestamp>,

    /// ID of the invoice.
    pub invoice: Expandable<Invoice>,

    /// Line items that make up the credit note.
    pub lines: List<CreditNoteLineItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Customer-facing text that appears on the credit note PDF.
    pub memo: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// A unique number that identifies this particular credit note and appears on the PDF of the credit note and its associated invoice.
    pub number: String,

    /// Amount that was credited outside of Stripe.
    pub out_of_band_amount: Option<i64>,

    /// The link to download the PDF of the credit note.
    pub pdf: String,

    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub reason: Option<CreditNoteReason>,

    /// Refund related to this credit note.
    pub refund: Option<Expandable<Refund>>,

    /// The details of the cost of shipping, including the ShippingRate applied to the invoice.
    pub shipping_cost: Option<InvoicesShippingCost>,

    /// Status of this credit note, one of `issued` or `void`.
    ///
    /// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
    pub status: CreditNoteStatus,

    /// The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding exclusive tax and invoice level discounts.
    pub subtotal: i64,

    /// The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding all tax and invoice level discounts.
    pub subtotal_excluding_tax: Option<i64>,

    /// The aggregate amounts calculated per tax rate for all line items.
    pub tax_amounts: Vec<CreditNoteTaxAmount>,

    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, including tax and all discount.
    pub total: i64,

    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, excluding tax, but including discounts.
    pub total_excluding_tax: Option<i64>,

    /// Type of this credit note, one of `pre_payment` or `post_payment`.
    ///
    /// A `pre_payment` credit note means it was issued when the invoice was open.
    /// A `post_payment` credit note means it was issued when the invoice was paid.
    #[serde(rename = "type")]
    pub type_: CreditNoteType,

    /// The time that the credit note was voided.
    pub voided_at: Option<Timestamp>,
}

impl CreditNote {
    /// Returns a list of credit notes.
    pub fn list(client: &Client, params: &ListCreditNotes<'_>) -> Response<List<CreditNote>> {
        client.get_query("/credit_notes", params)
    }

    /// Issue a credit note to adjust the amount of a finalized invoice.
    ///
    /// For a `status=open` invoice, a credit note reduces its `amount_due`.
    /// For a `status=paid` invoice, a credit note does not affect its `amount_due`.
    /// Instead, it can result in any combination of the following:  <ul> <li>Refund: create a new refund (using `refund_amount`) or link an existing refund (using `refund`).</li> <li>Customer balance credit: credit the customer’s balance (using `credit_amount`) which will be automatically applied to their next invoice when it’s finalized.</li> <li>Outside of Stripe credit: record the amount that is or will be credited outside of Stripe (using `out_of_band_amount`).</li> </ul>  For post-payment credit notes the sum of the refund, credit and outside of Stripe amounts must equal the credit note total.  You may issue multiple credit notes for an invoice.
    /// Each credit note will increment the invoice’s `pre_payment_credit_notes_amount` or `post_payment_credit_notes_amount` depending on its `status` at the time of credit note creation.
    pub fn create(client: &Client, params: CreateCreditNote<'_>) -> Response<CreditNote> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/credit_notes", &params)
    }

    /// Retrieves the credit note object with the given identifier.
    pub fn retrieve(client: &Client, id: &CreditNoteId, expand: &[&str]) -> Response<CreditNote> {
        client.get_query(&format!("/credit_notes/{}", id), Expand { expand })
    }

    /// Updates an existing credit note.
    pub fn update(
        client: &Client,
        id: &CreditNoteId,
        params: UpdateCreditNote<'_>,
    ) -> Response<CreditNote> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/credit_notes/{}", id), &params)
    }
}

impl Object for CreditNote {
    type Id = CreditNoteId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "credit_note"
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

/// The parameters for `CreditNote::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateCreditNote<'a> {
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// The integer amount in cents (or local equivalent) representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_amount: Option<i64>,

    /// The date when this credit note is in effect.
    ///
    /// Same as `created` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the credit note PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<Timestamp>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// ID of the invoice.
    pub invoice: InvoiceId,

    /// Line items that make up the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<CreateCreditNoteLines>>,

    /// The credit note's memo appears on the credit note PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The integer amount in cents (or local equivalent) representing the amount that is credited outside of Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_band_amount: Option<i64>,

    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<CreditNoteReason>,

    /// ID of an existing refund to link this credit note to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<RefundId>,

    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    ///
    /// If set, a refund will be created for the charge associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<i64>,

    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<CreateCreditNoteShippingCost>,
}

impl<'a> CreateCreditNote<'a> {
    pub fn new(invoice: InvoiceId) -> Self {
        CreateCreditNote {
            amount: Default::default(),
            credit_amount: Default::default(),
            effective_at: Default::default(),
            expand: Default::default(),
            invoice,
            lines: Default::default(),
            memo: Default::default(),
            metadata: Default::default(),
            out_of_band_amount: Default::default(),
            reason: Default::default(),
            refund: Default::default(),
            refund_amount: Default::default(),
            shipping_cost: Default::default(),
        }
    }
}

/// The parameters for `CreditNote::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListCreditNotes<'a> {
    /// Only return credit notes for the customer specified by this customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<CreditNoteId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Only return credit notes for the invoice specified by this invoice ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<InvoiceId>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<CreditNoteId>,
}

impl<'a> ListCreditNotes<'a> {
    pub fn new() -> Self {
        ListCreditNotes {
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            invoice: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListCreditNotes<'_> {
    type O = CreditNote;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `CreditNote::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateCreditNote<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Credit note memo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl<'a> UpdateCreditNote<'a> {
    pub fn new() -> Self {
        UpdateCreditNote {
            expand: Default::default(),
            memo: Default::default(),
            metadata: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCreditNoteLines {
    /// The line item amount to credit.
    ///
    /// Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// The description of the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The invoice line item to credit.
    ///
    /// Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,

    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// A list of up to 10 tax amounts for the credit note line item.
    ///
    /// Cannot be mixed with `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<Vec<CreateCreditNoteLinesTaxAmounts>>,

    /// The tax rates which apply to the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item` and cannot be mixed with `tax_amounts`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,

    /// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    #[serde(rename = "type")]
    pub type_: CreateCreditNoteLinesType,

    /// The integer unit amount in cents (or local equivalent) of the credit note line item.
    ///
    /// This `unit_amount` will be multiplied by the quantity to get the full amount to credit for this line item.
    /// Only valid when `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCreditNoteShippingCost {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCreditNoteLinesTaxAmounts {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,

    /// The id of the tax rate for this tax amount.
    ///
    /// The tax rate must have been automatically created by Stripe.
    pub tax_rate: String,

    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: i64,
}

/// An enum representing the possible values of an `CreateCreditNoteLines`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCreditNoteLinesType {
    CustomLineItem,
    InvoiceLineItem,
}

impl CreateCreditNoteLinesType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCreditNoteLinesType::CustomLineItem => "custom_line_item",
            CreateCreditNoteLinesType::InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl AsRef<str> for CreateCreditNoteLinesType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCreditNoteLinesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCreditNoteLinesType {
    fn default() -> Self {
        Self::CustomLineItem
    }
}

/// An enum representing the possible values of an `CreditNote`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl CreditNoteReason {
    pub fn as_str(self) -> &'static str {
        match self {
            CreditNoteReason::Duplicate => "duplicate",
            CreditNoteReason::Fraudulent => "fraudulent",
            CreditNoteReason::OrderChange => "order_change",
            CreditNoteReason::ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl AsRef<str> for CreditNoteReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreditNoteReason {
    fn default() -> Self {
        Self::Duplicate
    }
}

/// An enum representing the possible values of an `CreditNote`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteStatus {
    Issued,
    Void,
}

impl CreditNoteStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            CreditNoteStatus::Issued => "issued",
            CreditNoteStatus::Void => "void",
        }
    }
}

impl AsRef<str> for CreditNoteStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreditNoteStatus {
    fn default() -> Self {
        Self::Issued
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

/// An enum representing the possible values of an `CreditNote`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteType {
    PostPayment,
    PrePayment,
}

impl CreditNoteType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreditNoteType::PostPayment => "post_payment",
            CreditNoteType::PrePayment => "pre_payment",
        }
    }
}

impl AsRef<str> for CreditNoteType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreditNoteType {
    fn default() -> Self {
        Self::PostPayment
    }
}
