use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListPaymentLinkBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListPaymentLinkBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListPaymentLinkBuilder").finish_non_exhaustive()
    }
}
impl ListPaymentLinkBuilder {
    fn new() -> Self {
        Self { active: None, ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of your payment links.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListPaymentLink {
    inner: ListPaymentLinkBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListPaymentLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListPaymentLink").finish_non_exhaustive()
    }
}
impl ListPaymentLink {
    /// Construct a new `ListPaymentLink`.
    pub fn new() -> Self {
        Self { inner: ListPaymentLinkBuilder::new() }
    }
    /// Only return payment links that are active or inactive (e.g., pass `false` to list all inactive payment links).
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListPaymentLink {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPaymentLink {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::PaymentLink>> {
        stripe_client_core::ListPaginator::new_list("/payment_links", &self.inner)
    }
}

impl StripeRequest for ListPaymentLink {
    type Output = stripe_types::List<stripe_shared::PaymentLink>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/payment_links").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrievePaymentLinkBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrievePaymentLinkBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrievePaymentLinkBuilder").finish_non_exhaustive()
    }
}
impl RetrievePaymentLinkBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve a payment link.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrievePaymentLink {
    inner: RetrievePaymentLinkBuilder,
    payment_link: stripe_shared::PaymentLinkId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrievePaymentLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrievePaymentLink").finish_non_exhaustive()
    }
}
impl RetrievePaymentLink {
    /// Construct a new `RetrievePaymentLink`.
    pub fn new(payment_link: impl Into<stripe_shared::PaymentLinkId>) -> Self {
        Self { payment_link: payment_link.into(), inner: RetrievePaymentLinkBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrievePaymentLink {
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

impl StripeRequest for RetrievePaymentLink {
    type Output = stripe_shared::PaymentLink;

    fn build(&self) -> RequestBuilder {
        let payment_link = &self.payment_link;
        RequestBuilder::new(StripeMethod::Get, format!("/payment_links/{payment_link}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListLineItemsPaymentLinkBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListLineItemsPaymentLinkBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListLineItemsPaymentLinkBuilder").finish_non_exhaustive()
    }
}
impl ListLineItemsPaymentLinkBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// When retrieving a payment link, there is an includable **line_items** property containing the first handful of those items.
/// There is also a URL where you can retrieve the full (paginated) list of line items.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListLineItemsPaymentLink {
    inner: ListLineItemsPaymentLinkBuilder,
    payment_link: stripe_shared::PaymentLinkId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListLineItemsPaymentLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListLineItemsPaymentLink").finish_non_exhaustive()
    }
}
impl ListLineItemsPaymentLink {
    /// Construct a new `ListLineItemsPaymentLink`.
    pub fn new(payment_link: impl Into<stripe_shared::PaymentLinkId>) -> Self {
        Self { payment_link: payment_link.into(), inner: ListLineItemsPaymentLinkBuilder::new() }
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl ListLineItemsPaymentLink {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::CheckoutSessionItem>>
    {
        let payment_link = &self.payment_link;

        stripe_client_core::ListPaginator::new_list(
            format!("/payment_links/{payment_link}/line_items"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListLineItemsPaymentLink {
    type Output = stripe_types::List<stripe_shared::CheckoutSessionItem>;

    fn build(&self) -> RequestBuilder {
        let payment_link = &self.payment_link;
        RequestBuilder::new(StripeMethod::Get, format!("/payment_links/{payment_link}/line_items"))
            .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreatePaymentLinkBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    after_completion: Option<AfterCompletionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_promotion_codes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<CreatePaymentLinkAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_address_collection: Option<stripe_shared::BillingAddressCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consent_collection: Option<ConsentCollectionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_fields: Option<Vec<CreatePaymentLinkCustomFields>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_text: Option<CustomTextParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_creation: Option<stripe_shared::PaymentLinkCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inactive_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_creation: Option<CreatePaymentLinkInvoiceCreation>,
    line_items: Vec<CreatePaymentLinkLineItems>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_payments: Option<CreatePaymentLinkManagedPayments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_collection: Option<NameCollectionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_items: Option<Vec<OptionalItemParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent_data: Option<CreatePaymentLinkPaymentIntentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_collection: Option<stripe_shared::PaymentLinkCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_options: Option<CreatePaymentLinkPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_types: Option<Vec<stripe_shared::PaymentLinkPaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number_collection: Option<PhoneNumberCollectionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions: Option<RestrictionsParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_address_collection: Option<ShippingAddressCollectionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Vec<ShippingOptionParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submit_type: Option<stripe_shared::SubmitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_data: Option<CreatePaymentLinkSubscriptionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_id_collection: Option<TaxIdCollectionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<CreatePaymentLinkTransferData>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkBuilder").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkBuilder {
    fn new(line_items: impl Into<Vec<CreatePaymentLinkLineItems>>) -> Self {
        Self {
            after_completion: None,
            allow_promotion_codes: None,
            application_fee_amount: None,
            application_fee_percent: None,
            automatic_tax: None,
            billing_address_collection: None,
            consent_collection: None,
            currency: None,
            custom_fields: None,
            custom_text: None,
            customer_creation: None,
            expand: None,
            inactive_message: None,
            invoice_creation: None,
            line_items: line_items.into(),
            managed_payments: None,
            metadata: None,
            name_collection: None,
            on_behalf_of: None,
            optional_items: None,
            payment_intent_data: None,
            payment_method_collection: None,
            payment_method_options: None,
            payment_method_types: None,
            phone_number_collection: None,
            restrictions: None,
            shipping_address_collection: None,
            shipping_options: None,
            submit_type: None,
            subscription_data: None,
            tax_id_collection: None,
            transfer_data: None,
        }
    }
}
/// Configuration for automatic tax collection.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkAutomaticTax {
    /// Set to `true` to [calculate tax automatically](https://docs.stripe.com/tax) using the customer's location.
    ///
    /// Enabling this parameter causes the payment link to collect any billing address information necessary for tax calculation.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreatePaymentLinkAutomaticTaxLiability>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkAutomaticTax").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkAutomaticTaxLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkAutomaticTaxLiability").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkAutomaticTaxLiability {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Collect additional information from your customer using custom fields.
/// Up to 3 fields are supported.
/// You can't set this parameter if `ui_mode` is `custom`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkCustomFields {
    /// Configuration for `type=dropdown` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<CustomFieldDropdownParam>,
    /// String of your choice that your integration can use to reconcile this field.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,
    /// The label for the field, displayed to the customer.
    pub label: CustomFieldLabelParam,
    /// Configuration for `type=numeric` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<CreatePaymentLinkCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// Configuration for `type=text` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<CreatePaymentLinkCustomFieldsText>,
    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreatePaymentLinkCustomFieldsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkCustomFields").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkCustomFields {
    pub fn new(
        key: impl Into<String>,
        label: impl Into<CustomFieldLabelParam>,
        type_: impl Into<stripe_shared::CreatePaymentLinkCustomFieldsType>,
    ) -> Self {
        Self {
            dropdown: None,
            key: key.into(),
            label: label.into(),
            numeric: None,
            optional: None,
            text: None,
            type_: type_.into(),
        }
    }
}
/// Configuration for `type=numeric` fields.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkCustomFieldsNumeric {
    /// The value that pre-fills the field on the payment page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkCustomFieldsNumeric {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkCustomFieldsNumeric").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkCustomFieldsNumeric {
    pub fn new() -> Self {
        Self { default_value: None, maximum_length: None, minimum_length: None }
    }
}
impl Default for CreatePaymentLinkCustomFieldsNumeric {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for `type=text` fields.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkCustomFieldsText {
    /// The value that pre-fills the field on the payment page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkCustomFieldsText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkCustomFieldsText").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkCustomFieldsText {
    pub fn new() -> Self {
        Self { default_value: None, maximum_length: None, minimum_length: None }
    }
}
impl Default for CreatePaymentLinkCustomFieldsText {
    fn default() -> Self {
        Self::new()
    }
}
/// Generate a post-purchase Invoice for one-time payments.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkInvoiceCreation {
    /// Whether the feature is enabled
    pub enabled: bool,
    /// Invoice PDF configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<CreatePaymentLinkInvoiceCreationInvoiceData>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkInvoiceCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkInvoiceCreation").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkInvoiceCreation {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), invoice_data: None }
    }
}
/// Invoice PDF configuration.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkInvoiceCreationInvoiceData {
    /// The account tax IDs associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// Default custom fields to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldParams>>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreatePaymentLinkInvoiceCreationInvoiceDataIssuer>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CheckoutRenderingOptionsParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkInvoiceCreationInvoiceData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkInvoiceCreationInvoiceData").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkInvoiceCreationInvoiceData {
    pub fn new() -> Self {
        Self {
            account_tax_ids: None,
            custom_fields: None,
            description: None,
            footer: None,
            issuer: None,
            metadata: None,
            rendering_options: None,
        }
    }
}
impl Default for CreatePaymentLinkInvoiceCreationInvoiceData {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkInvoiceCreationInvoiceDataIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkInvoiceCreationInvoiceDataIssuer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkInvoiceCreationInvoiceDataIssuer").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkInvoiceCreationInvoiceDataIssuer {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// The line items representing what is being sold.
/// Each line item represents an item being sold.
/// Up to 20 line items are supported.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkLineItems {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<AdjustableQuantityParams>,
    /// The ID of the [Price](https://docs.stripe.com/api/prices) or [Plan](https://docs.stripe.com/api/plans) object.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreatePaymentLinkLineItemsPriceData>,
    /// The quantity of the line item being purchased.
    pub quantity: u64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkLineItems").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkLineItems {
    pub fn new(quantity: impl Into<u64>) -> Self {
        Self { adjustable_quantity: None, price: None, price_data: None, quantity: quantity.into() }
    }
}
/// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
/// One of `price` or `price_data` is required.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkLineItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) will belong to.
    /// One of `product` or `product_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// Data used to generate a new [Product](https://docs.stripe.com/api/products) object inline.
    /// One of `product` or `product_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<CreatePaymentLinkLineItemsPriceDataProductData>,
    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreatePaymentLinkLineItemsPriceDataRecurring>,
    /// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
    /// A non-negative integer in cents (or local equivalent) representing how much to charge.
    /// One of `unit_amount` or `unit_amount_decimal` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkLineItemsPriceData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkLineItemsPriceData").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkLineItemsPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>) -> Self {
        Self {
            currency: currency.into(),
            product: None,
            product_data: None,
            recurring: None,
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Data used to generate a new [Product](https://docs.stripe.com/api/products) object inline.
/// One of `product` or `product_data` is required.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkLineItemsPriceDataProductData {
    /// The product's description, meant to be displayable to the customer.
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    pub name: String,
    /// A [tax code](https://docs.stripe.com/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkLineItemsPriceDataProductData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkLineItemsPriceDataProductData").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkLineItemsPriceDataProductData {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            description: None,
            images: None,
            metadata: None,
            name: name.into(),
            tax_code: None,
            unit_label: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkLineItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: stripe_shared::PlanInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkLineItemsPriceDataRecurring {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkLineItemsPriceDataRecurring").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkLineItemsPriceDataRecurring {
    pub fn new(interval: impl Into<stripe_shared::PlanInterval>) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Settings for Managed Payments for this Payment Link and resulting [CheckoutSessions](/api/checkout/sessions/object), [PaymentIntents](/api/payment_intents/object), [Invoices](/api/invoices/object), and [Subscriptions](/api/subscriptions/object).
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkManagedPayments {
    /// Set to `true` to enable [Managed Payments](https://docs.stripe.com/payments/managed-payments), Stripe's merchant of record solution, for this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkManagedPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkManagedPayments").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkManagedPayments {
    pub fn new() -> Self {
        Self { enabled: None }
    }
}
impl Default for CreatePaymentLinkManagedPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkPaymentIntentData {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::CaptureMethod>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will declaratively set metadata on [Payment Intents](https://docs.stripe.com/api/payment_intents) generated from this payment link.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Indicates that you intend to [make future payments](https://docs.stripe.com/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
    ///
    /// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
    ///
    /// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
    ///
    /// If a Customer has been provided or Checkout creates a new Customer,Checkout will attach the payment method to the Customer.
    ///
    /// If Checkout does not create a Customer, the payment method is not attached to a Customer.
    /// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.
    ///
    /// When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::CreateSetupIntentUsage>,
    /// Text that appears on the customer's statement as the statement descriptor for a non-card charge.
    /// This value overrides the account's default statement descriptor.
    /// For information about requirements, including the 22-character limit, see [the Statement Descriptor docs](https://docs.stripe.com/get-started/account/statement-descriptors).
    ///
    /// Setting this value for a card charge returns an error.
    /// For card charges, set the [statement_descriptor_suffix](https://docs.stripe.com/get-started/account/statement-descriptors#dynamic) instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    /// Provides information about a card charge.
    /// Concatenated to the account's [statement descriptor prefix](https://docs.stripe.com/get-started/account/statement-descriptors#static) to form the complete statement descriptor that appears on the customer's statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    /// A string that identifies the resulting payment as part of a group.
    /// See the PaymentIntents [use case for connected accounts](https://docs.stripe.com/connect/separate-charges-and-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkPaymentIntentData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkPaymentIntentData").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkPaymentIntentData {
    pub fn new() -> Self {
        Self {
            capture_method: None,
            description: None,
            metadata: None,
            setup_future_usage: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
            transfer_group: None,
        }
    }
}
impl Default for CreatePaymentLinkPaymentIntentData {
    fn default() -> Self {
        Self::new()
    }
}
/// Payment-method-specific configuration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkPaymentMethodOptions {
    /// Configuration for `card` payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreatePaymentLinkPaymentMethodOptionsCard>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkPaymentMethodOptions").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkPaymentMethodOptions {
    pub fn new() -> Self {
        Self { card: None }
    }
}
impl Default for CreatePaymentLinkPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for `card` payment methods.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkPaymentMethodOptionsCard {
    /// Restrictions to apply to the card payment method. For example, you can block specific card brands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<CreatePaymentLinkPaymentMethodOptionsCardRestrictions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkPaymentMethodOptionsCard").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self { restrictions: None }
    }
}
impl Default for CreatePaymentLinkPaymentMethodOptionsCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Restrictions to apply to the card payment method. For example, you can block specific card brands.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkPaymentMethodOptionsCardRestrictions {
    /// The card brands to block.
    /// If a customer enters or selects a card belonging to a blocked brand, they can't complete the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brands_blocked: Option<Vec<stripe_shared::BrandsBlocked>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkPaymentMethodOptionsCardRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkPaymentMethodOptionsCardRestrictions")
            .finish_non_exhaustive()
    }
}
impl CreatePaymentLinkPaymentMethodOptionsCardRestrictions {
    pub fn new() -> Self {
        Self { brands_blocked: None }
    }
}
impl Default for CreatePaymentLinkPaymentMethodOptionsCardRestrictions {
    fn default() -> Self {
        Self::new()
    }
}
/// When creating a subscription, the specified configuration data will be used.
/// There must be at least one line item with a recurring price to use `subscription_data`.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreatePaymentLinkSubscriptionDataInvoiceSettings>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will declaratively set metadata on [Subscriptions](https://docs.stripe.com/api/subscriptions) generated from this payment link.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    /// Has to be at least 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<TrialSettingsConfig>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkSubscriptionData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkSubscriptionData").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkSubscriptionData {
    pub fn new() -> Self {
        Self {
            description: None,
            invoice_settings: None,
            metadata: None,
            trial_period_days: None,
            trial_settings: None,
        }
    }
}
impl Default for CreatePaymentLinkSubscriptionData {
    fn default() -> Self {
        Self::new()
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkSubscriptionDataInvoiceSettings {
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuer>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkSubscriptionDataInvoiceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkSubscriptionDataInvoiceSettings").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkSubscriptionDataInvoiceSettings {
    pub fn new() -> Self {
        Self { issuer: None }
    }
}
impl Default for CreatePaymentLinkSubscriptionDataInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuer")
            .finish_non_exhaustive()
    }
}
impl CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuer {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLinkTransferData {
    /// The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// If specified, successful charges will be attributed to the destination
    /// account for tax reporting, and the funds from charges will be transferred
    /// to the destination account. The ID of the resulting transfer will be
    /// returned on the successful charge's `transfer` field.
    pub destination: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLinkTransferData").finish_non_exhaustive()
    }
}
impl CreatePaymentLinkTransferData {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount: None, destination: destination.into() }
    }
}
/// Creates a payment link.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentLink {
    inner: CreatePaymentLinkBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentLink").finish_non_exhaustive()
    }
}
impl CreatePaymentLink {
    /// Construct a new `CreatePaymentLink`.
    pub fn new(line_items: impl Into<Vec<CreatePaymentLinkLineItems>>) -> Self {
        Self { inner: CreatePaymentLinkBuilder::new(line_items.into()) }
    }
    /// Behavior after the purchase is complete.
    pub fn after_completion(mut self, after_completion: impl Into<AfterCompletionParams>) -> Self {
        self.inner.after_completion = Some(after_completion.into());
        self
    }
    /// Enables user redeemable promotion codes.
    pub fn allow_promotion_codes(mut self, allow_promotion_codes: impl Into<bool>) -> Self {
        self.inner.allow_promotion_codes = Some(allow_promotion_codes.into());
        self
    }
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    /// Can only be applied when there are no line items with recurring prices.
    pub fn application_fee_amount(mut self, application_fee_amount: impl Into<i64>) -> Self {
        self.inner.application_fee_amount = Some(application_fee_amount.into());
        self
    }
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    pub fn application_fee_percent(mut self, application_fee_percent: impl Into<f64>) -> Self {
        self.inner.application_fee_percent = Some(application_fee_percent.into());
        self
    }
    /// Configuration for automatic tax collection.
    pub fn automatic_tax(
        mut self,
        automatic_tax: impl Into<CreatePaymentLinkAutomaticTax>,
    ) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// Configuration for collecting the customer's billing address. Defaults to `auto`.
    pub fn billing_address_collection(
        mut self,
        billing_address_collection: impl Into<stripe_shared::BillingAddressCollection>,
    ) -> Self {
        self.inner.billing_address_collection = Some(billing_address_collection.into());
        self
    }
    /// Configure fields to gather active consent from customers.
    pub fn consent_collection(
        mut self,
        consent_collection: impl Into<ConsentCollectionParams>,
    ) -> Self {
        self.inner.consent_collection = Some(consent_collection.into());
        self
    }
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies) and supported by each line item's price.
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// Collect additional information from your customer using custom fields.
    /// Up to 3 fields are supported.
    /// You can't set this parameter if `ui_mode` is `custom`.
    pub fn custom_fields(
        mut self,
        custom_fields: impl Into<Vec<CreatePaymentLinkCustomFields>>,
    ) -> Self {
        self.inner.custom_fields = Some(custom_fields.into());
        self
    }
    /// Display additional text for your customers using custom text.
    /// You can't set this parameter if `ui_mode` is `custom`.
    pub fn custom_text(mut self, custom_text: impl Into<CustomTextParam>) -> Self {
        self.inner.custom_text = Some(custom_text.into());
        self
    }
    /// Configures whether [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by this payment link create a [Customer](https://docs.stripe.com/api/customers).
    pub fn customer_creation(
        mut self,
        customer_creation: impl Into<stripe_shared::PaymentLinkCustomerCreation>,
    ) -> Self {
        self.inner.customer_creation = Some(customer_creation.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The custom message to be displayed to a customer when a payment link is no longer active.
    pub fn inactive_message(mut self, inactive_message: impl Into<String>) -> Self {
        self.inner.inactive_message = Some(inactive_message.into());
        self
    }
    /// Generate a post-purchase Invoice for one-time payments.
    pub fn invoice_creation(
        mut self,
        invoice_creation: impl Into<CreatePaymentLinkInvoiceCreation>,
    ) -> Self {
        self.inner.invoice_creation = Some(invoice_creation.into());
        self
    }
    /// Settings for Managed Payments for this Payment Link and resulting [CheckoutSessions](/api/checkout/sessions/object), [PaymentIntents](/api/payment_intents/object), [Invoices](/api/invoices/object), and [Subscriptions](/api/subscriptions/object).
    pub fn managed_payments(
        mut self,
        managed_payments: impl Into<CreatePaymentLinkManagedPayments>,
    ) -> Self {
        self.inner.managed_payments = Some(managed_payments.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// Metadata associated with this Payment Link will automatically be copied to [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by this payment link.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Controls settings applied for collecting the customer's name.
    pub fn name_collection(mut self, name_collection: impl Into<NameCollectionParams>) -> Self {
        self.inner.name_collection = Some(name_collection.into());
        self
    }
    /// The account on behalf of which to charge.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// A list of optional items the customer can add to their order at checkout.
    /// Use this parameter to pass one-time or recurring [Prices](https://docs.stripe.com/api/prices).
    /// There is a maximum of 10 optional items allowed on a payment link, and the existing limits on the number of line items allowed on a payment link apply to the combined number of line items and optional items.
    /// There is a maximum of 20 combined line items and optional items.
    pub fn optional_items(mut self, optional_items: impl Into<Vec<OptionalItemParams>>) -> Self {
        self.inner.optional_items = Some(optional_items.into());
        self
    }
    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    pub fn payment_intent_data(
        mut self,
        payment_intent_data: impl Into<CreatePaymentLinkPaymentIntentData>,
    ) -> Self {
        self.inner.payment_intent_data = Some(payment_intent_data.into());
        self
    }
    /// Specify whether Checkout should collect a payment method.
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.
    ///
    /// Can only be set in `subscription` mode. Defaults to `always`.
    ///
    /// If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://docs.stripe.com/payments/checkout/free-trials).
    pub fn payment_method_collection(
        mut self,
        payment_method_collection: impl Into<stripe_shared::PaymentLinkCustomerCreation>,
    ) -> Self {
        self.inner.payment_method_collection = Some(payment_method_collection.into());
        self
    }
    /// Payment-method-specific configuration.
    pub fn payment_method_options(
        mut self,
        payment_method_options: impl Into<CreatePaymentLinkPaymentMethodOptions>,
    ) -> Self {
        self.inner.payment_method_options = Some(payment_method_options.into());
        self
    }
    /// The list of payment method types that customers can use.
    /// If no value is passed, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods) (20+ payment methods [supported](https://docs.stripe.com/payments/payment-methods/integration-options#payment-method-product-support)).
    pub fn payment_method_types(
        mut self,
        payment_method_types: impl Into<Vec<stripe_shared::PaymentLinkPaymentMethodTypes>>,
    ) -> Self {
        self.inner.payment_method_types = Some(payment_method_types.into());
        self
    }
    /// Controls phone number collection settings during checkout.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts.
    pub fn phone_number_collection(
        mut self,
        phone_number_collection: impl Into<PhoneNumberCollectionParams>,
    ) -> Self {
        self.inner.phone_number_collection = Some(phone_number_collection.into());
        self
    }
    /// Settings that restrict the usage of a payment link.
    pub fn restrictions(mut self, restrictions: impl Into<RestrictionsParams>) -> Self {
        self.inner.restrictions = Some(restrictions.into());
        self
    }
    /// Configuration for collecting the customer's shipping address.
    pub fn shipping_address_collection(
        mut self,
        shipping_address_collection: impl Into<ShippingAddressCollectionParams>,
    ) -> Self {
        self.inner.shipping_address_collection = Some(shipping_address_collection.into());
        self
    }
    /// The shipping rate options to apply to [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by this payment link.
    pub fn shipping_options(
        mut self,
        shipping_options: impl Into<Vec<ShippingOptionParams>>,
    ) -> Self {
        self.inner.shipping_options = Some(shipping_options.into());
        self
    }
    /// Describes the type of transaction being performed in order to customize relevant text on the page, such as the submit button.
    /// Changing this value will also affect the hostname in the [url](https://docs.stripe.com/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`).
    pub fn submit_type(mut self, submit_type: impl Into<stripe_shared::SubmitType>) -> Self {
        self.inner.submit_type = Some(submit_type.into());
        self
    }
    /// When creating a subscription, the specified configuration data will be used.
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    pub fn subscription_data(
        mut self,
        subscription_data: impl Into<CreatePaymentLinkSubscriptionData>,
    ) -> Self {
        self.inner.subscription_data = Some(subscription_data.into());
        self
    }
    /// Controls tax ID collection during checkout.
    pub fn tax_id_collection(
        mut self,
        tax_id_collection: impl Into<TaxIdCollectionParams>,
    ) -> Self {
        self.inner.tax_id_collection = Some(tax_id_collection.into());
        self
    }
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    pub fn transfer_data(
        mut self,
        transfer_data: impl Into<CreatePaymentLinkTransferData>,
    ) -> Self {
        self.inner.transfer_data = Some(transfer_data.into());
        self
    }
}
impl CreatePaymentLink {
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

impl StripeRequest for CreatePaymentLink {
    type Output = stripe_shared::PaymentLink;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/payment_links").form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdatePaymentLinkBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_completion: Option<AfterCompletionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_promotion_codes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<UpdatePaymentLinkAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_address_collection: Option<stripe_shared::BillingAddressCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consent_collection: Option<ConsentCollectionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_fields: Option<Vec<UpdatePaymentLinkCustomFields>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_text: Option<CustomTextParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_creation: Option<stripe_shared::PaymentLinkCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inactive_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_creation: Option<UpdatePaymentLinkInvoiceCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_items: Option<Vec<UpdatePaymentLinkLineItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_collection: Option<NameCollectionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_items: Option<Vec<OptionalItemParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent_data: Option<UpdatePaymentLinkPaymentIntentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_collection: Option<stripe_shared::PaymentLinkCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_options: Option<UpdatePaymentLinkPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_types: Option<Vec<stripe_shared::PaymentLinkPaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number_collection: Option<PhoneNumberCollectionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions: Option<RestrictionsParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_address_collection: Option<ShippingAddressCollectionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Vec<ShippingOptionParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submit_type: Option<stripe_shared::SubmitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_data: Option<UpdatePaymentLinkSubscriptionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_id_collection: Option<TaxIdCollectionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<UpdatePaymentLinkTransferData>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkBuilder").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkBuilder {
    fn new() -> Self {
        Self {
            active: None,
            after_completion: None,
            allow_promotion_codes: None,
            application_fee_amount: None,
            application_fee_percent: None,
            automatic_tax: None,
            billing_address_collection: None,
            consent_collection: None,
            custom_fields: None,
            custom_text: None,
            customer_creation: None,
            expand: None,
            inactive_message: None,
            invoice_creation: None,
            line_items: None,
            metadata: None,
            name_collection: None,
            on_behalf_of: None,
            optional_items: None,
            payment_intent_data: None,
            payment_method_collection: None,
            payment_method_options: None,
            payment_method_types: None,
            phone_number_collection: None,
            restrictions: None,
            shipping_address_collection: None,
            shipping_options: None,
            submit_type: None,
            subscription_data: None,
            tax_id_collection: None,
            transfer_data: None,
        }
    }
}
/// Configuration for automatic tax collection.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkAutomaticTax {
    /// Set to `true` to [calculate tax automatically](https://docs.stripe.com/tax) using the customer's location.
    ///
    /// Enabling this parameter causes the payment link to collect any billing address information necessary for tax calculation.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpdatePaymentLinkAutomaticTaxLiability>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkAutomaticTax").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkAutomaticTaxLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkAutomaticTaxLiability").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkAutomaticTaxLiability {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Collect additional information from your customer using custom fields.
/// Up to 3 fields are supported.
/// You can't set this parameter if `ui_mode` is `custom`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkCustomFields {
    /// Configuration for `type=dropdown` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<CustomFieldDropdownParam>,
    /// String of your choice that your integration can use to reconcile this field.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,
    /// The label for the field, displayed to the customer.
    pub label: CustomFieldLabelParam,
    /// Configuration for `type=numeric` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<UpdatePaymentLinkCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// Configuration for `type=text` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<UpdatePaymentLinkCustomFieldsText>,
    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreatePaymentLinkCustomFieldsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkCustomFields").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkCustomFields {
    pub fn new(
        key: impl Into<String>,
        label: impl Into<CustomFieldLabelParam>,
        type_: impl Into<stripe_shared::CreatePaymentLinkCustomFieldsType>,
    ) -> Self {
        Self {
            dropdown: None,
            key: key.into(),
            label: label.into(),
            numeric: None,
            optional: None,
            text: None,
            type_: type_.into(),
        }
    }
}
/// Configuration for `type=numeric` fields.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkCustomFieldsNumeric {
    /// The value that pre-fills the field on the payment page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkCustomFieldsNumeric {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkCustomFieldsNumeric").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkCustomFieldsNumeric {
    pub fn new() -> Self {
        Self { default_value: None, maximum_length: None, minimum_length: None }
    }
}
impl Default for UpdatePaymentLinkCustomFieldsNumeric {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for `type=text` fields.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkCustomFieldsText {
    /// The value that pre-fills the field on the payment page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkCustomFieldsText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkCustomFieldsText").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkCustomFieldsText {
    pub fn new() -> Self {
        Self { default_value: None, maximum_length: None, minimum_length: None }
    }
}
impl Default for UpdatePaymentLinkCustomFieldsText {
    fn default() -> Self {
        Self::new()
    }
}
/// Generate a post-purchase Invoice for one-time payments.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkInvoiceCreation {
    /// Whether the feature is enabled
    pub enabled: bool,
    /// Invoice PDF configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<UpdatePaymentLinkInvoiceCreationInvoiceData>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkInvoiceCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkInvoiceCreation").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkInvoiceCreation {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), invoice_data: None }
    }
}
/// Invoice PDF configuration.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkInvoiceCreationInvoiceData {
    /// The account tax IDs associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// Default custom fields to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldParams>>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpdatePaymentLinkInvoiceCreationInvoiceDataIssuer>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CheckoutRenderingOptionsParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkInvoiceCreationInvoiceData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkInvoiceCreationInvoiceData").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkInvoiceCreationInvoiceData {
    pub fn new() -> Self {
        Self {
            account_tax_ids: None,
            custom_fields: None,
            description: None,
            footer: None,
            issuer: None,
            metadata: None,
            rendering_options: None,
        }
    }
}
impl Default for UpdatePaymentLinkInvoiceCreationInvoiceData {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkInvoiceCreationInvoiceDataIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkInvoiceCreationInvoiceDataIssuer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkInvoiceCreationInvoiceDataIssuer").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkInvoiceCreationInvoiceDataIssuer {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// The line items representing what is being sold.
/// Each line item represents an item being sold.
/// Up to 20 line items are supported.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkLineItems {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<AdjustableQuantityParams>,
    /// The ID of an existing line item on the payment link.
    pub id: String,
    /// The quantity of the line item being purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkLineItems").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkLineItems {
    pub fn new(id: impl Into<String>) -> Self {
        Self { adjustable_quantity: None, id: id.into(), quantity: None }
    }
}
/// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkPaymentIntentData {
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will declaratively set metadata on [Payment Intents](https://docs.stripe.com/api/payment_intents) generated from this payment link.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Indicates that you intend to [make future payments](https://docs.stripe.com/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
    ///
    /// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
    ///
    /// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
    ///
    /// If a Customer has been provided or Checkout creates a new Customer,Checkout will attach the payment method to the Customer.
    ///
    /// If Checkout does not create a Customer, the payment method is not attached to a Customer.
    /// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.
    ///
    /// When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::CreateSetupIntentUsage>,
    /// Text that appears on the customer's statement as the statement descriptor for a non-card charge.
    /// This value overrides the account's default statement descriptor.
    /// For information about requirements, including the 22-character limit, see [the Statement Descriptor docs](https://docs.stripe.com/get-started/account/statement-descriptors).
    ///
    /// Setting this value for a card charge returns an error.
    /// For card charges, set the [statement_descriptor_suffix](https://docs.stripe.com/get-started/account/statement-descriptors#dynamic) instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    /// Provides information about a card charge.
    /// Concatenated to the account's [statement descriptor prefix](https://docs.stripe.com/get-started/account/statement-descriptors#static) to form the complete statement descriptor that appears on the customer's statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    /// A string that identifies the resulting payment as part of a group.
    /// See the PaymentIntents [use case for connected accounts](https://docs.stripe.com/connect/separate-charges-and-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkPaymentIntentData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkPaymentIntentData").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkPaymentIntentData {
    pub fn new() -> Self {
        Self {
            description: None,
            metadata: None,
            setup_future_usage: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
            transfer_group: None,
        }
    }
}
impl Default for UpdatePaymentLinkPaymentIntentData {
    fn default() -> Self {
        Self::new()
    }
}
/// Payment-method-specific configuration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkPaymentMethodOptions {
    /// Configuration for `card` payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdatePaymentLinkPaymentMethodOptionsCard>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkPaymentMethodOptions").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkPaymentMethodOptions {
    pub fn new() -> Self {
        Self { card: None }
    }
}
impl Default for UpdatePaymentLinkPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for `card` payment methods.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkPaymentMethodOptionsCard {
    /// Restrictions to apply to the card payment method. For example, you can block specific card brands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<UpdatePaymentLinkPaymentMethodOptionsCardRestrictions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkPaymentMethodOptionsCard").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self { restrictions: None }
    }
}
impl Default for UpdatePaymentLinkPaymentMethodOptionsCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Restrictions to apply to the card payment method. For example, you can block specific card brands.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkPaymentMethodOptionsCardRestrictions {
    /// The card brands to block.
    /// If a customer enters or selects a card belonging to a blocked brand, they can't complete the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brands_blocked: Option<Vec<stripe_shared::BrandsBlocked>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkPaymentMethodOptionsCardRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkPaymentMethodOptionsCardRestrictions")
            .finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkPaymentMethodOptionsCardRestrictions {
    pub fn new() -> Self {
        Self { brands_blocked: None }
    }
}
impl Default for UpdatePaymentLinkPaymentMethodOptionsCardRestrictions {
    fn default() -> Self {
        Self::new()
    }
}
/// When creating a subscription, the specified configuration data will be used.
/// There must be at least one line item with a recurring price to use `subscription_data`.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkSubscriptionData {
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpdatePaymentLinkSubscriptionDataInvoiceSettings>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will declaratively set metadata on [Subscriptions](https://docs.stripe.com/api/subscriptions) generated from this payment link.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    /// Has to be at least 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<TrialSettingsConfig>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkSubscriptionData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkSubscriptionData").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkSubscriptionData {
    pub fn new() -> Self {
        Self {
            invoice_settings: None,
            metadata: None,
            trial_period_days: None,
            trial_settings: None,
        }
    }
}
impl Default for UpdatePaymentLinkSubscriptionData {
    fn default() -> Self {
        Self::new()
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkSubscriptionDataInvoiceSettings {
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuer>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkSubscriptionDataInvoiceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkSubscriptionDataInvoiceSettings").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkSubscriptionDataInvoiceSettings {
    pub fn new() -> Self {
        Self { issuer: None }
    }
}
impl Default for UpdatePaymentLinkSubscriptionDataInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuer")
            .finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuer {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLinkTransferData {
    /// The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// If specified, successful charges will be attributed to the destination
    ///  account for tax reporting, and the funds from charges will be transferred
    ///  to the destination account. The ID of the resulting transfer will be
    ///  returned on the successful charge's `transfer` field.
    pub destination: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLinkTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLinkTransferData").finish_non_exhaustive()
    }
}
impl UpdatePaymentLinkTransferData {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount: None, destination: destination.into() }
    }
}
/// Updates a payment link.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentLink {
    inner: UpdatePaymentLinkBuilder,
    payment_link: stripe_shared::PaymentLinkId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentLink").finish_non_exhaustive()
    }
}
impl UpdatePaymentLink {
    /// Construct a new `UpdatePaymentLink`.
    pub fn new(payment_link: impl Into<stripe_shared::PaymentLinkId>) -> Self {
        Self { payment_link: payment_link.into(), inner: UpdatePaymentLinkBuilder::new() }
    }
    /// Whether the payment link's `url` is active.
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Behavior after the purchase is complete.
    pub fn after_completion(mut self, after_completion: impl Into<AfterCompletionParams>) -> Self {
        self.inner.after_completion = Some(after_completion.into());
        self
    }
    /// Enables user redeemable promotion codes.
    pub fn allow_promotion_codes(mut self, allow_promotion_codes: impl Into<bool>) -> Self {
        self.inner.allow_promotion_codes = Some(allow_promotion_codes.into());
        self
    }
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    /// Can only be applied when there are no line items with recurring prices.
    pub fn application_fee_amount(mut self, application_fee_amount: impl Into<i64>) -> Self {
        self.inner.application_fee_amount = Some(application_fee_amount.into());
        self
    }
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    pub fn application_fee_percent(mut self, application_fee_percent: impl Into<f64>) -> Self {
        self.inner.application_fee_percent = Some(application_fee_percent.into());
        self
    }
    /// Configuration for automatic tax collection.
    pub fn automatic_tax(
        mut self,
        automatic_tax: impl Into<UpdatePaymentLinkAutomaticTax>,
    ) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// Configuration for collecting the customer's billing address. Defaults to `auto`.
    pub fn billing_address_collection(
        mut self,
        billing_address_collection: impl Into<stripe_shared::BillingAddressCollection>,
    ) -> Self {
        self.inner.billing_address_collection = Some(billing_address_collection.into());
        self
    }
    /// Configure fields to gather active consent from customers.
    pub fn consent_collection(
        mut self,
        consent_collection: impl Into<ConsentCollectionParams>,
    ) -> Self {
        self.inner.consent_collection = Some(consent_collection.into());
        self
    }
    /// Collect additional information from your customer using custom fields.
    /// Up to 3 fields are supported.
    /// You can't set this parameter if `ui_mode` is `custom`.
    pub fn custom_fields(
        mut self,
        custom_fields: impl Into<Vec<UpdatePaymentLinkCustomFields>>,
    ) -> Self {
        self.inner.custom_fields = Some(custom_fields.into());
        self
    }
    /// Display additional text for your customers using custom text.
    /// You can't set this parameter if `ui_mode` is `custom`.
    pub fn custom_text(mut self, custom_text: impl Into<CustomTextParam>) -> Self {
        self.inner.custom_text = Some(custom_text.into());
        self
    }
    /// Configures whether [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by this payment link create a [Customer](https://docs.stripe.com/api/customers).
    pub fn customer_creation(
        mut self,
        customer_creation: impl Into<stripe_shared::PaymentLinkCustomerCreation>,
    ) -> Self {
        self.inner.customer_creation = Some(customer_creation.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The custom message to be displayed to a customer when a payment link is no longer active.
    pub fn inactive_message(mut self, inactive_message: impl Into<String>) -> Self {
        self.inner.inactive_message = Some(inactive_message.into());
        self
    }
    /// Generate a post-purchase Invoice for one-time payments.
    pub fn invoice_creation(
        mut self,
        invoice_creation: impl Into<UpdatePaymentLinkInvoiceCreation>,
    ) -> Self {
        self.inner.invoice_creation = Some(invoice_creation.into());
        self
    }
    /// The line items representing what is being sold.
    /// Each line item represents an item being sold.
    /// Up to 20 line items are supported.
    pub fn line_items(mut self, line_items: impl Into<Vec<UpdatePaymentLinkLineItems>>) -> Self {
        self.inner.line_items = Some(line_items.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// Metadata associated with this Payment Link will automatically be copied to [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by this payment link.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Controls settings applied for collecting the customer's name.
    pub fn name_collection(mut self, name_collection: impl Into<NameCollectionParams>) -> Self {
        self.inner.name_collection = Some(name_collection.into());
        self
    }
    /// The account on behalf of which to charge.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// A list of optional items the customer can add to their order at checkout.
    /// Use this parameter to pass one-time or recurring [Prices](https://docs.stripe.com/api/prices).
    /// There is a maximum of 10 optional items allowed on a payment link, and the existing limits on the number of line items allowed on a payment link apply to the combined number of line items and optional items.
    /// There is a maximum of 20 combined line items and optional items.
    pub fn optional_items(mut self, optional_items: impl Into<Vec<OptionalItemParams>>) -> Self {
        self.inner.optional_items = Some(optional_items.into());
        self
    }
    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    pub fn payment_intent_data(
        mut self,
        payment_intent_data: impl Into<UpdatePaymentLinkPaymentIntentData>,
    ) -> Self {
        self.inner.payment_intent_data = Some(payment_intent_data.into());
        self
    }
    /// Specify whether Checkout should collect a payment method.
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.
    ///
    /// Can only be set in `subscription` mode. Defaults to `always`.
    ///
    /// If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://docs.stripe.com/payments/checkout/free-trials).
    pub fn payment_method_collection(
        mut self,
        payment_method_collection: impl Into<stripe_shared::PaymentLinkCustomerCreation>,
    ) -> Self {
        self.inner.payment_method_collection = Some(payment_method_collection.into());
        self
    }
    /// Payment-method-specific configuration.
    pub fn payment_method_options(
        mut self,
        payment_method_options: impl Into<UpdatePaymentLinkPaymentMethodOptions>,
    ) -> Self {
        self.inner.payment_method_options = Some(payment_method_options.into());
        self
    }
    /// The list of payment method types that customers can use.
    /// Pass an empty string to enable dynamic payment methods that use your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    pub fn payment_method_types(
        mut self,
        payment_method_types: impl Into<Vec<stripe_shared::PaymentLinkPaymentMethodTypes>>,
    ) -> Self {
        self.inner.payment_method_types = Some(payment_method_types.into());
        self
    }
    /// Controls phone number collection settings during checkout.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts.
    pub fn phone_number_collection(
        mut self,
        phone_number_collection: impl Into<PhoneNumberCollectionParams>,
    ) -> Self {
        self.inner.phone_number_collection = Some(phone_number_collection.into());
        self
    }
    /// Settings that restrict the usage of a payment link.
    pub fn restrictions(mut self, restrictions: impl Into<RestrictionsParams>) -> Self {
        self.inner.restrictions = Some(restrictions.into());
        self
    }
    /// Configuration for collecting the customer's shipping address.
    pub fn shipping_address_collection(
        mut self,
        shipping_address_collection: impl Into<ShippingAddressCollectionParams>,
    ) -> Self {
        self.inner.shipping_address_collection = Some(shipping_address_collection.into());
        self
    }
    /// The shipping rate options to apply to [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by this payment link.
    pub fn shipping_options(
        mut self,
        shipping_options: impl Into<Vec<ShippingOptionParams>>,
    ) -> Self {
        self.inner.shipping_options = Some(shipping_options.into());
        self
    }
    /// Describes the type of transaction being performed in order to customize relevant text on the page, such as the submit button.
    /// Changing this value will also affect the hostname in the [url](https://docs.stripe.com/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`).
    pub fn submit_type(mut self, submit_type: impl Into<stripe_shared::SubmitType>) -> Self {
        self.inner.submit_type = Some(submit_type.into());
        self
    }
    /// When creating a subscription, the specified configuration data will be used.
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    pub fn subscription_data(
        mut self,
        subscription_data: impl Into<UpdatePaymentLinkSubscriptionData>,
    ) -> Self {
        self.inner.subscription_data = Some(subscription_data.into());
        self
    }
    /// Controls tax ID collection during checkout.
    pub fn tax_id_collection(
        mut self,
        tax_id_collection: impl Into<TaxIdCollectionParams>,
    ) -> Self {
        self.inner.tax_id_collection = Some(tax_id_collection.into());
        self
    }
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    pub fn transfer_data(
        mut self,
        transfer_data: impl Into<UpdatePaymentLinkTransferData>,
    ) -> Self {
        self.inner.transfer_data = Some(transfer_data.into());
        self
    }
}
impl UpdatePaymentLink {
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

impl StripeRequest for UpdatePaymentLink {
    type Output = stripe_shared::PaymentLink;

    fn build(&self) -> RequestBuilder {
        let payment_link = &self.payment_link;
        RequestBuilder::new(StripeMethod::Post, format!("/payment_links/{payment_link}"))
            .form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct AfterCompletionConfirmationPageParams {
    /// A custom message to display to the customer after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AfterCompletionConfirmationPageParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AfterCompletionConfirmationPageParams").finish_non_exhaustive()
    }
}
impl AfterCompletionConfirmationPageParams {
    pub fn new() -> Self {
        Self { custom_message: None }
    }
}
impl Default for AfterCompletionConfirmationPageParams {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct AfterCompletionRedirectParams {
    /// The URL the customer will be redirected to after the purchase is complete.
    /// You can embed `{CHECKOUT_SESSION_ID}` into the URL to have the `id` of the completed [checkout session](https://docs.stripe.com/api/checkout/sessions/object#checkout_session_object-id) included.
    pub url: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AfterCompletionRedirectParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AfterCompletionRedirectParams").finish_non_exhaustive()
    }
}
impl AfterCompletionRedirectParams {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PaymentMethodReuseAgreementParams {
    /// Determines the position and visibility of the payment method reuse agreement in the UI.
    /// When set to `auto`, Stripe's.
    /// defaults will be used.
    /// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
    pub position: stripe_shared::Position,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodReuseAgreementParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodReuseAgreementParams").finish_non_exhaustive()
    }
}
impl PaymentMethodReuseAgreementParams {
    pub fn new(position: impl Into<stripe_shared::Position>) -> Self {
        Self { position: position.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CustomFieldOptionParam {
    /// The label for the option, displayed to the customer. Up to 100 characters.
    pub label: String,
    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomFieldOptionParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomFieldOptionParam").finish_non_exhaustive()
    }
}
impl CustomFieldOptionParam {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self { label: label.into(), value: value.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CustomFieldLabelParam {
    /// Custom text for the label, displayed to the customer. Up to 50 characters.
    pub custom: String,
    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreatePaymentLinkCustomFieldsLabelType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomFieldLabelParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomFieldLabelParam").finish_non_exhaustive()
    }
}
impl CustomFieldLabelParam {
    pub fn new(
        custom: impl Into<String>,
        type_: impl Into<stripe_shared::CreatePaymentLinkCustomFieldsLabelType>,
    ) -> Self {
        Self { custom: custom.into(), type_: type_.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CustomTextPositionParam {
    /// Text can be up to 1200 characters in length.
    pub message: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomTextPositionParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomTextPositionParam").finish_non_exhaustive()
    }
}
impl CustomTextPositionParam {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CustomFieldParams {
    /// The name of the custom field. This may be up to 40 characters.
    pub name: String,
    /// The value of the custom field. This may be up to 140 characters.
    pub value: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomFieldParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomFieldParams").finish_non_exhaustive()
    }
}
impl CustomFieldParams {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self { name: name.into(), value: value.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CheckoutRenderingOptionsParam {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<stripe_shared::AmountTaxDisplay>,
    /// ID of the invoice rendering template to use for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutRenderingOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckoutRenderingOptionsParam").finish_non_exhaustive()
    }
}
impl CheckoutRenderingOptionsParam {
    pub fn new() -> Self {
        Self { amount_tax_display: None, template: None }
    }
}
impl Default for CheckoutRenderingOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct AdjustableQuantityParams {
    /// Set to true if the quantity can be adjusted to any non-negative Integer.
    pub enabled: bool,
    /// The maximum quantity the customer can purchase.
    /// By default this value is 99.
    /// You can specify a value up to 999999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum quantity the customer can purchase.
    /// By default this value is 0.
    /// If there is only one item in the cart then that item's quantity cannot go down to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AdjustableQuantityParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AdjustableQuantityParams").finish_non_exhaustive()
    }
}
impl AdjustableQuantityParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), maximum: None, minimum: None }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct NameCollectionBusinessParams {
    /// Enable business name collection on the payment link. Defaults to `false`.
    pub enabled: bool,
    /// Whether the customer is required to provide their business name before checking out.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for NameCollectionBusinessParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("NameCollectionBusinessParams").finish_non_exhaustive()
    }
}
impl NameCollectionBusinessParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), optional: None }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct NameCollectionIndividualParams {
    /// Enable individual name collection on the payment link. Defaults to `false`.
    pub enabled: bool,
    /// Whether the customer is required to provide their full name before checking out.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for NameCollectionIndividualParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("NameCollectionIndividualParams").finish_non_exhaustive()
    }
}
impl NameCollectionIndividualParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), optional: None }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct OptionalItemAdjustableQuantityParams {
    /// Set to true if the quantity can be adjusted to any non-negative integer.
    pub enabled: bool,
    /// The maximum quantity of this item the customer can purchase. By default this value is 99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum quantity of this item the customer must purchase, if they choose to purchase it.
    /// Because this item is optional, the customer will always be able to remove it from their order, even if the `minimum` configured here is greater than 0.
    /// By default this value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OptionalItemAdjustableQuantityParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OptionalItemAdjustableQuantityParams").finish_non_exhaustive()
    }
}
impl OptionalItemAdjustableQuantityParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), maximum: None, minimum: None }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PhoneNumberCollectionParams {
    /// Set to `true` to enable phone number collection.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PhoneNumberCollectionParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhoneNumberCollectionParams").finish_non_exhaustive()
    }
}
impl PhoneNumberCollectionParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CompletedSessionsParams {
    /// The maximum number of checkout sessions that can be completed for the `completed_sessions` restriction to be met.
    pub limit: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CompletedSessionsParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CompletedSessionsParams").finish_non_exhaustive()
    }
}
impl CompletedSessionsParams {
    pub fn new(limit: impl Into<i64>) -> Self {
        Self { limit: limit.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ShippingAddressCollectionParams {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
    /// shipping locations.
    pub allowed_countries: Vec<stripe_shared::AllowedCountries>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShippingAddressCollectionParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShippingAddressCollectionParams").finish_non_exhaustive()
    }
}
impl ShippingAddressCollectionParams {
    pub fn new(allowed_countries: impl Into<Vec<stripe_shared::AllowedCountries>>) -> Self {
        Self { allowed_countries: allowed_countries.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ShippingOptionParams {
    /// The ID of the Shipping Rate to use for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShippingOptionParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShippingOptionParams").finish_non_exhaustive()
    }
}
impl ShippingOptionParams {
    pub fn new() -> Self {
        Self { shipping_rate: None }
    }
}
impl Default for ShippingOptionParams {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct EndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: stripe_shared::MissingPaymentMethod,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for EndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("EndBehavior").finish_non_exhaustive()
    }
}
impl EndBehavior {
    pub fn new(missing_payment_method: impl Into<stripe_shared::MissingPaymentMethod>) -> Self {
        Self { missing_payment_method: missing_payment_method.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct TaxIdCollectionParams {
    /// Enable tax ID collection during checkout. Defaults to `false`.
    pub enabled: bool,
    /// Describes whether a tax ID is required during checkout.
    /// Defaults to `never`.
    /// You can't set this parameter if `ui_mode` is `custom`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<stripe_shared::Required>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxIdCollectionParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxIdCollectionParams").finish_non_exhaustive()
    }
}
impl TaxIdCollectionParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), required: None }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct AfterCompletionParams {
    /// Configuration when `type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<AfterCompletionConfirmationPageParams>,
    /// Configuration when `type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<AfterCompletionRedirectParams>,
    /// The specified behavior after the purchase is complete. Either `redirect` or `hosted_confirmation`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreatePaymentLinkAfterCompletionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AfterCompletionParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AfterCompletionParams").finish_non_exhaustive()
    }
}
impl AfterCompletionParams {
    pub fn new(type_: impl Into<stripe_shared::CreatePaymentLinkAfterCompletionType>) -> Self {
        Self { hosted_confirmation: None, redirect: None, type_: type_.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConsentCollectionParams {
    /// Determines the display of payment method reuse agreement text in the UI.
    /// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_reuse_agreement: Option<PaymentMethodReuseAgreementParams>,
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    /// The Checkout.
    /// Session will determine whether to display an option to opt into promotional communication
    /// from the merchant depending on the customer's locale.
    /// Only available to US merchants and US customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<stripe_shared::Promotions>,
    /// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
    /// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<stripe_shared::TermsOfService>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConsentCollectionParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConsentCollectionParams").finish_non_exhaustive()
    }
}
impl ConsentCollectionParams {
    pub fn new() -> Self {
        Self { payment_method_reuse_agreement: None, promotions: None, terms_of_service: None }
    }
}
impl Default for ConsentCollectionParams {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CustomFieldDropdownParam {
    /// The value that pre-fills the field on the payment page.
    /// Must match a `value` in the `options` array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The options available for the customer to select. Up to 200 options allowed.
    pub options: Vec<CustomFieldOptionParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomFieldDropdownParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomFieldDropdownParam").finish_non_exhaustive()
    }
}
impl CustomFieldDropdownParam {
    pub fn new(options: impl Into<Vec<CustomFieldOptionParam>>) -> Self {
        Self { default_value: None, options: options.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CustomTextParam {
    /// Custom text that should be displayed after the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_submit: Option<CustomTextPositionParam>,
    /// Custom text that should be displayed alongside shipping address collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CustomTextPositionParam>,
    /// Custom text that should be displayed alongside the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<CustomTextPositionParam>,
    /// Custom text that should be displayed in place of the default terms of service agreement text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance: Option<CustomTextPositionParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomTextParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomTextParam").finish_non_exhaustive()
    }
}
impl CustomTextParam {
    pub fn new() -> Self {
        Self {
            after_submit: None,
            shipping_address: None,
            submit: None,
            terms_of_service_acceptance: None,
        }
    }
}
impl Default for CustomTextParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct NameCollectionParams {
    /// Controls settings applied for collecting the customer's business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<NameCollectionBusinessParams>,
    /// Controls settings applied for collecting the customer's individual name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<NameCollectionIndividualParams>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for NameCollectionParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("NameCollectionParams").finish_non_exhaustive()
    }
}
impl NameCollectionParams {
    pub fn new() -> Self {
        Self { business: None, individual: None }
    }
}
impl Default for NameCollectionParams {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct OptionalItemParams {
    /// When set, provides configuration for the customer to adjust the quantity of the line item created when a customer chooses to add this optional item to their order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<OptionalItemAdjustableQuantityParams>,
    /// The ID of the [Price](https://docs.stripe.com/api/prices) or [Plan](https://docs.stripe.com/api/plans) object.
    pub price: String,
    /// The initial quantity of the line item created when a customer chooses to add this optional item to their order.
    pub quantity: u64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OptionalItemParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OptionalItemParams").finish_non_exhaustive()
    }
}
impl OptionalItemParams {
    pub fn new(price: impl Into<String>, quantity: impl Into<u64>) -> Self {
        Self { adjustable_quantity: None, price: price.into(), quantity: quantity.into() }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RestrictionsParams {
    /// Configuration for the `completed_sessions` restriction type.
    pub completed_sessions: CompletedSessionsParams,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RestrictionsParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RestrictionsParams").finish_non_exhaustive()
    }
}
impl RestrictionsParams {
    pub fn new(completed_sessions: impl Into<CompletedSessionsParams>) -> Self {
        Self { completed_sessions: completed_sessions.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct TrialSettingsConfig {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: EndBehavior,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TrialSettingsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TrialSettingsConfig").finish_non_exhaustive()
    }
}
impl TrialSettingsConfig {
    pub fn new(end_behavior: impl Into<EndBehavior>) -> Self {
        Self { end_behavior: end_behavior.into() }
    }
}
/// The specified behavior after the purchase is complete. Either `redirect` or `hosted_confirmation`.
pub type CreatePaymentLinkAfterCompletionType = stripe_shared::CreatePaymentLinkAfterCompletionType;
/// Type of the account referenced in the request.
pub type CreatePaymentLinkAutomaticTaxLiabilityType = stripe_shared::CreateInvoiceIssuerType;
/// Configuration for collecting the customer's billing address. Defaults to `auto`.
pub type CreatePaymentLinkBillingAddressCollection = stripe_shared::BillingAddressCollection;
/// Determines the position and visibility of the payment method reuse agreement in the UI.
/// When set to `auto`, Stripe's.
/// defaults will be used.
/// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
pub type CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition =
    stripe_shared::Position;
/// If set to `auto`, enables the collection of customer consent for promotional communications.
/// The Checkout.
/// Session will determine whether to display an option to opt into promotional communication
/// from the merchant depending on the customer's locale.
/// Only available to US merchants and US customers.
pub type CreatePaymentLinkConsentCollectionPromotions = stripe_shared::Promotions;
/// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
/// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
pub type CreatePaymentLinkConsentCollectionTermsOfService = stripe_shared::TermsOfService;
/// The type of the label.
pub type CreatePaymentLinkCustomFieldsLabelType =
    stripe_shared::CreatePaymentLinkCustomFieldsLabelType;
/// The type of the field.
pub type CreatePaymentLinkCustomFieldsType = stripe_shared::CreatePaymentLinkCustomFieldsType;
/// Configures whether [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by this payment link create a [Customer](https://docs.stripe.com/api/customers).
pub type CreatePaymentLinkCustomerCreation = stripe_shared::PaymentLinkCustomerCreation;
/// Type of the account referenced in the request.
pub type CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType =
    stripe_shared::CreateInvoiceIssuerType;
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
pub type CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay =
    stripe_shared::AmountTaxDisplay;
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
pub type CreatePaymentLinkLineItemsPriceDataRecurringInterval = stripe_shared::PlanInterval;
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
pub type CreatePaymentLinkLineItemsPriceDataTaxBehavior = stripe_shared::PriceTaxBehavior;
/// Controls when the funds will be captured from the customer's account.
pub type CreatePaymentLinkPaymentIntentDataCaptureMethod = stripe_shared::CaptureMethod;
/// Indicates that you intend to [make future payments](https://docs.stripe.com/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
///
/// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
///
/// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
///
/// If a Customer has been provided or Checkout creates a new Customer,Checkout will attach the payment method to the Customer.
///
/// If Checkout does not create a Customer, the payment method is not attached to a Customer.
/// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.
///
/// When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
pub type CreatePaymentLinkPaymentIntentDataSetupFutureUsage = stripe_shared::CreateSetupIntentUsage;
/// Specify whether Checkout should collect a payment method.
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.
///
/// Can only be set in `subscription` mode. Defaults to `always`.
///
/// If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://docs.stripe.com/payments/checkout/free-trials).
pub type CreatePaymentLinkPaymentMethodCollection = stripe_shared::PaymentLinkCustomerCreation;
/// The card brands to block.
/// If a customer enters or selects a card belonging to a blocked brand, they can't complete the payment.
pub type CreatePaymentLinkPaymentMethodOptionsCardRestrictionsBrandsBlocked =
    stripe_shared::BrandsBlocked;
/// The list of payment method types that customers can use.
/// If no value is passed, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods) (20+ payment methods [supported](https://docs.stripe.com/payments/payment-methods/integration-options#payment-method-product-support)).
pub type CreatePaymentLinkPaymentMethodTypes = stripe_shared::PaymentLinkPaymentMethodTypes;
/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
/// shipping locations.
pub type CreatePaymentLinkShippingAddressCollectionAllowedCountries =
    stripe_shared::AllowedCountries;
/// Describes the type of transaction being performed in order to customize relevant text on the page, such as the submit button.
/// Changing this value will also affect the hostname in the [url](https://docs.stripe.com/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`).
pub type CreatePaymentLinkSubmitType = stripe_shared::SubmitType;
/// Type of the account referenced in the request.
pub type CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType =
    stripe_shared::CreateInvoiceIssuerType;
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
pub type CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod =
    stripe_shared::MissingPaymentMethod;
/// Describes whether a tax ID is required during checkout.
/// Defaults to `never`.
/// You can't set this parameter if `ui_mode` is `custom`.
pub type CreatePaymentLinkTaxIdCollectionRequired = stripe_shared::Required;
/// The specified behavior after the purchase is complete. Either `redirect` or `hosted_confirmation`.
pub type UpdatePaymentLinkAfterCompletionType = stripe_shared::CreatePaymentLinkAfterCompletionType;
/// Type of the account referenced in the request.
pub type UpdatePaymentLinkAutomaticTaxLiabilityType = stripe_shared::CreateInvoiceIssuerType;
/// Configuration for collecting the customer's billing address. Defaults to `auto`.
pub type UpdatePaymentLinkBillingAddressCollection = stripe_shared::BillingAddressCollection;
/// Determines the position and visibility of the payment method reuse agreement in the UI.
/// When set to `auto`, Stripe's.
/// defaults will be used.
/// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
pub type UpdatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition =
    stripe_shared::Position;
/// If set to `auto`, enables the collection of customer consent for promotional communications.
/// The Checkout.
/// Session will determine whether to display an option to opt into promotional communication
/// from the merchant depending on the customer's locale.
/// Only available to US merchants and US customers.
pub type UpdatePaymentLinkConsentCollectionPromotions = stripe_shared::Promotions;
/// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
/// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
pub type UpdatePaymentLinkConsentCollectionTermsOfService = stripe_shared::TermsOfService;
/// The type of the label.
pub type UpdatePaymentLinkCustomFieldsLabelType =
    stripe_shared::CreatePaymentLinkCustomFieldsLabelType;
/// The type of the field.
pub type UpdatePaymentLinkCustomFieldsType = stripe_shared::CreatePaymentLinkCustomFieldsType;
/// Configures whether [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by this payment link create a [Customer](https://docs.stripe.com/api/customers).
pub type UpdatePaymentLinkCustomerCreation = stripe_shared::PaymentLinkCustomerCreation;
/// Type of the account referenced in the request.
pub type UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType =
    stripe_shared::CreateInvoiceIssuerType;
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
pub type UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay =
    stripe_shared::AmountTaxDisplay;
/// Indicates that you intend to [make future payments](https://docs.stripe.com/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
///
/// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
///
/// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
///
/// If a Customer has been provided or Checkout creates a new Customer,Checkout will attach the payment method to the Customer.
///
/// If Checkout does not create a Customer, the payment method is not attached to a Customer.
/// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.
///
/// When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
pub type UpdatePaymentLinkPaymentIntentDataSetupFutureUsage = stripe_shared::CreateSetupIntentUsage;
/// Specify whether Checkout should collect a payment method.
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.
///
/// Can only be set in `subscription` mode. Defaults to `always`.
///
/// If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://docs.stripe.com/payments/checkout/free-trials).
pub type UpdatePaymentLinkPaymentMethodCollection = stripe_shared::PaymentLinkCustomerCreation;
/// The card brands to block.
/// If a customer enters or selects a card belonging to a blocked brand, they can't complete the payment.
pub type UpdatePaymentLinkPaymentMethodOptionsCardRestrictionsBrandsBlocked =
    stripe_shared::BrandsBlocked;
/// The list of payment method types that customers can use.
/// Pass an empty string to enable dynamic payment methods that use your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
pub type UpdatePaymentLinkPaymentMethodTypes = stripe_shared::PaymentLinkPaymentMethodTypes;
/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
/// shipping locations.
pub type UpdatePaymentLinkShippingAddressCollectionAllowedCountries =
    stripe_shared::AllowedCountries;
/// Describes the type of transaction being performed in order to customize relevant text on the page, such as the submit button.
/// Changing this value will also affect the hostname in the [url](https://docs.stripe.com/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`).
pub type UpdatePaymentLinkSubmitType = stripe_shared::SubmitType;
/// Type of the account referenced in the request.
pub type UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType =
    stripe_shared::CreateInvoiceIssuerType;
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
pub type UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod =
    stripe_shared::MissingPaymentMethod;
/// Describes whether a tax ID is required during checkout.
/// Defaults to `never`.
/// You can't set this parameter if `ui_mode` is `custom`.
pub type UpdatePaymentLinkTaxIdCollectionRequired = stripe_shared::Required;
/// Configuration when `type=hosted_confirmation`.
pub type CreatePaymentLinkAfterCompletionHostedConfirmation = AfterCompletionConfirmationPageParams;
/// Configuration when `type=redirect`.
pub type CreatePaymentLinkAfterCompletionRedirect = AfterCompletionRedirectParams;
/// Determines the display of payment method reuse agreement text in the UI.
/// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
pub type CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreement =
    PaymentMethodReuseAgreementParams;
/// The options available for the customer to select. Up to 200 options allowed.
pub type CreatePaymentLinkCustomFieldsDropdownOptions = CustomFieldOptionParam;
/// The label for the field, displayed to the customer.
pub type CreatePaymentLinkCustomFieldsLabel = CustomFieldLabelParam;
/// Custom text that should be displayed after the payment confirmation button.
pub type CreatePaymentLinkCustomTextAfterSubmit = CustomTextPositionParam;
/// Custom text that should be displayed alongside shipping address collection.
pub type CreatePaymentLinkCustomTextShippingAddress = CustomTextPositionParam;
/// Custom text that should be displayed alongside the payment confirmation button.
pub type CreatePaymentLinkCustomTextSubmit = CustomTextPositionParam;
/// Custom text that should be displayed in place of the default terms of service agreement text.
pub type CreatePaymentLinkCustomTextTermsOfServiceAcceptance = CustomTextPositionParam;
/// Default custom fields to be displayed on invoices for this customer.
pub type CreatePaymentLinkInvoiceCreationInvoiceDataCustomFields = CustomFieldParams;
/// Default options for invoice PDF rendering for this customer.
pub type CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions =
    CheckoutRenderingOptionsParam;
/// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
pub type CreatePaymentLinkLineItemsAdjustableQuantity = AdjustableQuantityParams;
/// Controls settings applied for collecting the customer's business name.
pub type CreatePaymentLinkNameCollectionBusiness = NameCollectionBusinessParams;
/// Controls settings applied for collecting the customer's individual name.
pub type CreatePaymentLinkNameCollectionIndividual = NameCollectionIndividualParams;
/// When set, provides configuration for the customer to adjust the quantity of the line item created when a customer chooses to add this optional item to their order.
pub type CreatePaymentLinkOptionalItemsAdjustableQuantity = OptionalItemAdjustableQuantityParams;
/// Controls phone number collection settings during checkout.
///
/// We recommend that you review your privacy policy and check with your legal contacts.
pub type CreatePaymentLinkPhoneNumberCollection = PhoneNumberCollectionParams;
/// Configuration for the `completed_sessions` restriction type.
pub type CreatePaymentLinkRestrictionsCompletedSessions = CompletedSessionsParams;
/// Configuration for collecting the customer's shipping address.
pub type CreatePaymentLinkShippingAddressCollection = ShippingAddressCollectionParams;
/// The shipping rate options to apply to [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by this payment link.
pub type CreatePaymentLinkShippingOptions = ShippingOptionParams;
/// Defines how the subscription should behave when the user's free trial ends.
pub type CreatePaymentLinkSubscriptionDataTrialSettingsEndBehavior = EndBehavior;
/// Controls tax ID collection during checkout.
pub type CreatePaymentLinkTaxIdCollection = TaxIdCollectionParams;
/// Configuration when `type=hosted_confirmation`.
pub type UpdatePaymentLinkAfterCompletionHostedConfirmation = AfterCompletionConfirmationPageParams;
/// Configuration when `type=redirect`.
pub type UpdatePaymentLinkAfterCompletionRedirect = AfterCompletionRedirectParams;
/// Determines the display of payment method reuse agreement text in the UI.
/// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
pub type UpdatePaymentLinkConsentCollectionPaymentMethodReuseAgreement =
    PaymentMethodReuseAgreementParams;
/// The options available for the customer to select. Up to 200 options allowed.
pub type UpdatePaymentLinkCustomFieldsDropdownOptions = CustomFieldOptionParam;
/// The label for the field, displayed to the customer.
pub type UpdatePaymentLinkCustomFieldsLabel = CustomFieldLabelParam;
/// Custom text that should be displayed after the payment confirmation button.
pub type UpdatePaymentLinkCustomTextAfterSubmit = CustomTextPositionParam;
/// Custom text that should be displayed alongside shipping address collection.
pub type UpdatePaymentLinkCustomTextShippingAddress = CustomTextPositionParam;
/// Custom text that should be displayed alongside the payment confirmation button.
pub type UpdatePaymentLinkCustomTextSubmit = CustomTextPositionParam;
/// Custom text that should be displayed in place of the default terms of service agreement text.
pub type UpdatePaymentLinkCustomTextTermsOfServiceAcceptance = CustomTextPositionParam;
/// Default custom fields to be displayed on invoices for this customer.
pub type UpdatePaymentLinkInvoiceCreationInvoiceDataCustomFields = CustomFieldParams;
/// Default options for invoice PDF rendering for this customer.
pub type UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions =
    CheckoutRenderingOptionsParam;
/// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
pub type UpdatePaymentLinkLineItemsAdjustableQuantity = AdjustableQuantityParams;
/// Controls settings applied for collecting the customer's business name.
pub type UpdatePaymentLinkNameCollectionBusiness = NameCollectionBusinessParams;
/// Controls settings applied for collecting the customer's individual name.
pub type UpdatePaymentLinkNameCollectionIndividual = NameCollectionIndividualParams;
/// When set, provides configuration for the customer to adjust the quantity of the line item created when a customer chooses to add this optional item to their order.
pub type UpdatePaymentLinkOptionalItemsAdjustableQuantity = OptionalItemAdjustableQuantityParams;
/// Controls phone number collection settings during checkout.
///
/// We recommend that you review your privacy policy and check with your legal contacts.
pub type UpdatePaymentLinkPhoneNumberCollection = PhoneNumberCollectionParams;
/// Configuration for the `completed_sessions` restriction type.
pub type UpdatePaymentLinkRestrictionsCompletedSessions = CompletedSessionsParams;
/// Configuration for collecting the customer's shipping address.
pub type UpdatePaymentLinkShippingAddressCollection = ShippingAddressCollectionParams;
/// The shipping rate options to apply to [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by this payment link.
pub type UpdatePaymentLinkShippingOptions = ShippingOptionParams;
/// Defines how the subscription should behave when the user's free trial ends.
pub type UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehavior = EndBehavior;
/// Controls tax ID collection during checkout.
pub type UpdatePaymentLinkTaxIdCollection = TaxIdCollectionParams;
/// Behavior after the purchase is complete.
pub type CreatePaymentLinkAfterCompletion = AfterCompletionParams;
/// Configure fields to gather active consent from customers.
pub type CreatePaymentLinkConsentCollection = ConsentCollectionParams;
/// Configuration for `type=dropdown` fields.
pub type CreatePaymentLinkCustomFieldsDropdown = CustomFieldDropdownParam;
/// Display additional text for your customers using custom text.
/// You can't set this parameter if `ui_mode` is `custom`.
pub type CreatePaymentLinkCustomText = CustomTextParam;
/// Controls settings applied for collecting the customer's name.
pub type CreatePaymentLinkNameCollection = NameCollectionParams;
/// A list of optional items the customer can add to their order at checkout.
/// Use this parameter to pass one-time or recurring [Prices](https://docs.stripe.com/api/prices).
/// There is a maximum of 10 optional items allowed on a payment link, and the existing limits on the number of line items allowed on a payment link apply to the combined number of line items and optional items.
/// There is a maximum of 20 combined line items and optional items.
pub type CreatePaymentLinkOptionalItems = OptionalItemParams;
/// Settings that restrict the usage of a payment link.
pub type CreatePaymentLinkRestrictions = RestrictionsParams;
/// Settings related to subscription trials.
pub type CreatePaymentLinkSubscriptionDataTrialSettings = TrialSettingsConfig;
/// Behavior after the purchase is complete.
pub type UpdatePaymentLinkAfterCompletion = AfterCompletionParams;
/// Configure fields to gather active consent from customers.
pub type UpdatePaymentLinkConsentCollection = ConsentCollectionParams;
/// Configuration for `type=dropdown` fields.
pub type UpdatePaymentLinkCustomFieldsDropdown = CustomFieldDropdownParam;
/// Display additional text for your customers using custom text.
/// You can't set this parameter if `ui_mode` is `custom`.
pub type UpdatePaymentLinkCustomText = CustomTextParam;
/// Controls settings applied for collecting the customer's name.
pub type UpdatePaymentLinkNameCollection = NameCollectionParams;
/// A list of optional items the customer can add to their order at checkout.
/// Use this parameter to pass one-time or recurring [Prices](https://docs.stripe.com/api/prices).
/// There is a maximum of 10 optional items allowed on a payment link, and the existing limits on the number of line items allowed on a payment link apply to the combined number of line items and optional items.
/// There is a maximum of 20 combined line items and optional items.
pub type UpdatePaymentLinkOptionalItems = OptionalItemParams;
/// Settings that restrict the usage of a payment link.
pub type UpdatePaymentLinkRestrictions = RestrictionsParams;
/// Settings related to subscription trials.
pub type UpdatePaymentLinkSubscriptionDataTrialSettings = TrialSettingsConfig;
