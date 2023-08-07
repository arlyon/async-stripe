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
    pub lines: Option<&'a [CreditNoteLineItemParams<'a>]>,
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
    pub reason: Option<Reason>,
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
    pub shipping_cost: Option<CreditNoteShippingCost<'a>>,
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
    pub lines: Option<&'a [CreditNoteLineItemParams<'a>]>,
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
    pub reason: Option<Reason>,
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
    pub shipping_cost: Option<CreditNoteShippingCost<'a>>,
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
        client.get_query(&format!("/credit_notes/{id}", id = id), self)
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
impl<'a> stripe::PaginationParams for ListCreditNote<'a> {}
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
        client.send_form(&format!("/credit_notes/{id}", id = id), self, http_types::Method::Post)
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
        client.send_form(
            &format!("/credit_notes/{id}/void", id = id),
            self,
            http_types::Method::Post,
        )
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
    pub lines: Option<&'a [CreditNoteLineItemParams<'a>]>,
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
    pub reason: Option<Reason>,
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
    pub shipping_cost: Option<CreditNoteShippingCost<'a>>,
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
impl<'a> stripe::PaginationParams for PreviewLinesCreditNote<'a> {}
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Type {
    CustomLineItem,
    InvoiceLineItem,
}

impl Type {
    pub fn as_str(self) -> &'static str {
        use Type::*;
        match self {
            CustomLineItem => "custom_line_item",
            InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl std::str::FromStr for Type {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Type::*;
        match s {
            "custom_line_item" => Ok(CustomLineItem),
            "invoice_line_item" => Ok(InvoiceLineItem),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Reason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl Reason {
    pub fn as_str(self) -> &'static str {
        use Reason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            OrderChange => "order_change",
            ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl std::str::FromStr for Reason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Reason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "order_change" => Ok(OrderChange),
            "product_unsatisfactory" => Ok(ProductUnsatisfactory),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for Reason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for Reason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreditNoteShippingCost<'a> {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
}
impl<'a> CreditNoteShippingCost<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreditNoteLineItemParams<'a> {
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
    pub type_: Type,
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
impl<'a> CreditNoteLineItemParams<'a> {
    pub fn new(type_: Type) -> Self {
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
