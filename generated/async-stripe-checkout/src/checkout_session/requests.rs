use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListCheckoutSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_details: Option<ListCheckoutSessionCustomerDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::CheckoutSessionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListCheckoutSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListCheckoutSessionBuilder").finish_non_exhaustive()
    }
}
impl ListCheckoutSessionBuilder {
    fn new() -> Self {
        Self {
            created: None,
            customer: None,
            customer_account: None,
            customer_details: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_intent: None,
            payment_link: None,
            starting_after: None,
            status: None,
            subscription: None,
        }
    }
}
/// Only return the Checkout Sessions for the Customer details specified.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListCheckoutSessionCustomerDetails {
    /// Customer's email address.
    pub email: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListCheckoutSessionCustomerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListCheckoutSessionCustomerDetails").finish_non_exhaustive()
    }
}
impl ListCheckoutSessionCustomerDetails {
    pub fn new(email: impl Into<String>) -> Self {
        Self { email: email.into() }
    }
}
/// Returns a list of Checkout Sessions.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListCheckoutSession {
    inner: ListCheckoutSessionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListCheckoutSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListCheckoutSession").finish_non_exhaustive()
    }
}
impl ListCheckoutSession {
    /// Construct a new `ListCheckoutSession`.
    pub fn new() -> Self {
        Self { inner: ListCheckoutSessionBuilder::new() }
    }
    /// Only return Checkout Sessions that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return the Checkout Sessions for the Customer specified.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// Only return the Checkout Sessions for the Account specified.
    pub fn customer_account(mut self, customer_account: impl Into<String>) -> Self {
        self.inner.customer_account = Some(customer_account.into());
        self
    }
    /// Only return the Checkout Sessions for the Customer details specified.
    pub fn customer_details(
        mut self,
        customer_details: impl Into<ListCheckoutSessionCustomerDetails>,
    ) -> Self {
        self.inner.customer_details = Some(customer_details.into());
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
    /// Only return the Checkout Session for the PaymentIntent specified.
    pub fn payment_intent(mut self, payment_intent: impl Into<String>) -> Self {
        self.inner.payment_intent = Some(payment_intent.into());
        self
    }
    /// Only return the Checkout Sessions for the Payment Link specified.
    pub fn payment_link(mut self, payment_link: impl Into<String>) -> Self {
        self.inner.payment_link = Some(payment_link.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return the Checkout Sessions matching the given status.
    pub fn status(mut self, status: impl Into<stripe_shared::CheckoutSessionStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
    /// Only return the Checkout Session for the subscription specified.
    pub fn subscription(mut self, subscription: impl Into<String>) -> Self {
        self.inner.subscription = Some(subscription.into());
        self
    }
}
impl Default for ListCheckoutSession {
    fn default() -> Self {
        Self::new()
    }
}
impl ListCheckoutSession {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::CheckoutSession>> {
        stripe_client_core::ListPaginator::new_list("/checkout/sessions", &self.inner)
    }
}

impl StripeRequest for ListCheckoutSession {
    type Output = stripe_types::List<stripe_shared::CheckoutSession>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/checkout/sessions").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveCheckoutSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveCheckoutSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveCheckoutSessionBuilder").finish_non_exhaustive()
    }
}
impl RetrieveCheckoutSessionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a Checkout Session object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveCheckoutSession {
    inner: RetrieveCheckoutSessionBuilder,
    session: stripe_shared::CheckoutSessionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveCheckoutSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveCheckoutSession").finish_non_exhaustive()
    }
}
impl RetrieveCheckoutSession {
    /// Construct a new `RetrieveCheckoutSession`.
    pub fn new(session: impl Into<stripe_shared::CheckoutSessionId>) -> Self {
        Self { session: session.into(), inner: RetrieveCheckoutSessionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveCheckoutSession {
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

impl StripeRequest for RetrieveCheckoutSession {
    type Output = stripe_shared::CheckoutSession;

    fn build(&self) -> RequestBuilder {
        let session = &self.session;
        RequestBuilder::new(StripeMethod::Get, format!("/checkout/sessions/{session}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListLineItemsCheckoutSessionBuilder {
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
impl std::fmt::Debug for ListLineItemsCheckoutSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListLineItemsCheckoutSessionBuilder").finish_non_exhaustive()
    }
}
impl ListLineItemsCheckoutSessionBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// When retrieving a Checkout Session, there is an includable **line_items** property containing the first handful of those items.
/// There is also a URL where you can retrieve the full (paginated) list of line items.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListLineItemsCheckoutSession {
    inner: ListLineItemsCheckoutSessionBuilder,
    session: stripe_shared::CheckoutSessionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListLineItemsCheckoutSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListLineItemsCheckoutSession").finish_non_exhaustive()
    }
}
impl ListLineItemsCheckoutSession {
    /// Construct a new `ListLineItemsCheckoutSession`.
    pub fn new(session: impl Into<stripe_shared::CheckoutSessionId>) -> Self {
        Self { session: session.into(), inner: ListLineItemsCheckoutSessionBuilder::new() }
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
impl ListLineItemsCheckoutSession {
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
        let session = &self.session;

        stripe_client_core::ListPaginator::new_list(
            format!("/checkout/sessions/{session}/line_items"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListLineItemsCheckoutSession {
    type Output = stripe_types::List<stripe_shared::CheckoutSessionItem>;

    fn build(&self) -> RequestBuilder {
        let session = &self.session;
        RequestBuilder::new(StripeMethod::Get, format!("/checkout/sessions/{session}/line_items"))
            .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateCheckoutSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    adaptive_pricing: Option<CreateCheckoutSessionAdaptivePricing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_expiration: Option<CreateCheckoutSessionAfterExpiration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_promotion_codes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<CreateCheckoutSessionAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_address_collection: Option<stripe_shared::BillingAddressCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branding_settings: Option<CreateCheckoutSessionBrandingSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consent_collection: Option<CreateCheckoutSessionConsentCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_fields: Option<Vec<CreateCheckoutSessionCustomFields>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_text: Option<CreateCheckoutSessionCustomText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_creation: Option<stripe_shared::PaymentLinkCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_update: Option<CreateCheckoutSessionCustomerUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<CreateCheckoutSessionDiscounts>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_payment_method_types: Option<Vec<stripe_shared::ExcludedPaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_creation: Option<CreateCheckoutSessionInvoiceCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_items: Option<Vec<CreateCheckoutSessionLineItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<stripe_shared::CheckoutSessionLocale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_payments: Option<CreateCheckoutSessionManagedPayments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<stripe_shared::CheckoutSessionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_collection: Option<CreateCheckoutSessionNameCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_items: Option<Vec<CreateCheckoutSessionOptionalItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_context: Option<stripe_shared::OriginContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent_data: Option<CreateCheckoutSessionPaymentIntentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_collection: Option<stripe_shared::PaymentLinkCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_data: Option<CreateCheckoutSessionPaymentMethodData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_options: Option<CreateCheckoutSessionPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_types: Option<Vec<CreateCheckoutSessionPaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<CreateCheckoutSessionPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number_collection: Option<CreateCheckoutSessionPhoneNumberCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_on_completion: Option<stripe_shared::RedirectOnCompletion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saved_payment_method_options: Option<CreateCheckoutSessionSavedPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    setup_intent_data: Option<CreateCheckoutSessionSetupIntentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_address_collection: Option<CreateCheckoutSessionShippingAddressCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Vec<ShippingOptionParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submit_type: Option<stripe_shared::SubmitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_data: Option<CreateCheckoutSessionSubscriptionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_id_collection: Option<CreateCheckoutSessionTaxIdCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_mode: Option<stripe_shared::UiMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wallet_options: Option<CreateCheckoutSessionWalletOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionBuilder").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionBuilder {
    fn new() -> Self {
        Self {
            adaptive_pricing: None,
            after_expiration: None,
            allow_promotion_codes: None,
            automatic_tax: None,
            billing_address_collection: None,
            branding_settings: None,
            cancel_url: None,
            client_reference_id: None,
            consent_collection: None,
            currency: None,
            custom_fields: None,
            custom_text: None,
            customer: None,
            customer_account: None,
            customer_creation: None,
            customer_email: None,
            customer_update: None,
            discounts: None,
            excluded_payment_method_types: None,
            expand: None,
            expires_at: None,
            integration_identifier: None,
            invoice_creation: None,
            line_items: None,
            locale: None,
            managed_payments: None,
            metadata: None,
            mode: None,
            name_collection: None,
            optional_items: None,
            origin_context: None,
            payment_intent_data: None,
            payment_method_collection: None,
            payment_method_configuration: None,
            payment_method_data: None,
            payment_method_options: None,
            payment_method_types: None,
            permissions: None,
            phone_number_collection: None,
            redirect_on_completion: None,
            return_url: None,
            saved_payment_method_options: None,
            setup_intent_data: None,
            shipping_address_collection: None,
            shipping_options: None,
            submit_type: None,
            subscription_data: None,
            success_url: None,
            tax_id_collection: None,
            ui_mode: None,
            wallet_options: None,
        }
    }
}
/// Settings for price localization with [Adaptive Pricing](https://docs.stripe.com/payments/checkout/adaptive-pricing).
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionAdaptivePricing {
    /// If set to `true`, Adaptive Pricing is available on [eligible sessions](https://docs.stripe.com/payments/currencies/localize-prices/adaptive-pricing?payment-ui=stripe-hosted#restrictions).
    /// Defaults to your [dashboard setting](https://dashboard.stripe.com/settings/adaptive-pricing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionAdaptivePricing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionAdaptivePricing").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionAdaptivePricing {
    pub fn new() -> Self {
        Self { enabled: None }
    }
}
impl Default for CreateCheckoutSessionAdaptivePricing {
    fn default() -> Self {
        Self::new()
    }
}
/// Configure actions after a Checkout Session has expired.
/// You can't set this parameter if `ui_mode` is `elements`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionAfterExpiration {
    /// Configure a Checkout Session that can be used to recover an expired session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery: Option<CreateCheckoutSessionAfterExpirationRecovery>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionAfterExpiration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionAfterExpiration").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionAfterExpiration {
    pub fn new() -> Self {
        Self { recovery: None }
    }
}
impl Default for CreateCheckoutSessionAfterExpiration {
    fn default() -> Self {
        Self::new()
    }
}
/// Configure a Checkout Session that can be used to recover an expired session.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionAfterExpirationRecovery {
    /// Enables user redeemable promotion codes on the recovered Checkout Sessions. Defaults to `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    /// If `true`, a recovery URL will be generated to recover this Checkout Session if it
    /// expires before a successful transaction is completed. It will be attached to the
    /// Checkout Session object upon expiration.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionAfterExpirationRecovery {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionAfterExpirationRecovery").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionAfterExpirationRecovery {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { allow_promotion_codes: None, enabled: enabled.into() }
    }
}
/// Settings for automatic tax lookup for this session and resulting payments, invoices, and subscriptions.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionAutomaticTax {
    /// Set to `true` to [calculate tax automatically](https://docs.stripe.com/tax) using the customer's location.
    ///
    /// Enabling this parameter causes Checkout to collect any billing address information necessary for tax calculation.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreateCheckoutSessionAutomaticTaxLiability>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionAutomaticTax").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionAutomaticTax {
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
pub struct CreateCheckoutSessionAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionAutomaticTaxLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionAutomaticTaxLiability").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionAutomaticTaxLiability {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// The branding settings for the Checkout Session.
/// This parameter is not allowed if ui_mode is `elements`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionBrandingSettings {
    /// A hex color value starting with `#` representing the background color for the Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// The border style for the Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_style: Option<stripe_shared::BorderStyle>,
    /// A hex color value starting with `#` representing the button color for the Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_color: Option<String>,
    /// A string to override the business name shown on the Checkout Session.
    /// This only shows at the top of the Checkout page, and your business name still appears in terms, receipts, and other places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The font family for the Checkout Session corresponding to one of the [supported font families](https://docs.stripe.com/payments/checkout/customization/appearance?payment-ui=stripe-hosted#font-compatibility).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<CreateCheckoutSessionBrandingSettingsFontFamily>,
    /// The icon for the Checkout Session. For best results, use a square image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<CreateCheckoutSessionBrandingSettingsIcon>,
    /// The logo for the Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<CreateCheckoutSessionBrandingSettingsLogo>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionBrandingSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionBrandingSettings").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionBrandingSettings {
    pub fn new() -> Self {
        Self {
            background_color: None,
            border_style: None,
            button_color: None,
            display_name: None,
            font_family: None,
            icon: None,
            logo: None,
        }
    }
}
impl Default for CreateCheckoutSessionBrandingSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The font family for the Checkout Session corresponding to one of the [supported font families](https://docs.stripe.com/payments/checkout/customization/appearance?payment-ui=stripe-hosted#font-compatibility).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateCheckoutSessionBrandingSettingsFontFamily {
    BeVietnamPro,
    Bitter,
    ChakraPetch,
    Default,
    Hahmlet,
    Inconsolata,
    Inter,
    Lato,
    Lora,
    MPlus1Code,
    Montserrat,
    NotoSans,
    NotoSansJp,
    NotoSerif,
    Nunito,
    OpenSans,
    Pridi,
    PtSans,
    PtSerif,
    Raleway,
    Roboto,
    RobotoSlab,
    SourceSansPro,
    TitilliumWeb,
    UbuntuMono,
    ZenMaruGothic,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateCheckoutSessionBrandingSettingsFontFamily {
    pub fn as_str(&self) -> &str {
        use CreateCheckoutSessionBrandingSettingsFontFamily::*;
        match self {
            BeVietnamPro => "be_vietnam_pro",
            Bitter => "bitter",
            ChakraPetch => "chakra_petch",
            Default => "default",
            Hahmlet => "hahmlet",
            Inconsolata => "inconsolata",
            Inter => "inter",
            Lato => "lato",
            Lora => "lora",
            MPlus1Code => "m_plus_1_code",
            Montserrat => "montserrat",
            NotoSans => "noto_sans",
            NotoSansJp => "noto_sans_jp",
            NotoSerif => "noto_serif",
            Nunito => "nunito",
            OpenSans => "open_sans",
            Pridi => "pridi",
            PtSans => "pt_sans",
            PtSerif => "pt_serif",
            Raleway => "raleway",
            Roboto => "roboto",
            RobotoSlab => "roboto_slab",
            SourceSansPro => "source_sans_pro",
            TitilliumWeb => "titillium_web",
            UbuntuMono => "ubuntu_mono",
            ZenMaruGothic => "zen_maru_gothic",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionBrandingSettingsFontFamily {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionBrandingSettingsFontFamily::*;
        match s {
            "be_vietnam_pro" => Ok(BeVietnamPro),
            "bitter" => Ok(Bitter),
            "chakra_petch" => Ok(ChakraPetch),
            "default" => Ok(Default),
            "hahmlet" => Ok(Hahmlet),
            "inconsolata" => Ok(Inconsolata),
            "inter" => Ok(Inter),
            "lato" => Ok(Lato),
            "lora" => Ok(Lora),
            "m_plus_1_code" => Ok(MPlus1Code),
            "montserrat" => Ok(Montserrat),
            "noto_sans" => Ok(NotoSans),
            "noto_sans_jp" => Ok(NotoSansJp),
            "noto_serif" => Ok(NotoSerif),
            "nunito" => Ok(Nunito),
            "open_sans" => Ok(OpenSans),
            "pridi" => Ok(Pridi),
            "pt_sans" => Ok(PtSans),
            "pt_serif" => Ok(PtSerif),
            "raleway" => Ok(Raleway),
            "roboto" => Ok(Roboto),
            "roboto_slab" => Ok(RobotoSlab),
            "source_sans_pro" => Ok(SourceSansPro),
            "titillium_web" => Ok(TitilliumWeb),
            "ubuntu_mono" => Ok(UbuntuMono),
            "zen_maru_gothic" => Ok(ZenMaruGothic),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateCheckoutSessionBrandingSettingsFontFamily"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionBrandingSettingsFontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateCheckoutSessionBrandingSettingsFontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionBrandingSettingsFontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateCheckoutSessionBrandingSettingsFontFamily))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateCheckoutSessionBrandingSettingsFontFamily {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionBrandingSettingsFontFamily {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The icon for the Checkout Session. For best results, use a square image.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionBrandingSettingsIcon {
    /// The ID of a [File upload](https://stripe.com/docs/api/files) representing the icon.
    /// Purpose must be `business_icon`.
    /// Required if `type` is `file` and disallowed otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// The type of image for the icon. Must be one of `file` or `url`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateCheckoutSessionBrandingSettingsIconType,
    /// The URL of the image. Required if `type` is `url` and disallowed otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionBrandingSettingsIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionBrandingSettingsIcon").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionBrandingSettingsIcon {
    pub fn new(
        type_: impl Into<stripe_shared::CreateCheckoutSessionBrandingSettingsIconType>,
    ) -> Self {
        Self { file: None, type_: type_.into(), url: None }
    }
}
/// The logo for the Checkout Session.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionBrandingSettingsLogo {
    /// The ID of a [File upload](https://stripe.com/docs/api/files) representing the logo.
    /// Purpose must be `business_logo`.
    /// Required if `type` is `file` and disallowed otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// The type of image for the logo. Must be one of `file` or `url`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateCheckoutSessionBrandingSettingsIconType,
    /// The URL of the image. Required if `type` is `url` and disallowed otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionBrandingSettingsLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionBrandingSettingsLogo").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionBrandingSettingsLogo {
    pub fn new(
        type_: impl Into<stripe_shared::CreateCheckoutSessionBrandingSettingsIconType>,
    ) -> Self {
        Self { file: None, type_: type_.into(), url: None }
    }
}
/// Configure fields for the Checkout Session to gather active consent from customers.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionConsentCollection {
    /// Determines the display of payment method reuse agreement text in the UI.
    /// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_reuse_agreement:
        Option<CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement>,
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
impl std::fmt::Debug for CreateCheckoutSessionConsentCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionConsentCollection").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionConsentCollection {
    pub fn new() -> Self {
        Self { payment_method_reuse_agreement: None, promotions: None, terms_of_service: None }
    }
}
impl Default for CreateCheckoutSessionConsentCollection {
    fn default() -> Self {
        Self::new()
    }
}
/// Determines the display of payment method reuse agreement text in the UI.
/// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement {
    /// Determines the position and visibility of the payment method reuse agreement in the UI.
    /// When set to `auto`, Stripe's.
    /// defaults will be used.
    /// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
    pub position: stripe_shared::Position,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement {
    pub fn new(position: impl Into<stripe_shared::Position>) -> Self {
        Self { position: position.into() }
    }
}
/// Collect additional information from your customer using custom fields.
/// Up to 3 fields are supported.
/// You can't set this parameter if `ui_mode` is `custom`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionCustomFields {
    /// Configuration for `type=dropdown` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<CreateCheckoutSessionCustomFieldsDropdown>,
    /// String of your choice that your integration can use to reconcile this field.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,
    /// The label for the field, displayed to the customer.
    pub label: CreateCheckoutSessionCustomFieldsLabel,
    /// Configuration for `type=numeric` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<CreateCheckoutSessionCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// Configuration for `type=text` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<CreateCheckoutSessionCustomFieldsText>,
    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreatePaymentLinkCustomFieldsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionCustomFields").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionCustomFields {
    pub fn new(
        key: impl Into<String>,
        label: impl Into<CreateCheckoutSessionCustomFieldsLabel>,
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
/// Configuration for `type=dropdown` fields.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsDropdown {
    /// The value that pre-fills the field on the payment page.
    /// Must match a `value` in the `options` array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The options available for the customer to select. Up to 200 options allowed.
    pub options: Vec<CreateCheckoutSessionCustomFieldsDropdownOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionCustomFieldsDropdown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionCustomFieldsDropdown").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionCustomFieldsDropdown {
    pub fn new(options: impl Into<Vec<CreateCheckoutSessionCustomFieldsDropdownOptions>>) -> Self {
        Self { default_value: None, options: options.into() }
    }
}
/// The options available for the customer to select. Up to 200 options allowed.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsDropdownOptions {
    /// The label for the option, displayed to the customer. Up to 100 characters.
    pub label: String,
    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionCustomFieldsDropdownOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionCustomFieldsDropdownOptions").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionCustomFieldsDropdownOptions {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self { label: label.into(), value: value.into() }
    }
}
/// The label for the field, displayed to the customer.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsLabel {
    /// Custom text for the label, displayed to the customer. Up to 50 characters.
    pub custom: String,
    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreatePaymentLinkCustomFieldsLabelType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionCustomFieldsLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionCustomFieldsLabel").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionCustomFieldsLabel {
    pub fn new(
        custom: impl Into<String>,
        type_: impl Into<stripe_shared::CreatePaymentLinkCustomFieldsLabelType>,
    ) -> Self {
        Self { custom: custom.into(), type_: type_.into() }
    }
}
/// Configuration for `type=numeric` fields.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsNumeric {
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
impl std::fmt::Debug for CreateCheckoutSessionCustomFieldsNumeric {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionCustomFieldsNumeric").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionCustomFieldsNumeric {
    pub fn new() -> Self {
        Self { default_value: None, maximum_length: None, minimum_length: None }
    }
}
impl Default for CreateCheckoutSessionCustomFieldsNumeric {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for `type=text` fields.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsText {
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
impl std::fmt::Debug for CreateCheckoutSessionCustomFieldsText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionCustomFieldsText").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionCustomFieldsText {
    pub fn new() -> Self {
        Self { default_value: None, maximum_length: None, minimum_length: None }
    }
}
impl Default for CreateCheckoutSessionCustomFieldsText {
    fn default() -> Self {
        Self::new()
    }
}
/// Display additional text for your customers using custom text.
/// You can't set this parameter if `ui_mode` is `custom`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionCustomText {
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
impl std::fmt::Debug for CreateCheckoutSessionCustomText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionCustomText").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionCustomText {
    pub fn new() -> Self {
        Self {
            after_submit: None,
            shipping_address: None,
            submit: None,
            terms_of_service_acceptance: None,
        }
    }
}
impl Default for CreateCheckoutSessionCustomText {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls what fields on Customer can be updated by the Checkout Session.
/// Can only be provided when `customer` is provided.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionCustomerUpdate {
    /// Describes whether Checkout saves the billing address onto `customer.address`.
    /// To always collect a full billing address, use `billing_address_collection`. Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_shared::CheckoutLinkWalletOptionsDisplay>,
    /// Describes whether Checkout saves the name onto `customer.name`. Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<stripe_shared::CheckoutLinkWalletOptionsDisplay>,
    /// Describes whether Checkout saves shipping information onto `customer.shipping`.
    /// To collect shipping information, use `shipping_address_collection`. Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<stripe_shared::CheckoutLinkWalletOptionsDisplay>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionCustomerUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionCustomerUpdate").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionCustomerUpdate {
    pub fn new() -> Self {
        Self { address: None, name: None, shipping: None }
    }
}
impl Default for CreateCheckoutSessionCustomerUpdate {
    fn default() -> Self {
        Self::new()
    }
}
/// The coupon or promotion code to apply to this Session. Currently, only up to one may be specified.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionDiscounts {
    /// The ID of the coupon to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    /// The ID of a promotion code to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionDiscounts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionDiscounts").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionDiscounts {
    pub fn new() -> Self {
        Self { coupon: None, promotion_code: None }
    }
}
impl Default for CreateCheckoutSessionDiscounts {
    fn default() -> Self {
        Self::new()
    }
}
/// Generate a post-purchase Invoice for one-time payments.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreation {
    /// Set to `true` to enable invoice creation.
    pub enabled: bool,
    /// Parameters passed when creating invoices for payment-mode Checkout Sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<CreateCheckoutSessionInvoiceCreationInvoiceData>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionInvoiceCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionInvoiceCreation").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionInvoiceCreation {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), invoice_data: None }
    }
}
/// Parameters passed when creating invoices for payment-mode Checkout Sessions.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceData {
    /// The account tax IDs associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// Default custom fields to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreateCheckoutSessionInvoiceCreationInvoiceDataCustomFields>>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionInvoiceCreationInvoiceData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionInvoiceCreationInvoiceData").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionInvoiceCreationInvoiceData {
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
impl Default for CreateCheckoutSessionInvoiceCreationInvoiceData {
    fn default() -> Self {
        Self::new()
    }
}
/// Default custom fields to be displayed on invoices for this customer.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceDataCustomFields {
    /// The name of the custom field. This may be up to 40 characters.
    pub name: String,
    /// The value of the custom field. This may be up to 140 characters.
    pub value: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionInvoiceCreationInvoiceDataCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionInvoiceCreationInvoiceDataCustomFields")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionInvoiceCreationInvoiceDataCustomFields {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self { name: name.into(), value: value.into() }
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Default options for invoice PDF rendering for this customer.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions {
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
impl std::fmt::Debug for CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions {
    pub fn new() -> Self {
        Self { amount_tax_display: None, template: None }
    }
}
impl Default for CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// A list of items the customer is purchasing.
/// Use this parameter to pass one-time or recurring [Prices](https://docs.stripe.com/api/prices).
/// The parameter is required for `payment` and `subscription` mode.
///
/// For `payment` mode, there is a maximum of 100 line items, however it is recommended to consolidate line items if there are more than a few dozen.
///
/// For `subscription` mode, there is a maximum of 20 line items with recurring Prices and 20 line items with one-time Prices.
/// Line items with one-time Prices will be on the initial invoice only.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionLineItems {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreateCheckoutSessionLineItemsAdjustableQuantity>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The ID of the [Price](https://docs.stripe.com/api/prices) or [Plan](https://docs.stripe.com/api/plans) object.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<PriceDataWithProductData>,
    /// The quantity of the line item being purchased.
    /// Quantity should not be defined when `recurring.usage_type=metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The [tax rates](https://docs.stripe.com/api/tax_rates) which apply to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionLineItems").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionLineItems {
    pub fn new() -> Self {
        Self {
            adjustable_quantity: None,
            metadata: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for CreateCheckoutSessionLineItems {
    fn default() -> Self {
        Self::new()
    }
}
/// When set, provides configuration for this item’s quantity to be adjusted by the customer during Checkout.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionLineItemsAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative integer.
    pub enabled: bool,
    /// The maximum quantity the customer can purchase for the Checkout Session.
    /// By default this value is 99.
    /// You can specify a value up to 999999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum quantity the customer must purchase for the Checkout Session.
    /// By default this value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionLineItemsAdjustableQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionLineItemsAdjustableQuantity").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionLineItemsAdjustableQuantity {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), maximum: None, minimum: None }
    }
}
/// Settings for Managed Payments for this Checkout Session and resulting [PaymentIntents](/api/payment_intents/object), [Invoices](/api/invoices/object), and [Subscriptions](/api/subscriptions/object).
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionManagedPayments {
    /// Set to `true` to enable [Managed Payments](https://docs.stripe.com/payments/managed-payments), Stripe's merchant of record solution, for this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionManagedPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionManagedPayments").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionManagedPayments {
    pub fn new() -> Self {
        Self { enabled: None }
    }
}
impl Default for CreateCheckoutSessionManagedPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls name collection settings for the session.
///
/// You can configure Checkout to collect your customers' business names, individual names, or both.
/// Each name field can be either required or optional.
///
/// If a [Customer](https://docs.stripe.com/api/customers) is created or provided, the names can be saved to the Customer object as well.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionNameCollection {
    /// Controls settings applied for collecting the customer's business name on the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<CreateCheckoutSessionNameCollectionBusiness>,
    /// Controls settings applied for collecting the customer's individual name on the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<CreateCheckoutSessionNameCollectionIndividual>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionNameCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionNameCollection").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionNameCollection {
    pub fn new() -> Self {
        Self { business: None, individual: None }
    }
}
impl Default for CreateCheckoutSessionNameCollection {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls settings applied for collecting the customer's business name on the session.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionNameCollectionBusiness {
    /// Enable business name collection on the Checkout Session. Defaults to `false`.
    pub enabled: bool,
    /// Whether the customer is required to provide a business name before completing the Checkout Session.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionNameCollectionBusiness {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionNameCollectionBusiness").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionNameCollectionBusiness {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), optional: None }
    }
}
/// Controls settings applied for collecting the customer's individual name on the session.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionNameCollectionIndividual {
    /// Enable individual name collection on the Checkout Session. Defaults to `false`.
    pub enabled: bool,
    /// Whether the customer is required to provide their name before completing the Checkout Session.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionNameCollectionIndividual {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionNameCollectionIndividual").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionNameCollectionIndividual {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), optional: None }
    }
}
/// A list of optional items the customer can add to their order at checkout.
/// Use this parameter to pass one-time or recurring [Prices](https://docs.stripe.com/api/prices).
///
/// There is a maximum of 10 optional items allowed on a Checkout Session, and the existing limits on the number of line items allowed on a Checkout Session apply to the combined number of line items and optional items.
///
/// For `payment` mode, there is a maximum of 100 combined line items and optional items, however it is recommended to consolidate items if there are more than a few dozen.
///
/// For `subscription` mode, there is a maximum of 20 line items and optional items with recurring Prices and 20 line items and optional items with one-time Prices.
///
/// You can't set this parameter if `ui_mode` is `custom`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionOptionalItems {
    /// When set, provides configuration for the customer to adjust the quantity of the line item created when a customer chooses to add this optional item to their order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreateCheckoutSessionOptionalItemsAdjustableQuantity>,
    /// The ID of the [Price](https://docs.stripe.com/api/prices) or [Plan](https://docs.stripe.com/api/plans) object.
    pub price: String,
    /// The initial quantity of the line item created when a customer chooses to add this optional item to their order.
    pub quantity: u64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionOptionalItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionOptionalItems").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionOptionalItems {
    pub fn new(price: impl Into<String>, quantity: impl Into<u64>) -> Self {
        Self { adjustable_quantity: None, price: price.into(), quantity: quantity.into() }
    }
}
/// When set, provides configuration for the customer to adjust the quantity of the line item created when a customer chooses to add this optional item to their order.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionOptionalItemsAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative integer.
    pub enabled: bool,
    /// The maximum quantity of this item the customer can purchase.
    /// By default this value is 99.
    /// You can specify a value up to 999999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum quantity of this item the customer must purchase, if they choose to purchase it.
    /// Because this item is optional, the customer will always be able to remove it from their order, even if the `minimum` configured here is greater than 0.
    /// By default this value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionOptionalItemsAdjustableQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionOptionalItemsAdjustableQuantity")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionOptionalItemsAdjustableQuantity {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), maximum: None, minimum: None }
    }
}
/// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentIntentData {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    /// The amount of the application fee collected will be capped at the total amount captured.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://docs.stripe.com/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::CaptureMethod>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The Stripe account ID for which these funds are intended. For details,
    /// see the PaymentIntents [use case for connected
    /// accounts](/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    /// Email address that the receipt for the resulting payment will be sent to.
    /// If `receipt_email` is specified for a payment in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    /// Indicates that you intend to [make future payments](https://docs.stripe.com/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
    ///
    /// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
    ///
    /// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
    ///
    /// If a Customer has been provided or Checkout creates a new Customer, Checkout will attach the payment method to the Customer.
    ///
    /// If Checkout does not create a Customer, the payment method is not attached to a Customer.
    /// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.
    ///
    /// When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::CreateSetupIntentUsage>,
    /// Shipping information for this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateCheckoutSessionPaymentIntentDataShipping>,
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
    /// The parameters used to automatically create a Transfer when the payment succeeds.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://docs.stripe.com/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateCheckoutSessionPaymentIntentDataTransferData>,
    /// A string that identifies the resulting payment as part of a group.
    /// See the PaymentIntents [use case for connected accounts](https://docs.stripe.com/connect/separate-charges-and-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentIntentData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentIntentData").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentIntentData {
    pub fn new() -> Self {
        Self {
            application_fee_amount: None,
            capture_method: None,
            description: None,
            metadata: None,
            on_behalf_of: None,
            receipt_email: None,
            setup_future_usage: None,
            shipping: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
            transfer_data: None,
            transfer_group: None,
        }
    }
}
impl Default for CreateCheckoutSessionPaymentIntentData {
    fn default() -> Self {
        Self::new()
    }
}
/// Shipping information for this payment.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataShipping {
    /// Shipping address.
    pub address: CreateCheckoutSessionPaymentIntentDataShippingAddress,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// Recipient name.
    pub name: String,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The tracking number for a physical product, obtained from the delivery service.
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentIntentDataShipping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentIntentDataShipping").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentIntentDataShipping {
    pub fn new(
        address: impl Into<CreateCheckoutSessionPaymentIntentDataShippingAddress>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            carrier: None,
            name: name.into(),
            phone: None,
            tracking_number: None,
        }
    }
}
/// Shipping address.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataShippingAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    pub line1: String,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentIntentDataShippingAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentIntentDataShippingAddress")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentIntentDataShippingAddress {
    pub fn new(line1: impl Into<String>) -> Self {
        Self {
            city: None,
            country: None,
            line1: line1.into(),
            line2: None,
            postal_code: None,
            state: None,
        }
    }
}
/// The parameters used to automatically create a Transfer when the payment succeeds.
/// For more information, see the PaymentIntents [use case for connected accounts](https://docs.stripe.com/payments/connected-accounts).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataTransferData {
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
impl std::fmt::Debug for CreateCheckoutSessionPaymentIntentDataTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentIntentDataTransferData").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentIntentDataTransferData {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount: None, destination: destination.into() }
    }
}
/// This parameter allows you to set some attributes on the payment method created during a Checkout session.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodData {
    /// Allow redisplay will be set on the payment method on confirmation and indicates whether this payment method can be shown again to the customer in a checkout flow.
    /// Only set this field if you wish to override the allow_redisplay value determined by Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<stripe_shared::CardAllowRedisplay>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodData").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodData {
    pub fn new() -> Self {
        Self { allow_redisplay: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodData {
    fn default() -> Self {
        Self::new()
    }
}
/// Payment-method-specific configuration.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptions {
    /// contains details about the ACSS Debit payment method options.
    /// You can't set this parameter if `ui_mode` is `elements`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebit>,
    /// contains details about the Affirm payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<PaymentMethodOptionsParam>,
    /// contains details about the Afterpay Clearpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<PaymentMethodOptionsParam>,
    /// contains details about the Alipay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreateCheckoutSessionPaymentMethodOptionsAlipay>,
    /// contains details about the Alma payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alma: Option<CreateCheckoutSessionPaymentMethodOptionsAlma>,
    /// contains details about the AmazonPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<CreateCheckoutSessionPaymentMethodOptionsAmazonPay>,
    /// contains details about the AU Becs Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit>,
    /// contains details about the Bacs Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreateCheckoutSessionPaymentMethodOptionsBacsDebit>,
    /// contains details about the Bancontact payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateCheckoutSessionPaymentMethodOptionsBancontact>,
    /// contains details about the Billie payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billie: Option<CreateCheckoutSessionPaymentMethodOptionsBillie>,
    /// contains details about the Boleto payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreateCheckoutSessionPaymentMethodOptionsBoleto>,
    /// contains details about the Card payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateCheckoutSessionPaymentMethodOptionsCard>,
    /// contains details about the Cashapp Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<CreateCheckoutSessionPaymentMethodOptionsCashapp>,
    /// contains details about the Crypto payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto: Option<CreateCheckoutSessionPaymentMethodOptionsCrypto>,
    /// contains details about the Customer Balance payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalance>,
    /// contains details about the DemoPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo_pay: Option<CreateCheckoutSessionPaymentMethodOptionsDemoPay>,
    /// contains details about the EPS payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreateCheckoutSessionPaymentMethodOptionsEps>,
    /// contains details about the FPX payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreateCheckoutSessionPaymentMethodOptionsFpx>,
    /// contains details about the Giropay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreateCheckoutSessionPaymentMethodOptionsGiropay>,
    /// contains details about the Grabpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreateCheckoutSessionPaymentMethodOptionsGrabpay>,
    /// contains details about the Ideal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreateCheckoutSessionPaymentMethodOptionsIdeal>,
    /// contains details about the Kakao Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kakao_pay: Option<CreateCheckoutSessionPaymentMethodOptionsKakaoPay>,
    /// contains details about the Klarna payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateCheckoutSessionPaymentMethodOptionsKlarna>,
    /// contains details about the Konbini payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreateCheckoutSessionPaymentMethodOptionsKonbini>,
    /// contains details about the Korean card payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<CreateCheckoutSessionPaymentMethodOptionsKrCard>,
    /// contains details about the Link payment method options (Link is also known as Onelink in the UK).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateCheckoutSessionPaymentMethodOptionsLink>,
    /// contains details about the Mobilepay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<PaymentMethodOptionsParam>,
    /// contains details about the Multibanco payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<CreateCheckoutSessionPaymentMethodOptionsMultibanco>,
    /// contains details about the Naver Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<CreateCheckoutSessionPaymentMethodOptionsNaverPay>,
    /// contains details about the OXXO payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreateCheckoutSessionPaymentMethodOptionsOxxo>,
    /// contains details about the P24 payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreateCheckoutSessionPaymentMethodOptionsP24>,
    /// contains details about the Pay By Bank payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub pay_by_bank: Option<miniserde::json::Value>,
    /// contains details about the PAYCO payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payco: Option<PaymentMethodOptionsParam>,
    /// contains details about the PayNow payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreateCheckoutSessionPaymentMethodOptionsPaynow>,
    /// contains details about the PayPal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CreateCheckoutSessionPaymentMethodOptionsPaypal>,
    /// contains details about the PayTo payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto: Option<CreateCheckoutSessionPaymentMethodOptionsPayto>,
    /// contains details about the Pix payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreateCheckoutSessionPaymentMethodOptionsPix>,
    /// contains details about the RevolutPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<CreateCheckoutSessionPaymentMethodOptionsRevolutPay>,
    /// contains details about the Samsung Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<PaymentMethodOptionsParam>,
    /// contains details about the Satispay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satispay: Option<CreateCheckoutSessionPaymentMethodOptionsSatispay>,
    /// contains details about the Scalapay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalapay: Option<CreateCheckoutSessionPaymentMethodOptionsScalapay>,
    /// contains details about the Sepa Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateCheckoutSessionPaymentMethodOptionsSepaDebit>,
    /// contains details about the Sofort payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateCheckoutSessionPaymentMethodOptionsSofort>,
    /// contains details about the Sunbit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunbit: Option<PaymentMethodOptionsParam>,
    /// contains details about the Swish payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<CreateCheckoutSessionPaymentMethodOptionsSwish>,
    /// contains details about the TWINT payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twint: Option<CreateCheckoutSessionPaymentMethodOptionsTwint>,
    /// contains details about the UPI payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi: Option<CreateCheckoutSessionPaymentMethodOptionsUpi>,
    /// contains details about the Us Bank Account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccount>,
    /// contains details about the WeChat Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreateCheckoutSessionPaymentMethodOptionsWechatPay>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptions").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptions {
    pub fn new() -> Self {
        Self {
            acss_debit: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            alma: None,
            amazon_pay: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            billie: None,
            boleto: None,
            card: None,
            cashapp: None,
            crypto: None,
            customer_balance: None,
            demo_pay: None,
            eps: None,
            fpx: None,
            giropay: None,
            grabpay: None,
            ideal: None,
            kakao_pay: None,
            klarna: None,
            konbini: None,
            kr_card: None,
            link: None,
            mobilepay: None,
            multibanco: None,
            naver_pay: None,
            oxxo: None,
            p24: None,
            pay_by_bank: None,
            payco: None,
            paynow: None,
            paypal: None,
            payto: None,
            pix: None,
            revolut_pay: None,
            samsung_pay: None,
            satispay: None,
            scalapay: None,
            sepa_debit: None,
            sofort: None,
            sunbit: None,
            swish: None,
            twint: None,
            upi: None,
            us_bank_account: None,
            wechat_pay: None,
        }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the ACSS Debit payment method options.
/// You can't set this parameter if `ui_mode` is `elements`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebit {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// This is only accepted for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_shared::Currency>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::SetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsAcssDebit").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self {
            currency: None,
            mandate_options: None,
            setup_future_usage: None,
            target_date: None,
            verification_method: None,
        }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsAcssDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,.
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,
    /// List of Stripe products where this mandate can be selected automatically.
    /// Only usable in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for: Option<Vec<stripe_shared::DefaultFor>>,
    /// Description of the mandate interval.
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<stripe_shared::PaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<stripe_shared::TransactionType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions {
    pub fn new() -> Self {
        Self {
            custom_mandate_url: None,
            default_for: None,
            interval_description: None,
            payment_schedule: None,
            transaction_type: None,
        }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Alipay payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAlipay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAlipay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsAlipay").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsAlipay {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsAlipay {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Alma payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAlma {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAlma {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsAlma").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsAlma {
    pub fn new() -> Self {
        Self { capture_method: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsAlma {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the AmazonPay payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAmazonPay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAmazonPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsAmazonPay").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsAmazonPay {
    pub fn new() -> Self {
        Self { capture_method: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsAmazonPay {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the AU Becs Debit payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit {
    pub fn new() -> Self {
        Self { setup_future_usage: None, target_date: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Bacs Debit payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBacsDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateCheckoutSessionPaymentMethodOptionsBacsDebitMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::SetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsBacsDebit").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsBacsDebit {
    pub fn new() -> Self {
        Self { mandate_options: None, setup_future_usage: None, target_date: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsBacsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBacsDebitMandateOptions {
    /// Prefix used to generate the Mandate reference.
    /// Must be at most 12 characters long.
    /// Must consist of only uppercase letters, numbers, spaces, or the following special characters: '/', '_', '-', '&', '.'.
    /// Cannot begin with 'DDIC' or 'STRIPE'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsBacsDebitMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsBacsDebitMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsBacsDebitMandateOptions {
    pub fn new() -> Self {
        Self { reference_prefix: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsBacsDebitMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Bancontact payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBancontact {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsBancontact {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsBancontact")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsBancontact {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Billie payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBillie {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsBillie {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsBillie").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsBillie {
    pub fn new() -> Self {
        Self { capture_method: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsBillie {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Boleto payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBoleto {
    /// The number of calendar days before a Boleto voucher expires.
    /// For example, if you create a Boleto voucher on Monday and you set expires_after_days to 2, the Boleto invoice will expire on Wednesday at 23:59 America/Sao_Paulo time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::SetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsBoleto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsBoleto").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsBoleto {
    pub fn new() -> Self {
        Self { expires_after_days: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsBoleto {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Card payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCard {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
    /// Installment options for card payments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<CreateCheckoutSessionPaymentMethodOptionsCardInstallments>,
    /// Request ability to [capture beyond the standard authorization validity window](/payments/extended-authorization) for this CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization:
        Option<stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture>,
    /// Request ability to [increment the authorization](/payments/incremental-authorization) for this CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization:
        Option<stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture>,
    /// Request ability to make [multiple captures](/payments/multicapture) for this CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_multicapture:
        Option<stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture>,
    /// Request ability to [overcapture](/payments/overcapture) for this CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_overcapture:
        Option<stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<stripe_shared::RequestThreeDSecure>,
    /// Restrictions to apply to the card payment method.
    /// For example, you can block specific card brands.
    /// You can't set this parameter if `ui_mode` is `custom`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<CreateCheckoutSessionPaymentMethodOptionsCardRestrictions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::CreateSetupIntentUsage>,
    /// Provides information about a card payment that customers see on their statements.
    /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<String>,
    /// Provides information about a card payment that customers see on their statements.
    /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 17 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsCard").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self {
            capture_method: None,
            installments: None,
            request_extended_authorization: None,
            request_incremental_authorization: None,
            request_multicapture: None,
            request_overcapture: None,
            request_three_d_secure: None,
            restrictions: None,
            setup_future_usage: None,
            statement_descriptor_suffix_kana: None,
            statement_descriptor_suffix_kanji: None,
        }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Installment options for card payments
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this Checkout Session.
    /// Setting to false will prevent any installment plan from applying to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCardInstallments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsCardInstallments")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsCardInstallments {
    pub fn new() -> Self {
        Self { enabled: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsCardInstallments {
    fn default() -> Self {
        Self::new()
    }
}
/// Restrictions to apply to the card payment method.
/// For example, you can block specific card brands.
/// You can't set this parameter if `ui_mode` is `custom`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCardRestrictions {
    /// The card brands to block.
    /// If a customer enters or selects a card belonging to a blocked brand, they can't complete the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brands_blocked: Option<Vec<stripe_shared::BrandsBlocked>>,
    /// Card funding types to block for this Checkout Session.
    /// Supported values are `credit`, `debit`, and `prepaid`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_types_blocked: Option<Vec<stripe_shared::FundingTypesBlocked>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCardRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsCardRestrictions")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsCardRestrictions {
    pub fn new() -> Self {
        Self { brands_blocked: None, funding_types_blocked: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsCardRestrictions {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Cashapp Pay payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCashapp {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::SetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCashapp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsCashapp").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsCashapp {
    pub fn new() -> Self {
        Self { capture_method: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsCashapp {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Crypto payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCrypto {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCrypto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsCrypto").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsCrypto {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsCrypto {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Customer Balance payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCustomerBalance {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<stripe_shared::FundingType>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCustomerBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsCustomerBalance")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalance {
    pub fn new() -> Self {
        Self { bank_transfer: None, funding_type: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsCustomerBalance {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer {
    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer:
        Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
    /// List of address types that should be returned in the financial_addresses response.
    /// If not specified, all valid types will be returned.
    ///
    /// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<Vec<stripe_shared::RequestedAddressTypes>>,
    /// The list of bank transfer types that this PaymentIntent is allowed to use for funding.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateFundingInstructionsCustomerBankTransferType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer {
    pub fn new(
        type_: impl Into<stripe_shared::CreateFundingInstructionsCustomerBankTransferType>,
    ) -> Self {
        Self { eu_bank_transfer: None, requested_address_types: None, type_: type_.into() }
    }
}
/// Configuration for eu_bank_transfer funding type.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    /// The desired country code of the bank account information.
    /// Permitted values include: `DE`, `FR`, `IE`, or `NL`.
    pub country: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer",
        )
        .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    pub fn new(country: impl Into<String>) -> Self {
        Self { country: country.into() }
    }
}
/// contains details about the DemoPay payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsDemoPay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsDemoPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsDemoPay").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsDemoPay {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsDemoPay {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the EPS payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsEps {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsEps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsEps").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsEps {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsEps {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the FPX payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsFpx {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsFpx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsFpx").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsFpx {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsFpx {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Giropay payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsGiropay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsGiropay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsGiropay").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsGiropay {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsGiropay {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Grabpay payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsGrabpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsGrabpay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsGrabpay").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsGrabpay {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsGrabpay {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Ideal payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsIdeal {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsIdeal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsIdeal").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsIdeal {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsIdeal {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Kakao Pay payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKakaoPay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKakaoPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsKakaoPay").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsKakaoPay {
    pub fn new() -> Self {
        Self { capture_method: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsKakaoPay {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Klarna payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKlarna {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
    /// Subscription details if the Checkout Session sets up a future subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptions>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsKlarna").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsKlarna {
    pub fn new() -> Self {
        Self { capture_method: None, setup_future_usage: None, subscriptions: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsKlarna {
    fn default() -> Self {
        Self::new()
    }
}
/// Subscription details if the Checkout Session sets up a future subscription.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptions {
    /// Unit of time between subscription charges.
    pub interval: stripe_shared::PlanInterval,
    /// The number of intervals (specified in the `interval` attribute) between subscription charges.
    /// For example, `interval=month` and `interval_count=3` charges every 3 months.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Name for subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Describes the upcoming charge for this subscription.
    pub next_billing: CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsNextBilling,
    /// A non-customer-facing reference to correlate subscription charges in the Klarna app.
    /// Use a value that persists across subscription charges.
    pub reference: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptions")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptions {
    pub fn new(
        interval: impl Into<stripe_shared::PlanInterval>,
        next_billing: impl Into<CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsNextBilling>,
        reference: impl Into<String>,
    ) -> Self {
        Self {
            interval: interval.into(),
            interval_count: None,
            name: None,
            next_billing: next_billing.into(),
            reference: reference.into(),
        }
    }
}
/// Describes the upcoming charge for this subscription.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsNextBilling {
    /// The amount of the next charge for the subscription.
    pub amount: i64,
    /// The date of the next charge for the subscription in YYYY-MM-DD format.
    pub date: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsNextBilling {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsNextBilling")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsNextBilling {
    pub fn new(amount: impl Into<i64>, date: impl Into<String>) -> Self {
        Self { amount: amount.into(), date: date.into() }
    }
}
/// contains details about the Konbini payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKonbini {
    /// The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire.
    /// For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
    /// Defaults to 3 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKonbini {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsKonbini").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsKonbini {
    pub fn new() -> Self {
        Self { expires_after_days: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsKonbini {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Korean card payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKrCard {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKrCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsKrCard").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsKrCard {
    pub fn new() -> Self {
        Self { capture_method: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsKrCard {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Link payment method options (Link is also known as Onelink in the UK).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsLink {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsLink").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsLink {
    pub fn new() -> Self {
        Self { capture_method: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsLink {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Multibanco payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsMultibanco {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsMultibanco {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsMultibanco")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsMultibanco {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsMultibanco {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Naver Pay payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsNaverPay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsNaverPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsNaverPay").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsNaverPay {
    pub fn new() -> Self {
        Self { capture_method: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsNaverPay {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the OXXO payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsOxxo {
    /// The number of calendar days before an OXXO voucher expires.
    /// For example, if you create an OXXO voucher on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsOxxo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsOxxo").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsOxxo {
    pub fn new() -> Self {
        Self { expires_after_days: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsOxxo {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the P24 payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsP24 {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
    /// Confirm that the payer has accepted the P24 terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsP24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsP24").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsP24 {
    pub fn new() -> Self {
        Self { setup_future_usage: None, tos_shown_and_accepted: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsP24 {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the PayNow payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPaynow {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPaynow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsPaynow").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsPaynow {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsPaynow {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the PayPal payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPaypal {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
    /// [Preferred locale](https://docs.stripe.com/payments/paypal/supported-locales) of the PayPal checkout page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<stripe_shared::PreferredLocale>,
    /// A reference of the PayPal transaction visible to customer which is mapped to PayPal's invoice ID.
    /// This must be a globally unique ID if you have configured in your PayPal settings to block multiple payments per invoice ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// The risk correlation ID for an on-session payment using a saved PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_correlation_id: Option<String>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    ///
    /// If you've already set `setup_future_usage` and you're performing a request using a publishable key, you can only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPaypal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsPaypal").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsPaypal {
    pub fn new() -> Self {
        Self {
            capture_method: None,
            preferred_locale: None,
            reference: None,
            risk_correlation_id: None,
            setup_future_usage: None,
        }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsPaypal {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the PayTo payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPayto {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateCheckoutSessionPaymentMethodOptionsPaytoMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsPayto").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsPayto {
    pub fn new() -> Self {
        Self { mandate_options: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsPayto {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPaytoMandateOptions {
    /// Amount that will be collected. It is required when `amount_type` is `fixed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The type of amount that will be collected.
    /// The amount charged must be exact or up to the value of `amount` param for `fixed` or `maximum` type respectively.
    /// Defaults to `maximum`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<stripe_shared::AmountType>,
    /// Date, in YYYY-MM-DD format, after which payments will not be collected. Defaults to no end date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// The periodicity at which payments will be collected. Defaults to `adhoc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<stripe_shared::MandatePaytoPaymentSchedule>,
    /// The number of payments that will be made during a payment period.
    /// Defaults to 1 except for when `payment_schedule` is `adhoc`.
    /// In that case, it defaults to no limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments_per_period: Option<i64>,
    /// The purpose for which payments are made. Has a default value based on your merchant category code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<stripe_shared::Purpose>,
    /// Date, in YYYY-MM-DD format, from which payments will be collected. Defaults to confirmation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPaytoMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsPaytoMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsPaytoMandateOptions {
    pub fn new() -> Self {
        Self {
            amount: None,
            amount_type: None,
            end_date: None,
            payment_schedule: None,
            payments_per_period: None,
            purpose: None,
            start_date: None,
        }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsPaytoMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Pix payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPix {
    /// Determines if the amount includes the IOF tax. Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_includes_iof: Option<stripe_shared::MandatePixAmountIncludesIof>,
    /// The number of seconds (between 10 and 1209600) after which Pix payment will expire.
    /// Defaults to 86400 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
    /// Additional fields for mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateCheckoutSessionPaymentMethodOptionsPixMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsPix").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsPix {
    pub fn new() -> Self {
        Self {
            amount_includes_iof: None,
            expires_after_seconds: None,
            mandate_options: None,
            setup_future_usage: None,
        }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsPix {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for mandate creation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPixMandateOptions {
    /// Amount to be charged for future payments.
    /// Required when `amount_type=fixed`.
    /// If not provided for `amount_type=maximum`, defaults to 40000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Determines if the amount includes the IOF tax. Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_includes_iof: Option<stripe_shared::MandatePixAmountIncludesIof>,
    /// Type of amount. Defaults to `maximum`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<stripe_shared::AmountType>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Only `brl` is supported currently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`.
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Schedule at which the future payments will be charged. Defaults to `monthly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<stripe_shared::MandatePixPaymentSchedule>,
    /// Subscription name displayed to buyers in their bank app. Defaults to the displayable business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Start date of the mandate, in `YYYY-MM-DD`.
    /// Start date should be at least 3 days in the future.
    /// Defaults to 3 days after the current date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPixMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsPixMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsPixMandateOptions {
    pub fn new() -> Self {
        Self {
            amount: None,
            amount_includes_iof: None,
            amount_type: None,
            currency: None,
            end_date: None,
            payment_schedule: None,
            reference: None,
            start_date: None,
        }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsPixMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the RevolutPay payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsRevolutPay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsRevolutPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsRevolutPay")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsRevolutPay {
    pub fn new() -> Self {
        Self { capture_method: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsRevolutPay {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Satispay payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSatispay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsSatispay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsSatispay").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsSatispay {
    pub fn new() -> Self {
        Self { capture_method: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsSatispay {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Scalapay payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsScalapay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsScalapay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsScalapay").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsScalapay {
    pub fn new() -> Self {
        Self { capture_method: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsScalapay {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Sepa Debit payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateCheckoutSessionPaymentMethodOptionsSepaDebitMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::SetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsSepaDebit").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self { mandate_options: None, setup_future_usage: None, target_date: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsSepaDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSepaDebitMandateOptions {
    /// Prefix used to generate the Mandate reference.
    /// Must be at most 12 characters long.
    /// Must consist of only uppercase letters, numbers, spaces, or the following special characters: '/', '_', '-', '&', '.'.
    /// Cannot begin with 'STRIPE'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsSepaDebitMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsSepaDebitMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsSepaDebitMandateOptions {
    pub fn new() -> Self {
        Self { reference_prefix: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsSepaDebitMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Sofort payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSofort {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsSofort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsSofort").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsSofort {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsSofort {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Swish payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSwish {
    /// The order reference that will be displayed to customers in the Swish application.
    /// Defaults to the `id` of the Payment Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsSwish {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsSwish").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsSwish {
    pub fn new() -> Self {
        Self { reference: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsSwish {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the TWINT payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsTwint {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsTwint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsTwint").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsTwint {
    pub fn new() -> Self {
        Self { setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsTwint {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the UPI payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsUpi {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateCheckoutSessionPaymentMethodOptionsUpiMandateOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::SetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsUpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsUpi").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsUpi {
    pub fn new() -> Self {
        Self { mandate_options: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsUpi {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsUpiMandateOptions {
    /// Amount to be charged for future payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<stripe_shared::AmountType>,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// End date of the mandate or subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsUpiMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsUpiMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsUpiMandateOptions {
    pub fn new() -> Self {
        Self { amount: None, amount_type: None, description: None, end_date: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsUpiMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Us Bank Account payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::SetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<stripe_shared::CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsUsBankAccount")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self {
            financial_connections: None,
            setup_future_usage: None,
            target_date: None,
            verification_method: None,
        }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Financial Connections Session creation
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections {
    /// The list of permissions to request.
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<stripe_shared::Permissions>>,
    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<stripe_shared::Prefetch>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections {
    pub fn new() -> Self {
        Self { permissions: None, prefetch: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the WeChat Pay payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsWechatPay {
    /// The app ID registered with WeChat Pay. Only required when client is ios or android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// The client type that the end customer will pay from
    pub client: stripe_shared::Client,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsWechatPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPaymentMethodOptionsWechatPay").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPaymentMethodOptionsWechatPay {
    pub fn new(client: impl Into<stripe_shared::Client>) -> Self {
        Self { app_id: None, client: client.into(), setup_future_usage: None }
    }
}
/// A list of the types of payment methods (e.g., `card`) this Checkout Session can accept.
///
/// You can omit this attribute to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
/// See [Dynamic Payment Methods](https://docs.stripe.com/payments/payment-methods/integration-options#using-dynamic-payment-methods) for more details.
///
/// Read more about the supported payment methods and their requirements in our [payment
/// method details guide](/docs/payments/checkout/payment-methods).
///
/// If multiple payment methods are passed, Checkout will dynamically reorder them to
/// prioritize the most relevant payment methods based on the customer's location and
/// other characteristics.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateCheckoutSessionPaymentMethodTypes {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    Alma,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Bizum,
    Blik,
    Boleto,
    Card,
    Cashapp,
    Crypto,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    KakaoPay,
    Klarna,
    Konbini,
    KrCard,
    Link,
    MbWay,
    Mobilepay,
    Multibanco,
    NaverPay,
    NzBankAccount,
    Oxxo,
    P24,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Payto,
    Pix,
    Promptpay,
    RevolutPay,
    SamsungPay,
    Satispay,
    Scalapay,
    SepaDebit,
    Sofort,
    Sunbit,
    Swish,
    Twint,
    Upi,
    UsBankAccount,
    WechatPay,
    Zip,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateCheckoutSessionPaymentMethodTypes {
    pub fn as_str(&self) -> &str {
        use CreateCheckoutSessionPaymentMethodTypes::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            Alma => "alma",
            AmazonPay => "amazon_pay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Billie => "billie",
            Bizum => "bizum",
            Blik => "blik",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            Crypto => "crypto",
            CustomerBalance => "customer_balance",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            KakaoPay => "kakao_pay",
            Klarna => "klarna",
            Konbini => "konbini",
            KrCard => "kr_card",
            Link => "link",
            MbWay => "mb_way",
            Mobilepay => "mobilepay",
            Multibanco => "multibanco",
            NaverPay => "naver_pay",
            NzBankAccount => "nz_bank_account",
            Oxxo => "oxxo",
            P24 => "p24",
            PayByBank => "pay_by_bank",
            Payco => "payco",
            Paynow => "paynow",
            Paypal => "paypal",
            Payto => "payto",
            Pix => "pix",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SamsungPay => "samsung_pay",
            Satispay => "satispay",
            Scalapay => "scalapay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            Sunbit => "sunbit",
            Swish => "swish",
            Twint => "twint",
            Upi => "upi",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodTypes {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodTypes::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "alma" => Ok(Alma),
            "amazon_pay" => Ok(AmazonPay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "billie" => Ok(Billie),
            "bizum" => Ok(Bizum),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "crypto" => Ok(Crypto),
            "customer_balance" => Ok(CustomerBalance),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "kakao_pay" => Ok(KakaoPay),
            "klarna" => Ok(Klarna),
            "konbini" => Ok(Konbini),
            "kr_card" => Ok(KrCard),
            "link" => Ok(Link),
            "mb_way" => Ok(MbWay),
            "mobilepay" => Ok(Mobilepay),
            "multibanco" => Ok(Multibanco),
            "naver_pay" => Ok(NaverPay),
            "nz_bank_account" => Ok(NzBankAccount),
            "oxxo" => Ok(Oxxo),
            "p24" => Ok(P24),
            "pay_by_bank" => Ok(PayByBank),
            "payco" => Ok(Payco),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "payto" => Ok(Payto),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "samsung_pay" => Ok(SamsungPay),
            "satispay" => Ok(Satispay),
            "scalapay" => Ok(Scalapay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "sunbit" => Ok(Sunbit),
            "swish" => Ok(Swish),
            "twint" => Ok(Twint),
            "upi" => Ok(Upi),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateCheckoutSessionPaymentMethodTypes"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateCheckoutSessionPaymentMethodTypes)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// This property is used to set up permissions for various actions (e.g., update) on the CheckoutSession object.
/// Can only be set when creating `embedded` or `custom` sessions.
///
/// For specific permissions, please refer to their dedicated subsections, such as `permissions.update_shipping_details`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPermissions {
    /// Determines which entity is allowed to update the shipping details.
    ///
    /// Default is `client_only`.
    /// Stripe Checkout client will automatically update the shipping details.
    /// If set to `server_only`, only your server is allowed to update the shipping details.
    ///
    /// This parameter is only supported when `ui_mode=elements`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_shipping_details: Option<stripe_shared::UpdateShippingDetails>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPermissions").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPermissions {
    pub fn new() -> Self {
        Self { update_shipping_details: None }
    }
}
impl Default for CreateCheckoutSessionPermissions {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls phone number collection settings for the session.
///
/// We recommend that you review your privacy policy and check with your legal contacts
/// before using this feature.
/// Learn more about [collecting phone numbers with Checkout](https://docs.stripe.com/payments/checkout/phone-numbers).
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionPhoneNumberCollection {
    /// Set to `true` to enable phone number collection.
    ///
    /// Can only be set in `payment` and `subscription` mode.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionPhoneNumberCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionPhoneNumberCollection").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionPhoneNumberCollection {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Controls saved payment method settings for the session.
/// Only available in `payment` and `subscription` mode.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionSavedPaymentMethodOptions {
    /// Uses the `allow_redisplay` value of each saved payment method to filter the set presented to a returning customer.
    /// By default, only saved payment methods with ’allow_redisplay: ‘always’ are shown in Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay_filters: Option<Vec<stripe_shared::CardAllowRedisplay>>,
    /// Enable customers to choose if they wish to remove their saved payment methods. Disabled by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_remove: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    /// Enable customers to choose if they wish to save their payment method for future use.
    /// Disabled by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_save: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSavedPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionSavedPaymentMethodOptions").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionSavedPaymentMethodOptions {
    pub fn new() -> Self {
        Self {
            allow_redisplay_filters: None,
            payment_method_remove: None,
            payment_method_save: None,
        }
    }
}
impl Default for CreateCheckoutSessionSavedPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in `setup` mode.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionSetupIntentData {
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The Stripe account for which the setup is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSetupIntentData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionSetupIntentData").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionSetupIntentData {
    pub fn new() -> Self {
        Self { description: None, metadata: None, on_behalf_of: None }
    }
}
impl Default for CreateCheckoutSessionSetupIntentData {
    fn default() -> Self {
        Self::new()
    }
}
/// When set, provides configuration for Checkout to collect a shipping address from a customer.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionShippingAddressCollection {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
    /// shipping locations.
    pub allowed_countries: Vec<stripe_shared::AllowedCountries>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionShippingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionShippingAddressCollection").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionShippingAddressCollection {
    pub fn new(allowed_countries: impl Into<Vec<stripe_shared::AllowedCountries>>) -> Self {
        Self { allowed_countries: allowed_countries.into() }
    }
}
/// A subset of parameters to be passed to subscription creation for Checkout Sessions in `subscription` mode.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// To use an application fee percent, the request must be made on behalf of another account, using the `Stripe-Account` header or an OAuth key.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// A future timestamp to anchor the subscription's billing cycle for new subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<stripe_types::Timestamp>,
    /// Configures when the subscription schedule's billing cycle anchors to a specific day of the week or month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor_config:
        Option<CreateCheckoutSessionSubscriptionDataBillingCycleAnchorConfig>,
    /// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<CreateCheckoutSessionSubscriptionDataBillingMode>,
    /// The tax rates that will apply to any subscription item that does not have
    /// `tax_rates` set. Invoices created will have their `default_tax_rates` populated
    /// from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription
    /// for rendering in the [customer portal](https://docs.stripe.com/customer-management).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateCheckoutSessionSubscriptionDataInvoiceSettings>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The account on behalf of which to charge, for each of the subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    /// Specifies an interval for how often to bill for any pending invoice items.
    /// It is analogous to calling [Create an invoice](https://docs.stripe.com/api#create_invoice) for the given subscription at the specified interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_item_interval:
        Option<CreateCheckoutSessionSubscriptionDataPendingInvoiceItemInterval>,
    /// Determines how to handle prorations resulting from the `billing_cycle_anchor`.
    /// If no value is passed, the default is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreateCheckoutSessionSubscriptionDataProrationBehavior>,
    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateCheckoutSessionSubscriptionDataTransferData>,
    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    /// Has to be at least 48 hours in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<stripe_types::Timestamp>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    /// Has to be at least 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<CreateCheckoutSessionSubscriptionDataTrialSettings>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSubscriptionData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionSubscriptionData").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionSubscriptionData {
    pub fn new() -> Self {
        Self {
            application_fee_percent: None,
            billing_cycle_anchor: None,
            billing_cycle_anchor_config: None,
            billing_mode: None,
            default_tax_rates: None,
            description: None,
            invoice_settings: None,
            metadata: None,
            on_behalf_of: None,
            pending_invoice_item_interval: None,
            proration_behavior: None,
            transfer_data: None,
            trial_end: None,
            trial_period_days: None,
            trial_settings: None,
        }
    }
}
impl Default for CreateCheckoutSessionSubscriptionData {
    fn default() -> Self {
        Self::new()
    }
}
/// Configures when the subscription schedule's billing cycle anchors to a specific day of the week or month.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataBillingCycleAnchorConfig {
    /// The day of the month the anchor should be. Ranges from 1 to 31.
    pub day_of_month: i64,
    /// The hour of the day the anchor should be. Ranges from 0 to 23.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<i64>,
    /// The minute of the hour the anchor should be. Ranges from 0 to 59.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<i64>,
    /// The month to start full cycle periods. Ranges from 1 to 12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    /// The second of the minute the anchor should be. Ranges from 0 to 59.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataBillingCycleAnchorConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionSubscriptionDataBillingCycleAnchorConfig")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionSubscriptionDataBillingCycleAnchorConfig {
    pub fn new(day_of_month: impl Into<i64>) -> Self {
        Self {
            day_of_month: day_of_month.into(),
            hour: None,
            minute: None,
            month: None,
            second: None,
        }
    }
}
/// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataBillingMode {
    /// Configure behavior for flexible billing mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible: Option<CreateCheckoutSessionSubscriptionDataBillingModeFlexible>,
    /// Controls the calculation and orchestration of prorations and invoices for subscriptions.
    /// If no value is passed, the default is `flexible`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateSubscriptionBillingModeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataBillingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionSubscriptionDataBillingMode").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionSubscriptionDataBillingMode {
    pub fn new(type_: impl Into<stripe_shared::CreateSubscriptionBillingModeType>) -> Self {
        Self { flexible: None, type_: type_.into() }
    }
}
/// Configure behavior for flexible billing mode.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataBillingModeFlexible {
    /// Controls how invoices and invoice items display proration amounts and discount amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_discounts: Option<stripe_shared::ProrationDiscounts>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataBillingModeFlexible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionSubscriptionDataBillingModeFlexible")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionSubscriptionDataBillingModeFlexible {
    pub fn new() -> Self {
        Self { proration_discounts: None }
    }
}
impl Default for CreateCheckoutSessionSubscriptionDataBillingModeFlexible {
    fn default() -> Self {
        Self::new()
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataInvoiceSettings {
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataInvoiceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionSubscriptionDataInvoiceSettings")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionSubscriptionDataInvoiceSettings {
    pub fn new() -> Self {
        Self { issuer: None }
    }
}
impl Default for CreateCheckoutSessionSubscriptionDataInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer {
    pub fn new(type_: impl Into<stripe_shared::CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Specifies an interval for how often to bill for any pending invoice items.
/// It is analogous to calling [Create an invoice](https://docs.stripe.com/api#create_invoice) for the given subscription at the specified interval.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataPendingInvoiceItemInterval {
    /// Specifies invoicing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: stripe_shared::PlanInterval,
    /// The number of intervals between invoices.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataPendingInvoiceItemInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionSubscriptionDataPendingInvoiceItemInterval")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionSubscriptionDataPendingInvoiceItemInterval {
    pub fn new(interval: impl Into<stripe_shared::PlanInterval>) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Determines how to handle prorations resulting from the `billing_cycle_anchor`.
/// If no value is passed, the default is `create_prorations`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateCheckoutSessionSubscriptionDataProrationBehavior {
    CreateProrations,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateCheckoutSessionSubscriptionDataProrationBehavior {
    pub fn as_str(&self) -> &str {
        use CreateCheckoutSessionSubscriptionDataProrationBehavior::*;
        match self {
            CreateProrations => "create_prorations",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionSubscriptionDataProrationBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionSubscriptionDataProrationBehavior::*;
        match s {
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateCheckoutSessionSubscriptionDataProrationBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionSubscriptionDataProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateCheckoutSessionSubscriptionDataProrationBehavior))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateCheckoutSessionSubscriptionDataProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionSubscriptionDataProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTransferData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionSubscriptionDataTransferData").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionSubscriptionDataTransferData {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount_percent: None, destination: destination.into() }
    }
}
/// Settings related to subscription trials.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTrialSettings {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataTrialSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionSubscriptionDataTrialSettings").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionSubscriptionDataTrialSettings {
    pub fn new(
        end_behavior: impl Into<CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior>,
    ) -> Self {
        Self { end_behavior: end_behavior.into() }
    }
}
/// Defines how the subscription should behave when the user's free trial ends.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: stripe_shared::MissingPaymentMethod,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior")
            .finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior {
    pub fn new(missing_payment_method: impl Into<stripe_shared::MissingPaymentMethod>) -> Self {
        Self { missing_payment_method: missing_payment_method.into() }
    }
}
/// Controls tax ID collection during checkout.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionTaxIdCollection {
    /// Enable tax ID collection during checkout. Defaults to `false`.
    pub enabled: bool,
    /// Describes whether a tax ID is required during checkout.
    /// Defaults to `never`.
    /// You can't set this parameter if `ui_mode` is `custom`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<stripe_shared::Required>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionTaxIdCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionTaxIdCollection").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionTaxIdCollection {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), required: None }
    }
}
/// Wallet-specific configuration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionWalletOptions {
    /// contains details about the Link wallet options (Link is also known as Onelink in the UK).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateCheckoutSessionWalletOptionsLink>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionWalletOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionWalletOptions").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionWalletOptions {
    pub fn new() -> Self {
        Self { link: None }
    }
}
impl Default for CreateCheckoutSessionWalletOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// contains details about the Link wallet options (Link is also known as Onelink in the UK).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSessionWalletOptionsLink {
    /// Specifies whether Checkout should display Link as a payment option.
    /// By default, Checkout will display all the supported wallets that the Checkout Session was created with.
    /// This is the `auto` behavior, and it is the default choice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<stripe_shared::CheckoutLinkWalletOptionsDisplay>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSessionWalletOptionsLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSessionWalletOptionsLink").finish_non_exhaustive()
    }
}
impl CreateCheckoutSessionWalletOptionsLink {
    pub fn new() -> Self {
        Self { display: None }
    }
}
impl Default for CreateCheckoutSessionWalletOptionsLink {
    fn default() -> Self {
        Self::new()
    }
}
/// Creates a Checkout Session object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCheckoutSession {
    inner: CreateCheckoutSessionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCheckoutSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCheckoutSession").finish_non_exhaustive()
    }
}
impl CreateCheckoutSession {
    /// Construct a new `CreateCheckoutSession`.
    pub fn new() -> Self {
        Self { inner: CreateCheckoutSessionBuilder::new() }
    }
    /// Settings for price localization with [Adaptive Pricing](https://docs.stripe.com/payments/checkout/adaptive-pricing).
    pub fn adaptive_pricing(
        mut self,
        adaptive_pricing: impl Into<CreateCheckoutSessionAdaptivePricing>,
    ) -> Self {
        self.inner.adaptive_pricing = Some(adaptive_pricing.into());
        self
    }
    /// Configure actions after a Checkout Session has expired.
    /// You can't set this parameter if `ui_mode` is `elements`.
    pub fn after_expiration(
        mut self,
        after_expiration: impl Into<CreateCheckoutSessionAfterExpiration>,
    ) -> Self {
        self.inner.after_expiration = Some(after_expiration.into());
        self
    }
    /// Enables user redeemable promotion codes.
    pub fn allow_promotion_codes(mut self, allow_promotion_codes: impl Into<bool>) -> Self {
        self.inner.allow_promotion_codes = Some(allow_promotion_codes.into());
        self
    }
    /// Settings for automatic tax lookup for this session and resulting payments, invoices, and subscriptions.
    pub fn automatic_tax(
        mut self,
        automatic_tax: impl Into<CreateCheckoutSessionAutomaticTax>,
    ) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// Specify whether Checkout should collect the customer's billing address. Defaults to `auto`.
    pub fn billing_address_collection(
        mut self,
        billing_address_collection: impl Into<stripe_shared::BillingAddressCollection>,
    ) -> Self {
        self.inner.billing_address_collection = Some(billing_address_collection.into());
        self
    }
    /// The branding settings for the Checkout Session.
    /// This parameter is not allowed if ui_mode is `elements`.
    pub fn branding_settings(
        mut self,
        branding_settings: impl Into<CreateCheckoutSessionBrandingSettings>,
    ) -> Self {
        self.inner.branding_settings = Some(branding_settings.into());
        self
    }
    /// If set, Checkout displays a back button and customers will be directed to this URL if they decide to cancel payment and return to your website.
    /// This parameter is not allowed if ui_mode is `embedded_page` or `elements`.
    pub fn cancel_url(mut self, cancel_url: impl Into<String>) -> Self {
        self.inner.cancel_url = Some(cancel_url.into());
        self
    }
    /// A unique string to reference the Checkout Session. This can be a
    /// customer ID, a cart ID, or similar, and can be used to reconcile the
    /// session with your internal systems.
    pub fn client_reference_id(mut self, client_reference_id: impl Into<String>) -> Self {
        self.inner.client_reference_id = Some(client_reference_id.into());
        self
    }
    /// Configure fields for the Checkout Session to gather active consent from customers.
    pub fn consent_collection(
        mut self,
        consent_collection: impl Into<CreateCheckoutSessionConsentCollection>,
    ) -> Self {
        self.inner.consent_collection = Some(consent_collection.into());
        self
    }
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Required in `setup` mode when `payment_method_types` is not set.
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// Collect additional information from your customer using custom fields.
    /// Up to 3 fields are supported.
    /// You can't set this parameter if `ui_mode` is `custom`.
    pub fn custom_fields(
        mut self,
        custom_fields: impl Into<Vec<CreateCheckoutSessionCustomFields>>,
    ) -> Self {
        self.inner.custom_fields = Some(custom_fields.into());
        self
    }
    /// Display additional text for your customers using custom text.
    /// You can't set this parameter if `ui_mode` is `custom`.
    pub fn custom_text(mut self, custom_text: impl Into<CreateCheckoutSessionCustomText>) -> Self {
        self.inner.custom_text = Some(custom_text.into());
        self
    }
    /// ID of an existing Customer, if one exists.
    /// In `payment` mode, the customer’s most recently saved card.
    /// payment method will be used to prefill the email, name, card details, and billing address
    /// on the Checkout page.
    /// In `subscription` mode, the customer’s [default payment method](https://docs.stripe.com/api/customers/update#update_customer-invoice_settings-default_payment_method).
    /// will be used if it’s a card, otherwise the most recently saved card will be used.
    /// A valid billing address, billing name and billing email are required on the payment method for Checkout to prefill the customer's card details.
    ///
    /// If the Customer already has a valid [email](https://docs.stripe.com/api/customers/object#customer_object-email) set, the email will be prefilled and not editable in Checkout.
    /// If the Customer does not have a valid `email`, Checkout will set the email entered during the session on the Customer.
    ///
    /// If blank for Checkout Sessions in `subscription` mode or with `customer_creation` set as `always` in `payment` mode, Checkout will create a new Customer object based on information provided during the payment flow.
    ///
    /// You can set [`payment_intent_data.setup_future_usage`](https://docs.stripe.com/api/checkout/sessions/create#create_checkout_session-payment_intent_data-setup_future_usage) to have Checkout automatically attach the payment method to the Customer you pass in for future reuse.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// ID of an existing Account, if one exists. Has the same behavior as `customer`.
    pub fn customer_account(mut self, customer_account: impl Into<String>) -> Self {
        self.inner.customer_account = Some(customer_account.into());
        self
    }
    /// Configure whether a Checkout Session creates a [Customer](https://docs.stripe.com/api/customers) during Session confirmation.
    ///
    /// When a Customer is not created, you can still retrieve email, address, and other customer data entered in Checkout.
    /// with [customer_details](https://docs.stripe.com/api/checkout/sessions/object#checkout_session_object-customer_details).
    ///
    /// Sessions that don't create Customers instead are grouped by [guest customers](https://docs.stripe.com/payments/checkout/guest-customers).
    /// in the Dashboard.
    /// Promotion codes limited to first time customers will return invalid for these Sessions.
    ///
    /// Can only be set in `payment` and `setup` mode.
    pub fn customer_creation(
        mut self,
        customer_creation: impl Into<stripe_shared::PaymentLinkCustomerCreation>,
    ) -> Self {
        self.inner.customer_creation = Some(customer_creation.into());
        self
    }
    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file. To access information about the customer once a session is
    /// complete, use the `customer` field.
    pub fn customer_email(mut self, customer_email: impl Into<String>) -> Self {
        self.inner.customer_email = Some(customer_email.into());
        self
    }
    /// Controls what fields on Customer can be updated by the Checkout Session.
    /// Can only be provided when `customer` is provided.
    pub fn customer_update(
        mut self,
        customer_update: impl Into<CreateCheckoutSessionCustomerUpdate>,
    ) -> Self {
        self.inner.customer_update = Some(customer_update.into());
        self
    }
    /// The coupon or promotion code to apply to this Session. Currently, only up to one may be specified.
    pub fn discounts(mut self, discounts: impl Into<Vec<CreateCheckoutSessionDiscounts>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// A list of the types of payment methods (e.g., `card`) that should be excluded from this Checkout Session.
    /// This should only be used when payment methods for this Checkout Session are managed through the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    pub fn excluded_payment_method_types(
        mut self,
        excluded_payment_method_types: impl Into<Vec<stripe_shared::ExcludedPaymentMethodTypes>>,
    ) -> Self {
        self.inner.excluded_payment_method_types = Some(excluded_payment_method_types.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The Epoch time in seconds at which the Checkout Session will expire.
    /// It can be anywhere from 30 minutes to 24 hours after Checkout Session creation.
    /// By default, this value is 24 hours from creation.
    pub fn expires_at(mut self, expires_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.expires_at = Some(expires_at.into());
        self
    }
    /// The integration identifier for this Checkout Session.
    /// Multiple Checkout Sessions can have the same integration identifier.
    pub fn integration_identifier(mut self, integration_identifier: impl Into<String>) -> Self {
        self.inner.integration_identifier = Some(integration_identifier.into());
        self
    }
    /// Generate a post-purchase Invoice for one-time payments.
    pub fn invoice_creation(
        mut self,
        invoice_creation: impl Into<CreateCheckoutSessionInvoiceCreation>,
    ) -> Self {
        self.inner.invoice_creation = Some(invoice_creation.into());
        self
    }
    /// A list of items the customer is purchasing.
    /// Use this parameter to pass one-time or recurring [Prices](https://docs.stripe.com/api/prices).
    /// The parameter is required for `payment` and `subscription` mode.
    ///
    /// For `payment` mode, there is a maximum of 100 line items, however it is recommended to consolidate line items if there are more than a few dozen.
    ///
    /// For `subscription` mode, there is a maximum of 20 line items with recurring Prices and 20 line items with one-time Prices.
    /// Line items with one-time Prices will be on the initial invoice only.
    pub fn line_items(
        mut self,
        line_items: impl Into<Vec<CreateCheckoutSessionLineItems>>,
    ) -> Self {
        self.inner.line_items = Some(line_items.into());
        self
    }
    /// The IETF language tag of the locale Checkout is displayed in.
    /// If blank or `auto`, the browser's locale is used.
    pub fn locale(mut self, locale: impl Into<stripe_shared::CheckoutSessionLocale>) -> Self {
        self.inner.locale = Some(locale.into());
        self
    }
    /// Settings for Managed Payments for this Checkout Session and resulting [PaymentIntents](/api/payment_intents/object), [Invoices](/api/invoices/object), and [Subscriptions](/api/subscriptions/object).
    pub fn managed_payments(
        mut self,
        managed_payments: impl Into<CreateCheckoutSessionManagedPayments>,
    ) -> Self {
        self.inner.managed_payments = Some(managed_payments.into());
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
    /// The mode of the Checkout Session.
    /// Pass `subscription` if the Checkout Session includes at least one recurring item.
    pub fn mode(mut self, mode: impl Into<stripe_shared::CheckoutSessionMode>) -> Self {
        self.inner.mode = Some(mode.into());
        self
    }
    /// Controls name collection settings for the session.
    ///
    /// You can configure Checkout to collect your customers' business names, individual names, or both.
    /// Each name field can be either required or optional.
    ///
    /// If a [Customer](https://docs.stripe.com/api/customers) is created or provided, the names can be saved to the Customer object as well.
    pub fn name_collection(
        mut self,
        name_collection: impl Into<CreateCheckoutSessionNameCollection>,
    ) -> Self {
        self.inner.name_collection = Some(name_collection.into());
        self
    }
    /// A list of optional items the customer can add to their order at checkout.
    /// Use this parameter to pass one-time or recurring [Prices](https://docs.stripe.com/api/prices).
    ///
    /// There is a maximum of 10 optional items allowed on a Checkout Session, and the existing limits on the number of line items allowed on a Checkout Session apply to the combined number of line items and optional items.
    ///
    /// For `payment` mode, there is a maximum of 100 combined line items and optional items, however it is recommended to consolidate items if there are more than a few dozen.
    ///
    /// For `subscription` mode, there is a maximum of 20 line items and optional items with recurring Prices and 20 line items and optional items with one-time Prices.
    ///
    /// You can't set this parameter if `ui_mode` is `custom`.
    pub fn optional_items(
        mut self,
        optional_items: impl Into<Vec<CreateCheckoutSessionOptionalItems>>,
    ) -> Self {
        self.inner.optional_items = Some(optional_items.into());
        self
    }
    /// Where the user is coming from.
    /// This informs the optimizations that are applied to the session.
    /// You can't set this parameter if `ui_mode` is `elements`.
    pub fn origin_context(
        mut self,
        origin_context: impl Into<stripe_shared::OriginContext>,
    ) -> Self {
        self.inner.origin_context = Some(origin_context.into());
        self
    }
    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    pub fn payment_intent_data(
        mut self,
        payment_intent_data: impl Into<CreateCheckoutSessionPaymentIntentData>,
    ) -> Self {
        self.inner.payment_intent_data = Some(payment_intent_data.into());
        self
    }
    /// Specify whether Checkout should collect a payment method.
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.
    /// This may occur if the Checkout Session includes a free trial or a discount.
    ///
    /// Can only be set in `subscription` mode. Defaults to `always`.
    ///
    /// If you'd like information on how to collect a payment method outside of Checkout, read the guide on configuring [subscriptions with a free trial](https://docs.stripe.com/payments/checkout/free-trials).
    pub fn payment_method_collection(
        mut self,
        payment_method_collection: impl Into<stripe_shared::PaymentLinkCustomerCreation>,
    ) -> Self {
        self.inner.payment_method_collection = Some(payment_method_collection.into());
        self
    }
    /// The ID of the payment method configuration to use with this Checkout session.
    pub fn payment_method_configuration(
        mut self,
        payment_method_configuration: impl Into<String>,
    ) -> Self {
        self.inner.payment_method_configuration = Some(payment_method_configuration.into());
        self
    }
    /// This parameter allows you to set some attributes on the payment method created during a Checkout session.
    pub fn payment_method_data(
        mut self,
        payment_method_data: impl Into<CreateCheckoutSessionPaymentMethodData>,
    ) -> Self {
        self.inner.payment_method_data = Some(payment_method_data.into());
        self
    }
    /// Payment-method-specific configuration.
    pub fn payment_method_options(
        mut self,
        payment_method_options: impl Into<CreateCheckoutSessionPaymentMethodOptions>,
    ) -> Self {
        self.inner.payment_method_options = Some(payment_method_options.into());
        self
    }
    /// A list of the types of payment methods (e.g., `card`) this Checkout Session can accept.
    ///
    /// You can omit this attribute to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    /// See [Dynamic Payment Methods](https://docs.stripe.com/payments/payment-methods/integration-options#using-dynamic-payment-methods) for more details.
    ///
    /// Read more about the supported payment methods and their requirements in our [payment
    /// method details guide](/docs/payments/checkout/payment-methods).
    ///
    /// If multiple payment methods are passed, Checkout will dynamically reorder them to
    /// prioritize the most relevant payment methods based on the customer's location and
    /// other characteristics.
    pub fn payment_method_types(
        mut self,
        payment_method_types: impl Into<Vec<CreateCheckoutSessionPaymentMethodTypes>>,
    ) -> Self {
        self.inner.payment_method_types = Some(payment_method_types.into());
        self
    }
    /// This property is used to set up permissions for various actions (e.g., update) on the CheckoutSession object.
    /// Can only be set when creating `embedded` or `custom` sessions.
    ///
    /// For specific permissions, please refer to their dedicated subsections, such as `permissions.update_shipping_details`.
    pub fn permissions(mut self, permissions: impl Into<CreateCheckoutSessionPermissions>) -> Self {
        self.inner.permissions = Some(permissions.into());
        self
    }
    /// Controls phone number collection settings for the session.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts
    /// before using this feature.
    /// Learn more about [collecting phone numbers with Checkout](https://docs.stripe.com/payments/checkout/phone-numbers).
    pub fn phone_number_collection(
        mut self,
        phone_number_collection: impl Into<CreateCheckoutSessionPhoneNumberCollection>,
    ) -> Self {
        self.inner.phone_number_collection = Some(phone_number_collection.into());
        self
    }
    /// This parameter applies to `ui_mode: embedded_page`.
    /// Learn more about the [redirect behavior](https://docs.stripe.com/payments/checkout/custom-success-page?payment-ui=embedded-form) of embedded sessions.
    /// Defaults to `always`.
    pub fn redirect_on_completion(
        mut self,
        redirect_on_completion: impl Into<stripe_shared::RedirectOnCompletion>,
    ) -> Self {
        self.inner.redirect_on_completion = Some(redirect_on_completion.into());
        self
    }
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the
    /// payment method's app or site.
    /// This parameter is required if `ui_mode` is `embedded_page` or `elements`.
    /// and redirect-based payment methods are enabled on the session.
    pub fn return_url(mut self, return_url: impl Into<String>) -> Self {
        self.inner.return_url = Some(return_url.into());
        self
    }
    /// Controls saved payment method settings for the session.
    /// Only available in `payment` and `subscription` mode.
    pub fn saved_payment_method_options(
        mut self,
        saved_payment_method_options: impl Into<CreateCheckoutSessionSavedPaymentMethodOptions>,
    ) -> Self {
        self.inner.saved_payment_method_options = Some(saved_payment_method_options.into());
        self
    }
    /// A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in `setup` mode.
    pub fn setup_intent_data(
        mut self,
        setup_intent_data: impl Into<CreateCheckoutSessionSetupIntentData>,
    ) -> Self {
        self.inner.setup_intent_data = Some(setup_intent_data.into());
        self
    }
    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    pub fn shipping_address_collection(
        mut self,
        shipping_address_collection: impl Into<CreateCheckoutSessionShippingAddressCollection>,
    ) -> Self {
        self.inner.shipping_address_collection = Some(shipping_address_collection.into());
        self
    }
    /// The shipping rate options to apply to this Session. Up to a maximum of 5.
    pub fn shipping_options(
        mut self,
        shipping_options: impl Into<Vec<ShippingOptionParams>>,
    ) -> Self {
        self.inner.shipping_options = Some(shipping_options.into());
        self
    }
    /// Describes the type of transaction being performed by Checkout in order
    /// to customize relevant text on the page, such as the submit button.
    ///  `submit_type` can only be specified on Checkout Sessions in
    /// `payment` or `subscription` mode. If blank or `auto`, `pay` is used.
    /// You can't set this parameter if `ui_mode` is `elements`.
    pub fn submit_type(mut self, submit_type: impl Into<stripe_shared::SubmitType>) -> Self {
        self.inner.submit_type = Some(submit_type.into());
        self
    }
    /// A subset of parameters to be passed to subscription creation for Checkout Sessions in `subscription` mode.
    pub fn subscription_data(
        mut self,
        subscription_data: impl Into<CreateCheckoutSessionSubscriptionData>,
    ) -> Self {
        self.inner.subscription_data = Some(subscription_data.into());
        self
    }
    /// The URL to which Stripe should send customers when payment or setup
    /// is complete.
    /// This parameter is not allowed if ui_mode is `embedded_page` or `elements`. If you'd like to use
    /// information from the successful Checkout Session on your page, read the
    /// guide on [customizing your success page](https://docs.stripe.com/payments/checkout/custom-success-page).
    pub fn success_url(mut self, success_url: impl Into<String>) -> Self {
        self.inner.success_url = Some(success_url.into());
        self
    }
    /// Controls tax ID collection during checkout.
    pub fn tax_id_collection(
        mut self,
        tax_id_collection: impl Into<CreateCheckoutSessionTaxIdCollection>,
    ) -> Self {
        self.inner.tax_id_collection = Some(tax_id_collection.into());
        self
    }
    /// The UI mode of the Session. Defaults to `hosted_page`.
    pub fn ui_mode(mut self, ui_mode: impl Into<stripe_shared::UiMode>) -> Self {
        self.inner.ui_mode = Some(ui_mode.into());
        self
    }
    /// Wallet-specific configuration.
    pub fn wallet_options(
        mut self,
        wallet_options: impl Into<CreateCheckoutSessionWalletOptions>,
    ) -> Self {
        self.inner.wallet_options = Some(wallet_options.into());
        self
    }
}
impl Default for CreateCheckoutSession {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateCheckoutSession {
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

impl StripeRequest for CreateCheckoutSession {
    type Output = stripe_shared::CheckoutSession;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/checkout/sessions").form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateCheckoutSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    collected_information: Option<UpdateCheckoutSessionCollectedInformation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_items: Option<Vec<UpdateCheckoutSessionLineItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Vec<ShippingOptionParams>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateCheckoutSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateCheckoutSessionBuilder").finish_non_exhaustive()
    }
}
impl UpdateCheckoutSessionBuilder {
    fn new() -> Self {
        Self {
            collected_information: None,
            expand: None,
            line_items: None,
            metadata: None,
            shipping_options: None,
        }
    }
}
/// Information about the customer collected within the Checkout Session.
/// Can only be set when updating `embedded` or `custom` sessions.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateCheckoutSessionCollectedInformation {
    /// The shipping details to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<UpdateCheckoutSessionCollectedInformationShippingDetails>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateCheckoutSessionCollectedInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateCheckoutSessionCollectedInformation").finish_non_exhaustive()
    }
}
impl UpdateCheckoutSessionCollectedInformation {
    pub fn new() -> Self {
        Self { shipping_details: None }
    }
}
impl Default for UpdateCheckoutSessionCollectedInformation {
    fn default() -> Self {
        Self::new()
    }
}
/// The shipping details to apply to this Session.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateCheckoutSessionCollectedInformationShippingDetails {
    /// The address of the customer
    pub address: UpdateCheckoutSessionCollectedInformationShippingDetailsAddress,
    /// The name of customer
    pub name: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateCheckoutSessionCollectedInformationShippingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateCheckoutSessionCollectedInformationShippingDetails")
            .finish_non_exhaustive()
    }
}
impl UpdateCheckoutSessionCollectedInformationShippingDetails {
    pub fn new(
        address: impl Into<UpdateCheckoutSessionCollectedInformationShippingDetailsAddress>,
        name: impl Into<String>,
    ) -> Self {
        Self { address: address.into(), name: name.into() }
    }
}
/// The address of the customer
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateCheckoutSessionCollectedInformationShippingDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Address line 1, such as the street, PO Box, or company name.
    pub line1: String,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateCheckoutSessionCollectedInformationShippingDetailsAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateCheckoutSessionCollectedInformationShippingDetailsAddress")
            .finish_non_exhaustive()
    }
}
impl UpdateCheckoutSessionCollectedInformationShippingDetailsAddress {
    pub fn new(country: impl Into<String>, line1: impl Into<String>) -> Self {
        Self {
            city: None,
            country: country.into(),
            line1: line1.into(),
            line2: None,
            postal_code: None,
            state: None,
        }
    }
}
/// A list of items the customer is purchasing.
///
/// When updating line items, you must retransmit the entire array of line items.
///
/// To retain an existing line item, specify its `id`.
///
/// To update an existing line item, specify its `id` along with the new values of the fields to update.
///
/// To add a new line item, specify one of `price` or `price_data` and `quantity`.
///
/// To remove an existing line item, omit the line item's ID from the retransmitted array.
///
/// To reorder a line item, specify it at the desired position in the retransmitted array.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateCheckoutSessionLineItems {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<UpdateCheckoutSessionLineItemsAdjustableQuantity>,
    /// ID of an existing line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The ID of the [Price](https://docs.stripe.com/api/prices).
    /// One of `price` or `price_data` is required when creating a new line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
    /// One of `price` or `price_data` is required when creating a new line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<PriceDataWithProductData>,
    /// The quantity of the line item being purchased.
    /// Quantity should not be defined when `recurring.usage_type=metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The [tax rates](https://docs.stripe.com/api/tax_rates) which apply to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateCheckoutSessionLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateCheckoutSessionLineItems").finish_non_exhaustive()
    }
}
impl UpdateCheckoutSessionLineItems {
    pub fn new() -> Self {
        Self {
            adjustable_quantity: None,
            id: None,
            metadata: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for UpdateCheckoutSessionLineItems {
    fn default() -> Self {
        Self::new()
    }
}
/// When set, provides configuration for this item’s quantity to be adjusted by the customer during Checkout.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateCheckoutSessionLineItemsAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any positive integer.
    /// Setting to false will remove any previously specified constraints on quantity.
    pub enabled: bool,
    /// The maximum quantity the customer can purchase for the Checkout Session.
    /// By default this value is 99.
    /// You can specify a value up to 999999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum quantity the customer must purchase for the Checkout Session.
    /// By default this value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateCheckoutSessionLineItemsAdjustableQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateCheckoutSessionLineItemsAdjustableQuantity").finish_non_exhaustive()
    }
}
impl UpdateCheckoutSessionLineItemsAdjustableQuantity {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), maximum: None, minimum: None }
    }
}
/// Updates a Checkout Session object.
///
/// Related guide: <a href="/payments/advanced/dynamic-updates">Dynamically update a Checkout Session</a>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateCheckoutSession {
    inner: UpdateCheckoutSessionBuilder,
    session: stripe_shared::CheckoutSessionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateCheckoutSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateCheckoutSession").finish_non_exhaustive()
    }
}
impl UpdateCheckoutSession {
    /// Construct a new `UpdateCheckoutSession`.
    pub fn new(session: impl Into<stripe_shared::CheckoutSessionId>) -> Self {
        Self { session: session.into(), inner: UpdateCheckoutSessionBuilder::new() }
    }
    /// Information about the customer collected within the Checkout Session.
    /// Can only be set when updating `embedded` or `custom` sessions.
    pub fn collected_information(
        mut self,
        collected_information: impl Into<UpdateCheckoutSessionCollectedInformation>,
    ) -> Self {
        self.inner.collected_information = Some(collected_information.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A list of items the customer is purchasing.
    ///
    /// When updating line items, you must retransmit the entire array of line items.
    ///
    /// To retain an existing line item, specify its `id`.
    ///
    /// To update an existing line item, specify its `id` along with the new values of the fields to update.
    ///
    /// To add a new line item, specify one of `price` or `price_data` and `quantity`.
    ///
    /// To remove an existing line item, omit the line item's ID from the retransmitted array.
    ///
    /// To reorder a line item, specify it at the desired position in the retransmitted array.
    pub fn line_items(
        mut self,
        line_items: impl Into<Vec<UpdateCheckoutSessionLineItems>>,
    ) -> Self {
        self.inner.line_items = Some(line_items.into());
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
    /// The shipping rate options to apply to this Session. Up to a maximum of 5.
    pub fn shipping_options(
        mut self,
        shipping_options: impl Into<Vec<ShippingOptionParams>>,
    ) -> Self {
        self.inner.shipping_options = Some(shipping_options.into());
        self
    }
}
impl UpdateCheckoutSession {
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

impl StripeRequest for UpdateCheckoutSession {
    type Output = stripe_shared::CheckoutSession;

    fn build(&self) -> RequestBuilder {
        let session = &self.session;
        RequestBuilder::new(StripeMethod::Post, format!("/checkout/sessions/{session}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ExpireCheckoutSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ExpireCheckoutSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExpireCheckoutSessionBuilder").finish_non_exhaustive()
    }
}
impl ExpireCheckoutSessionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// A Checkout Session can be expired when it is in one of these statuses: `open`
///
/// After it expires, a customer can’t complete a Checkout Session and customers loading the Checkout Session see a message saying the Checkout Session is expired.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ExpireCheckoutSession {
    inner: ExpireCheckoutSessionBuilder,
    session: stripe_shared::CheckoutSessionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ExpireCheckoutSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExpireCheckoutSession").finish_non_exhaustive()
    }
}
impl ExpireCheckoutSession {
    /// Construct a new `ExpireCheckoutSession`.
    pub fn new(session: impl Into<stripe_shared::CheckoutSessionId>) -> Self {
        Self { session: session.into(), inner: ExpireCheckoutSessionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ExpireCheckoutSession {
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

impl StripeRequest for ExpireCheckoutSession {
    type Output = stripe_shared::CheckoutSession;

    fn build(&self) -> RequestBuilder {
        let session = &self.session;
        RequestBuilder::new(StripeMethod::Post, format!("/checkout/sessions/{session}/expire"))
            .form(&self.inner)
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ProductData {
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
impl std::fmt::Debug for ProductData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ProductData").finish_non_exhaustive()
    }
}
impl ProductData {
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RecurringAdhoc {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: stripe_shared::PlanInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RecurringAdhoc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RecurringAdhoc").finish_non_exhaustive()
    }
}
impl RecurringAdhoc {
    pub fn new(interval: impl Into<stripe_shared::PlanInterval>) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PaymentMethodOptionsParam {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsParam").finish_non_exhaustive()
    }
}
impl PaymentMethodOptionsParam {
    pub fn new() -> Self {
        Self { capture_method: None, setup_future_usage: None }
    }
}
impl Default for PaymentMethodOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeliveryEstimateBound {
    /// A unit of time.
    pub unit: stripe_shared::Unit,
    /// Must be greater than 0.
    pub value: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeliveryEstimateBound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeliveryEstimateBound").finish_non_exhaustive()
    }
}
impl DeliveryEstimateBound {
    pub fn new(unit: impl Into<stripe_shared::Unit>, value: impl Into<i64>) -> Self {
        Self { unit: unit.into(), value: value.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CurrencyOption {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CurrencyOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CurrencyOption").finish_non_exhaustive()
    }
}
impl CurrencyOption {
    pub fn new(amount: impl Into<i64>) -> Self {
        Self { amount: amount.into(), tax_behavior: None }
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PriceDataWithProductData {
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
    pub product_data: Option<ProductData>,
    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<RecurringAdhoc>,
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
impl std::fmt::Debug for PriceDataWithProductData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PriceDataWithProductData").finish_non_exhaustive()
    }
}
impl PriceDataWithProductData {
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeliveryEstimate {
    /// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<DeliveryEstimateBound>,
    /// The lower bound of the estimated range. If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<DeliveryEstimateBound>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeliveryEstimate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeliveryEstimate").finish_non_exhaustive()
    }
}
impl DeliveryEstimate {
    pub fn new() -> Self {
        Self { maximum: None, minimum: None }
    }
}
impl Default for DeliveryEstimate {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<std::collections::HashMap<stripe_types::Currency, CurrencyOption>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FixedAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FixedAmount").finish_non_exhaustive()
    }
}
impl FixedAmount {
    pub fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self { amount: amount.into(), currency: currency.into(), currency_options: None }
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct MethodParams {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<DeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    pub display_name: String,
    /// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<FixedAmount>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
    /// A [tax code](https://docs.stripe.com/tax/tax-categories) ID.
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// The type of calculation to use on the shipping rate.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<stripe_shared::ShippingRateType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MethodParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MethodParams").finish_non_exhaustive()
    }
}
impl MethodParams {
    pub fn new(display_name: impl Into<String>) -> Self {
        Self {
            delivery_estimate: None,
            display_name: display_name.into(),
            fixed_amount: None,
            metadata: None,
            tax_behavior: None,
            tax_code: None,
            type_: None,
        }
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ShippingOptionParams {
    /// The ID of the Shipping Rate to use for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
    /// Parameters to be passed to Shipping Rate creation for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<MethodParams>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShippingOptionParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShippingOptionParams").finish_non_exhaustive()
    }
}
impl ShippingOptionParams {
    pub fn new() -> Self {
        Self { shipping_rate: None, shipping_rate_data: None }
    }
}
impl Default for ShippingOptionParams {
    fn default() -> Self {
        Self::new()
    }
}
/// Only return the Checkout Sessions matching the given status.
pub type ListCheckoutSessionStatus = stripe_shared::CheckoutSessionStatus;
/// Type of the account referenced in the request.
pub type CreateCheckoutSessionAutomaticTaxLiabilityType = stripe_shared::CreateInvoiceIssuerType;
/// Specify whether Checkout should collect the customer's billing address. Defaults to `auto`.
pub type CreateCheckoutSessionBillingAddressCollection = stripe_shared::BillingAddressCollection;
/// The border style for the Checkout Session.
pub type CreateCheckoutSessionBrandingSettingsBorderStyle = stripe_shared::BorderStyle;
/// The type of image for the icon. Must be one of `file` or `url`.
pub type CreateCheckoutSessionBrandingSettingsIconType =
    stripe_shared::CreateCheckoutSessionBrandingSettingsIconType;
/// The type of image for the logo. Must be one of `file` or `url`.
pub type CreateCheckoutSessionBrandingSettingsLogoType =
    stripe_shared::CreateCheckoutSessionBrandingSettingsIconType;
/// Determines the position and visibility of the payment method reuse agreement in the UI.
/// When set to `auto`, Stripe's.
/// defaults will be used.
/// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
pub type CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition =
    stripe_shared::Position;
/// If set to `auto`, enables the collection of customer consent for promotional communications.
/// The Checkout.
/// Session will determine whether to display an option to opt into promotional communication
/// from the merchant depending on the customer's locale.
/// Only available to US merchants and US customers.
pub type CreateCheckoutSessionConsentCollectionPromotions = stripe_shared::Promotions;
/// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
/// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
pub type CreateCheckoutSessionConsentCollectionTermsOfService = stripe_shared::TermsOfService;
/// The type of the label.
pub type CreateCheckoutSessionCustomFieldsLabelType =
    stripe_shared::CreatePaymentLinkCustomFieldsLabelType;
/// The type of the field.
pub type CreateCheckoutSessionCustomFieldsType = stripe_shared::CreatePaymentLinkCustomFieldsType;
/// Configure whether a Checkout Session creates a [Customer](https://docs.stripe.com/api/customers) during Session confirmation.
///
/// When a Customer is not created, you can still retrieve email, address, and other customer data entered in Checkout.
/// with [customer_details](https://docs.stripe.com/api/checkout/sessions/object#checkout_session_object-customer_details).
///
/// Sessions that don't create Customers instead are grouped by [guest customers](https://docs.stripe.com/payments/checkout/guest-customers).
/// in the Dashboard.
/// Promotion codes limited to first time customers will return invalid for these Sessions.
///
/// Can only be set in `payment` and `setup` mode.
pub type CreateCheckoutSessionCustomerCreation = stripe_shared::PaymentLinkCustomerCreation;
/// Describes whether Checkout saves the billing address onto `customer.address`.
/// To always collect a full billing address, use `billing_address_collection`. Defaults to `never`.
pub type CreateCheckoutSessionCustomerUpdateAddress =
    stripe_shared::CheckoutLinkWalletOptionsDisplay;
/// Describes whether Checkout saves the name onto `customer.name`. Defaults to `never`.
pub type CreateCheckoutSessionCustomerUpdateName = stripe_shared::CheckoutLinkWalletOptionsDisplay;
/// Describes whether Checkout saves shipping information onto `customer.shipping`.
/// To collect shipping information, use `shipping_address_collection`. Defaults to `never`.
pub type CreateCheckoutSessionCustomerUpdateShipping =
    stripe_shared::CheckoutLinkWalletOptionsDisplay;
/// A list of the types of payment methods (e.g., `card`) that should be excluded from this Checkout Session.
/// This should only be used when payment methods for this Checkout Session are managed through the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
pub type CreateCheckoutSessionExcludedPaymentMethodTypes =
    stripe_shared::ExcludedPaymentMethodTypes;
/// Type of the account referenced in the request.
pub type CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType =
    stripe_shared::CreateInvoiceIssuerType;
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
pub type CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay =
    stripe_shared::AmountTaxDisplay;
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
pub type CreateCheckoutSessionLineItemsPriceDataRecurringInterval = stripe_shared::PlanInterval;
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
pub type CreateCheckoutSessionLineItemsPriceDataTaxBehavior = stripe_shared::PriceTaxBehavior;
/// The IETF language tag of the locale Checkout is displayed in.
/// If blank or `auto`, the browser's locale is used.
pub type CreateCheckoutSessionLocale = stripe_shared::CheckoutSessionLocale;
/// The mode of the Checkout Session.
/// Pass `subscription` if the Checkout Session includes at least one recurring item.
pub type CreateCheckoutSessionMode = stripe_shared::CheckoutSessionMode;
/// Where the user is coming from.
/// This informs the optimizations that are applied to the session.
/// You can't set this parameter if `ui_mode` is `elements`.
pub type CreateCheckoutSessionOriginContext = stripe_shared::OriginContext;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentIntentDataCaptureMethod = stripe_shared::CaptureMethod;
/// Indicates that you intend to [make future payments](https://docs.stripe.com/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
///
/// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
///
/// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
///
/// If a Customer has been provided or Checkout creates a new Customer, Checkout will attach the payment method to the Customer.
///
/// If Checkout does not create a Customer, the payment method is not attached to a Customer.
/// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.
///
/// When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
pub type CreateCheckoutSessionPaymentIntentDataSetupFutureUsage =
    stripe_shared::CreateSetupIntentUsage;
/// Specify whether Checkout should collect a payment method.
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.
/// This may occur if the Checkout Session includes a free trial or a discount.
///
/// Can only be set in `subscription` mode. Defaults to `always`.
///
/// If you'd like information on how to collect a payment method outside of Checkout, read the guide on configuring [subscriptions with a free trial](https://docs.stripe.com/payments/checkout/free-trials).
pub type CreateCheckoutSessionPaymentMethodCollection = stripe_shared::PaymentLinkCustomerCreation;
/// Allow redisplay will be set on the payment method on confirmation and indicates whether this payment method can be shown again to the customer in a checkout flow.
/// Only set this field if you wish to override the allow_redisplay value determined by Checkout.
pub type CreateCheckoutSessionPaymentMethodDataAllowRedisplay = stripe_shared::CardAllowRedisplay;
/// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
/// Must be a [supported currency](https://stripe.com/docs/currencies).
/// This is only accepted for Checkout Sessions in `setup` mode.
pub type CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency = stripe_shared::Currency;
/// List of Stripe products where this mandate can be selected automatically.
/// Only usable in `setup` mode.
pub type CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor =
    stripe_shared::DefaultFor;
/// Payment schedule for the mandate.
pub type CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule =
    stripe_shared::PaymentSchedule;
/// Transaction type of the mandate.
pub type CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType =
    stripe_shared::TransactionType;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage =
    stripe_shared::SetupFutureUsage;
/// Verification method for the intent
pub type CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod =
    stripe_shared::VerificationMethod;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsAffirmCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpayCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsAlmaCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsAmazonPayCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsAmazonPaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsPixSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage =
    stripe_shared::SetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsBillieCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage =
    stripe_shared::SetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsCardCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Request ability to [capture beyond the standard authorization validity window](/payments/extended-authorization) for this CheckoutSession.
pub type CreateCheckoutSessionPaymentMethodOptionsCardRequestExtendedAuthorization =
    stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture;
/// Request ability to [increment the authorization](/payments/incremental-authorization) for this CheckoutSession.
pub type CreateCheckoutSessionPaymentMethodOptionsCardRequestIncrementalAuthorization =
    stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture;
/// Request ability to make [multiple captures](/payments/multicapture) for this CheckoutSession.
pub type CreateCheckoutSessionPaymentMethodOptionsCardRequestMulticapture =
    stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture;
/// Request ability to [overcapture](/payments/overcapture) for this CheckoutSession.
pub type CreateCheckoutSessionPaymentMethodOptionsCardRequestOvercapture =
    stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture;
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// If not provided, this value defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
pub type CreateCheckoutSessionPaymentMethodOptionsCardRequestThreeDSecure =
    stripe_shared::RequestThreeDSecure;
/// The card brands to block.
/// If a customer enters or selects a card belonging to a blocked brand, they can't complete the payment.
pub type CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsBrandsBlocked =
    stripe_shared::BrandsBlocked;
/// Card funding types to block for this Checkout Session.
/// Supported values are `credit`, `debit`, and `prepaid`.
pub type CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsFundingTypesBlocked =
    stripe_shared::FundingTypesBlocked;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage =
    stripe_shared::CreateSetupIntentUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsCashappCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage =
    stripe_shared::SetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsCryptoSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// List of address types that should be returned in the financial_addresses response.
/// If not specified, all valid types will be returned.
///
/// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
pub type CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes =
    stripe_shared::RequestedAddressTypes;
/// The list of bank transfer types that this PaymentIntent is allowed to use for funding.
pub type CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType =
    stripe_shared::CreateFundingInstructionsCustomerBankTransferType;
/// The funding method type to be used when there are not enough funds in the customer balance.
/// Permitted values include: `bank_transfer`.
pub type CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType =
    stripe_shared::FundingType;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsDemoPaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsPixSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsKakaoPayCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsKakaoPaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsPixSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsKlarnaCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Unit of time between subscription charges.
pub type CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval =
    stripe_shared::PlanInterval;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsKrCardCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsKrCardSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsPixSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsLinkCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsPixSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsMobilepayCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsMobilepaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsMultibancoSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsNaverPayCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsNaverPaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsPixSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsPaycoCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsPaycoSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// [Preferred locale](https://docs.stripe.com/payments/paypal/supported-locales) of the PayPal checkout page that the customer is redirected to.
pub type CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale =
    stripe_shared::PreferredLocale;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
///
/// If you've already set `setup_future_usage` and you're performing a request using a publishable key, you can only update the value from `on_session` to `off_session`.
pub type CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsPixSetupFutureUsage;
/// The type of amount that will be collected.
/// The amount charged must be exact or up to the value of `amount` param for `fixed` or `maximum` type respectively.
/// Defaults to `maximum`.
pub type CreateCheckoutSessionPaymentMethodOptionsPaytoMandateOptionsAmountType =
    stripe_shared::AmountType;
/// The periodicity at which payments will be collected. Defaults to `adhoc`.
pub type CreateCheckoutSessionPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule =
    stripe_shared::MandatePaytoPaymentSchedule;
/// The purpose for which payments are made. Has a default value based on your merchant category code.
pub type CreateCheckoutSessionPaymentMethodOptionsPaytoMandateOptionsPurpose =
    stripe_shared::Purpose;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsPaytoSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsPixSetupFutureUsage;
/// Determines if the amount includes the IOF tax. Defaults to `never`.
pub type CreateCheckoutSessionPaymentMethodOptionsPixAmountIncludesIof =
    stripe_shared::MandatePixAmountIncludesIof;
/// Determines if the amount includes the IOF tax. Defaults to `never`.
pub type CreateCheckoutSessionPaymentMethodOptionsPixMandateOptionsAmountIncludesIof =
    stripe_shared::MandatePixAmountIncludesIof;
/// Type of amount. Defaults to `maximum`.
pub type CreateCheckoutSessionPaymentMethodOptionsPixMandateOptionsAmountType =
    stripe_shared::AmountType;
/// Schedule at which the future payments will be charged. Defaults to `monthly`.
pub type CreateCheckoutSessionPaymentMethodOptionsPixMandateOptionsPaymentSchedule =
    stripe_shared::MandatePixPaymentSchedule;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsPixSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsPixSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsRevolutPayCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsPixSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsSamsungPayCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsSamsungPaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsSatispayCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsScalapayCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage =
    stripe_shared::SetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Controls when the funds will be captured from the customer's account.
pub type CreateCheckoutSessionPaymentMethodOptionsSunbitCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsSunbitSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsTwintSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsPixSetupFutureUsage;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type CreateCheckoutSessionPaymentMethodOptionsUpiMandateOptionsAmountType =
    stripe_shared::AmountType;
pub type CreateCheckoutSessionPaymentMethodOptionsUpiSetupFutureUsage =
    stripe_shared::SetupFutureUsage;
/// The list of permissions to request.
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
pub type CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions =
    stripe_shared::Permissions;
/// List of data features that you would like to retrieve upon account creation.
pub type CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch =
    stripe_shared::Prefetch;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage =
    stripe_shared::SetupFutureUsage;
/// Verification method for the intent
pub type CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod =
    stripe_shared::CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod;
/// The client type that the end customer will pay from
pub type CreateCheckoutSessionPaymentMethodOptionsWechatPayClient = stripe_shared::Client;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
/// Determines which entity is allowed to update the shipping details.
///
/// Default is `client_only`.
/// Stripe Checkout client will automatically update the shipping details.
/// If set to `server_only`, only your server is allowed to update the shipping details.
///
/// This parameter is only supported when `ui_mode=elements`.
pub type CreateCheckoutSessionPermissionsUpdateShippingDetails =
    stripe_shared::UpdateShippingDetails;
/// This parameter applies to `ui_mode: embedded_page`.
/// Learn more about the [redirect behavior](https://docs.stripe.com/payments/checkout/custom-success-page?payment-ui=embedded-form) of embedded sessions.
/// Defaults to `always`.
pub type CreateCheckoutSessionRedirectOnCompletion = stripe_shared::RedirectOnCompletion;
/// Uses the `allow_redisplay` value of each saved payment method to filter the set presented to a returning customer.
/// By default, only saved payment methods with ’allow_redisplay: ‘always’ are shown in Checkout.
pub type CreateCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters =
    stripe_shared::CardAllowRedisplay;
/// Enable customers to choose if they wish to remove their saved payment methods. Disabled by default.
pub type CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// Enable customers to choose if they wish to save their payment method for future use.
/// Disabled by default.
pub type CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
/// shipping locations.
pub type CreateCheckoutSessionShippingAddressCollectionAllowedCountries =
    stripe_shared::AllowedCountries;
/// A unit of time.
pub type CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit =
    stripe_shared::Unit;
/// A unit of time.
pub type CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit =
    stripe_shared::Unit;
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
pub type CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior =
    stripe_shared::PriceTaxBehavior;
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
pub type CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior =
    stripe_shared::PriceTaxBehavior;
/// The type of calculation to use on the shipping rate.
pub type CreateCheckoutSessionShippingOptionsShippingRateDataType = stripe_shared::ShippingRateType;
/// Describes the type of transaction being performed by Checkout in order
/// to customize relevant text on the page, such as the submit button.
///  `submit_type` can only be specified on Checkout Sessions in
/// `payment` or `subscription` mode. If blank or `auto`, `pay` is used.
/// You can't set this parameter if `ui_mode` is `elements`.
pub type CreateCheckoutSessionSubmitType = stripe_shared::SubmitType;
/// Controls how invoices and invoice items display proration amounts and discount amounts.
pub type CreateCheckoutSessionSubscriptionDataBillingModeFlexibleProrationDiscounts =
    stripe_shared::ProrationDiscounts;
/// Controls the calculation and orchestration of prorations and invoices for subscriptions.
/// If no value is passed, the default is `flexible`.
pub type CreateCheckoutSessionSubscriptionDataBillingModeType =
    stripe_shared::CreateSubscriptionBillingModeType;
/// Type of the account referenced in the request.
pub type CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType =
    stripe_shared::CreateInvoiceIssuerType;
/// Specifies invoicing frequency. Either `day`, `week`, `month` or `year`.
pub type CreateCheckoutSessionSubscriptionDataPendingInvoiceItemIntervalInterval =
    stripe_shared::PlanInterval;
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
pub type CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod =
    stripe_shared::MissingPaymentMethod;
/// Describes whether a tax ID is required during checkout.
/// Defaults to `never`.
/// You can't set this parameter if `ui_mode` is `custom`.
pub type CreateCheckoutSessionTaxIdCollectionRequired = stripe_shared::Required;
/// The UI mode of the Session. Defaults to `hosted_page`.
pub type CreateCheckoutSessionUiMode = stripe_shared::UiMode;
/// Specifies whether Checkout should display Link as a payment option.
/// By default, Checkout will display all the supported wallets that the Checkout Session was created with.
/// This is the `auto` behavior, and it is the default choice.
pub type CreateCheckoutSessionWalletOptionsLinkDisplay =
    stripe_shared::CheckoutLinkWalletOptionsDisplay;
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
pub type UpdateCheckoutSessionLineItemsPriceDataRecurringInterval = stripe_shared::PlanInterval;
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
pub type UpdateCheckoutSessionLineItemsPriceDataTaxBehavior = stripe_shared::PriceTaxBehavior;
/// A unit of time.
pub type UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit =
    stripe_shared::Unit;
/// A unit of time.
pub type UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit =
    stripe_shared::Unit;
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
pub type UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior =
    stripe_shared::PriceTaxBehavior;
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
pub type UpdateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior =
    stripe_shared::PriceTaxBehavior;
/// The type of calculation to use on the shipping rate.
pub type UpdateCheckoutSessionShippingOptionsShippingRateDataType = stripe_shared::ShippingRateType;
/// Custom text that should be displayed after the payment confirmation button.
pub type CreateCheckoutSessionCustomTextAfterSubmit = CustomTextPositionParam;
/// Custom text that should be displayed alongside shipping address collection.
pub type CreateCheckoutSessionCustomTextShippingAddress = CustomTextPositionParam;
/// Custom text that should be displayed alongside the payment confirmation button.
pub type CreateCheckoutSessionCustomTextSubmit = CustomTextPositionParam;
/// Custom text that should be displayed in place of the default terms of service agreement text.
pub type CreateCheckoutSessionCustomTextTermsOfServiceAcceptance = CustomTextPositionParam;
/// Data used to generate a new [Product](https://docs.stripe.com/api/products) object inline.
/// One of `product` or `product_data` is required.
pub type CreateCheckoutSessionLineItemsPriceDataProductData = ProductData;
/// The recurring components of a price such as `interval` and `interval_count`.
pub type CreateCheckoutSessionLineItemsPriceDataRecurring = RecurringAdhoc;
/// contains details about the Affirm payment method options.
pub type CreateCheckoutSessionPaymentMethodOptionsAffirm = PaymentMethodOptionsParam;
/// contains details about the Afterpay Clearpay payment method options.
pub type CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay = PaymentMethodOptionsParam;
/// contains details about the Mobilepay payment method options.
pub type CreateCheckoutSessionPaymentMethodOptionsMobilepay = PaymentMethodOptionsParam;
/// contains details about the PAYCO payment method options.
pub type CreateCheckoutSessionPaymentMethodOptionsPayco = PaymentMethodOptionsParam;
/// contains details about the Samsung Pay payment method options.
pub type CreateCheckoutSessionPaymentMethodOptionsSamsungPay = PaymentMethodOptionsParam;
/// contains details about the Sunbit payment method options.
pub type CreateCheckoutSessionPaymentMethodOptionsSunbit = PaymentMethodOptionsParam;
/// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
pub type CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum =
    DeliveryEstimateBound;
/// The lower bound of the estimated range. If empty, represents no lower bound.
pub type CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum =
    DeliveryEstimateBound;
/// Shipping rates defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
pub type CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions =
    CurrencyOption;
/// Data used to generate a new [Product](https://docs.stripe.com/api/products) object inline.
/// One of `product` or `product_data` is required.
pub type UpdateCheckoutSessionLineItemsPriceDataProductData = ProductData;
/// The recurring components of a price such as `interval` and `interval_count`.
pub type UpdateCheckoutSessionLineItemsPriceDataRecurring = RecurringAdhoc;
/// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
pub type UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum =
    DeliveryEstimateBound;
/// The lower bound of the estimated range. If empty, represents no lower bound.
pub type UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum =
    DeliveryEstimateBound;
/// Shipping rates defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
pub type UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions =
    CurrencyOption;
/// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
/// One of `price` or `price_data` is required.
pub type CreateCheckoutSessionLineItemsPriceData = PriceDataWithProductData;
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
/// This will appear on CheckoutSessions.
pub type CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate = DeliveryEstimate;
/// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
pub type CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount = FixedAmount;
/// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
/// One of `price` or `price_data` is required when creating a new line item.
pub type UpdateCheckoutSessionLineItemsPriceData = PriceDataWithProductData;
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
/// This will appear on CheckoutSessions.
pub type UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate = DeliveryEstimate;
/// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
pub type UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmount = FixedAmount;
/// Parameters to be passed to Shipping Rate creation for this shipping option.
pub type CreateCheckoutSessionShippingOptionsShippingRateData = MethodParams;
/// Parameters to be passed to Shipping Rate creation for this shipping option.
pub type UpdateCheckoutSessionShippingOptionsShippingRateData = MethodParams;
/// The shipping rate options to apply to this Session. Up to a maximum of 5.
pub type CreateCheckoutSessionShippingOptions = ShippingOptionParams;
/// The shipping rate options to apply to this Session. Up to a maximum of 5.
pub type UpdateCheckoutSessionShippingOptions = ShippingOptionParams;
