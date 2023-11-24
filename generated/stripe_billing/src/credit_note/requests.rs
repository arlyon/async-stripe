#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub effective_at: Option<stripe_types::Timestamp>,
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
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
    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<CreateCreditNoteShippingCost<'a>>,
}
impl<'a> CreateCreditNote<'a> {
    pub fn new(invoice: &'a str) -> Self {
        Self {
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
    /// A list of up to 10 tax amounts for the credit note line item.
    ///
    /// Cannot be mixed with `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<&'a [CreateCreditNoteLinesTaxAmounts<'a>]>,
    /// The tax rates which apply to the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item` and cannot be mixed with `tax_amounts`.
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
            tax_amounts: Default::default(),
            tax_rates: Default::default(),
            type_,
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// A list of up to 10 tax amounts for the credit note line item.
///
/// Cannot be mixed with `tax_rates`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCreditNoteLinesTaxAmounts<'a> {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,
    /// The id of the tax rate for this tax amount.
    ///
    /// The tax rate must have been automatically created by Stripe.
    pub tax_rate: &'a str,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: i64,
}
impl<'a> CreateCreditNoteLinesTaxAmounts<'a> {
    pub fn new(amount: i64, tax_rate: &'a str, taxable_amount: i64) -> Self {
        Self { amount, tax_rate, taxable_amount }
    }
}
/// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCreditNoteLinesType {
    CustomLineItem,
    InvoiceLineItem,
}

impl CreateCreditNoteLinesType {
    pub fn as_str(self) -> &'static str {
        use CreateCreditNoteLinesType::*;
        match self {
            CustomLineItem => "custom_line_item",
            InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl std::str::FromStr for CreateCreditNoteLinesType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCreditNoteLinesType::*;
        match s {
            "custom_line_item" => Ok(CustomLineItem),
            "invoice_line_item" => Ok(InvoiceLineItem),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCreditNoteLinesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCreditNoteLinesType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl CreateCreditNoteReason {
    pub fn as_str(self) -> &'static str {
        use CreateCreditNoteReason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            OrderChange => "order_change",
            ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl std::str::FromStr for CreateCreditNoteReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCreditNoteReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "order_change" => Ok(OrderChange),
            "product_unsatisfactory" => Ok(ProductUnsatisfactory),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCreditNoteReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCreditNoteShippingCost<'a> {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
}
impl<'a> CreateCreditNoteShippingCost<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CreateCreditNote<'a> {
    /// Issue a credit note to adjust the amount of a finalized invoice.
    ///
    /// For a `status=open` invoice, a credit note reduces its `amount_due`.
    /// For a `status=paid` invoice, a credit note does not affect its `amount_due`.
    /// Instead, it can result in any combination of the following:  <ul> <li>Refund: create a new refund (using `refund_amount`) or link an existing refund (using `refund`).</li> <li>Customer balance credit: credit the customer’s balance (using `credit_amount`) which will be automatically applied to their next invoice when it’s finalized.</li> <li>Outside of Stripe credit: record the amount that is or will be credited outside of Stripe (using `out_of_band_amount`).</li> </ul>  For post-payment credit notes the sum of the refund, credit and outside of Stripe amounts must equal the credit note total.  You may issue multiple credit notes for an invoice.
    /// Each credit note will increment the invoice’s `pre_payment_credit_notes_amount` or `post_payment_credit_notes_amount` depending on its `status` at the time of credit note creation.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::CreditNote> {
        client.send_form("/credit_notes", self, http_types::Method::Post)
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
    /// The date when this credit note is in effect.
    ///
    /// Same as `created` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the credit note PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<stripe_types::Timestamp>,
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
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
    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<PreviewCreditNoteShippingCost<'a>>,
}
impl<'a> PreviewCreditNote<'a> {
    pub fn new(invoice: &'a str) -> Self {
        Self {
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
    /// A list of up to 10 tax amounts for the credit note line item.
    ///
    /// Cannot be mixed with `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<&'a [PreviewCreditNoteLinesTaxAmounts<'a>]>,
    /// The tax rates which apply to the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item` and cannot be mixed with `tax_amounts`.
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
            tax_amounts: Default::default(),
            tax_rates: Default::default(),
            type_,
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// A list of up to 10 tax amounts for the credit note line item.
///
/// Cannot be mixed with `tax_rates`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PreviewCreditNoteLinesTaxAmounts<'a> {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,
    /// The id of the tax rate for this tax amount.
    ///
    /// The tax rate must have been automatically created by Stripe.
    pub tax_rate: &'a str,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: i64,
}
impl<'a> PreviewCreditNoteLinesTaxAmounts<'a> {
    pub fn new(amount: i64, tax_rate: &'a str, taxable_amount: i64) -> Self {
        Self { amount, tax_rate, taxable_amount }
    }
}
/// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PreviewCreditNoteLinesType {
    CustomLineItem,
    InvoiceLineItem,
}

impl PreviewCreditNoteLinesType {
    pub fn as_str(self) -> &'static str {
        use PreviewCreditNoteLinesType::*;
        match self {
            CustomLineItem => "custom_line_item",
            InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl std::str::FromStr for PreviewCreditNoteLinesType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PreviewCreditNoteLinesType::*;
        match s {
            "custom_line_item" => Ok(CustomLineItem),
            "invoice_line_item" => Ok(InvoiceLineItem),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PreviewCreditNoteLinesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PreviewCreditNoteLinesType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PreviewCreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl PreviewCreditNoteReason {
    pub fn as_str(self) -> &'static str {
        use PreviewCreditNoteReason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            OrderChange => "order_change",
            ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl std::str::FromStr for PreviewCreditNoteReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PreviewCreditNoteReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "order_change" => Ok(OrderChange),
            "product_unsatisfactory" => Ok(ProductUnsatisfactory),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PreviewCreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PreviewCreditNoteReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PreviewCreditNoteShippingCost<'a> {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
}
impl<'a> PreviewCreditNoteShippingCost<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> PreviewCreditNote<'a> {
    /// Get a preview of a credit note without creating it.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::CreditNote> {
        client.get_query("/credit_notes/preview", self)
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
impl<'a> RetrieveCreditNote<'a> {
    /// Retrieves the credit note object with the given identifier.
    pub fn send(
        &self,
        client: &stripe::Client,
        id: &stripe_types::credit_note::CreditNoteId,
    ) -> stripe::Response<stripe_types::CreditNote> {
        client.get_query(&format!("/credit_notes/{id}"), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListCreditNote<'a> {
    /// Only return credit notes for the customer specified by this customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
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
    pub starting_after: Option<&'a str>,
}
impl<'a> ListCreditNote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListCreditNote<'a> {
    /// Returns a list of credit notes.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::CreditNote>> {
        client.get_query("/credit_notes", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::CreditNote> {
        stripe::ListPaginator::from_params("/credit_notes", self)
    }
}
impl<'a> stripe::PaginationParams for ListCreditNote<'a> {}
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateCreditNote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateCreditNote<'a> {
    /// Updates an existing credit note.
    pub fn send(
        &self,
        client: &stripe::Client,
        id: &stripe_types::credit_note::CreditNoteId,
    ) -> stripe::Response<stripe_types::CreditNote> {
        client.send_form(&format!("/credit_notes/{id}"), self, http_types::Method::Post)
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
impl<'a> VoidCreditNoteCreditNote<'a> {
    /// Marks a credit note as void.
    ///
    /// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
    pub fn send(
        &self,
        client: &stripe::Client,
        id: &stripe_types::credit_note::CreditNoteId,
    ) -> stripe::Response<stripe_types::CreditNote> {
        client.send_form(&format!("/credit_notes/{id}/void"), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PreviewLinesCreditNote<'a> {
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
    pub effective_at: Option<stripe_types::Timestamp>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
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
    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<PreviewLinesCreditNoteShippingCost<'a>>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> PreviewLinesCreditNote<'a> {
    pub fn new(invoice: &'a str) -> Self {
        Self {
            amount: Default::default(),
            credit_amount: Default::default(),
            effective_at: Default::default(),
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
            shipping_cost: Default::default(),
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
    /// A list of up to 10 tax amounts for the credit note line item.
    ///
    /// Cannot be mixed with `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<&'a [PreviewLinesCreditNoteLinesTaxAmounts<'a>]>,
    /// The tax rates which apply to the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item` and cannot be mixed with `tax_amounts`.
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
            tax_amounts: Default::default(),
            tax_rates: Default::default(),
            type_,
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// A list of up to 10 tax amounts for the credit note line item.
///
/// Cannot be mixed with `tax_rates`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PreviewLinesCreditNoteLinesTaxAmounts<'a> {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,
    /// The id of the tax rate for this tax amount.
    ///
    /// The tax rate must have been automatically created by Stripe.
    pub tax_rate: &'a str,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: i64,
}
impl<'a> PreviewLinesCreditNoteLinesTaxAmounts<'a> {
    pub fn new(amount: i64, tax_rate: &'a str, taxable_amount: i64) -> Self {
        Self { amount, tax_rate, taxable_amount }
    }
}
/// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PreviewLinesCreditNoteLinesType {
    CustomLineItem,
    InvoiceLineItem,
}

impl PreviewLinesCreditNoteLinesType {
    pub fn as_str(self) -> &'static str {
        use PreviewLinesCreditNoteLinesType::*;
        match self {
            CustomLineItem => "custom_line_item",
            InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl std::str::FromStr for PreviewLinesCreditNoteLinesType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PreviewLinesCreditNoteLinesType::*;
        match s {
            "custom_line_item" => Ok(CustomLineItem),
            "invoice_line_item" => Ok(InvoiceLineItem),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PreviewLinesCreditNoteLinesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PreviewLinesCreditNoteLinesType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PreviewLinesCreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl PreviewLinesCreditNoteReason {
    pub fn as_str(self) -> &'static str {
        use PreviewLinesCreditNoteReason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            OrderChange => "order_change",
            ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl std::str::FromStr for PreviewLinesCreditNoteReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PreviewLinesCreditNoteReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "order_change" => Ok(OrderChange),
            "product_unsatisfactory" => Ok(ProductUnsatisfactory),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PreviewLinesCreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PreviewLinesCreditNoteReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PreviewLinesCreditNoteShippingCost<'a> {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
}
impl<'a> PreviewLinesCreditNoteShippingCost<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> PreviewLinesCreditNote<'a> {
    /// When retrieving a credit note preview, you’ll get a **lines** property containing the first handful of those items.
    ///
    /// This URL you can retrieve the full (paginated) list of line items.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::CreditNoteLineItem>> {
        client.get_query("/credit_notes/preview/lines", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::CreditNoteLineItem> {
        stripe::ListPaginator::from_params("/credit_notes/preview/lines", self)
    }
}
impl<'a> stripe::PaginationParams for PreviewLinesCreditNote<'a> {}
