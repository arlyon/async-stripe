use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListCreditNoteBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListCreditNoteBuilder<'a> {
    fn new() -> Self {
        Self {
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            invoice: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Returns a list of credit notes.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListCreditNote<'a> {
    inner: ListCreditNoteBuilder<'a>,
}
impl<'a> ListCreditNote<'a> {
    /// Construct a new `ListCreditNote`.
    pub fn new() -> Self {
        Self { inner: ListCreditNoteBuilder::new() }
    }
    /// Only return credit notes that were created during the given date interval.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// Only return credit notes for the customer specified by this customer ID.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Only return credit notes for the invoice specified by this invoice ID.
    pub fn invoice(mut self, invoice: &'a str) -> Self {
        self.inner.invoice = Some(invoice);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListCreditNote<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListCreditNote<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::CreditNote>> {
        stripe_client_core::ListPaginator::new_list("/credit_notes", self.inner)
    }
}

impl StripeRequest for ListCreditNote<'_> {
    type Output = stripe_types::List<stripe_shared::CreditNote>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/credit_notes").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveCreditNoteBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCreditNoteBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the credit note object with the given identifier.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveCreditNote<'a> {
    inner: RetrieveCreditNoteBuilder<'a>,
    id: &'a stripe_shared::CreditNoteId,
}
impl<'a> RetrieveCreditNote<'a> {
    /// Construct a new `RetrieveCreditNote`.
    pub fn new(id: &'a stripe_shared::CreditNoteId) -> Self {
        Self { id, inner: RetrieveCreditNoteBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveCreditNote<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveCreditNote<'_> {
    type Output = stripe_shared::CreditNote;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/credit_notes/{id}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct PreviewCreditNoteBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    invoice: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<&'a [PreviewCreditNoteLines<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    out_of_band_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<stripe_shared::CreditNoteReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<CreditNoteShippingCost<'a>>,
}
impl<'a> PreviewCreditNoteBuilder<'a> {
    fn new(invoice: &'a str) -> Self {
        Self {
            amount: None,
            credit_amount: None,
            effective_at: None,
            expand: None,
            invoice,
            lines: None,
            memo: None,
            metadata: None,
            out_of_band_amount: None,
            reason: None,
            refund: None,
            refund_amount: None,
            shipping_cost: None,
        }
    }
}
/// Line items that make up the credit note.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PreviewCreditNoteLines<'a> {
    /// The line item amount to credit. Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The description of the credit note line item. Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The invoice line item to credit. Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<&'a str>,
    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of up to 10 tax amounts for the credit note line item. Cannot be mixed with `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<&'a [TaxAmountWithTaxRateParam<'a>]>,
    /// The tax rates which apply to the credit note line item.
    /// Only valid when the `type` is `custom_line_item` and cannot be mixed with `tax_amounts`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
    /// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`
    #[serde(rename = "type")]
    pub type_: PreviewCreditNoteLinesType,
    /// The integer unit amount in cents (or local equivalent) of the credit note line item.
    /// This `unit_amount` will be multiplied by the quantity to get the full amount to credit for this line item.
    /// Only valid when `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> PreviewCreditNoteLines<'a> {
    pub fn new(type_: PreviewCreditNoteLinesType) -> Self {
        Self {
            amount: None,
            description: None,
            invoice_line_item: None,
            quantity: None,
            tax_amounts: None,
            tax_rates: None,
            type_,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PreviewCreditNoteLinesType::*;
        match s {
            "custom_line_item" => Ok(CustomLineItem),
            "invoice_line_item" => Ok(InvoiceLineItem),
            _ => Err(stripe_types::StripeParseError),
        }
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PreviewCreditNoteLinesType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PreviewCreditNoteLinesType"))
    }
}
/// Get a preview of a credit note without creating it.
#[derive(Clone, Debug, serde::Serialize)]
pub struct PreviewCreditNote<'a> {
    inner: PreviewCreditNoteBuilder<'a>,
}
impl<'a> PreviewCreditNote<'a> {
    /// Construct a new `PreviewCreditNote`.
    pub fn new(invoice: &'a str) -> Self {
        Self { inner: PreviewCreditNoteBuilder::new(invoice) }
    }
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    pub fn credit_amount(mut self, credit_amount: i64) -> Self {
        self.inner.credit_amount = Some(credit_amount);
        self
    }
    /// The date when this credit note is in effect.
    /// Same as `created` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the credit note PDF.
    pub fn effective_at(mut self, effective_at: stripe_types::Timestamp) -> Self {
        self.inner.effective_at = Some(effective_at);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Line items that make up the credit note.
    pub fn lines(mut self, lines: &'a [PreviewCreditNoteLines<'a>]) -> Self {
        self.inner.lines = Some(lines);
        self
    }
    /// The credit note's memo appears on the credit note PDF.
    pub fn memo(mut self, memo: &'a str) -> Self {
        self.inner.memo = Some(memo);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount that is credited outside of Stripe.
    pub fn out_of_band_amount(mut self, out_of_band_amount: i64) -> Self {
        self.inner.out_of_band_amount = Some(out_of_band_amount);
        self
    }
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub fn reason(mut self, reason: stripe_shared::CreditNoteReason) -> Self {
        self.inner.reason = Some(reason);
        self
    }
    /// ID of an existing refund to link this credit note to.
    pub fn refund(mut self, refund: &'a str) -> Self {
        self.inner.refund = Some(refund);
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    /// If set, a refund will be created for the charge associated with the invoice.
    pub fn refund_amount(mut self, refund_amount: i64) -> Self {
        self.inner.refund_amount = Some(refund_amount);
        self
    }
    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    pub fn shipping_cost(mut self, shipping_cost: CreditNoteShippingCost<'a>) -> Self {
        self.inner.shipping_cost = Some(shipping_cost);
        self
    }
}
impl PreviewCreditNote<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for PreviewCreditNote<'_> {
    type Output = stripe_shared::CreditNote;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/credit_notes/preview").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct PreviewLinesCreditNoteBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    invoice: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<&'a [PreviewLinesCreditNoteLines<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    out_of_band_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<stripe_shared::CreditNoteReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<CreditNoteShippingCost<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> PreviewLinesCreditNoteBuilder<'a> {
    fn new(invoice: &'a str) -> Self {
        Self {
            amount: None,
            credit_amount: None,
            effective_at: None,
            ending_before: None,
            expand: None,
            invoice,
            limit: None,
            lines: None,
            memo: None,
            metadata: None,
            out_of_band_amount: None,
            reason: None,
            refund: None,
            refund_amount: None,
            shipping_cost: None,
            starting_after: None,
        }
    }
}
/// Line items that make up the credit note.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PreviewLinesCreditNoteLines<'a> {
    /// The line item amount to credit. Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The description of the credit note line item. Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The invoice line item to credit. Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<&'a str>,
    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of up to 10 tax amounts for the credit note line item. Cannot be mixed with `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<&'a [TaxAmountWithTaxRateParam<'a>]>,
    /// The tax rates which apply to the credit note line item.
    /// Only valid when the `type` is `custom_line_item` and cannot be mixed with `tax_amounts`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
    /// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`
    #[serde(rename = "type")]
    pub type_: PreviewLinesCreditNoteLinesType,
    /// The integer unit amount in cents (or local equivalent) of the credit note line item.
    /// This `unit_amount` will be multiplied by the quantity to get the full amount to credit for this line item.
    /// Only valid when `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> PreviewLinesCreditNoteLines<'a> {
    pub fn new(type_: PreviewLinesCreditNoteLinesType) -> Self {
        Self {
            amount: None,
            description: None,
            invoice_line_item: None,
            quantity: None,
            tax_amounts: None,
            tax_rates: None,
            type_,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PreviewLinesCreditNoteLinesType::*;
        match s {
            "custom_line_item" => Ok(CustomLineItem),
            "invoice_line_item" => Ok(InvoiceLineItem),
            _ => Err(stripe_types::StripeParseError),
        }
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PreviewLinesCreditNoteLinesType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PreviewLinesCreditNoteLinesType")
        })
    }
}
/// When retrieving a credit note preview, you’ll get a **lines** property containing the first handful of those items.
/// This URL you can retrieve the full (paginated) list of line items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct PreviewLinesCreditNote<'a> {
    inner: PreviewLinesCreditNoteBuilder<'a>,
}
impl<'a> PreviewLinesCreditNote<'a> {
    /// Construct a new `PreviewLinesCreditNote`.
    pub fn new(invoice: &'a str) -> Self {
        Self { inner: PreviewLinesCreditNoteBuilder::new(invoice) }
    }
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    pub fn credit_amount(mut self, credit_amount: i64) -> Self {
        self.inner.credit_amount = Some(credit_amount);
        self
    }
    /// The date when this credit note is in effect.
    /// Same as `created` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the credit note PDF.
    pub fn effective_at(mut self, effective_at: stripe_types::Timestamp) -> Self {
        self.inner.effective_at = Some(effective_at);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// Line items that make up the credit note.
    pub fn lines(mut self, lines: &'a [PreviewLinesCreditNoteLines<'a>]) -> Self {
        self.inner.lines = Some(lines);
        self
    }
    /// The credit note's memo appears on the credit note PDF.
    pub fn memo(mut self, memo: &'a str) -> Self {
        self.inner.memo = Some(memo);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount that is credited outside of Stripe.
    pub fn out_of_band_amount(mut self, out_of_band_amount: i64) -> Self {
        self.inner.out_of_band_amount = Some(out_of_band_amount);
        self
    }
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub fn reason(mut self, reason: stripe_shared::CreditNoteReason) -> Self {
        self.inner.reason = Some(reason);
        self
    }
    /// ID of an existing refund to link this credit note to.
    pub fn refund(mut self, refund: &'a str) -> Self {
        self.inner.refund = Some(refund);
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    /// If set, a refund will be created for the charge associated with the invoice.
    pub fn refund_amount(mut self, refund_amount: i64) -> Self {
        self.inner.refund_amount = Some(refund_amount);
        self
    }
    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    pub fn shipping_cost(mut self, shipping_cost: CreditNoteShippingCost<'a>) -> Self {
        self.inner.shipping_cost = Some(shipping_cost);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl PreviewLinesCreditNote<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::CreditNoteLineItem>>
    {
        stripe_client_core::ListPaginator::new_list("/credit_notes/preview/lines", self.inner)
    }
}

impl StripeRequest for PreviewLinesCreditNote<'_> {
    type Output = stripe_types::List<stripe_shared::CreditNoteLineItem>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/credit_notes/preview/lines").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateCreditNoteBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    invoice: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<&'a [CreateCreditNoteLines<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    out_of_band_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<stripe_shared::CreditNoteReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<CreditNoteShippingCost<'a>>,
}
impl<'a> CreateCreditNoteBuilder<'a> {
    fn new(invoice: &'a str) -> Self {
        Self {
            amount: None,
            credit_amount: None,
            effective_at: None,
            expand: None,
            invoice,
            lines: None,
            memo: None,
            metadata: None,
            out_of_band_amount: None,
            reason: None,
            refund: None,
            refund_amount: None,
            shipping_cost: None,
        }
    }
}
/// Line items that make up the credit note.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCreditNoteLines<'a> {
    /// The line item amount to credit. Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The description of the credit note line item. Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The invoice line item to credit. Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<&'a str>,
    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of up to 10 tax amounts for the credit note line item. Cannot be mixed with `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<&'a [TaxAmountWithTaxRateParam<'a>]>,
    /// The tax rates which apply to the credit note line item.
    /// Only valid when the `type` is `custom_line_item` and cannot be mixed with `tax_amounts`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
    /// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`
    #[serde(rename = "type")]
    pub type_: CreateCreditNoteLinesType,
    /// The integer unit amount in cents (or local equivalent) of the credit note line item.
    /// This `unit_amount` will be multiplied by the quantity to get the full amount to credit for this line item.
    /// Only valid when `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateCreditNoteLines<'a> {
    pub fn new(type_: CreateCreditNoteLinesType) -> Self {
        Self {
            amount: None,
            description: None,
            invoice_line_item: None,
            quantity: None,
            tax_amounts: None,
            tax_rates: None,
            type_,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCreditNoteLinesType::*;
        match s {
            "custom_line_item" => Ok(CustomLineItem),
            "invoice_line_item" => Ok(InvoiceLineItem),
            _ => Err(stripe_types::StripeParseError),
        }
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCreditNoteLinesType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateCreditNoteLinesType"))
    }
}
/// Issue a credit note to adjust the amount of a finalized invoice.
/// For a `status=open` invoice, a credit note reduces.
/// its `amount_due`.
/// For a `status=paid` invoice, a credit note does not affect its `amount_due`.
/// Instead, it can result.
/// in any combination of the following:
///
/// <ul>
/// <li>Refund: create a new refund (using `refund_amount`) or link an existing refund (using `refund`).</li>.
/// <li>Customer balance credit: credit the customer’s balance (using `credit_amount`) which will be automatically applied to their next invoice when it’s finalized.</li>.
/// <li>Outside of Stripe credit: record the amount that is or will be credited outside of Stripe (using `out_of_band_amount`).</li>.
/// </ul>
///
/// For post-payment credit notes the sum of the refund, credit and outside of Stripe amounts must equal the credit note total.
///
/// You may issue multiple credit notes for an invoice.
/// Each credit note will increment the invoice’s `pre_payment_credit_notes_amount`.
/// or `post_payment_credit_notes_amount` depending on its `status` at the time of credit note creation.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCreditNote<'a> {
    inner: CreateCreditNoteBuilder<'a>,
}
impl<'a> CreateCreditNote<'a> {
    /// Construct a new `CreateCreditNote`.
    pub fn new(invoice: &'a str) -> Self {
        Self { inner: CreateCreditNoteBuilder::new(invoice) }
    }
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    pub fn credit_amount(mut self, credit_amount: i64) -> Self {
        self.inner.credit_amount = Some(credit_amount);
        self
    }
    /// The date when this credit note is in effect.
    /// Same as `created` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the credit note PDF.
    pub fn effective_at(mut self, effective_at: stripe_types::Timestamp) -> Self {
        self.inner.effective_at = Some(effective_at);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Line items that make up the credit note.
    pub fn lines(mut self, lines: &'a [CreateCreditNoteLines<'a>]) -> Self {
        self.inner.lines = Some(lines);
        self
    }
    /// The credit note's memo appears on the credit note PDF.
    pub fn memo(mut self, memo: &'a str) -> Self {
        self.inner.memo = Some(memo);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount that is credited outside of Stripe.
    pub fn out_of_band_amount(mut self, out_of_band_amount: i64) -> Self {
        self.inner.out_of_band_amount = Some(out_of_band_amount);
        self
    }
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub fn reason(mut self, reason: stripe_shared::CreditNoteReason) -> Self {
        self.inner.reason = Some(reason);
        self
    }
    /// ID of an existing refund to link this credit note to.
    pub fn refund(mut self, refund: &'a str) -> Self {
        self.inner.refund = Some(refund);
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    /// If set, a refund will be created for the charge associated with the invoice.
    pub fn refund_amount(mut self, refund_amount: i64) -> Self {
        self.inner.refund_amount = Some(refund_amount);
        self
    }
    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    pub fn shipping_cost(mut self, shipping_cost: CreditNoteShippingCost<'a>) -> Self {
        self.inner.shipping_cost = Some(shipping_cost);
        self
    }
}
impl CreateCreditNote<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateCreditNote<'_> {
    type Output = stripe_shared::CreditNote;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/credit_notes").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateCreditNoteBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateCreditNoteBuilder<'a> {
    fn new() -> Self {
        Self { expand: None, memo: None, metadata: None }
    }
}
/// Updates an existing credit note.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCreditNote<'a> {
    inner: UpdateCreditNoteBuilder<'a>,
    id: &'a stripe_shared::CreditNoteId,
}
impl<'a> UpdateCreditNote<'a> {
    /// Construct a new `UpdateCreditNote`.
    pub fn new(id: &'a stripe_shared::CreditNoteId) -> Self {
        Self { id, inner: UpdateCreditNoteBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Credit note memo.
    pub fn memo(mut self, memo: &'a str) -> Self {
        self.inner.memo = Some(memo);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
}
impl UpdateCreditNote<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdateCreditNote<'_> {
    type Output = stripe_shared::CreditNote;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/credit_notes/{id}")).form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct VoidCreditNoteCreditNoteBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> VoidCreditNoteCreditNoteBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Marks a credit note as void.
/// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
#[derive(Clone, Debug, serde::Serialize)]
pub struct VoidCreditNoteCreditNote<'a> {
    inner: VoidCreditNoteCreditNoteBuilder<'a>,
    id: &'a stripe_shared::CreditNoteId,
}
impl<'a> VoidCreditNoteCreditNote<'a> {
    /// Construct a new `VoidCreditNoteCreditNote`.
    pub fn new(id: &'a stripe_shared::CreditNoteId) -> Self {
        Self { id, inner: VoidCreditNoteCreditNoteBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl VoidCreditNoteCreditNote<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for VoidCreditNoteCreditNote<'_> {
    type Output = stripe_shared::CreditNote;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/credit_notes/{id}/void"))
            .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct TaxAmountWithTaxRateParam<'a> {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,
    /// The id of the tax rate for this tax amount.
    /// The tax rate must have been automatically created by Stripe.
    pub tax_rate: &'a str,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: i64,
}
impl<'a> TaxAmountWithTaxRateParam<'a> {
    pub fn new(amount: i64, tax_rate: &'a str, taxable_amount: i64) -> Self {
        Self { amount, tax_rate, taxable_amount }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreditNoteShippingCost<'a> {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
}
impl<'a> CreditNoteShippingCost<'a> {
    pub fn new() -> Self {
        Self { shipping_rate: None }
    }
}
impl<'a> Default for CreditNoteShippingCost<'a> {
    fn default() -> Self {
        Self::new()
    }
}
