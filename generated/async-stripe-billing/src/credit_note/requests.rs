use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_account: Option<String>,
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListCreditNoteBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListCreditNoteBuilder").finish_non_exhaustive()
    }
}
impl ListCreditNoteBuilder {
    fn new() -> Self {
        Self {
            created: None,
            customer: None,
            customer_account: None,
            ending_before: None,
            expand: None,
            invoice: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Returns a list of credit notes.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListCreditNote {
    inner: ListCreditNoteBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListCreditNote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListCreditNote").finish_non_exhaustive()
    }
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
    /// Only return credit notes for the account representing the customer specified by this account ID.
    pub fn customer_account(mut self, customer_account: impl Into<String>) -> Self {
        self.inner.customer_account = Some(customer_account.into());
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveCreditNoteBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveCreditNoteBuilder").finish_non_exhaustive()
    }
}
impl RetrieveCreditNoteBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the credit note object with the given identifier.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveCreditNote {
    inner: RetrieveCreditNoteBuilder,
    id: stripe_shared::CreditNoteId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveCreditNote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveCreditNote").finish_non_exhaustive()
    }
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct PreviewCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_type: Option<stripe_shared::EmailType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    invoice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<Vec<CreditNoteLineItemParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    out_of_band_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<stripe_shared::Reason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refunds: Option<Vec<CreditNoteRefundParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<CreditNoteShippingCost>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PreviewCreditNoteBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PreviewCreditNoteBuilder").finish_non_exhaustive()
    }
}
impl PreviewCreditNoteBuilder {
    fn new(invoice: impl Into<String>) -> Self {
        Self {
            amount: None,
            credit_amount: None,
            effective_at: None,
            email_type: None,
            expand: None,
            invoice: invoice.into(),
            lines: None,
            memo: None,
            metadata: None,
            out_of_band_amount: None,
            reason: None,
            refund_amount: None,
            refunds: None,
            shipping_cost: None,
        }
    }
}
/// Get a preview of a credit note without creating it.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PreviewCreditNote {
    inner: PreviewCreditNoteBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PreviewCreditNote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PreviewCreditNote").finish_non_exhaustive()
    }
}
impl PreviewCreditNote {
    /// Construct a new `PreviewCreditNote`.
    pub fn new(invoice: impl Into<String>) -> Self {
        Self { inner: PreviewCreditNoteBuilder::new(invoice.into()) }
    }
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    /// One of `amount`, `lines`, or `shipping_cost` must be provided.
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
    /// Type of email to send to the customer, one of `credit_note` or `none` and the default is `credit_note`.
    pub fn email_type(mut self, email_type: impl Into<stripe_shared::EmailType>) -> Self {
        self.inner.email_type = Some(email_type.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Line items that make up the credit note.
    /// One of `amount`, `lines`, or `shipping_cost` must be provided.
    pub fn lines(mut self, lines: impl Into<Vec<CreditNoteLineItemParams>>) -> Self {
        self.inner.lines = Some(lines.into());
        self
    }
    /// The credit note's memo appears on the credit note PDF.
    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.inner.memo = Some(memo.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
    pub fn reason(mut self, reason: impl Into<stripe_shared::Reason>) -> Self {
        self.inner.reason = Some(reason.into());
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    /// If set, a refund will be created for the charge associated with the invoice.
    pub fn refund_amount(mut self, refund_amount: impl Into<i64>) -> Self {
        self.inner.refund_amount = Some(refund_amount.into());
        self
    }
    /// Refunds to link to this credit note.
    pub fn refunds(mut self, refunds: impl Into<Vec<CreditNoteRefundParams>>) -> Self {
        self.inner.refunds = Some(refunds.into());
        self
    }
    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    /// One of `amount`, `lines`, or `shipping_cost` must be provided.
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct PreviewLinesCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_type: Option<stripe_shared::EmailType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    invoice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<Vec<CreditNoteLineItemParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    out_of_band_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<stripe_shared::Reason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refunds: Option<Vec<CreditNoteRefundParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<CreditNoteShippingCost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PreviewLinesCreditNoteBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PreviewLinesCreditNoteBuilder").finish_non_exhaustive()
    }
}
impl PreviewLinesCreditNoteBuilder {
    fn new(invoice: impl Into<String>) -> Self {
        Self {
            amount: None,
            credit_amount: None,
            effective_at: None,
            email_type: None,
            ending_before: None,
            expand: None,
            invoice: invoice.into(),
            limit: None,
            lines: None,
            memo: None,
            metadata: None,
            out_of_band_amount: None,
            reason: None,
            refund_amount: None,
            refunds: None,
            shipping_cost: None,
            starting_after: None,
        }
    }
}
/// When retrieving a credit note preview, you’ll get a **lines** property containing the first handful of those items.
/// This URL you can retrieve the full (paginated) list of line items.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PreviewLinesCreditNote {
    inner: PreviewLinesCreditNoteBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PreviewLinesCreditNote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PreviewLinesCreditNote").finish_non_exhaustive()
    }
}
impl PreviewLinesCreditNote {
    /// Construct a new `PreviewLinesCreditNote`.
    pub fn new(invoice: impl Into<String>) -> Self {
        Self { inner: PreviewLinesCreditNoteBuilder::new(invoice.into()) }
    }
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    /// One of `amount`, `lines`, or `shipping_cost` must be provided.
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
    /// Type of email to send to the customer, one of `credit_note` or `none` and the default is `credit_note`.
    pub fn email_type(mut self, email_type: impl Into<stripe_shared::EmailType>) -> Self {
        self.inner.email_type = Some(email_type.into());
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
    /// One of `amount`, `lines`, or `shipping_cost` must be provided.
    pub fn lines(mut self, lines: impl Into<Vec<CreditNoteLineItemParams>>) -> Self {
        self.inner.lines = Some(lines.into());
        self
    }
    /// The credit note's memo appears on the credit note PDF.
    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.inner.memo = Some(memo.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
    pub fn reason(mut self, reason: impl Into<stripe_shared::Reason>) -> Self {
        self.inner.reason = Some(reason.into());
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    /// If set, a refund will be created for the charge associated with the invoice.
    pub fn refund_amount(mut self, refund_amount: impl Into<i64>) -> Self {
        self.inner.refund_amount = Some(refund_amount.into());
        self
    }
    /// Refunds to link to this credit note.
    pub fn refunds(mut self, refunds: impl Into<Vec<CreditNoteRefundParams>>) -> Self {
        self.inner.refunds = Some(refunds.into());
        self
    }
    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    /// One of `amount`, `lines`, or `shipping_cost` must be provided.
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_type: Option<stripe_shared::EmailType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    invoice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<Vec<CreditNoteLineItemParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    out_of_band_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<stripe_shared::Reason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refunds: Option<Vec<CreditNoteRefundParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<CreditNoteShippingCost>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCreditNoteBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCreditNoteBuilder").finish_non_exhaustive()
    }
}
impl CreateCreditNoteBuilder {
    fn new(invoice: impl Into<String>) -> Self {
        Self {
            amount: None,
            credit_amount: None,
            effective_at: None,
            email_type: None,
            expand: None,
            invoice: invoice.into(),
            lines: None,
            memo: None,
            metadata: None,
            out_of_band_amount: None,
            reason: None,
            refund_amount: None,
            refunds: None,
            shipping_cost: None,
        }
    }
}
/// Issue a credit note to adjust the amount of a finalized invoice.
/// A credit note will first reduce the invoice’s `amount_remaining` (and `amount_due`), but not below zero.
/// This amount is indicated by the credit note’s `pre_payment_amount`.
/// The excess amount is indicated by `post_payment_amount`, and it can result in any combination of the following:.
///
/// <ul>
/// <li>Refunds: create a new refund (using `refund_amount`) or link existing refunds (using `refunds`).</li>.
/// <li>Customer balance credit: credit the customer’s balance (using `credit_amount`) which will be automatically applied to their next invoice when it’s finalized.</li>.
/// <li>Outside of Stripe credit: record the amount that is or will be credited outside of Stripe (using `out_of_band_amount`).</li>.
/// </ul>
///
/// The sum of refunds, customer balance credits, and outside of Stripe credits must equal the `post_payment_amount`.
///
/// You may issue multiple credit notes for an invoice.
/// Each credit note may increment the invoice’s `pre_payment_credit_notes_amount`,.
/// `post_payment_credit_notes_amount`, or both, depending on the invoice’s `amount_remaining` at the time of credit note creation.
///
/// For invoices that also have refunds created through the [Refund API](https://stripe.com/docs/api/refunds), the credit note API subtracts those refund amounts from the maximum creditable amount.
/// This prevents the combined credit notes and refunds from exceeding the invoice amount.
/// If you use both, ensure the combined total does not exceed the invoice’s paid amount.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCreditNote {
    inner: CreateCreditNoteBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCreditNote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCreditNote").finish_non_exhaustive()
    }
}
impl CreateCreditNote {
    /// Construct a new `CreateCreditNote`.
    pub fn new(invoice: impl Into<String>) -> Self {
        Self { inner: CreateCreditNoteBuilder::new(invoice.into()) }
    }
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    /// One of `amount`, `lines`, or `shipping_cost` must be provided.
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
    /// Type of email to send to the customer, one of `credit_note` or `none` and the default is `credit_note`.
    pub fn email_type(mut self, email_type: impl Into<stripe_shared::EmailType>) -> Self {
        self.inner.email_type = Some(email_type.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Line items that make up the credit note.
    /// One of `amount`, `lines`, or `shipping_cost` must be provided.
    pub fn lines(mut self, lines: impl Into<Vec<CreditNoteLineItemParams>>) -> Self {
        self.inner.lines = Some(lines.into());
        self
    }
    /// The credit note's memo appears on the credit note PDF.
    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.inner.memo = Some(memo.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
    pub fn reason(mut self, reason: impl Into<stripe_shared::Reason>) -> Self {
        self.inner.reason = Some(reason.into());
        self
    }
    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    /// If set, a refund will be created for the charge associated with the invoice.
    pub fn refund_amount(mut self, refund_amount: impl Into<i64>) -> Self {
        self.inner.refund_amount = Some(refund_amount.into());
        self
    }
    /// Refunds to link to this credit note.
    pub fn refunds(mut self, refunds: impl Into<Vec<CreditNoteRefundParams>>) -> Self {
        self.inner.refunds = Some(refunds.into());
        self
    }
    /// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
    /// One of `amount`, `lines`, or `shipping_cost` must be provided.
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateCreditNoteBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateCreditNoteBuilder").finish_non_exhaustive()
    }
}
impl UpdateCreditNoteBuilder {
    fn new() -> Self {
        Self { expand: None, memo: None, metadata: None }
    }
}
/// Updates an existing credit note.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateCreditNote {
    inner: UpdateCreditNoteBuilder,
    id: stripe_shared::CreditNoteId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateCreditNote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateCreditNote").finish_non_exhaustive()
    }
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
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct VoidCreditNoteCreditNoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for VoidCreditNoteCreditNoteBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VoidCreditNoteCreditNoteBuilder").finish_non_exhaustive()
    }
}
impl VoidCreditNoteCreditNoteBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Marks a credit note as void.
/// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct VoidCreditNoteCreditNote {
    inner: VoidCreditNoteCreditNoteBuilder,
    id: stripe_shared::CreditNoteId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for VoidCreditNoteCreditNote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VoidCreditNoteCreditNote").finish_non_exhaustive()
    }
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

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct TaxAmountWithTaxRateParam {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,
    /// The id of the tax rate for this tax amount.
    /// The tax rate must have been automatically created by Stripe.
    pub tax_rate: String,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxAmountWithTaxRateParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxAmountWithTaxRateParam").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PaymentRecordRefundParams {
    /// The ID of the PaymentRecord with the refund to link to this credit note.
    pub payment_record: String,
    /// The PaymentRecord refund group to link to this credit note.
    /// For refunds processed off-Stripe, this will correspond to the `processor_details.custom.refund_reference` field provided when reporting the refund on the PaymentRecord.
    pub refund_group: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentRecordRefundParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentRecordRefundParams").finish_non_exhaustive()
    }
}
impl PaymentRecordRefundParams {
    pub fn new(payment_record: impl Into<String>, refund_group: impl Into<String>) -> Self {
        Self { payment_record: payment_record.into(), refund_group: refund_group.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreditNoteShippingCost {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNoteShippingCost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreditNoteShippingCost").finish_non_exhaustive()
    }
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreditNoteLineItemParams {
    /// The line item amount to credit.
    /// Only valid when `type` is `invoice_line_item`.
    /// If invoice is set up with `automatic_tax[enabled]=true`, this amount is tax exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The description of the credit note line item. Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The invoice line item to credit. Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of up to 20 tax amounts for the credit note line item.
    /// Not valid when `tax_rates` is used or if invoice is set up with `automatic_tax[enabled]=true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<Vec<TaxAmountWithTaxRateParam>>,
    /// The tax rates which apply to the credit note line item.
    /// Only valid when the `type` is `custom_line_item` and `tax_amounts` is not used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
    /// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    /// `custom_line_item` is not valid when the invoice is set up with `automatic_tax[enabled]=true`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreditNoteLineItemType,
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNoteLineItemParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreditNoteLineItemParams").finish_non_exhaustive()
    }
}
impl CreditNoteLineItemParams {
    pub fn new(type_: impl Into<stripe_shared::CreditNoteLineItemType>) -> Self {
        Self {
            amount: None,
            description: None,
            invoice_line_item: None,
            metadata: None,
            quantity: None,
            tax_amounts: None,
            tax_rates: None,
            type_: type_.into(),
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreditNoteRefundParams {
    /// Amount of the refund that applies to this credit note, in cents (or local equivalent).
    /// Defaults to the entire refund amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_refunded: Option<i64>,
    /// The PaymentRecord refund details to link to this credit note.
    /// Required when `type` is `payment_record_refund`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_record_refund: Option<PaymentRecordRefundParams>,
    /// ID of an existing refund to link this credit note to. Required when `type` is `refund`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<String>,
    /// Type of the refund, one of `refund` or `payment_record_refund`. Defaults to `refund`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<stripe_shared::CreditNoteRefundType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNoteRefundParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreditNoteRefundParams").finish_non_exhaustive()
    }
}
impl CreditNoteRefundParams {
    pub fn new() -> Self {
        Self { amount_refunded: None, payment_record_refund: None, refund: None, type_: None }
    }
}
impl Default for CreditNoteRefundParams {
    fn default() -> Self {
        Self::new()
    }
}
/// Type of email to send to the customer, one of `credit_note` or `none` and the default is `credit_note`.
pub type PreviewCreditNoteEmailType = stripe_shared::EmailType;
/// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
/// `custom_line_item` is not valid when the invoice is set up with `automatic_tax[enabled]=true`.
pub type PreviewCreditNoteLinesType = stripe_shared::CreditNoteLineItemType;
/// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
pub type PreviewCreditNoteReason = stripe_shared::Reason;
/// Type of the refund, one of `refund` or `payment_record_refund`. Defaults to `refund`.
pub type PreviewCreditNoteRefundsType = stripe_shared::CreditNoteRefundType;
/// Type of email to send to the customer, one of `credit_note` or `none` and the default is `credit_note`.
pub type PreviewLinesCreditNoteEmailType = stripe_shared::EmailType;
/// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
/// `custom_line_item` is not valid when the invoice is set up with `automatic_tax[enabled]=true`.
pub type PreviewLinesCreditNoteLinesType = stripe_shared::CreditNoteLineItemType;
/// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
pub type PreviewLinesCreditNoteReason = stripe_shared::Reason;
/// Type of the refund, one of `refund` or `payment_record_refund`. Defaults to `refund`.
pub type PreviewLinesCreditNoteRefundsType = stripe_shared::CreditNoteRefundType;
/// Type of email to send to the customer, one of `credit_note` or `none` and the default is `credit_note`.
pub type CreateCreditNoteEmailType = stripe_shared::EmailType;
/// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
/// `custom_line_item` is not valid when the invoice is set up with `automatic_tax[enabled]=true`.
pub type CreateCreditNoteLinesType = stripe_shared::CreditNoteLineItemType;
/// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
pub type CreateCreditNoteReason = stripe_shared::Reason;
/// Type of the refund, one of `refund` or `payment_record_refund`. Defaults to `refund`.
pub type CreateCreditNoteRefundsType = stripe_shared::CreditNoteRefundType;
/// A list of up to 20 tax amounts for the credit note line item.
/// Not valid when `tax_rates` is used or if invoice is set up with `automatic_tax[enabled]=true`.
pub type PreviewCreditNoteLinesTaxAmounts = TaxAmountWithTaxRateParam;
/// The PaymentRecord refund details to link to this credit note.
/// Required when `type` is `payment_record_refund`.
pub type PreviewCreditNoteRefundsPaymentRecordRefund = PaymentRecordRefundParams;
/// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
/// One of `amount`, `lines`, or `shipping_cost` must be provided.
pub type PreviewCreditNoteShippingCost = CreditNoteShippingCost;
/// A list of up to 20 tax amounts for the credit note line item.
/// Not valid when `tax_rates` is used or if invoice is set up with `automatic_tax[enabled]=true`.
pub type PreviewLinesCreditNoteLinesTaxAmounts = TaxAmountWithTaxRateParam;
/// The PaymentRecord refund details to link to this credit note.
/// Required when `type` is `payment_record_refund`.
pub type PreviewLinesCreditNoteRefundsPaymentRecordRefund = PaymentRecordRefundParams;
/// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
/// One of `amount`, `lines`, or `shipping_cost` must be provided.
pub type PreviewLinesCreditNoteShippingCost = CreditNoteShippingCost;
/// A list of up to 20 tax amounts for the credit note line item.
/// Not valid when `tax_rates` is used or if invoice is set up with `automatic_tax[enabled]=true`.
pub type CreateCreditNoteLinesTaxAmounts = TaxAmountWithTaxRateParam;
/// The PaymentRecord refund details to link to this credit note.
/// Required when `type` is `payment_record_refund`.
pub type CreateCreditNoteRefundsPaymentRecordRefund = PaymentRecordRefundParams;
/// When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
/// One of `amount`, `lines`, or `shipping_cost` must be provided.
pub type CreateCreditNoteShippingCost = CreditNoteShippingCost;
/// Line items that make up the credit note.
/// One of `amount`, `lines`, or `shipping_cost` must be provided.
pub type PreviewCreditNoteLines = CreditNoteLineItemParams;
/// Refunds to link to this credit note.
pub type PreviewCreditNoteRefunds = CreditNoteRefundParams;
/// Line items that make up the credit note.
/// One of `amount`, `lines`, or `shipping_cost` must be provided.
pub type PreviewLinesCreditNoteLines = CreditNoteLineItemParams;
/// Refunds to link to this credit note.
pub type PreviewLinesCreditNoteRefunds = CreditNoteRefundParams;
/// Line items that make up the credit note.
/// One of `amount`, `lines`, or `shipping_cost` must be provided.
pub type CreateCreditNoteLines = CreditNoteLineItemParams;
/// Refunds to link to this credit note.
pub type CreateCreditNoteRefunds = CreditNoteRefundParams;
