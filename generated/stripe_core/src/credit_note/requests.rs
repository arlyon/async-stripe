use stripe::{Client, Response};

impl stripe_core::credit_note::CreditNote {
    /// Issue a credit note to adjust the amount of a finalized invoice.
    ///
    /// For a `status=open` invoice, a credit note reduces its `amount_due`.
    /// For a `status=paid` invoice, a credit note does not affect its `amount_due`.
    /// Instead, it can result in any combination of the following:  <ul> <li>Refund: create a new refund (using `refund_amount`) or link an existing refund (using `refund`).</li> <li>Customer balance credit: credit the customer’s balance (using `credit_amount`) which will be automatically applied to their next invoice when it’s finalized.</li> <li>Outside of Stripe credit: record the amount that is or will be credited outside of Stripe (using `out_of_band_amount`).</li> </ul>  For post-payment credit notes the sum of the refund, credit and outside of Stripe amounts must equal the credit note total.  You may issue multiple credit notes for an invoice.
    /// Each credit note will increment the invoice’s `pre_payment_credit_notes_amount` or `post_payment_credit_notes_amount` depending on its `status` at the time of credit note creation.
    pub fn create(
        client: &Client,
        params: CreateCreditNote,
    ) -> Response<stripe_core::credit_note::CreditNote> {
        client.send_form("/credit_notes", params, http_types::Method::Post)
    }
    /// Get a preview of a credit note without creating it.
    pub fn preview(
        client: &Client,
        params: PreviewCreditNote,
    ) -> Response<stripe_core::credit_note::CreditNote> {
        client.get_query("/credit_notes/preview", params)
    }
    /// Retrieves the credit note object with the given identifier.
    pub fn retrieve(
        client: &Client,
        id: &str,
        params: RetrieveCreditNote,
    ) -> Response<stripe_core::credit_note::CreditNote> {
        client.get_query(&format!("/credit_notes/{id}", id = id), params)
    }
    /// Returns a list of credit notes.
    pub fn list(
        client: &Client,
        params: ListCreditNote,
    ) -> Response<stripe_types::List<stripe_core::credit_note::CreditNote>> {
        client.get_query("/credit_notes", params)
    }
    /// Updates an existing credit note.
    pub fn update(
        client: &Client,
        id: &str,
        params: UpdateCreditNote,
    ) -> Response<stripe_core::credit_note::CreditNote> {
        client.send_form(&format!("/credit_notes/{id}", id = id), params, http_types::Method::Post)
    }
    /// Marks a credit note as void.
    ///
    /// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
    pub fn void_credit_note(
        client: &Client,
        id: &str,
        params: VoidCreditNoteCreditNote,
    ) -> Response<stripe_core::credit_note::CreditNote> {
        client.send_form(
            &format!("/credit_notes/{id}/void", id = id),
            params,
            http_types::Method::Post,
        )
    }
    /// When retrieving a credit note preview, you’ll get a **lines** property containing the first handful of those items.
    ///
    /// This URL you can retrieve the full (paginated) list of line items.
    pub fn preview_lines(
        client: &Client,
        params: PreviewLinesCreditNote,
    ) -> Response<stripe_types::List<stripe_core::credit_note_line_item::CreditNoteLineItem>> {
        client.get_query("/credit_notes/preview/lines", params)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCreditNote<'a> {
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The integer amount in cents (or local equivalent) representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_amount: Option<i64>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// ID of the invoice.
    pub invoice: &'a str,
    /// Line items that make up the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<&'a [CreateCreditNoteLines<'a>]>,
    /// The credit note's memo appears on the credit note PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// The integer amount in cents (or local equivalent) representing the amount that is credited outside of Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_band_amount: Option<i64>,
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<CreateCreditNoteReason>,
    /// ID of an existing refund to link this credit note to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<&'a str>,
    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    ///
    /// If set, a refund will be created for the charge associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<i64>,
}
impl<'a> CreateCreditNote<'a> {
    pub fn new(invoice: &'a str) -> Self {
        Self {
            amount: Default::default(),
            credit_amount: Default::default(),
            expand: Default::default(),
            invoice,
            lines: Default::default(),
            memo: Default::default(),
            metadata: Default::default(),
            out_of_band_amount: Default::default(),
            reason: Default::default(),
            refund: Default::default(),
            refund_amount: Default::default(),
        }
    }
}
/// Line items that make up the credit note.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCreditNoteLines<'a> {
    /// The line item amount to credit.
    ///
    /// Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The description of the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The invoice line item to credit.
    ///
    /// Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<&'a str>,
    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
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
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateCreditNoteLines<'a> {
    pub fn new(type_: CreateCreditNoteLinesType) -> Self {
        Self {
            amount: Default::default(),
            description: Default::default(),
            invoice_line_item: Default::default(),
            quantity: Default::default(),
            tax_rates: Default::default(),
            type_,
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateCreditNoteLinesType {
    CustomLineItem,
    InvoiceLineItem,
}

impl CreateCreditNoteLinesType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomLineItem => "custom_line_item",
            Self::InvoiceLineItem => "invoice_line_item",
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
/// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateCreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl CreateCreditNoteReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::OrderChange => "order_change",
            Self::ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl AsRef<str> for CreateCreditNoteReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PreviewCreditNote<'a> {
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The integer amount in cents (or local equivalent) representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_amount: Option<i64>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// ID of the invoice.
    pub invoice: &'a str,
    /// Line items that make up the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<&'a [PreviewCreditNoteLines<'a>]>,
    /// The credit note's memo appears on the credit note PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// The integer amount in cents (or local equivalent) representing the amount that is credited outside of Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_band_amount: Option<i64>,
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<PreviewCreditNoteReason>,
    /// ID of an existing refund to link this credit note to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<&'a str>,
    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    ///
    /// If set, a refund will be created for the charge associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<i64>,
}
impl<'a> PreviewCreditNote<'a> {
    pub fn new(invoice: &'a str) -> Self {
        Self {
            amount: Default::default(),
            credit_amount: Default::default(),
            expand: Default::default(),
            invoice,
            lines: Default::default(),
            memo: Default::default(),
            metadata: Default::default(),
            out_of_band_amount: Default::default(),
            reason: Default::default(),
            refund: Default::default(),
            refund_amount: Default::default(),
        }
    }
}
/// Line items that make up the credit note.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PreviewCreditNoteLines<'a> {
    /// The line item amount to credit.
    ///
    /// Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The description of the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The invoice line item to credit.
    ///
    /// Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<&'a str>,
    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
    /// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    #[serde(rename = "type")]
    pub type_: PreviewCreditNoteLinesType,
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
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> PreviewCreditNoteLines<'a> {
    pub fn new(type_: PreviewCreditNoteLinesType) -> Self {
        Self {
            amount: Default::default(),
            description: Default::default(),
            invoice_line_item: Default::default(),
            quantity: Default::default(),
            tax_rates: Default::default(),
            type_,
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewCreditNoteLinesType {
    CustomLineItem,
    InvoiceLineItem,
}

impl PreviewCreditNoteLinesType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomLineItem => "custom_line_item",
            Self::InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl AsRef<str> for PreviewCreditNoteLinesType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PreviewCreditNoteLinesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewCreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl PreviewCreditNoteReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::OrderChange => "order_change",
            Self::ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl AsRef<str> for PreviewCreditNoteReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PreviewCreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCreditNote<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCreditNote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListCreditNote<'a> {
    /// Only return credit notes for the customer specified by this customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Only return credit notes for the invoice specified by this invoice ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<&'a str>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListCreditNote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCreditNote<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Credit note memo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
}
impl<'a> UpdateCreditNote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct VoidCreditNoteCreditNote<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> VoidCreditNoteCreditNote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct PreviewLinesCreditNote<'a> {
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The integer amount in cents (or local equivalent) representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_amount: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// ID of the invoice.
    pub invoice: &'a str,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Line items that make up the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<&'a [PreviewLinesCreditNoteLines<'a>]>,
    /// The credit note's memo appears on the credit note PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// The integer amount in cents (or local equivalent) representing the amount that is credited outside of Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_band_amount: Option<i64>,
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<PreviewLinesCreditNoteReason>,
    /// ID of an existing refund to link this credit note to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<&'a str>,
    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    ///
    /// If set, a refund will be created for the charge associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> PreviewLinesCreditNote<'a> {
    pub fn new(invoice: &'a str) -> Self {
        Self {
            amount: Default::default(),
            credit_amount: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            invoice,
            limit: Default::default(),
            lines: Default::default(),
            memo: Default::default(),
            metadata: Default::default(),
            out_of_band_amount: Default::default(),
            reason: Default::default(),
            refund: Default::default(),
            refund_amount: Default::default(),
            starting_after: Default::default(),
        }
    }
}
/// Line items that make up the credit note.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PreviewLinesCreditNoteLines<'a> {
    /// The line item amount to credit.
    ///
    /// Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The description of the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The invoice line item to credit.
    ///
    /// Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<&'a str>,
    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
    /// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    #[serde(rename = "type")]
    pub type_: PreviewLinesCreditNoteLinesType,
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
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> PreviewLinesCreditNoteLines<'a> {
    pub fn new(type_: PreviewLinesCreditNoteLinesType) -> Self {
        Self {
            amount: Default::default(),
            description: Default::default(),
            invoice_line_item: Default::default(),
            quantity: Default::default(),
            tax_rates: Default::default(),
            type_,
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewLinesCreditNoteLinesType {
    CustomLineItem,
    InvoiceLineItem,
}

impl PreviewLinesCreditNoteLinesType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomLineItem => "custom_line_item",
            Self::InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl AsRef<str> for PreviewLinesCreditNoteLinesType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PreviewLinesCreditNoteLinesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewLinesCreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl PreviewLinesCreditNoteReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::OrderChange => "order_change",
            Self::ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl AsRef<str> for PreviewLinesCreditNoteReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PreviewLinesCreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
