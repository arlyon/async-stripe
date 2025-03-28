// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{CreditNoteId, CustomerId, InvoiceId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{BillingCreditBalanceTransaction, CreditNoteLineItem, Currency, Customer, CustomerBalanceTransaction, Discount, Invoice, InvoicesResourceShippingCost, Refund};
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

    /// The pretax credit amounts (ex: discount, credit grants, etc) for all line items.
    pub pretax_credit_amounts: Vec<CreditNotesPretaxCreditAmount>,

    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub reason: Option<CreditNoteReason>,

    /// Refunds related to this credit note.
    pub refunds: Vec<CreditNoteRefund>,

    /// The details of the cost of shipping, including the ShippingRate applied to the invoice.
    pub shipping_cost: Option<InvoicesResourceShippingCost>,

    /// Status of this credit note, one of `issued` or `void`.
    ///
    /// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
    pub status: CreditNoteStatus,

    /// The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding exclusive tax and invoice level discounts.
    pub subtotal: i64,

    /// The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding all tax and invoice level discounts.
    pub subtotal_excluding_tax: Option<i64>,

    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, including tax and all discount.
    pub total: i64,

    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, excluding tax, but including discounts.
    pub total_excluding_tax: Option<i64>,

    /// The aggregate tax information for all line items.
    pub total_taxes: Option<Vec<BillingBillResourceInvoicingTaxesTax>>,

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
    pub fn update(client: &Client, id: &CreditNoteId, params: UpdateCreditNote<'_>) -> Response<CreditNote> {
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
pub struct CreditNoteRefund {

    /// Amount of the refund that applies to this credit note, in cents (or local equivalent).
    pub amount_refunded: i64,

    /// ID of the refund.
    pub refund: Expandable<Refund>,
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

    /// Type of email to send to the customer, one of `credit_note` or `none` and the default is `credit_note`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<CreditNoteEmailType>,

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

    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    ///
    /// If set, a refund will be created for the charge associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<i64>,

    /// Refunds to link to this credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Vec<CreateCreditNoteRefunds>>,

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
            email_type: Default::default(),
            expand: Default::default(),
            invoice,
            lines: Default::default(),
            memo: Default::default(),
            metadata: Default::default(),
            out_of_band_amount: Default::default(),
            reason: Default::default(),
            refund_amount: Default::default(),
            refunds: Default::default(),
            shipping_cost: Default::default(),
        }
    }
}

/// The parameters for `CreditNote::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListCreditNotes<'a> {

    /// Only return credit notes that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

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
            created: Default::default(),
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
            }}
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
    /// If invoice is set up with `automatic_tax[enabled]=true`, this amount is tax exclusive.
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
pub struct CreateCreditNoteRefunds {

    /// Amount of the refund that applies to this credit note, in cents (or local equivalent).
    ///
    /// Defaults to the entire refund amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_refunded: Option<i64>,

    /// ID of an existing refund to link this credit note to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<String>,
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

/// An enum representing the possible values of an `CreateCreditNote`'s `email_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteEmailType {
    CreditNote,
    None,
}

impl CreditNoteEmailType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreditNoteEmailType::CreditNote => "credit_note",
            CreditNoteEmailType::None => "none",
        }
    }
}

impl AsRef<str> for CreditNoteEmailType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteEmailType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreditNoteEmailType {
    fn default() -> Self {
        Self::CreditNote
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
