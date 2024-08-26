use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListCreditNoteBuilder {
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
pub struct ListCreditNote {
    inner: ListCreditNoteBuilder,
}
impl ListCreditNote {
    /// Construct a new `ListCreditNote`.
    pub fn new() -> Self {
        Self { inner: ListCreditNoteBuilder::new() }
    }
    /// Only return credit notes that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return credit notes for the customer specified by this customer ID.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Only return credit notes for the invoice specified by this invoice ID.
    pub fn invoice(mut self, invoice: impl Into<String>) -> Self {
        self.inner.invoice = Some(invoice.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListCreditNote {
    fn default() -> Self {
        Self::new()
    }
}
impl ListCreditNote {
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
        stripe_client_core::ListPaginator::new_list("/credit_notes", &self.inner)
    }
}

impl StripeRequest for ListCreditNote {
    type Output = stripe_types::List<stripe_shared::CreditNote>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/credit_notes").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveCreditNoteBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the credit note object with the given identifier.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveCreditNote {
    inner: RetrieveCreditNoteBuilder,
    id: stripe_shared::CreditNoteId,
}
impl RetrieveCreditNote {
    /// Construct a new `RetrieveCreditNote`.
    pub fn new(id: impl Into<stripe_shared::CreditNoteId>) -> Self {
        Self { id: id.into(), inner: RetrieveCreditNoteBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveCreditNote {
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

impl StripeRequest for RetrieveCreditNote {
    type Output = stripe_shared::CreditNote;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/credit_notes/{id}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct PreviewCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    invoice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<Vec<PreviewCreditNoteLines>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    out_of_band_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<stripe_shared::CreditNoteReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<CreditNoteShippingCost>,
}
impl PreviewCreditNoteBuilder {
    fn new(invoice: impl Into<String>) -> Self {
        Self {
            amount: None,
            credit_amount: None,
            effective_at: None,
            expand: None,
            invoice: invoice.into(),
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct PreviewCreditNoteLines {
    /// The line item amount to credit. Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The description of the credit note line item. Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The invoice line item to credit. Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,
    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of up to 10 tax amounts for the credit note line item. Cannot be mixed with `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<Vec<TaxAmountWithTaxRateParam>>,
    /// The tax rates which apply to the credit note line item.
    /// Only valid when the `type` is `custom_line_item` and cannot be mixed with `tax_amounts`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
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
    pub unit_amount_decimal: Option<String>,
}
impl PreviewCreditNoteLines {
    pub fn new(type_: impl Into<PreviewCreditNoteLinesType>) -> Self {
        Self {
            amount: None,
            description: None,
            invoice_line_item: None,
            quantity: None,
            tax_amounts: None,
            tax_rates: None,
            type_: type_.into(),
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
pub struct PreviewCreditNote {
    inner: PreviewCreditNoteBuilder,
}
impl PreviewCreditNote {
    /// Construct a new `PreviewCreditNote`.
    pub fn new(invoice: impl Into<String>) -> Self {
        Self { inner: PreviewCreditNoteBuilder::new(invoice.into()) }
    }
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    pub fn credit_amount(mut self, credit_amount: impl Into<i64>) -> Self {
        self.inner.credit_amount = Some(credit_amount.into());
        self
    }
    /// The date when this credit note is in effect.
    /// Same as `created` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the credit note PDF.
    pub fn effective_at(mut self, effective_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.effective_at = Some(effective_at.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Line items that make up the credit note.
    pub fn lines(mut self, lines: impl Into<Vec<PreviewCreditNoteLines>>) -> Self {
        self.inner.lines = Some(lines.into());
        self
    }
    /// The credit note's memo appears on the credit note PDF.
    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.inner.memo = Some(memo.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount that is credited outside of Stripe.
    pub fn out_of_band_amount(mut self, out_of_band_amount: impl Into<i64>) -> Self {
        self.inner.out_of_band_amount = Some(out_of_band_amount.into());
        self
    }
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub fn reason(mut self, reason: impl Into<stripe_shared::CreditNoteReason>) -> Self {
        self.inner.reason = Some(reason.into());
        self
    }
    /// ID of an existing refund to link this credit note to.
    pub fn refund(mut self, refund: impl Into<String>) -> Self {
        self.inner.refund = Some(refund.into());
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    /// If set, a refund will be created for the charge associated with the invoice.
    pub fn refund_amount(mut self, refund_amount: impl Into<i64>) -> Self {
        self.inner.refund_amount = Some(refund_amount.into());
        self
    }
    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    pub fn shipping_cost(mut self, shipping_cost: impl Into<CreditNoteShippingCost>) -> Self {
        self.inner.shipping_cost = Some(shipping_cost.into());
        self
    }
}
impl PreviewCreditNote {
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

impl StripeRequest for PreviewCreditNote {
    type Output = stripe_shared::CreditNote;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/credit_notes/preview").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct PreviewLinesCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    invoice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<Vec<PreviewLinesCreditNoteLines>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    out_of_band_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<stripe_shared::CreditNoteReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<CreditNoteShippingCost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl PreviewLinesCreditNoteBuilder {
    fn new(invoice: impl Into<String>) -> Self {
        Self {
            amount: None,
            credit_amount: None,
            effective_at: None,
            ending_before: None,
            expand: None,
            invoice: invoice.into(),
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct PreviewLinesCreditNoteLines {
    /// The line item amount to credit. Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The description of the credit note line item. Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The invoice line item to credit. Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,
    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of up to 10 tax amounts for the credit note line item. Cannot be mixed with `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<Vec<TaxAmountWithTaxRateParam>>,
    /// The tax rates which apply to the credit note line item.
    /// Only valid when the `type` is `custom_line_item` and cannot be mixed with `tax_amounts`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
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
    pub unit_amount_decimal: Option<String>,
}
impl PreviewLinesCreditNoteLines {
    pub fn new(type_: impl Into<PreviewLinesCreditNoteLinesType>) -> Self {
        Self {
            amount: None,
            description: None,
            invoice_line_item: None,
            quantity: None,
            tax_amounts: None,
            tax_rates: None,
            type_: type_.into(),
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
pub struct PreviewLinesCreditNote {
    inner: PreviewLinesCreditNoteBuilder,
}
impl PreviewLinesCreditNote {
    /// Construct a new `PreviewLinesCreditNote`.
    pub fn new(invoice: impl Into<String>) -> Self {
        Self { inner: PreviewLinesCreditNoteBuilder::new(invoice.into()) }
    }
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    pub fn credit_amount(mut self, credit_amount: impl Into<i64>) -> Self {
        self.inner.credit_amount = Some(credit_amount.into());
        self
    }
    /// The date when this credit note is in effect.
    /// Same as `created` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the credit note PDF.
    pub fn effective_at(mut self, effective_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.effective_at = Some(effective_at.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// Line items that make up the credit note.
    pub fn lines(mut self, lines: impl Into<Vec<PreviewLinesCreditNoteLines>>) -> Self {
        self.inner.lines = Some(lines.into());
        self
    }
    /// The credit note's memo appears on the credit note PDF.
    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.inner.memo = Some(memo.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount that is credited outside of Stripe.
    pub fn out_of_band_amount(mut self, out_of_band_amount: impl Into<i64>) -> Self {
        self.inner.out_of_band_amount = Some(out_of_band_amount.into());
        self
    }
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub fn reason(mut self, reason: impl Into<stripe_shared::CreditNoteReason>) -> Self {
        self.inner.reason = Some(reason.into());
        self
    }
    /// ID of an existing refund to link this credit note to.
    pub fn refund(mut self, refund: impl Into<String>) -> Self {
        self.inner.refund = Some(refund.into());
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    /// If set, a refund will be created for the charge associated with the invoice.
    pub fn refund_amount(mut self, refund_amount: impl Into<i64>) -> Self {
        self.inner.refund_amount = Some(refund_amount.into());
        self
    }
    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    pub fn shipping_cost(mut self, shipping_cost: impl Into<CreditNoteShippingCost>) -> Self {
        self.inner.shipping_cost = Some(shipping_cost.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl PreviewLinesCreditNote {
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
        stripe_client_core::ListPaginator::new_list("/credit_notes/preview/lines", &self.inner)
    }
}

impl StripeRequest for PreviewLinesCreditNote {
    type Output = stripe_types::List<stripe_shared::CreditNoteLineItem>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/credit_notes/preview/lines").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    invoice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<Vec<CreateCreditNoteLines>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    out_of_band_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<stripe_shared::CreditNoteReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<CreditNoteShippingCost>,
}
impl CreateCreditNoteBuilder {
    fn new(invoice: impl Into<String>) -> Self {
        Self {
            amount: None,
            credit_amount: None,
            effective_at: None,
            expand: None,
            invoice: invoice.into(),
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCreditNoteLines {
    /// The line item amount to credit. Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The description of the credit note line item. Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The invoice line item to credit. Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,
    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of up to 10 tax amounts for the credit note line item. Cannot be mixed with `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<Vec<TaxAmountWithTaxRateParam>>,
    /// The tax rates which apply to the credit note line item.
    /// Only valid when the `type` is `custom_line_item` and cannot be mixed with `tax_amounts`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
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
    pub unit_amount_decimal: Option<String>,
}
impl CreateCreditNoteLines {
    pub fn new(type_: impl Into<CreateCreditNoteLinesType>) -> Self {
        Self {
            amount: None,
            description: None,
            invoice_line_item: None,
            quantity: None,
            tax_amounts: None,
            tax_rates: None,
            type_: type_.into(),
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
pub struct CreateCreditNote {
    inner: CreateCreditNoteBuilder,
}
impl CreateCreditNote {
    /// Construct a new `CreateCreditNote`.
    pub fn new(invoice: impl Into<String>) -> Self {
        Self { inner: CreateCreditNoteBuilder::new(invoice.into()) }
    }
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    pub fn credit_amount(mut self, credit_amount: impl Into<i64>) -> Self {
        self.inner.credit_amount = Some(credit_amount.into());
        self
    }
    /// The date when this credit note is in effect.
    /// Same as `created` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the credit note PDF.
    pub fn effective_at(mut self, effective_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.effective_at = Some(effective_at.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Line items that make up the credit note.
    pub fn lines(mut self, lines: impl Into<Vec<CreateCreditNoteLines>>) -> Self {
        self.inner.lines = Some(lines.into());
        self
    }
    /// The credit note's memo appears on the credit note PDF.
    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.inner.memo = Some(memo.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount that is credited outside of Stripe.
    pub fn out_of_band_amount(mut self, out_of_band_amount: impl Into<i64>) -> Self {
        self.inner.out_of_band_amount = Some(out_of_band_amount.into());
        self
    }
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub fn reason(mut self, reason: impl Into<stripe_shared::CreditNoteReason>) -> Self {
        self.inner.reason = Some(reason.into());
        self
    }
    /// ID of an existing refund to link this credit note to.
    pub fn refund(mut self, refund: impl Into<String>) -> Self {
        self.inner.refund = Some(refund.into());
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    /// If set, a refund will be created for the charge associated with the invoice.
    pub fn refund_amount(mut self, refund_amount: impl Into<i64>) -> Self {
        self.inner.refund_amount = Some(refund_amount.into());
        self
    }
    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    pub fn shipping_cost(mut self, shipping_cost: impl Into<CreditNoteShippingCost>) -> Self {
        self.inner.shipping_cost = Some(shipping_cost.into());
        self
    }
}
impl CreateCreditNote {
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

impl StripeRequest for CreateCreditNote {
    type Output = stripe_shared::CreditNote;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/credit_notes").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl UpdateCreditNoteBuilder {
    fn new() -> Self {
        Self { expand: None, memo: None, metadata: None }
    }
}
/// Updates an existing credit note.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCreditNote {
    inner: UpdateCreditNoteBuilder,
    id: stripe_shared::CreditNoteId,
}
impl UpdateCreditNote {
    /// Construct a new `UpdateCreditNote`.
    pub fn new(id: impl Into<stripe_shared::CreditNoteId>) -> Self {
        Self { id: id.into(), inner: UpdateCreditNoteBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Credit note memo.
    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.inner.memo = Some(memo.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl UpdateCreditNote {
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

impl StripeRequest for UpdateCreditNote {
    type Output = stripe_shared::CreditNote;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/credit_notes/{id}")).form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct VoidCreditNoteCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl VoidCreditNoteCreditNoteBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Marks a credit note as void.
/// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
#[derive(Clone, Debug, serde::Serialize)]
pub struct VoidCreditNoteCreditNote {
    inner: VoidCreditNoteCreditNoteBuilder,
    id: stripe_shared::CreditNoteId,
}
impl VoidCreditNoteCreditNote {
    /// Construct a new `VoidCreditNoteCreditNote`.
    pub fn new(id: impl Into<stripe_shared::CreditNoteId>) -> Self {
        Self { id: id.into(), inner: VoidCreditNoteCreditNoteBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl VoidCreditNoteCreditNote {
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

impl StripeRequest for VoidCreditNoteCreditNote {
    type Output = stripe_shared::CreditNote;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/credit_notes/{id}/void"))
            .form(&self.inner)
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct TaxAmountWithTaxRateParam {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,
    /// The id of the tax rate for this tax amount.
    /// The tax rate must have been automatically created by Stripe.
    pub tax_rate: String,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: i64,
}
impl TaxAmountWithTaxRateParam {
    pub fn new(
        amount: impl Into<i64>,
        tax_rate: impl Into<String>,
        taxable_amount: impl Into<i64>,
    ) -> Self {
        Self {
            amount: amount.into(),
            tax_rate: tax_rate.into(),
            taxable_amount: taxable_amount.into(),
        }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreditNoteShippingCost {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
}
impl CreditNoteShippingCost {
    pub fn new() -> Self {
        Self { shipping_rate: None }
    }
}
impl Default for CreditNoteShippingCost {
    fn default() -> Self {
        Self::new()
    }
}
