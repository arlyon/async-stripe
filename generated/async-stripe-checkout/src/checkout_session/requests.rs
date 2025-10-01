use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListCheckoutSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
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
impl ListCheckoutSessionBuilder {
    fn new() -> Self {
        Self {
            created: None,
            customer: None,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListCheckoutSessionCustomerDetails {
    /// Customer's email address.
    pub email: String,
}
impl ListCheckoutSessionCustomerDetails {
    pub fn new(email: impl Into<String>) -> Self {
        Self { email: email.into() }
    }
}
/// Returns a list of Checkout Sessions.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListCheckoutSession {
    inner: ListCheckoutSessionBuilder,
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
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveCheckoutSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveCheckoutSessionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a Checkout Session object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveCheckoutSession {
    inner: RetrieveCheckoutSessionBuilder,
    session: stripe_shared::CheckoutSessionId,
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
#[derive(Clone, Debug, serde::Serialize)]
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
impl ListLineItemsCheckoutSessionBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// When retrieving a Checkout Session, there is an includable **line_items** property containing the first handful of those items.
/// There is also a URL where you can retrieve the full (paginated) list of line items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListLineItemsCheckoutSession {
    inner: ListLineItemsCheckoutSessionBuilder,
    session: stripe_shared::CheckoutSessionId,
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
#[derive(Clone, Debug, serde::Serialize)]
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
    billing_address_collection: Option<stripe_shared::CheckoutSessionBillingAddressCollection>,
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
    customer_creation: Option<CreateCheckoutSessionCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_update: Option<CreateCheckoutSessionCustomerUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<CreateCheckoutSessionDiscounts>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_payment_method_types: Option<Vec<CreateCheckoutSessionExcludedPaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_creation: Option<CreateCheckoutSessionInvoiceCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_items: Option<Vec<CreateCheckoutSessionLineItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<stripe_shared::CheckoutSessionLocale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<stripe_shared::CheckoutSessionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_collection: Option<CreateCheckoutSessionNameCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_items: Option<Vec<CreateCheckoutSessionOptionalItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_context: Option<stripe_shared::CheckoutSessionOriginContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent_data: Option<CreateCheckoutSessionPaymentIntentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_collection: Option<CreateCheckoutSessionPaymentMethodCollection>,
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
    redirect_on_completion: Option<stripe_shared::CheckoutSessionRedirectOnCompletion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saved_payment_method_options: Option<CreateCheckoutSessionSavedPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    setup_intent_data: Option<CreateCheckoutSessionSetupIntentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_address_collection: Option<CreateCheckoutSessionShippingAddressCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Vec<CreateCheckoutSessionShippingOptions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submit_type: Option<stripe_shared::CheckoutSessionSubmitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_data: Option<CreateCheckoutSessionSubscriptionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_id_collection: Option<CreateCheckoutSessionTaxIdCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_mode: Option<stripe_shared::CheckoutSessionUiMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wallet_options: Option<CreateCheckoutSessionWalletOptions>,
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
            customer_creation: None,
            customer_email: None,
            customer_update: None,
            discounts: None,
            excluded_payment_method_types: None,
            expand: None,
            expires_at: None,
            invoice_creation: None,
            line_items: None,
            locale: None,
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionAdaptivePricing {
    /// If set to `true`, Adaptive Pricing is available on [eligible sessions](https://docs.stripe.com/payments/currencies/localize-prices/adaptive-pricing?payment-ui=stripe-hosted#restrictions).
    /// Defaults to your [dashboard setting](https://dashboard.stripe.com/settings/adaptive-pricing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionAfterExpiration {
    /// Configure a Checkout Session that can be used to recover an expired session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery: Option<CreateCheckoutSessionAfterExpirationRecovery>,
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionAfterExpirationRecovery {
    /// Enables user redeemable promotion codes on the recovered Checkout Sessions. Defaults to `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    /// If `true`, a recovery URL will be generated to recover this Checkout Session if it
    /// expires before a successful transaction is completed. It will be attached to the
    /// Checkout Session object upon expiration.
    pub enabled: bool,
}
impl CreateCheckoutSessionAfterExpirationRecovery {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { allow_promotion_codes: None, enabled: enabled.into() }
    }
}
/// Settings for automatic tax lookup for this session and resulting payments, invoices, and subscriptions.
#[derive(Clone, Debug, serde::Serialize)]
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
impl CreateCheckoutSessionAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionAutomaticTaxLiabilityType,
}
impl CreateCheckoutSessionAutomaticTaxLiability {
    pub fn new(type_: impl Into<CreateCheckoutSessionAutomaticTaxLiabilityType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl CreateCheckoutSessionAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionAutomaticTaxLiabilityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionAutomaticTaxLiabilityType",
            )
        })
    }
}
/// The branding settings for the Checkout Session.
/// This parameter is not allowed if ui_mode is `custom`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionBrandingSettings {
    /// A hex color value starting with `#` representing the background color for the Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// The border style for the Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_style: Option<CreateCheckoutSessionBrandingSettingsBorderStyle>,
    /// A hex color value starting with `#` representing the button color for the Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_color: Option<String>,
    /// A string to override the business name shown on the Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The font family for the Checkout Session corresponding to one of the [supported font families](https://docs.stripe.com/payments/checkout/customization/appearance?payment-ui=stripe-hosted#font-compatibility).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<CreateCheckoutSessionBrandingSettingsFontFamily>,
    /// The icon for the Checkout Session. You cannot set both `logo` and `icon`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<CreateCheckoutSessionBrandingSettingsIcon>,
    /// The logo for the Checkout Session. You cannot set both `logo` and `icon`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<CreateCheckoutSessionBrandingSettingsLogo>,
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
/// The border style for the Checkout Session.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionBrandingSettingsBorderStyle {
    Pill,
    Rectangular,
    Rounded,
}
impl CreateCheckoutSessionBrandingSettingsBorderStyle {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionBrandingSettingsBorderStyle::*;
        match self {
            Pill => "pill",
            Rectangular => "rectangular",
            Rounded => "rounded",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionBrandingSettingsBorderStyle {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionBrandingSettingsBorderStyle::*;
        match s {
            "pill" => Ok(Pill),
            "rectangular" => Ok(Rectangular),
            "rounded" => Ok(Rounded),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionBrandingSettingsBorderStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionBrandingSettingsBorderStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionBrandingSettingsBorderStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionBrandingSettingsBorderStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionBrandingSettingsBorderStyle",
            )
        })
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
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionBrandingSettingsFontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionBrandingSettingsFontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Ok(Self::from_str(&s).unwrap())
    }
}
/// The icon for the Checkout Session. You cannot set both `logo` and `icon`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionBrandingSettingsIcon {
    /// The ID of a [File upload](https://stripe.com/docs/api/files) representing the icon.
    /// Purpose must be `business_icon`.
    /// Required if `type` is `file` and disallowed otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// The type of image for the icon. Must be one of `file` or `url`.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionBrandingSettingsIconType,
    /// The URL of the image. Required if `type` is `url` and disallowed otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl CreateCheckoutSessionBrandingSettingsIcon {
    pub fn new(type_: impl Into<CreateCheckoutSessionBrandingSettingsIconType>) -> Self {
        Self { file: None, type_: type_.into(), url: None }
    }
}
/// The type of image for the icon. Must be one of `file` or `url`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionBrandingSettingsIconType {
    File,
    Url,
}
impl CreateCheckoutSessionBrandingSettingsIconType {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionBrandingSettingsIconType::*;
        match self {
            File => "file",
            Url => "url",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionBrandingSettingsIconType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionBrandingSettingsIconType::*;
        match s {
            "file" => Ok(File),
            "url" => Ok(Url),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionBrandingSettingsIconType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionBrandingSettingsIconType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionBrandingSettingsIconType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionBrandingSettingsIconType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionBrandingSettingsIconType",
            )
        })
    }
}
/// The logo for the Checkout Session. You cannot set both `logo` and `icon`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionBrandingSettingsLogo {
    /// The ID of a [File upload](https://stripe.com/docs/api/files) representing the logo.
    /// Purpose must be `business_logo`.
    /// Required if `type` is `file` and disallowed otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// The type of image for the logo. Must be one of `file` or `url`.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionBrandingSettingsLogoType,
    /// The URL of the image. Required if `type` is `url` and disallowed otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl CreateCheckoutSessionBrandingSettingsLogo {
    pub fn new(type_: impl Into<CreateCheckoutSessionBrandingSettingsLogoType>) -> Self {
        Self { file: None, type_: type_.into(), url: None }
    }
}
/// The type of image for the logo. Must be one of `file` or `url`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionBrandingSettingsLogoType {
    File,
    Url,
}
impl CreateCheckoutSessionBrandingSettingsLogoType {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionBrandingSettingsLogoType::*;
        match self {
            File => "file",
            Url => "url",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionBrandingSettingsLogoType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionBrandingSettingsLogoType::*;
        match s {
            "file" => Ok(File),
            "url" => Ok(Url),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionBrandingSettingsLogoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionBrandingSettingsLogoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionBrandingSettingsLogoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionBrandingSettingsLogoType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionBrandingSettingsLogoType",
            )
        })
    }
}
/// Configure fields for the Checkout Session to gather active consent from customers.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionConsentCollection {
    /// Determines the display of payment method reuse agreement text in the UI.
    /// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_reuse_agreement:
        Option<CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement>,
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    /// The Checkout.
    /// Session will determine whether to display an option to opt into promotional communication
    /// from the merchant depending on the customer's locale. Only available to US merchants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<CreateCheckoutSessionConsentCollectionPromotions>,
    /// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
    /// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<CreateCheckoutSessionConsentCollectionTermsOfService>,
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement {
    /// Determines the position and visibility of the payment method reuse agreement in the UI.
    /// When set to `auto`, Stripe's.
    /// defaults will be used.
    /// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
    pub position: CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition,
}
impl CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement {
    pub fn new(
        position: impl Into<CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition>,
    ) -> Self {
        Self { position: position.into() }
    }
}
/// Determines the position and visibility of the payment method reuse agreement in the UI.
/// When set to `auto`, Stripe's.
/// defaults will be used.
/// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
}
impl CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition::*;
        match self {
            Auto => "auto",
            Hidden => "hidden",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition::*;
        match s {
            "auto" => Ok(Auto),
            "hidden" => Ok(Hidden),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition"))
    }
}
/// If set to `auto`, enables the collection of customer consent for promotional communications.
/// The Checkout.
/// Session will determine whether to display an option to opt into promotional communication
/// from the merchant depending on the customer's locale. Only available to US merchants.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionConsentCollectionPromotions {
    Auto,
    None,
}
impl CreateCheckoutSessionConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionConsentCollectionPromotions::*;
        match self {
            Auto => "auto",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionConsentCollectionPromotions {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionConsentCollectionPromotions::*;
        match s {
            "auto" => Ok(Auto),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionConsentCollectionPromotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionConsentCollectionPromotions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionConsentCollectionPromotions",
            )
        })
    }
}
/// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
/// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionConsentCollectionTermsOfService {
    None,
    Required,
}
impl CreateCheckoutSessionConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionConsentCollectionTermsOfService::*;
        match self {
            None => "none",
            Required => "required",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionConsentCollectionTermsOfService {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionConsentCollectionTermsOfService::*;
        match s {
            "none" => Ok(None),
            "required" => Ok(Required),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionConsentCollectionTermsOfService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionConsentCollectionTermsOfService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionConsentCollectionTermsOfService",
            )
        })
    }
}
/// Collect additional information from your customer using custom fields.
/// Up to 3 fields are supported.
#[derive(Clone, Debug, serde::Serialize)]
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
    pub type_: CreateCheckoutSessionCustomFieldsType,
}
impl CreateCheckoutSessionCustomFields {
    pub fn new(
        key: impl Into<String>,
        label: impl Into<CreateCheckoutSessionCustomFieldsLabel>,
        type_: impl Into<CreateCheckoutSessionCustomFieldsType>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsDropdown {
    /// The value that will pre-fill the field on the payment page.Must match a `value` in the `options` array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The options available for the customer to select. Up to 200 options allowed.
    pub options: Vec<CreateCheckoutSessionCustomFieldsDropdownOptions>,
}
impl CreateCheckoutSessionCustomFieldsDropdown {
    pub fn new(options: impl Into<Vec<CreateCheckoutSessionCustomFieldsDropdownOptions>>) -> Self {
        Self { default_value: None, options: options.into() }
    }
}
/// The options available for the customer to select. Up to 200 options allowed.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsDropdownOptions {
    /// The label for the option, displayed to the customer. Up to 100 characters.
    pub label: String,
    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: String,
}
impl CreateCheckoutSessionCustomFieldsDropdownOptions {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self { label: label.into(), value: value.into() }
    }
}
/// The label for the field, displayed to the customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsLabel {
    /// Custom text for the label, displayed to the customer. Up to 50 characters.
    pub custom: String,
    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionCustomFieldsLabelType,
}
impl CreateCheckoutSessionCustomFieldsLabel {
    pub fn new(
        custom: impl Into<String>,
        type_: impl Into<CreateCheckoutSessionCustomFieldsLabelType>,
    ) -> Self {
        Self { custom: custom.into(), type_: type_.into() }
    }
}
/// The type of the label.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionCustomFieldsLabelType {
    Custom,
}
impl CreateCheckoutSessionCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionCustomFieldsLabelType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionCustomFieldsLabelType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionCustomFieldsLabelType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionCustomFieldsLabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionCustomFieldsLabelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateCheckoutSessionCustomFieldsLabelType")
        })
    }
}
/// Configuration for `type=numeric` fields.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsNumeric {
    /// The value that will pre-fill the field on the payment page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsText {
    /// The value that will pre-fill the field on the payment page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
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
/// The type of the field.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}
impl CreateCheckoutSessionCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionCustomFieldsType::*;
        match self {
            Dropdown => "dropdown",
            Numeric => "numeric",
            Text => "text",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionCustomFieldsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionCustomFieldsType::*;
        match s {
            "dropdown" => Ok(Dropdown),
            "numeric" => Ok(Numeric),
            "text" => Ok(Text),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionCustomFieldsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionCustomFieldsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateCheckoutSessionCustomFieldsType")
        })
    }
}
/// Display additional text for your customers using custom text.
#[derive(Clone, Debug, serde::Serialize)]
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
/// Configure whether a Checkout Session creates a [Customer](https://stripe.com/docs/api/customers) during Session confirmation.
///
/// When a Customer is not created, you can still retrieve email, address, and other customer data entered in Checkout.
/// with [customer_details](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-customer_details).
///
/// Sessions that don't create Customers instead are grouped by [guest customers](https://stripe.com/docs/payments/checkout/guest-customers).
/// in the Dashboard.
/// Promotion codes limited to first time customers will return invalid for these Sessions.
///
/// Can only be set in `payment` and `setup` mode.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionCustomerCreation {
    Always,
    IfRequired,
}
impl CreateCheckoutSessionCustomerCreation {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionCustomerCreation::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionCustomerCreation {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionCustomerCreation::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionCustomerCreation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateCheckoutSessionCustomerCreation")
        })
    }
}
/// Controls what fields on Customer can be updated by the Checkout Session.
/// Can only be provided when `customer` is provided.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionCustomerUpdate {
    /// Describes whether Checkout saves the billing address onto `customer.address`.
    /// To always collect a full billing address, use `billing_address_collection`. Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateCheckoutSessionCustomerUpdateAddress>,
    /// Describes whether Checkout saves the name onto `customer.name`. Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CreateCheckoutSessionCustomerUpdateName>,
    /// Describes whether Checkout saves shipping information onto `customer.shipping`.
    /// To collect shipping information, use `shipping_address_collection`. Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateCheckoutSessionCustomerUpdateShipping>,
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
/// Describes whether Checkout saves the billing address onto `customer.address`.
/// To always collect a full billing address, use `billing_address_collection`. Defaults to `never`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionCustomerUpdateAddress {
    Auto,
    Never,
}
impl CreateCheckoutSessionCustomerUpdateAddress {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionCustomerUpdateAddress::*;
        match self {
            Auto => "auto",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionCustomerUpdateAddress {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionCustomerUpdateAddress::*;
        match s {
            "auto" => Ok(Auto),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionCustomerUpdateAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionCustomerUpdateAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionCustomerUpdateAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionCustomerUpdateAddress {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateCheckoutSessionCustomerUpdateAddress")
        })
    }
}
/// Describes whether Checkout saves the name onto `customer.name`. Defaults to `never`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionCustomerUpdateName {
    Auto,
    Never,
}
impl CreateCheckoutSessionCustomerUpdateName {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionCustomerUpdateName::*;
        match self {
            Auto => "auto",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionCustomerUpdateName {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionCustomerUpdateName::*;
        match s {
            "auto" => Ok(Auto),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionCustomerUpdateName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionCustomerUpdateName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionCustomerUpdateName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionCustomerUpdateName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateCheckoutSessionCustomerUpdateName")
        })
    }
}
/// Describes whether Checkout saves shipping information onto `customer.shipping`.
/// To collect shipping information, use `shipping_address_collection`. Defaults to `never`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionCustomerUpdateShipping {
    Auto,
    Never,
}
impl CreateCheckoutSessionCustomerUpdateShipping {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionCustomerUpdateShipping::*;
        match self {
            Auto => "auto",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionCustomerUpdateShipping {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionCustomerUpdateShipping::*;
        match s {
            "auto" => Ok(Auto),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionCustomerUpdateShipping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionCustomerUpdateShipping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionCustomerUpdateShipping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionCustomerUpdateShipping {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionCustomerUpdateShipping",
            )
        })
    }
}
/// The coupon or promotion code to apply to this Session. Currently, only up to one may be specified.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionDiscounts {
    /// The ID of the coupon to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    /// The ID of a promotion code to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
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
/// A list of the types of payment methods (e.g., `card`) that should be excluded from this Checkout Session.
/// This should only be used when payment methods for this Checkout Session are managed through the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateCheckoutSessionExcludedPaymentMethodTypes {
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
    Pix,
    Promptpay,
    RevolutPay,
    SamsungPay,
    Satispay,
    SepaDebit,
    Sofort,
    Swish,
    Twint,
    UsBankAccount,
    WechatPay,
    Zip,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateCheckoutSessionExcludedPaymentMethodTypes {
    pub fn as_str(&self) -> &str {
        use CreateCheckoutSessionExcludedPaymentMethodTypes::*;
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
            Pix => "pix",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SamsungPay => "samsung_pay",
            Satispay => "satispay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            Swish => "swish",
            Twint => "twint",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionExcludedPaymentMethodTypes {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionExcludedPaymentMethodTypes::*;
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
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "samsung_pay" => Ok(SamsungPay),
            "satispay" => Ok(Satispay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "swish" => Ok(Swish),
            "twint" => Ok(Twint),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionExcludedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionExcludedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionExcludedPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionExcludedPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Generate a post-purchase Invoice for one-time payments.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreation {
    /// Set to `true` to enable invoice creation.
    pub enabled: bool,
    /// Parameters passed when creating invoices for payment-mode Checkout Sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<CreateCheckoutSessionInvoiceCreationInvoiceData>,
}
impl CreateCheckoutSessionInvoiceCreation {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), invoice_data: None }
    }
}
/// Parameters passed when creating invoices for payment-mode Checkout Sessions.
#[derive(Clone, Debug, serde::Serialize)]
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
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceDataCustomFields {
    /// The name of the custom field. This may be up to 40 characters.
    pub name: String,
    /// The value of the custom field. This may be up to 140 characters.
    pub value: String,
}
impl CreateCheckoutSessionInvoiceCreationInvoiceDataCustomFields {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self { name: name.into(), value: value.into() }
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType,
}
impl CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer {
    pub fn new(
        type_: impl Into<CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType {
    Account,
    Self_,
}
impl CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType",
            )
        })
    }
}
/// Default options for invoice PDF rendering for this customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display:
        Option<CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay>,
    /// ID of the invoice rendering template to use for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
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
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}
impl CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay"))
    }
}
/// A list of items the customer is purchasing.
/// Use this parameter to pass one-time or recurring [Prices](https://stripe.com/docs/api/prices).
/// The parameter is required for `payment` and `subscription` mode.
///
/// For `payment` mode, there is a maximum of 100 line items, however it is recommended to consolidate line items if there are more than a few dozen.
///
/// For `subscription` mode, there is a maximum of 20 line items with recurring Prices and 20 line items with one-time Prices.
/// Line items with one-time Prices will be on the initial invoice only.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionLineItems {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreateCheckoutSessionLineItemsAdjustableQuantity>,
    /// The [tax rates](https://stripe.com/docs/api/tax_rates) that will be applied to this line item depending on the customer's billing/shipping address.
    /// We currently support the following countries: US, GB, AU, and all countries in the EU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_tax_rates: Option<Vec<String>>,
    /// The ID of the [Price](https://stripe.com/docs/api/prices) or [Plan](https://stripe.com/docs/api/plans) object.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateCheckoutSessionLineItemsPriceData>,
    /// The quantity of the line item being purchased.
    /// Quantity should not be defined when `recurring.usage_type=metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The [tax rates](https://stripe.com/docs/api/tax_rates) which apply to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl CreateCheckoutSessionLineItems {
    pub fn new() -> Self {
        Self {
            adjustable_quantity: None,
            dynamic_tax_rates: None,
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
impl CreateCheckoutSessionLineItemsAdjustableQuantity {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), maximum: None, minimum: None }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
/// One of `price` or `price_data` is required.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceData {
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
    pub product_data: Option<CreateCheckoutSessionLineItemsPriceDataProductData>,
    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreateCheckoutSessionLineItemsPriceDataRecurring>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateCheckoutSessionLineItemsPriceDataTaxBehavior>,
    /// A non-negative integer in cents (or local equivalent) representing how much to charge.
    /// One of `unit_amount` or `unit_amount_decimal` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreateCheckoutSessionLineItemsPriceData {
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceDataProductData {
    /// The product's description, meant to be displayable to the customer.
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    pub name: String,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,
}
impl CreateCheckoutSessionLineItemsPriceDataProductData {
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: CreateCheckoutSessionLineItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreateCheckoutSessionLineItemsPriceDataRecurring {
    pub fn new(
        interval: impl Into<CreateCheckoutSessionLineItemsPriceDataRecurringInterval>,
    ) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionLineItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionLineItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionLineItemsPriceDataRecurringInterval",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionLineItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionLineItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionLineItemsPriceDataTaxBehavior",
            )
        })
    }
}
/// Controls name collection settings for the session.
///
/// You can configure Checkout to collect your customers' business names, individual names, or both.
/// Each name field can be either required or optional.
///
/// If a [Customer](https://stripe.com/docs/api/customers) is created or provided, the names can be saved to the Customer object as well.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionNameCollection {
    /// Controls settings applied for collecting the customer's business name on the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<CreateCheckoutSessionNameCollectionBusiness>,
    /// Controls settings applied for collecting the customer's individual name on the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<CreateCheckoutSessionNameCollectionIndividual>,
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionNameCollectionBusiness {
    /// Enable business name collection on the Checkout Session. Defaults to `false`.
    pub enabled: bool,
    /// Whether the customer is required to provide a business name before completing the Checkout Session.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
impl CreateCheckoutSessionNameCollectionBusiness {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), optional: None }
    }
}
/// Controls settings applied for collecting the customer's individual name on the session.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionNameCollectionIndividual {
    /// Enable individual name collection on the Checkout Session. Defaults to `false`.
    pub enabled: bool,
    /// Whether the customer is required to provide their name before completing the Checkout Session.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
impl CreateCheckoutSessionNameCollectionIndividual {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), optional: None }
    }
}
/// A list of optional items the customer can add to their order at checkout.
/// Use this parameter to pass one-time or recurring [Prices](https://stripe.com/docs/api/prices).
///
/// There is a maximum of 10 optional items allowed on a Checkout Session, and the existing limits on the number of line items allowed on a Checkout Session apply to the combined number of line items and optional items.
///
/// For `payment` mode, there is a maximum of 100 combined line items and optional items, however it is recommended to consolidate items if there are more than a few dozen.
///
/// For `subscription` mode, there is a maximum of 20 line items and optional items with recurring Prices and 20 line items and optional items with one-time Prices.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionOptionalItems {
    /// When set, provides configuration for the customer to adjust the quantity of the line item created when a customer chooses to add this optional item to their order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreateCheckoutSessionOptionalItemsAdjustableQuantity>,
    /// The ID of the [Price](https://stripe.com/docs/api/prices) or [Plan](https://stripe.com/docs/api/plans) object.
    pub price: String,
    /// The initial quantity of the line item created when a customer chooses to add this optional item to their order.
    pub quantity: u64,
}
impl CreateCheckoutSessionOptionalItems {
    pub fn new(price: impl Into<String>, quantity: impl Into<u64>) -> Self {
        Self { adjustable_quantity: None, price: price.into(), quantity: quantity.into() }
    }
}
/// When set, provides configuration for the customer to adjust the quantity of the line item created when a customer chooses to add this optional item to their order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
impl CreateCheckoutSessionOptionalItemsAdjustableQuantity {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), maximum: None, minimum: None }
    }
}
/// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentIntentData {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    /// The amount of the application fee collected will be capped at the total amount captured.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentIntentDataCaptureMethod>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    /// Indicates that you intend to [make future payments](https://stripe.com/docs/payments/payment-intents#future-usage) with the payment.
    /// method collected by this Checkout Session.
    ///
    /// When setting this to `on_session`, Checkout will show a notice to the
    /// customer that their payment details will be saved.
    ///
    /// When setting this to `off_session`, Checkout will show a notice to the
    /// customer that their payment details will be saved and used for future
    /// payments.
    ///
    /// If a Customer has been provided or Checkout creates a new Customer,
    /// Checkout will attach the payment method to the Customer.
    ///
    /// If Checkout does not create a Customer, the payment method is not attached
    /// to a Customer. To reuse the payment method, you can retrieve it from the
    /// Checkout Session's PaymentIntent.
    ///
    /// When processing card payments, Checkout also uses `setup_future_usage`
    /// to dynamically optimize your payment flow and comply with regional
    /// legislation and network rules, such as SCA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentIntentDataSetupFutureUsage>,
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
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateCheckoutSessionPaymentIntentDataTransferData>,
    /// A string that identifies the resulting payment as part of a group.
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}
impl CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentIntentDataCaptureMethod::*;
        match self {
            Automatic => "automatic",
            AutomaticAsync => "automatic_async",
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentIntentDataCaptureMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "automatic_async" => Ok(AutomaticAsync),
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentIntentDataCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to [make future payments](https://stripe.com/docs/payments/payment-intents#future-usage) with the payment.
/// method collected by this Checkout Session.
///
/// When setting this to `on_session`, Checkout will show a notice to the
/// customer that their payment details will be saved.
///
/// When setting this to `off_session`, Checkout will show a notice to the
/// customer that their payment details will be saved and used for future
/// payments.
///
/// If a Customer has been provided or Checkout creates a new Customer,
/// Checkout will attach the payment method to the Customer.
///
/// If Checkout does not create a Customer, the payment method is not attached
/// to a Customer. To reuse the payment method, you can retrieve it from the
/// Checkout Session's PaymentIntent.
///
/// When processing card payments, Checkout also uses `setup_future_usage`
/// to dynamically optimize your payment flow and comply with regional
/// legislation and network rules, such as SCA.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}
impl CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentIntentDataSetupFutureUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentIntentDataSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentIntentDataSetupFutureUsage",
            )
        })
    }
}
/// Shipping information for this payment.
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
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
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
/// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
#[derive(Clone, Debug, serde::Serialize)]
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
impl CreateCheckoutSessionPaymentIntentDataTransferData {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount: None, destination: destination.into() }
    }
}
/// Specify whether Checkout should collect a payment method.
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.
/// This may occur if the Checkout Session includes a free trial or a discount.
///
/// Can only be set in `subscription` mode. Defaults to `always`.
///
/// If you'd like information on how to collect a payment method outside of Checkout, read the guide on configuring [subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodCollection {
    Always,
    IfRequired,
}
impl CreateCheckoutSessionPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodCollection::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodCollection {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodCollection::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodCollection",
            )
        })
    }
}
/// This parameter allows you to set some attributes on the payment method created during a Checkout session.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodData {
    /// Allow redisplay will be set on the payment method on confirmation and indicates whether this payment method can be shown again to the customer in a checkout flow.
    /// Only set this field if you wish to override the allow_redisplay value determined by Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<CreateCheckoutSessionPaymentMethodDataAllowRedisplay>,
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
/// Allow redisplay will be set on the payment method on confirmation and indicates whether this payment method can be shown again to the customer in a checkout flow.
/// Only set this field if you wish to override the allow_redisplay value determined by Checkout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodDataAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl CreateCheckoutSessionPaymentMethodDataAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodDataAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodDataAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodDataAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodDataAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodDataAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodDataAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodDataAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodDataAllowRedisplay",
            )
        })
    }
}
/// Payment-method-specific configuration.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptions {
    /// contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebit>,
    /// contains details about the Affirm payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreateCheckoutSessionPaymentMethodOptionsAffirm>,
    /// contains details about the Afterpay Clearpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay>,
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
    /// contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateCheckoutSessionPaymentMethodOptionsLink>,
    /// contains details about the Mobilepay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<CreateCheckoutSessionPaymentMethodOptionsMobilepay>,
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
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub pay_by_bank: Option<miniserde::json::Value>,
    /// contains details about the PAYCO payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payco: Option<CreateCheckoutSessionPaymentMethodOptionsPayco>,
    /// contains details about the PayNow payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreateCheckoutSessionPaymentMethodOptionsPaynow>,
    /// contains details about the PayPal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CreateCheckoutSessionPaymentMethodOptionsPaypal>,
    /// contains details about the Pix payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreateCheckoutSessionPaymentMethodOptionsPix>,
    /// contains details about the RevolutPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<CreateCheckoutSessionPaymentMethodOptionsRevolutPay>,
    /// contains details about the Samsung Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<CreateCheckoutSessionPaymentMethodOptionsSamsungPay>,
    /// contains details about the Satispay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satispay: Option<CreateCheckoutSessionPaymentMethodOptionsSatispay>,
    /// contains details about the Sepa Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateCheckoutSessionPaymentMethodOptionsSepaDebit>,
    /// contains details about the Sofort payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateCheckoutSessionPaymentMethodOptionsSofort>,
    /// contains details about the Swish payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<CreateCheckoutSessionPaymentMethodOptionsSwish>,
    /// contains details about the Us Bank Account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccount>,
    /// contains details about the WeChat Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreateCheckoutSessionPaymentMethodOptionsWechatPay>,
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
            pix: None,
            revolut_pay: None,
            samsung_pay: None,
            satispay: None,
            sepa_debit: None,
            sofort: None,
            swish: None,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebit {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// This is only accepted for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency>,
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
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod>,
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
/// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
/// Must be a [supported currency](https://stripe.com/docs/currencies).
/// This is only accepted for Checkout Sessions in `setup` mode.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
}
impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency::*;
        match self {
            Cad => "cad",
            Usd => "usd",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency::*;
        match s {
            "cad" => Ok(Cad),
            "usd" => Ok(Usd),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency",
            )
        })
    }
}
/// Additional fields for Mandate creation
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,.
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,
    /// List of Stripe products where this mandate can be selected automatically.
    /// Only usable in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<Vec<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor>>,
    /// Description of the mandate interval.
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
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
/// List of Stripe products where this mandate can be selected automatically.
/// Only usable in `setup` mode.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}
impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::*;
        match self {
            Invoice => "invoice",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::*;
        match s {
            "invoice" => Ok(Invoice),
            "subscription" => Ok(Subscription),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor"))
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}
impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule"))
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}
impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType"))
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage"))
    }
}
/// Verification method for the intent
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod"))
    }
}
/// contains details about the Affirm payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAffirm {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsAffirmCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsAffirm {
    pub fn new() -> Self {
        Self { capture_method: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsAffirm {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAffirmCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsAffirmCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAffirmCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsAffirmCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAffirmCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAffirmCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAffirmCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsAffirmCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsAffirmCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsAffirmCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage",
            )
        })
    }
}
/// contains details about the Afterpay Clearpay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpayCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay {
    pub fn new() -> Self {
        Self { capture_method: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpayCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpayCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpayCaptureMethod"))
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage"))
    }
}
/// contains details about the Alipay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage",
            )
        })
    }
}
/// contains details about the Alma payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAlma {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsAlmaCaptureMethod>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAlmaCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsAlmaCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAlmaCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsAlmaCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAlmaCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAlmaCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAlmaCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsAlmaCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsAlmaCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsAlmaCaptureMethod",
            )
        })
    }
}
/// contains details about the AmazonPay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAmazonPay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsAmazonPayCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAmazonPaySetupFutureUsage>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAmazonPayCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsAmazonPayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAmazonPayCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsAmazonPayCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAmazonPayCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAmazonPayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAmazonPayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsAmazonPayCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsAmazonPayCaptureMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsAmazonPayCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAmazonPaySetupFutureUsage {
    None,
    OffSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsAmazonPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAmazonPaySetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsAmazonPaySetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAmazonPaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAmazonPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAmazonPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsAmazonPaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsAmazonPaySetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsAmazonPaySetupFutureUsage"))
    }
}
/// contains details about the AU Becs Debit payment method options.
#[derive(Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage"))
    }
}
/// contains details about the Bacs Debit payment method options.
#[derive(Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBacsDebitMandateOptions {
    /// Prefix used to generate the Mandate reference.
    /// Must be at most 12 characters long.
    /// Must consist of only uppercase letters, numbers, spaces, or the following special characters: '/', '_', '-', '&', '.'.
    /// Cannot begin with 'DDIC' or 'STRIPE'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage"))
    }
}
/// contains details about the Bancontact payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage"))
    }
}
/// contains details about the Billie payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBillie {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsBillieCaptureMethod>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsBillieCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsBillieCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsBillieCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsBillieCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsBillieCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsBillieCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsBillieCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsBillieCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsBillieCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsBillieCaptureMethod",
            )
        })
    }
}
/// contains details about the Boleto payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage",
            )
        })
    }
}
/// contains details about the Card payment method options.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCard {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsCardCaptureMethod>,
    /// Installment options for card payments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<CreateCheckoutSessionPaymentMethodOptionsCardInstallments>,
    /// Request ability to [capture beyond the standard authorization validity window](/payments/extended-authorization) for this CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization:
        Option<CreateCheckoutSessionPaymentMethodOptionsCardRequestExtendedAuthorization>,
    /// Request ability to [increment the authorization](/payments/incremental-authorization) for this CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization:
        Option<CreateCheckoutSessionPaymentMethodOptionsCardRequestIncrementalAuthorization>,
    /// Request ability to make [multiple captures](/payments/multicapture) for this CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_multicapture:
        Option<CreateCheckoutSessionPaymentMethodOptionsCardRequestMulticapture>,
    /// Request ability to [overcapture](/payments/overcapture) for this CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_overcapture:
        Option<CreateCheckoutSessionPaymentMethodOptionsCardRequestOvercapture>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<CreateCheckoutSessionPaymentMethodOptionsCardRequestThreeDSecure>,
    /// Restrictions to apply to the card payment method. For example, you can block specific card brands.
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
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCardCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCardCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsCardCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCardCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsCardCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsCardCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsCardCaptureMethod",
            )
        })
    }
}
/// Installment options for card payments
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this Checkout Session.
    /// Setting to false will prevent any installment plan from applying to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
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
/// Request ability to [capture beyond the standard authorization validity window](/payments/extended-authorization) for this CheckoutSession.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCardRequestExtendedAuthorization {
    IfAvailable,
    Never,
}
impl CreateCheckoutSessionPaymentMethodOptionsCardRequestExtendedAuthorization {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCardRequestExtendedAuthorization::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsCardRequestExtendedAuthorization
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCardRequestExtendedAuthorization::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsCardRequestExtendedAuthorization
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionPaymentMethodOptionsCardRequestExtendedAuthorization
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCardRequestExtendedAuthorization
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsCardRequestExtendedAuthorization"))
    }
}
/// Request ability to [increment the authorization](/payments/incremental-authorization) for this CheckoutSession.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCardRequestIncrementalAuthorization {
    IfAvailable,
    Never,
}
impl CreateCheckoutSessionPaymentMethodOptionsCardRequestIncrementalAuthorization {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCardRequestIncrementalAuthorization::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsCardRequestIncrementalAuthorization
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCardRequestIncrementalAuthorization::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsCardRequestIncrementalAuthorization
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionPaymentMethodOptionsCardRequestIncrementalAuthorization
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionPaymentMethodOptionsCardRequestIncrementalAuthorization
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCardRequestIncrementalAuthorization
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsCardRequestIncrementalAuthorization"))
    }
}
/// Request ability to make [multiple captures](/payments/multicapture) for this CheckoutSession.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCardRequestMulticapture {
    IfAvailable,
    Never,
}
impl CreateCheckoutSessionPaymentMethodOptionsCardRequestMulticapture {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCardRequestMulticapture::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsCardRequestMulticapture {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCardRequestMulticapture::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCardRequestMulticapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCardRequestMulticapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsCardRequestMulticapture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCardRequestMulticapture
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsCardRequestMulticapture"))
    }
}
/// Request ability to [overcapture](/payments/overcapture) for this CheckoutSession.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCardRequestOvercapture {
    IfAvailable,
    Never,
}
impl CreateCheckoutSessionPaymentMethodOptionsCardRequestOvercapture {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCardRequestOvercapture::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsCardRequestOvercapture {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCardRequestOvercapture::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCardRequestOvercapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCardRequestOvercapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsCardRequestOvercapture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCardRequestOvercapture
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsCardRequestOvercapture",
            )
        })
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// If not provided, this value defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}
impl CreateCheckoutSessionPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
            Challenge => "challenge",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsCardRequestThreeDSecure {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            "challenge" => Ok(Challenge),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsCardRequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCardRequestThreeDSecure
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsCardRequestThreeDSecure"))
    }
}
/// Restrictions to apply to the card payment method. For example, you can block specific card brands.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCardRestrictions {
    /// Specify the card brands to block in the Checkout Session.
    /// If a customer enters or selects a card belonging to a blocked brand, they can't complete the Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brands_blocked:
        Option<Vec<CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsBrandsBlocked>>,
}
impl CreateCheckoutSessionPaymentMethodOptionsCardRestrictions {
    pub fn new() -> Self {
        Self { brands_blocked: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsCardRestrictions {
    fn default() -> Self {
        Self::new()
    }
}
/// Specify the card brands to block in the Checkout Session.
/// If a customer enters or selects a card belonging to a blocked brand, they can't complete the Session.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsBrandsBlocked {
    AmericanExpress,
    DiscoverGlobalNetwork,
    Mastercard,
    Visa,
}
impl CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsBrandsBlocked {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsBrandsBlocked::*;
        match self {
            AmericanExpress => "american_express",
            DiscoverGlobalNetwork => "discover_global_network",
            Mastercard => "mastercard",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsBrandsBlocked {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsBrandsBlocked::*;
        match s {
            "american_express" => Ok(AmericanExpress),
            "discover_global_network" => Ok(DiscoverGlobalNetwork),
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsBrandsBlocked {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsBrandsBlocked {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsBrandsBlocked {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsBrandsBlocked
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsCardRestrictionsBrandsBlocked"))
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage {
    OffSession,
    OnSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage",
            )
        })
    }
}
/// contains details about the Cashapp Pay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCashapp {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsCashappCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCashappCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsCashappCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCashappCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsCashappCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCashappCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCashappCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCashappCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsCashappCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCashappCaptureMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsCashappCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage"))
    }
}
/// contains details about the Customer Balance payment method options.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCustomerBalance {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer {
    /// Configuration for eu_bank_transfer funding type.
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
        /// List of address types that should be returned in the financial_addresses response.
    /// If not specified, all valid types will be returned.
    ///
    /// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[serde(skip_serializing_if = "Option::is_none")]
pub requested_address_types: Option<Vec<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes>>,
    /// The list of bank transfer types that this PaymentIntent is allowed to use for funding.
#[serde(rename = "type")]
pub type_: CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType,

}
impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer {
    pub fn new(
        type_: impl Into<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType>,
    ) -> Self {
        Self { eu_bank_transfer: None, requested_address_types: None, type_: type_.into() }
    }
}
/// Configuration for eu_bank_transfer funding type.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    /// The desired country code of the bank account information.
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}
impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    pub fn new(country: impl Into<String>) -> Self {
        Self { country: country.into() }
    }
}
/// List of address types that should be returned in the financial_addresses response.
/// If not specified, all valid types will be returned.
///
/// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    Aba,
    Iban,
    Sepa,
    SortCode,
    Spei,
    Swift,
    Zengin,
}
impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::*;
        match self {
            Aba => "aba",
            Iban => "iban",
            Sepa => "sepa",
            SortCode => "sort_code",
            Spei => "spei",
            Swift => "swift",
            Zengin => "zengin",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::*;
        match s {
            "aba" => Ok(Aba),
            "iban" => Ok(Iban),
            "sepa" => Ok(Sepa),
            "sort_code" => Ok(SortCode),
            "spei" => Ok(Spei),
            "swift" => Ok(Swift),
            "zengin" => Ok(Zengin),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes"))
    }
}
/// The list of bank transfer types that this PaymentIntent is allowed to use for funding.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}
impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            GbBankTransfer => "gb_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
            MxBankTransfer => "mx_bank_transfer",
            UsBankTransfer => "us_bank_transfer",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "gb_bank_transfer" => Ok(GbBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            "mx_bank_transfer" => Ok(MxBankTransfer),
            "us_bank_transfer" => Ok(UsBankTransfer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType"))
    }
}
/// The funding method type to be used when there are not enough funds in the customer balance.
/// Permitted values include: `bank_transfer`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}
impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType::*;
        match self {
            BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType"))
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage"))
    }
}
/// contains details about the DemoPay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsDemoPaySetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsDemoPaySetupFutureUsage {
    None,
    OffSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsDemoPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsDemoPaySetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsDemoPaySetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsDemoPaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsDemoPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsDemoPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsDemoPaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsDemoPaySetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsDemoPaySetupFutureUsage"))
    }
}
/// contains details about the EPS payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage",
            )
        })
    }
}
/// contains details about the FPX payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage",
            )
        })
    }
}
/// contains details about the Giropay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage"))
    }
}
/// contains details about the Grabpay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage"))
    }
}
/// contains details about the Ideal payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage",
            )
        })
    }
}
/// contains details about the Kakao Pay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKakaoPay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsKakaoPayCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsKakaoPaySetupFutureUsage>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsKakaoPayCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsKakaoPayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsKakaoPayCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsKakaoPayCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsKakaoPayCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsKakaoPayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKakaoPayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsKakaoPayCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsKakaoPayCaptureMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsKakaoPayCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsKakaoPaySetupFutureUsage {
    None,
    OffSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsKakaoPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsKakaoPaySetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsKakaoPaySetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsKakaoPaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsKakaoPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKakaoPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsKakaoPaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsKakaoPaySetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsKakaoPaySetupFutureUsage"))
    }
}
/// contains details about the Klarna payment method options.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKlarna {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsKlarnaCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage>,
    /// Subscription details if the Checkout Session sets up a future subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptions>>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsKlarnaCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsKlarnaCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsKlarnaCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsKlarnaCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsKlarnaCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsKlarnaCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKlarnaCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsKlarnaCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsKlarnaCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsKlarnaCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage",
            )
        })
    }
}
/// Subscription details if the Checkout Session sets up a future subscription.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptions {
    /// Unit of time between subscription charges.
    pub interval: CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval,
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
impl CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptions {
    pub fn new(
        interval: impl Into<CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval>,
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
/// Unit of time between subscription charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval {
    Day,
    Month,
    Week,
    Year,
}
impl CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsInterval"))
    }
}
/// Describes the upcoming charge for this subscription.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsNextBilling {
    /// The amount of the next charge for the subscription.
    pub amount: i64,
    /// The date of the next charge for the subscription in YYYY-MM-DD format.
    pub date: String,
}
impl CreateCheckoutSessionPaymentMethodOptionsKlarnaSubscriptionsNextBilling {
    pub fn new(amount: impl Into<i64>, date: impl Into<String>) -> Self {
        Self { amount: amount.into(), date: date.into() }
    }
}
/// contains details about the Konbini payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage"))
    }
}
/// contains details about the Korean card payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKrCard {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsKrCardCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsKrCardSetupFutureUsage>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsKrCardCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsKrCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsKrCardCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsKrCardCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsKrCardCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsKrCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKrCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsKrCardCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsKrCardCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsKrCardCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsKrCardSetupFutureUsage {
    None,
    OffSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsKrCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsKrCardSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsKrCardSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsKrCardSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsKrCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsKrCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsKrCardSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsKrCardSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsKrCardSetupFutureUsage",
            )
        })
    }
}
/// contains details about the Link payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsLink {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsLinkCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsLinkCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsLinkCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsLinkCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsLinkCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsLinkCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsLinkCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsLinkCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsLinkCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsLinkCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage",
            )
        })
    }
}
/// contains details about the Mobilepay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsMobilepay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsMobilepayCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsMobilepaySetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsMobilepay {
    pub fn new() -> Self {
        Self { capture_method: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsMobilepay {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsMobilepayCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsMobilepayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsMobilepayCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsMobilepayCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsMobilepayCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsMobilepayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsMobilepayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsMobilepayCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsMobilepayCaptureMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsMobilepayCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsMobilepaySetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsMobilepaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsMobilepaySetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsMobilepaySetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsMobilepaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsMobilepaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsMobilepaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsMobilepaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsMobilepaySetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsMobilepaySetupFutureUsage"))
    }
}
/// contains details about the Multibanco payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsMultibancoSetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsMultibancoSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsMultibancoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsMultibancoSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsMultibancoSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsMultibancoSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsMultibancoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsMultibancoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsMultibancoSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsMultibancoSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsMultibancoSetupFutureUsage"))
    }
}
/// contains details about the Naver Pay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsNaverPay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsNaverPayCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsNaverPaySetupFutureUsage>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsNaverPayCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsNaverPayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsNaverPayCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsNaverPayCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsNaverPayCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsNaverPayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsNaverPayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsNaverPayCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsNaverPayCaptureMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsNaverPayCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsNaverPaySetupFutureUsage {
    None,
    OffSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsNaverPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsNaverPaySetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsNaverPaySetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsNaverPaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsNaverPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsNaverPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsNaverPaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsNaverPaySetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsNaverPaySetupFutureUsage"))
    }
}
/// contains details about the OXXO payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage",
            )
        })
    }
}
/// contains details about the P24 payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage>,
    /// Confirm that the payer has accepted the P24 terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage",
            )
        })
    }
}
/// contains details about the PAYCO payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPayco {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsPaycoCaptureMethod>,
}
impl CreateCheckoutSessionPaymentMethodOptionsPayco {
    pub fn new() -> Self {
        Self { capture_method: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsPayco {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsPaycoCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsPaycoCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsPaycoCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsPaycoCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsPaycoCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsPaycoCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPaycoCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsPaycoCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsPaycoCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsPaycoCaptureMethod",
            )
        })
    }
}
/// contains details about the PayNow payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage",
            )
        })
    }
}
/// contains details about the PayPal payment method options.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPaypal {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod>,
    /// [Preferred locale](https://stripe.com/docs/payments/paypal/supported-locales) of the PayPal checkout page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale>,
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
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod",
            )
        })
    }
}
/// [Preferred locale](https://stripe.com/docs/payments/paypal/supported-locales) of the PayPal checkout page that the customer is redirected to.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    CsMinusCz,
    DaMinusDk,
    DeMinusAt,
    DeMinusDe,
    DeMinusLu,
    ElMinusGr,
    EnMinusGb,
    EnMinusUs,
    EsMinusEs,
    FiMinusFi,
    FrMinusBe,
    FrMinusFr,
    FrMinusLu,
    HuMinusHu,
    ItMinusIt,
    NlMinusBe,
    NlMinusNl,
    PlMinusPl,
    PtMinusPt,
    SkMinusSk,
    SvMinusSe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    pub fn as_str(&self) -> &str {
        use CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::*;
        match self {
            CsMinusCz => "cs-CZ",
            DaMinusDk => "da-DK",
            DeMinusAt => "de-AT",
            DeMinusDe => "de-DE",
            DeMinusLu => "de-LU",
            ElMinusGr => "el-GR",
            EnMinusGb => "en-GB",
            EnMinusUs => "en-US",
            EsMinusEs => "es-ES",
            FiMinusFi => "fi-FI",
            FrMinusBe => "fr-BE",
            FrMinusFr => "fr-FR",
            FrMinusLu => "fr-LU",
            HuMinusHu => "hu-HU",
            ItMinusIt => "it-IT",
            NlMinusBe => "nl-BE",
            NlMinusNl => "nl-NL",
            PlMinusPl => "pl-PL",
            PtMinusPt => "pt-PT",
            SkMinusSk => "sk-SK",
            SvMinusSe => "sv-SE",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::*;
        match s {
            "cs-CZ" => Ok(CsMinusCz),
            "da-DK" => Ok(DaMinusDk),
            "de-AT" => Ok(DeMinusAt),
            "de-DE" => Ok(DeMinusDe),
            "de-LU" => Ok(DeMinusLu),
            "el-GR" => Ok(ElMinusGr),
            "en-GB" => Ok(EnMinusGb),
            "en-US" => Ok(EnMinusUs),
            "es-ES" => Ok(EsMinusEs),
            "fi-FI" => Ok(FiMinusFi),
            "fr-BE" => Ok(FrMinusBe),
            "fr-FR" => Ok(FrMinusFr),
            "fr-LU" => Ok(FrMinusLu),
            "hu-HU" => Ok(HuMinusHu),
            "it-IT" => Ok(ItMinusIt),
            "nl-BE" => Ok(NlMinusBe),
            "nl-NL" => Ok(NlMinusNl),
            "pl-PL" => Ok(PlMinusPl),
            "pt-PT" => Ok(PtMinusPt),
            "sk-SK" => Ok(SkMinusSk),
            "sv-SE" => Ok(SvMinusSe),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage {
    None,
    OffSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage",
            )
        })
    }
}
/// contains details about the Pix payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPix {
    /// Determines if the amount includes the IOF tax. Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_includes_iof: Option<CreateCheckoutSessionPaymentMethodOptionsPixAmountIncludesIof>,
    /// The number of seconds (between 10 and 1209600) after which Pix payment will expire.
    /// Defaults to 86400 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsPixSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsPix {
    pub fn new() -> Self {
        Self { amount_includes_iof: None, expires_after_seconds: None, setup_future_usage: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsPix {
    fn default() -> Self {
        Self::new()
    }
}
/// Determines if the amount includes the IOF tax. Defaults to `never`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsPixAmountIncludesIof {
    Always,
    Never,
}
impl CreateCheckoutSessionPaymentMethodOptionsPixAmountIncludesIof {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsPixAmountIncludesIof::*;
        match self {
            Always => "always",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsPixAmountIncludesIof {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsPixAmountIncludesIof::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsPixAmountIncludesIof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPixAmountIncludesIof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsPixAmountIncludesIof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsPixAmountIncludesIof
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsPixAmountIncludesIof",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsPixSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsPixSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsPixSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsPixSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsPixSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsPixSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsPixSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsPixSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsPixSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsPixSetupFutureUsage",
            )
        })
    }
}
/// contains details about the RevolutPay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsRevolutPay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsRevolutPayCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsRevolutPayCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsRevolutPayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsRevolutPayCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsRevolutPayCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsRevolutPayCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsRevolutPayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsRevolutPayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsRevolutPayCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsRevolutPayCaptureMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsRevolutPayCaptureMethod"))
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage {
    None,
    OffSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage"))
    }
}
/// contains details about the Samsung Pay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSamsungPay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsSamsungPayCaptureMethod>,
}
impl CreateCheckoutSessionPaymentMethodOptionsSamsungPay {
    pub fn new() -> Self {
        Self { capture_method: None }
    }
}
impl Default for CreateCheckoutSessionPaymentMethodOptionsSamsungPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsSamsungPayCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsSamsungPayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsSamsungPayCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsSamsungPayCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsSamsungPayCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsSamsungPayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsSamsungPayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsSamsungPayCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsSamsungPayCaptureMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsSamsungPayCaptureMethod"))
    }
}
/// contains details about the Satispay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSatispay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsSatispayCaptureMethod>,
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsSatispayCaptureMethod {
    Manual,
}
impl CreateCheckoutSessionPaymentMethodOptionsSatispayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsSatispayCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsSatispayCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsSatispayCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsSatispayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsSatispayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsSatispayCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsSatispayCaptureMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsSatispayCaptureMethod",
            )
        })
    }
}
/// contains details about the Sepa Debit payment method options.
#[derive(Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSepaDebitMandateOptions {
    /// Prefix used to generate the Mandate reference.
    /// Must be at most 12 characters long.
    /// Must consist of only uppercase letters, numbers, spaces, or the following special characters: '/', '_', '-', '&', '.'.
    /// Cannot begin with 'STRIPE'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage"))
    }
}
/// contains details about the Sofort payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage",
            )
        })
    }
}
/// contains details about the Swish payment method options.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSwish {
    /// The order reference that will be displayed to customers in the Swish application.
    /// Defaults to the `id` of the Payment Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
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
/// contains details about the Us Bank Account payment method options.
#[derive(Clone, Debug, serde::Serialize)]
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
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections {
    /// The list of permissions to request.
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        Vec<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions>,
    >,
    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<
        Vec<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch>,
    >,
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
/// The list of permissions to request.
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}
impl CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions"))
    }
}
/// List of data features that you would like to retrieve upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    Balances,
    Ownership,
    Transactions,
}
impl CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch"))
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}
impl CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage"))
    }
}
/// Verification method for the intent
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
}
impl CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod"))
    }
}
/// contains details about the WeChat Pay payment method options.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsWechatPay {
    /// The app ID registered with WeChat Pay. Only required when client is ios or android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// The client type that the end customer will pay from
    pub client: CreateCheckoutSessionPaymentMethodOptionsWechatPayClient,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsWechatPay {
    pub fn new(
        client: impl Into<CreateCheckoutSessionPaymentMethodOptionsWechatPayClient>,
    ) -> Self {
        Self { app_id: None, client: client.into(), setup_future_usage: None }
    }
}
/// The client type that the end customer will pay from
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}
impl CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsWechatPayClient::*;
        match self {
            Android => "android",
            Ios => "ios",
            Web => "web",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsWechatPayClient::*;
        match s {
            "android" => Ok(Android),
            "ios" => Ok(Ios),
            "web" => Ok(Web),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPaymentMethodOptionsWechatPayClient",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}
impl CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage"))
    }
}
/// A list of the types of payment methods (e.g., `card`) this Checkout Session can accept.
///
/// You can omit this attribute to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
/// See [Dynamic Payment Methods](https://stripe.com/docs/payments/payment-methods/integration-options#using-dynamic-payment-methods) for more details.
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
    Pix,
    Promptpay,
    RevolutPay,
    SamsungPay,
    Satispay,
    SepaDebit,
    Sofort,
    Swish,
    Twint,
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
            Pix => "pix",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SamsungPay => "samsung_pay",
            Satispay => "satispay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            Swish => "swish",
            Twint => "twint",
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
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "samsung_pay" => Ok(SamsungPay),
            "satispay" => Ok(Satispay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "swish" => Ok(Swish),
            "twint" => Ok(Twint),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Ok(Self::from_str(&s).unwrap())
    }
}
/// This property is used to set up permissions for various actions (e.g., update) on the CheckoutSession object.
/// Can only be set when creating `embedded` or `custom` sessions.
///
/// For specific permissions, please refer to their dedicated subsections, such as `permissions.update_shipping_details`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPermissions {
    /// Determines which entity is allowed to update the shipping details.
    ///
    /// Default is `client_only`.
    /// Stripe Checkout client will automatically update the shipping details.
    /// If set to `server_only`, only your server is allowed to update the shipping details.
    ///
    /// When set to `server_only`, you must add the onShippingDetailsChange event handler when initializing the Stripe Checkout client and manually update the shipping details from your server using the Stripe API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_shipping_details: Option<CreateCheckoutSessionPermissionsUpdateShippingDetails>,
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
/// Determines which entity is allowed to update the shipping details.
///
/// Default is `client_only`.
/// Stripe Checkout client will automatically update the shipping details.
/// If set to `server_only`, only your server is allowed to update the shipping details.
///
/// When set to `server_only`, you must add the onShippingDetailsChange event handler when initializing the Stripe Checkout client and manually update the shipping details from your server using the Stripe API.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionPermissionsUpdateShippingDetails {
    ClientOnly,
    ServerOnly,
}
impl CreateCheckoutSessionPermissionsUpdateShippingDetails {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPermissionsUpdateShippingDetails::*;
        match self {
            ClientOnly => "client_only",
            ServerOnly => "server_only",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPermissionsUpdateShippingDetails {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPermissionsUpdateShippingDetails::*;
        match s {
            "client_only" => Ok(ClientOnly),
            "server_only" => Ok(ServerOnly),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionPermissionsUpdateShippingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionPermissionsUpdateShippingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionPermissionsUpdateShippingDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionPermissionsUpdateShippingDetails {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionPermissionsUpdateShippingDetails",
            )
        })
    }
}
/// Controls phone number collection settings for the session.
///
/// We recommend that you review your privacy policy and check with your legal contacts
/// before using this feature.
/// Learn more about [collecting phone numbers with Checkout](https://stripe.com/docs/payments/checkout/phone-numbers).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPhoneNumberCollection {
    /// Set to `true` to enable phone number collection.
    ///
    /// Can only be set in `payment` and `subscription` mode.
    pub enabled: bool,
}
impl CreateCheckoutSessionPhoneNumberCollection {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Controls saved payment method settings for the session.
/// Only available in `payment` and `subscription` mode.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionSavedPaymentMethodOptions {
    /// Uses the `allow_redisplay` value of each saved payment method to filter the set presented to a returning customer.
    /// By default, only saved payment methods with ’allow_redisplay: ‘always’ are shown in Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay_filters:
        Option<Vec<CreateCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters>>,
    /// Enable customers to choose if they wish to remove their saved payment methods. Disabled by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_remove:
        Option<CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove>,
    /// Enable customers to choose if they wish to save their payment method for future use.
    /// Disabled by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_save:
        Option<CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave>,
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
/// Uses the `allow_redisplay` value of each saved payment method to filter the set presented to a returning customer.
/// By default, only saved payment methods with ’allow_redisplay: ‘always’ are shown in Checkout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    Always,
    Limited,
    Unspecified,
}
impl CreateCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters"))
    }
}
/// Enable customers to choose if they wish to remove their saved payment methods. Disabled by default.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    Disabled,
    Enabled,
}
impl CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove"))
    }
}
/// Enable customers to choose if they wish to save their payment method for future use.
/// Disabled by default.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    Disabled,
    Enabled,
}
impl CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave",
            )
        })
    }
}
/// A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in `setup` mode.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionSetupIntentData {
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The Stripe account for which the setup is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionShippingAddressCollection {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
    /// shipping locations.
    pub allowed_countries: Vec<CreateCheckoutSessionShippingAddressCollectionAllowedCountries>,
}
impl CreateCheckoutSessionShippingAddressCollection {
    pub fn new(
        allowed_countries: impl Into<
            Vec<CreateCheckoutSessionShippingAddressCollectionAllowedCountries>,
        >,
    ) -> Self {
        Self { allowed_countries: allowed_countries.into() }
    }
}
/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
/// shipping locations.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    Ac,
    Ad,
    Ae,
    Af,
    Ag,
    Ai,
    Al,
    Am,
    Ao,
    Aq,
    Ar,
    At,
    Au,
    Aw,
    Ax,
    Az,
    Ba,
    Bb,
    Bd,
    Be,
    Bf,
    Bg,
    Bh,
    Bi,
    Bj,
    Bl,
    Bm,
    Bn,
    Bo,
    Bq,
    Br,
    Bs,
    Bt,
    Bv,
    Bw,
    By,
    Bz,
    Ca,
    Cd,
    Cf,
    Cg,
    Ch,
    Ci,
    Ck,
    Cl,
    Cm,
    Cn,
    Co,
    Cr,
    Cv,
    Cw,
    Cy,
    Cz,
    De,
    Dj,
    Dk,
    Dm,
    Do,
    Dz,
    Ec,
    Ee,
    Eg,
    Eh,
    Er,
    Es,
    Et,
    Fi,
    Fj,
    Fk,
    Fo,
    Fr,
    Ga,
    Gb,
    Gd,
    Ge,
    Gf,
    Gg,
    Gh,
    Gi,
    Gl,
    Gm,
    Gn,
    Gp,
    Gq,
    Gr,
    Gs,
    Gt,
    Gu,
    Gw,
    Gy,
    Hk,
    Hn,
    Hr,
    Ht,
    Hu,
    Id,
    Ie,
    Il,
    Im,
    In,
    Io,
    Iq,
    Is,
    It,
    Je,
    Jm,
    Jo,
    Jp,
    Ke,
    Kg,
    Kh,
    Ki,
    Km,
    Kn,
    Kr,
    Kw,
    Ky,
    Kz,
    La,
    Lb,
    Lc,
    Li,
    Lk,
    Lr,
    Ls,
    Lt,
    Lu,
    Lv,
    Ly,
    Ma,
    Mc,
    Md,
    Me,
    Mf,
    Mg,
    Mk,
    Ml,
    Mm,
    Mn,
    Mo,
    Mq,
    Mr,
    Ms,
    Mt,
    Mu,
    Mv,
    Mw,
    Mx,
    My,
    Mz,
    Na,
    Nc,
    Ne,
    Ng,
    Ni,
    Nl,
    No,
    Np,
    Nr,
    Nu,
    Nz,
    Om,
    Pa,
    Pe,
    Pf,
    Pg,
    Ph,
    Pk,
    Pl,
    Pm,
    Pn,
    Pr,
    Ps,
    Pt,
    Py,
    Qa,
    Re,
    Ro,
    Rs,
    Ru,
    Rw,
    Sa,
    Sb,
    Sc,
    Sd,
    Se,
    Sg,
    Sh,
    Si,
    Sj,
    Sk,
    Sl,
    Sm,
    Sn,
    So,
    Sr,
    Ss,
    St,
    Sv,
    Sx,
    Sz,
    Ta,
    Tc,
    Td,
    Tf,
    Tg,
    Th,
    Tj,
    Tk,
    Tl,
    Tm,
    Tn,
    To,
    Tr,
    Tt,
    Tv,
    Tw,
    Tz,
    Ua,
    Ug,
    Us,
    Uy,
    Uz,
    Va,
    Vc,
    Ve,
    Vg,
    Vn,
    Vu,
    Wf,
    Ws,
    Xk,
    Ye,
    Yt,
    Za,
    Zm,
    Zw,
    Zz,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    pub fn as_str(&self) -> &str {
        use CreateCheckoutSessionShippingAddressCollectionAllowedCountries::*;
        match self {
            Ac => "AC",
            Ad => "AD",
            Ae => "AE",
            Af => "AF",
            Ag => "AG",
            Ai => "AI",
            Al => "AL",
            Am => "AM",
            Ao => "AO",
            Aq => "AQ",
            Ar => "AR",
            At => "AT",
            Au => "AU",
            Aw => "AW",
            Ax => "AX",
            Az => "AZ",
            Ba => "BA",
            Bb => "BB",
            Bd => "BD",
            Be => "BE",
            Bf => "BF",
            Bg => "BG",
            Bh => "BH",
            Bi => "BI",
            Bj => "BJ",
            Bl => "BL",
            Bm => "BM",
            Bn => "BN",
            Bo => "BO",
            Bq => "BQ",
            Br => "BR",
            Bs => "BS",
            Bt => "BT",
            Bv => "BV",
            Bw => "BW",
            By => "BY",
            Bz => "BZ",
            Ca => "CA",
            Cd => "CD",
            Cf => "CF",
            Cg => "CG",
            Ch => "CH",
            Ci => "CI",
            Ck => "CK",
            Cl => "CL",
            Cm => "CM",
            Cn => "CN",
            Co => "CO",
            Cr => "CR",
            Cv => "CV",
            Cw => "CW",
            Cy => "CY",
            Cz => "CZ",
            De => "DE",
            Dj => "DJ",
            Dk => "DK",
            Dm => "DM",
            Do => "DO",
            Dz => "DZ",
            Ec => "EC",
            Ee => "EE",
            Eg => "EG",
            Eh => "EH",
            Er => "ER",
            Es => "ES",
            Et => "ET",
            Fi => "FI",
            Fj => "FJ",
            Fk => "FK",
            Fo => "FO",
            Fr => "FR",
            Ga => "GA",
            Gb => "GB",
            Gd => "GD",
            Ge => "GE",
            Gf => "GF",
            Gg => "GG",
            Gh => "GH",
            Gi => "GI",
            Gl => "GL",
            Gm => "GM",
            Gn => "GN",
            Gp => "GP",
            Gq => "GQ",
            Gr => "GR",
            Gs => "GS",
            Gt => "GT",
            Gu => "GU",
            Gw => "GW",
            Gy => "GY",
            Hk => "HK",
            Hn => "HN",
            Hr => "HR",
            Ht => "HT",
            Hu => "HU",
            Id => "ID",
            Ie => "IE",
            Il => "IL",
            Im => "IM",
            In => "IN",
            Io => "IO",
            Iq => "IQ",
            Is => "IS",
            It => "IT",
            Je => "JE",
            Jm => "JM",
            Jo => "JO",
            Jp => "JP",
            Ke => "KE",
            Kg => "KG",
            Kh => "KH",
            Ki => "KI",
            Km => "KM",
            Kn => "KN",
            Kr => "KR",
            Kw => "KW",
            Ky => "KY",
            Kz => "KZ",
            La => "LA",
            Lb => "LB",
            Lc => "LC",
            Li => "LI",
            Lk => "LK",
            Lr => "LR",
            Ls => "LS",
            Lt => "LT",
            Lu => "LU",
            Lv => "LV",
            Ly => "LY",
            Ma => "MA",
            Mc => "MC",
            Md => "MD",
            Me => "ME",
            Mf => "MF",
            Mg => "MG",
            Mk => "MK",
            Ml => "ML",
            Mm => "MM",
            Mn => "MN",
            Mo => "MO",
            Mq => "MQ",
            Mr => "MR",
            Ms => "MS",
            Mt => "MT",
            Mu => "MU",
            Mv => "MV",
            Mw => "MW",
            Mx => "MX",
            My => "MY",
            Mz => "MZ",
            Na => "NA",
            Nc => "NC",
            Ne => "NE",
            Ng => "NG",
            Ni => "NI",
            Nl => "NL",
            No => "NO",
            Np => "NP",
            Nr => "NR",
            Nu => "NU",
            Nz => "NZ",
            Om => "OM",
            Pa => "PA",
            Pe => "PE",
            Pf => "PF",
            Pg => "PG",
            Ph => "PH",
            Pk => "PK",
            Pl => "PL",
            Pm => "PM",
            Pn => "PN",
            Pr => "PR",
            Ps => "PS",
            Pt => "PT",
            Py => "PY",
            Qa => "QA",
            Re => "RE",
            Ro => "RO",
            Rs => "RS",
            Ru => "RU",
            Rw => "RW",
            Sa => "SA",
            Sb => "SB",
            Sc => "SC",
            Sd => "SD",
            Se => "SE",
            Sg => "SG",
            Sh => "SH",
            Si => "SI",
            Sj => "SJ",
            Sk => "SK",
            Sl => "SL",
            Sm => "SM",
            Sn => "SN",
            So => "SO",
            Sr => "SR",
            Ss => "SS",
            St => "ST",
            Sv => "SV",
            Sx => "SX",
            Sz => "SZ",
            Ta => "TA",
            Tc => "TC",
            Td => "TD",
            Tf => "TF",
            Tg => "TG",
            Th => "TH",
            Tj => "TJ",
            Tk => "TK",
            Tl => "TL",
            Tm => "TM",
            Tn => "TN",
            To => "TO",
            Tr => "TR",
            Tt => "TT",
            Tv => "TV",
            Tw => "TW",
            Tz => "TZ",
            Ua => "UA",
            Ug => "UG",
            Us => "US",
            Uy => "UY",
            Uz => "UZ",
            Va => "VA",
            Vc => "VC",
            Ve => "VE",
            Vg => "VG",
            Vn => "VN",
            Vu => "VU",
            Wf => "WF",
            Ws => "WS",
            Xk => "XK",
            Ye => "YE",
            Yt => "YT",
            Za => "ZA",
            Zm => "ZM",
            Zw => "ZW",
            Zz => "ZZ",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionShippingAddressCollectionAllowedCountries::*;
        match s {
            "AC" => Ok(Ac),
            "AD" => Ok(Ad),
            "AE" => Ok(Ae),
            "AF" => Ok(Af),
            "AG" => Ok(Ag),
            "AI" => Ok(Ai),
            "AL" => Ok(Al),
            "AM" => Ok(Am),
            "AO" => Ok(Ao),
            "AQ" => Ok(Aq),
            "AR" => Ok(Ar),
            "AT" => Ok(At),
            "AU" => Ok(Au),
            "AW" => Ok(Aw),
            "AX" => Ok(Ax),
            "AZ" => Ok(Az),
            "BA" => Ok(Ba),
            "BB" => Ok(Bb),
            "BD" => Ok(Bd),
            "BE" => Ok(Be),
            "BF" => Ok(Bf),
            "BG" => Ok(Bg),
            "BH" => Ok(Bh),
            "BI" => Ok(Bi),
            "BJ" => Ok(Bj),
            "BL" => Ok(Bl),
            "BM" => Ok(Bm),
            "BN" => Ok(Bn),
            "BO" => Ok(Bo),
            "BQ" => Ok(Bq),
            "BR" => Ok(Br),
            "BS" => Ok(Bs),
            "BT" => Ok(Bt),
            "BV" => Ok(Bv),
            "BW" => Ok(Bw),
            "BY" => Ok(By),
            "BZ" => Ok(Bz),
            "CA" => Ok(Ca),
            "CD" => Ok(Cd),
            "CF" => Ok(Cf),
            "CG" => Ok(Cg),
            "CH" => Ok(Ch),
            "CI" => Ok(Ci),
            "CK" => Ok(Ck),
            "CL" => Ok(Cl),
            "CM" => Ok(Cm),
            "CN" => Ok(Cn),
            "CO" => Ok(Co),
            "CR" => Ok(Cr),
            "CV" => Ok(Cv),
            "CW" => Ok(Cw),
            "CY" => Ok(Cy),
            "CZ" => Ok(Cz),
            "DE" => Ok(De),
            "DJ" => Ok(Dj),
            "DK" => Ok(Dk),
            "DM" => Ok(Dm),
            "DO" => Ok(Do),
            "DZ" => Ok(Dz),
            "EC" => Ok(Ec),
            "EE" => Ok(Ee),
            "EG" => Ok(Eg),
            "EH" => Ok(Eh),
            "ER" => Ok(Er),
            "ES" => Ok(Es),
            "ET" => Ok(Et),
            "FI" => Ok(Fi),
            "FJ" => Ok(Fj),
            "FK" => Ok(Fk),
            "FO" => Ok(Fo),
            "FR" => Ok(Fr),
            "GA" => Ok(Ga),
            "GB" => Ok(Gb),
            "GD" => Ok(Gd),
            "GE" => Ok(Ge),
            "GF" => Ok(Gf),
            "GG" => Ok(Gg),
            "GH" => Ok(Gh),
            "GI" => Ok(Gi),
            "GL" => Ok(Gl),
            "GM" => Ok(Gm),
            "GN" => Ok(Gn),
            "GP" => Ok(Gp),
            "GQ" => Ok(Gq),
            "GR" => Ok(Gr),
            "GS" => Ok(Gs),
            "GT" => Ok(Gt),
            "GU" => Ok(Gu),
            "GW" => Ok(Gw),
            "GY" => Ok(Gy),
            "HK" => Ok(Hk),
            "HN" => Ok(Hn),
            "HR" => Ok(Hr),
            "HT" => Ok(Ht),
            "HU" => Ok(Hu),
            "ID" => Ok(Id),
            "IE" => Ok(Ie),
            "IL" => Ok(Il),
            "IM" => Ok(Im),
            "IN" => Ok(In),
            "IO" => Ok(Io),
            "IQ" => Ok(Iq),
            "IS" => Ok(Is),
            "IT" => Ok(It),
            "JE" => Ok(Je),
            "JM" => Ok(Jm),
            "JO" => Ok(Jo),
            "JP" => Ok(Jp),
            "KE" => Ok(Ke),
            "KG" => Ok(Kg),
            "KH" => Ok(Kh),
            "KI" => Ok(Ki),
            "KM" => Ok(Km),
            "KN" => Ok(Kn),
            "KR" => Ok(Kr),
            "KW" => Ok(Kw),
            "KY" => Ok(Ky),
            "KZ" => Ok(Kz),
            "LA" => Ok(La),
            "LB" => Ok(Lb),
            "LC" => Ok(Lc),
            "LI" => Ok(Li),
            "LK" => Ok(Lk),
            "LR" => Ok(Lr),
            "LS" => Ok(Ls),
            "LT" => Ok(Lt),
            "LU" => Ok(Lu),
            "LV" => Ok(Lv),
            "LY" => Ok(Ly),
            "MA" => Ok(Ma),
            "MC" => Ok(Mc),
            "MD" => Ok(Md),
            "ME" => Ok(Me),
            "MF" => Ok(Mf),
            "MG" => Ok(Mg),
            "MK" => Ok(Mk),
            "ML" => Ok(Ml),
            "MM" => Ok(Mm),
            "MN" => Ok(Mn),
            "MO" => Ok(Mo),
            "MQ" => Ok(Mq),
            "MR" => Ok(Mr),
            "MS" => Ok(Ms),
            "MT" => Ok(Mt),
            "MU" => Ok(Mu),
            "MV" => Ok(Mv),
            "MW" => Ok(Mw),
            "MX" => Ok(Mx),
            "MY" => Ok(My),
            "MZ" => Ok(Mz),
            "NA" => Ok(Na),
            "NC" => Ok(Nc),
            "NE" => Ok(Ne),
            "NG" => Ok(Ng),
            "NI" => Ok(Ni),
            "NL" => Ok(Nl),
            "NO" => Ok(No),
            "NP" => Ok(Np),
            "NR" => Ok(Nr),
            "NU" => Ok(Nu),
            "NZ" => Ok(Nz),
            "OM" => Ok(Om),
            "PA" => Ok(Pa),
            "PE" => Ok(Pe),
            "PF" => Ok(Pf),
            "PG" => Ok(Pg),
            "PH" => Ok(Ph),
            "PK" => Ok(Pk),
            "PL" => Ok(Pl),
            "PM" => Ok(Pm),
            "PN" => Ok(Pn),
            "PR" => Ok(Pr),
            "PS" => Ok(Ps),
            "PT" => Ok(Pt),
            "PY" => Ok(Py),
            "QA" => Ok(Qa),
            "RE" => Ok(Re),
            "RO" => Ok(Ro),
            "RS" => Ok(Rs),
            "RU" => Ok(Ru),
            "RW" => Ok(Rw),
            "SA" => Ok(Sa),
            "SB" => Ok(Sb),
            "SC" => Ok(Sc),
            "SD" => Ok(Sd),
            "SE" => Ok(Se),
            "SG" => Ok(Sg),
            "SH" => Ok(Sh),
            "SI" => Ok(Si),
            "SJ" => Ok(Sj),
            "SK" => Ok(Sk),
            "SL" => Ok(Sl),
            "SM" => Ok(Sm),
            "SN" => Ok(Sn),
            "SO" => Ok(So),
            "SR" => Ok(Sr),
            "SS" => Ok(Ss),
            "ST" => Ok(St),
            "SV" => Ok(Sv),
            "SX" => Ok(Sx),
            "SZ" => Ok(Sz),
            "TA" => Ok(Ta),
            "TC" => Ok(Tc),
            "TD" => Ok(Td),
            "TF" => Ok(Tf),
            "TG" => Ok(Tg),
            "TH" => Ok(Th),
            "TJ" => Ok(Tj),
            "TK" => Ok(Tk),
            "TL" => Ok(Tl),
            "TM" => Ok(Tm),
            "TN" => Ok(Tn),
            "TO" => Ok(To),
            "TR" => Ok(Tr),
            "TT" => Ok(Tt),
            "TV" => Ok(Tv),
            "TW" => Ok(Tw),
            "TZ" => Ok(Tz),
            "UA" => Ok(Ua),
            "UG" => Ok(Ug),
            "US" => Ok(Us),
            "UY" => Ok(Uy),
            "UZ" => Ok(Uz),
            "VA" => Ok(Va),
            "VC" => Ok(Vc),
            "VE" => Ok(Ve),
            "VG" => Ok(Vg),
            "VN" => Ok(Vn),
            "VU" => Ok(Vu),
            "WF" => Ok(Wf),
            "WS" => Ok(Ws),
            "XK" => Ok(Xk),
            "YE" => Ok(Ye),
            "YT" => Ok(Yt),
            "ZA" => Ok(Za),
            "ZM" => Ok(Zm),
            "ZW" => Ok(Zw),
            "ZZ" => Ok(Zz),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionShippingAddressCollectionAllowedCountries
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// The shipping rate options to apply to this Session. Up to a maximum of 5.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionShippingOptions {
    /// The ID of the Shipping Rate to use for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
    /// Parameters to be passed to Shipping Rate creation for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<CreateCheckoutSessionShippingOptionsShippingRateData>,
}
impl CreateCheckoutSessionShippingOptions {
    pub fn new() -> Self {
        Self { shipping_rate: None, shipping_rate_data: None }
    }
}
impl Default for CreateCheckoutSessionShippingOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Parameters to be passed to Shipping Rate creation for this shipping option.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateData {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate:
        Option<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    pub display_name: String,
    /// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// The type of calculation to use on the shipping rate.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateCheckoutSessionShippingOptionsShippingRateDataType>,
}
impl CreateCheckoutSessionShippingOptionsShippingRateData {
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
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
/// This will appear on CheckoutSessions.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum:
        Option<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum>,
    /// The lower bound of the estimated range. If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum:
        Option<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum>,
}
impl CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate {
    pub fn new() -> Self {
        Self { maximum: None, minimum: None }
    }
}
impl Default for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate {
    fn default() -> Self {
        Self::new()
    }
}
/// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum {
    pub fn new(
        unit: impl Into<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit>,
        value: impl Into<i64>,
    ) -> Self {
        Self { unit: unit.into(), value: value.into() }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}
impl CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit"))
    }
}
/// The lower bound of the estimated range. If empty, represents no lower bound.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum {
    pub fn new(
        unit: impl Into<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit>,
        value: impl Into<i64>,
    ) -> Self {
        Self { unit: unit.into(), value: value.into() }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}
impl CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit"))
    }
}
/// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        std::collections::HashMap<
            stripe_types::Currency,
            CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions,
        >,
    >,
}
impl CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount {
    pub fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self { amount: amount.into(), currency: currency.into(), currency_options: None }
    }
}
/// Shipping rates defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<
        CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior,
    >,
}
impl CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions {
    pub fn new(amount: impl Into<i64>) -> Self {
        Self { amount: amount.into(), tax_behavior: None }
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior"))
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior",
            )
        })
    }
}
/// The type of calculation to use on the shipping rate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataType {
    FixedAmount,
}
impl CreateCheckoutSessionShippingOptionsShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionShippingOptionsShippingRateDataType::*;
        match self {
            FixedAmount => "fixed_amount",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionShippingOptionsShippingRateDataType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionShippingOptionsShippingRateDataType::*;
        match s {
            "fixed_amount" => Ok(FixedAmount),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionShippingOptionsShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionShippingOptionsShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionShippingOptionsShippingRateDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionShippingOptionsShippingRateDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionShippingOptionsShippingRateDataType",
            )
        })
    }
}
/// A subset of parameters to be passed to subscription creation for Checkout Sessions in `subscription` mode.
#[derive(Clone, Debug, serde::Serialize)]
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
    /// for rendering in the [customer portal](https://stripe.com/docs/customer-management).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateCheckoutSessionSubscriptionDataInvoiceSettings>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The account on behalf of which to charge, for each of the subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
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
impl CreateCheckoutSessionSubscriptionData {
    pub fn new() -> Self {
        Self {
            application_fee_percent: None,
            billing_cycle_anchor: None,
            billing_mode: None,
            default_tax_rates: None,
            description: None,
            invoice_settings: None,
            metadata: None,
            on_behalf_of: None,
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
/// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataBillingMode {
    /// Configure behavior for flexible billing mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible: Option<CreateCheckoutSessionSubscriptionDataBillingModeFlexible>,
    /// Controls the calculation and orchestration of prorations and invoices for subscriptions.
    /// If no value is passed, the default is `flexible`.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionSubscriptionDataBillingModeType,
}
impl CreateCheckoutSessionSubscriptionDataBillingMode {
    pub fn new(type_: impl Into<CreateCheckoutSessionSubscriptionDataBillingModeType>) -> Self {
        Self { flexible: None, type_: type_.into() }
    }
}
/// Configure behavior for flexible billing mode.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataBillingModeFlexible {
    /// Controls how invoices and invoice items display proration amounts and discount amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_discounts:
        Option<CreateCheckoutSessionSubscriptionDataBillingModeFlexibleProrationDiscounts>,
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
/// Controls how invoices and invoice items display proration amounts and discount amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionSubscriptionDataBillingModeFlexibleProrationDiscounts {
    Included,
    Itemized,
}
impl CreateCheckoutSessionSubscriptionDataBillingModeFlexibleProrationDiscounts {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionSubscriptionDataBillingModeFlexibleProrationDiscounts::*;
        match self {
            Included => "included",
            Itemized => "itemized",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionSubscriptionDataBillingModeFlexibleProrationDiscounts
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionSubscriptionDataBillingModeFlexibleProrationDiscounts::*;
        match s {
            "included" => Ok(Included),
            "itemized" => Ok(Itemized),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionSubscriptionDataBillingModeFlexibleProrationDiscounts
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionSubscriptionDataBillingModeFlexibleProrationDiscounts
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionSubscriptionDataBillingModeFlexibleProrationDiscounts
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionSubscriptionDataBillingModeFlexibleProrationDiscounts
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionSubscriptionDataBillingModeFlexibleProrationDiscounts"))
    }
}
/// Controls the calculation and orchestration of prorations and invoices for subscriptions.
/// If no value is passed, the default is `flexible`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionSubscriptionDataBillingModeType {
    Classic,
    Flexible,
}
impl CreateCheckoutSessionSubscriptionDataBillingModeType {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionSubscriptionDataBillingModeType::*;
        match self {
            Classic => "classic",
            Flexible => "flexible",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionSubscriptionDataBillingModeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionSubscriptionDataBillingModeType::*;
        match s {
            "classic" => Ok(Classic),
            "flexible" => Ok(Flexible),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionSubscriptionDataBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionSubscriptionDataBillingModeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionSubscriptionDataBillingModeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionSubscriptionDataBillingModeType",
            )
        })
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataInvoiceSettings {
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType,
}
impl CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer {
    pub fn new(
        type_: impl Into<CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType {
    Account,
    Self_,
}
impl CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType",
            )
        })
    }
}
/// Determines how to handle prorations resulting from the `billing_cycle_anchor`.
/// If no value is passed, the default is `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionSubscriptionDataProrationBehavior {
    CreateProrations,
    None,
}
impl CreateCheckoutSessionSubscriptionDataProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionSubscriptionDataProrationBehavior::*;
        match self {
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionSubscriptionDataProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionSubscriptionDataProrationBehavior::*;
        match s {
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionSubscriptionDataProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionSubscriptionDataProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionSubscriptionDataProrationBehavior",
            )
        })
    }
}
/// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTransferData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: String,
}
impl CreateCheckoutSessionSubscriptionDataTransferData {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount_percent: None, destination: destination.into() }
    }
}
/// Settings related to subscription trials.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTrialSettings {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior,
}
impl CreateCheckoutSessionSubscriptionDataTrialSettings {
    pub fn new(
        end_behavior: impl Into<CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior>,
    ) -> Self {
        Self { end_behavior: end_behavior.into() }
    }
}
/// Defines how the subscription should behave when the user's free trial ends.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method:
        CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
}
impl CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior {
    pub fn new(
        missing_payment_method: impl Into<
            CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
        >,
    ) -> Self {
        Self { missing_payment_method: missing_payment_method.into() }
    }
}
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}
impl CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match self {
            Cancel => "cancel",
            CreateInvoice => "create_invoice",
            Pause => "pause",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match s {
            "cancel" => Ok(Cancel),
            "create_invoice" => Ok(CreateInvoice),
            "pause" => Ok(Pause),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod"))
    }
}
/// Controls tax ID collection during checkout.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionTaxIdCollection {
    /// Enable tax ID collection during checkout. Defaults to `false`.
    pub enabled: bool,
    /// Describes whether a tax ID is required during checkout. Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<CreateCheckoutSessionTaxIdCollectionRequired>,
}
impl CreateCheckoutSessionTaxIdCollection {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), required: None }
    }
}
/// Describes whether a tax ID is required during checkout. Defaults to `never`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionTaxIdCollectionRequired {
    IfSupported,
    Never,
}
impl CreateCheckoutSessionTaxIdCollectionRequired {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionTaxIdCollectionRequired::*;
        match self {
            IfSupported => "if_supported",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionTaxIdCollectionRequired {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionTaxIdCollectionRequired::*;
        match s {
            "if_supported" => Ok(IfSupported),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionTaxIdCollectionRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionTaxIdCollectionRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionTaxIdCollectionRequired {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionTaxIdCollectionRequired {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionTaxIdCollectionRequired",
            )
        })
    }
}
/// Wallet-specific configuration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionWalletOptions {
    /// contains details about the Link wallet options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateCheckoutSessionWalletOptionsLink>,
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
/// contains details about the Link wallet options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionWalletOptionsLink {
    /// Specifies whether Checkout should display Link as a payment option.
    /// By default, Checkout will display all the supported wallets that the Checkout Session was created with.
    /// This is the `auto` behavior, and it is the default choice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<CreateCheckoutSessionWalletOptionsLinkDisplay>,
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
/// Specifies whether Checkout should display Link as a payment option.
/// By default, Checkout will display all the supported wallets that the Checkout Session was created with.
/// This is the `auto` behavior, and it is the default choice.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCheckoutSessionWalletOptionsLinkDisplay {
    Auto,
    Never,
}
impl CreateCheckoutSessionWalletOptionsLinkDisplay {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionWalletOptionsLinkDisplay::*;
        match self {
            Auto => "auto",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionWalletOptionsLinkDisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionWalletOptionsLinkDisplay::*;
        match s {
            "auto" => Ok(Auto),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCheckoutSessionWalletOptionsLinkDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCheckoutSessionWalletOptionsLinkDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCheckoutSessionWalletOptionsLinkDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionWalletOptionsLinkDisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCheckoutSessionWalletOptionsLinkDisplay",
            )
        })
    }
}
/// Creates a Checkout Session object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSession {
    inner: CreateCheckoutSessionBuilder,
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
        billing_address_collection: impl Into<stripe_shared::CheckoutSessionBillingAddressCollection>,
    ) -> Self {
        self.inner.billing_address_collection = Some(billing_address_collection.into());
        self
    }
    /// The branding settings for the Checkout Session.
    /// This parameter is not allowed if ui_mode is `custom`.
    pub fn branding_settings(
        mut self,
        branding_settings: impl Into<CreateCheckoutSessionBrandingSettings>,
    ) -> Self {
        self.inner.branding_settings = Some(branding_settings.into());
        self
    }
    /// If set, Checkout displays a back button and customers will be directed to this URL if they decide to cancel payment and return to your website.
    /// This parameter is not allowed if ui_mode is `embedded` or `custom`.
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
    pub fn custom_fields(
        mut self,
        custom_fields: impl Into<Vec<CreateCheckoutSessionCustomFields>>,
    ) -> Self {
        self.inner.custom_fields = Some(custom_fields.into());
        self
    }
    /// Display additional text for your customers using custom text.
    pub fn custom_text(mut self, custom_text: impl Into<CreateCheckoutSessionCustomText>) -> Self {
        self.inner.custom_text = Some(custom_text.into());
        self
    }
    /// ID of an existing Customer, if one exists.
    /// In `payment` mode, the customer’s most recently saved card.
    /// payment method will be used to prefill the email, name, card details, and billing address
    /// on the Checkout page.
    /// In `subscription` mode, the customer’s [default payment method](https://stripe.com/docs/api/customers/update#update_customer-invoice_settings-default_payment_method).
    /// will be used if it’s a card, otherwise the most recently saved card will be used.
    /// A valid billing address, billing name and billing email are required on the payment method for Checkout to prefill the customer's card details.
    ///
    /// If the Customer already has a valid [email](https://stripe.com/docs/api/customers/object#customer_object-email) set, the email will be prefilled and not editable in Checkout.
    /// If the Customer does not have a valid `email`, Checkout will set the email entered during the session on the Customer.
    ///
    /// If blank for Checkout Sessions in `subscription` mode or with `customer_creation` set as `always` in `payment` mode, Checkout will create a new Customer object based on information provided during the payment flow.
    ///
    /// You can set [`payment_intent_data.setup_future_usage`](https://stripe.com/docs/api/checkout/sessions/create#create_checkout_session-payment_intent_data-setup_future_usage) to have Checkout automatically attach the payment method to the Customer you pass in for future reuse.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// Configure whether a Checkout Session creates a [Customer](https://stripe.com/docs/api/customers) during Session confirmation.
    ///
    /// When a Customer is not created, you can still retrieve email, address, and other customer data entered in Checkout.
    /// with [customer_details](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-customer_details).
    ///
    /// Sessions that don't create Customers instead are grouped by [guest customers](https://stripe.com/docs/payments/checkout/guest-customers).
    /// in the Dashboard.
    /// Promotion codes limited to first time customers will return invalid for these Sessions.
    ///
    /// Can only be set in `payment` and `setup` mode.
    pub fn customer_creation(
        mut self,
        customer_creation: impl Into<CreateCheckoutSessionCustomerCreation>,
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
        excluded_payment_method_types: impl Into<Vec<CreateCheckoutSessionExcludedPaymentMethodTypes>>,
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
    /// Generate a post-purchase Invoice for one-time payments.
    pub fn invoice_creation(
        mut self,
        invoice_creation: impl Into<CreateCheckoutSessionInvoiceCreation>,
    ) -> Self {
        self.inner.invoice_creation = Some(invoice_creation.into());
        self
    }
    /// A list of items the customer is purchasing.
    /// Use this parameter to pass one-time or recurring [Prices](https://stripe.com/docs/api/prices).
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
    /// If a [Customer](https://stripe.com/docs/api/customers) is created or provided, the names can be saved to the Customer object as well.
    pub fn name_collection(
        mut self,
        name_collection: impl Into<CreateCheckoutSessionNameCollection>,
    ) -> Self {
        self.inner.name_collection = Some(name_collection.into());
        self
    }
    /// A list of optional items the customer can add to their order at checkout.
    /// Use this parameter to pass one-time or recurring [Prices](https://stripe.com/docs/api/prices).
    ///
    /// There is a maximum of 10 optional items allowed on a Checkout Session, and the existing limits on the number of line items allowed on a Checkout Session apply to the combined number of line items and optional items.
    ///
    /// For `payment` mode, there is a maximum of 100 combined line items and optional items, however it is recommended to consolidate items if there are more than a few dozen.
    ///
    /// For `subscription` mode, there is a maximum of 20 line items and optional items with recurring Prices and 20 line items and optional items with one-time Prices.
    pub fn optional_items(
        mut self,
        optional_items: impl Into<Vec<CreateCheckoutSessionOptionalItems>>,
    ) -> Self {
        self.inner.optional_items = Some(optional_items.into());
        self
    }
    /// Where the user is coming from. This informs the optimizations that are applied to the session.
    pub fn origin_context(
        mut self,
        origin_context: impl Into<stripe_shared::CheckoutSessionOriginContext>,
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
    /// If you'd like information on how to collect a payment method outside of Checkout, read the guide on configuring [subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
    pub fn payment_method_collection(
        mut self,
        payment_method_collection: impl Into<CreateCheckoutSessionPaymentMethodCollection>,
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
    /// See [Dynamic Payment Methods](https://stripe.com/docs/payments/payment-methods/integration-options#using-dynamic-payment-methods) for more details.
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
    /// Learn more about [collecting phone numbers with Checkout](https://stripe.com/docs/payments/checkout/phone-numbers).
    pub fn phone_number_collection(
        mut self,
        phone_number_collection: impl Into<CreateCheckoutSessionPhoneNumberCollection>,
    ) -> Self {
        self.inner.phone_number_collection = Some(phone_number_collection.into());
        self
    }
    /// This parameter applies to `ui_mode: embedded`.
    /// Learn more about the [redirect behavior](https://stripe.com/docs/payments/checkout/custom-success-page?payment-ui=embedded-form) of embedded sessions.
    /// Defaults to `always`.
    pub fn redirect_on_completion(
        mut self,
        redirect_on_completion: impl Into<stripe_shared::CheckoutSessionRedirectOnCompletion>,
    ) -> Self {
        self.inner.redirect_on_completion = Some(redirect_on_completion.into());
        self
    }
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the
    /// payment method's app or site. This parameter is required if `ui_mode` is `embedded` or `custom`
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
        shipping_options: impl Into<Vec<CreateCheckoutSessionShippingOptions>>,
    ) -> Self {
        self.inner.shipping_options = Some(shipping_options.into());
        self
    }
    /// Describes the type of transaction being performed by Checkout in order
    /// to customize relevant text on the page, such as the submit button.
    ///  `submit_type` can only be specified on Checkout Sessions in
    /// `payment` or `subscription` mode. If blank or `auto`, `pay` is used.
    pub fn submit_type(
        mut self,
        submit_type: impl Into<stripe_shared::CheckoutSessionSubmitType>,
    ) -> Self {
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
    /// This parameter is not allowed if ui_mode is `embedded` or `custom`. If you'd like to use
    /// information from the successful Checkout Session on your page, read the
    /// guide on [customizing your success page](https://stripe.com/docs/payments/checkout/custom-success-page).
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
    /// The UI mode of the Session. Defaults to `hosted`.
    pub fn ui_mode(mut self, ui_mode: impl Into<stripe_shared::CheckoutSessionUiMode>) -> Self {
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
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateCheckoutSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    collected_information: Option<UpdateCheckoutSessionCollectedInformation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Vec<UpdateCheckoutSessionShippingOptions>>,
}
impl UpdateCheckoutSessionBuilder {
    fn new() -> Self {
        Self { collected_information: None, expand: None, metadata: None, shipping_options: None }
    }
}
/// Information about the customer collected within the Checkout Session.
/// Can only be set when updating `embedded` or `custom` sessions.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCheckoutSessionCollectedInformation {
    /// The shipping details to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<UpdateCheckoutSessionCollectedInformationShippingDetails>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCheckoutSessionCollectedInformationShippingDetails {
    /// The address of the customer
    pub address: UpdateCheckoutSessionCollectedInformationShippingDetailsAddress,
    /// The name of customer
    pub name: String,
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
#[derive(Clone, Debug, serde::Serialize)]
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
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
/// The shipping rate options to apply to this Session. Up to a maximum of 5.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCheckoutSessionShippingOptions {
    /// The ID of the Shipping Rate to use for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
    /// Parameters to be passed to Shipping Rate creation for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<UpdateCheckoutSessionShippingOptionsShippingRateData>,
}
impl UpdateCheckoutSessionShippingOptions {
    pub fn new() -> Self {
        Self { shipping_rate: None, shipping_rate_data: None }
    }
}
impl Default for UpdateCheckoutSessionShippingOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Parameters to be passed to Shipping Rate creation for this shipping option.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCheckoutSessionShippingOptionsShippingRateData {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate:
        Option<UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    pub display_name: String,
    /// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmount>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// The type of calculation to use on the shipping rate.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpdateCheckoutSessionShippingOptionsShippingRateDataType>,
}
impl UpdateCheckoutSessionShippingOptionsShippingRateData {
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
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
/// This will appear on CheckoutSessions.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum:
        Option<UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum>,
    /// The lower bound of the estimated range. If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum:
        Option<UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum>,
}
impl UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate {
    pub fn new() -> Self {
        Self { maximum: None, minimum: None }
    }
}
impl Default for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate {
    fn default() -> Self {
        Self::new()
    }
}
/// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum {
    pub fn new(
        unit: impl Into<UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit>,
        value: impl Into<i64>,
    ) -> Self {
        Self { unit: unit.into(), value: value.into() }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}
impl UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        use UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr
    for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit"))
    }
}
/// The lower bound of the estimated range. If empty, represents no lower bound.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum {
    pub fn new(
        unit: impl Into<UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit>,
        value: impl Into<i64>,
    ) -> Self {
        Self { unit: unit.into(), value: value.into() }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}
impl UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        use UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr
    for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit"))
    }
}
/// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        std::collections::HashMap<
            stripe_types::Currency,
            UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions,
        >,
    >,
}
impl UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmount {
    pub fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self { amount: amount.into(), currency: currency.into(), currency_options: None }
    }
}
/// Shipping rates defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<
        UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior,
    >,
}
impl UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions {
    pub fn new(amount: impl Into<i64>) -> Self {
        Self { amount: amount.into(), tax_behavior: None }
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr
    for UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior"))
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpdateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior",
            )
        })
    }
}
/// The type of calculation to use on the shipping rate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCheckoutSessionShippingOptionsShippingRateDataType {
    FixedAmount,
}
impl UpdateCheckoutSessionShippingOptionsShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        use UpdateCheckoutSessionShippingOptionsShippingRateDataType::*;
        match self {
            FixedAmount => "fixed_amount",
        }
    }
}

impl std::str::FromStr for UpdateCheckoutSessionShippingOptionsShippingRateDataType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCheckoutSessionShippingOptionsShippingRateDataType::*;
        match s {
            "fixed_amount" => Ok(FixedAmount),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateCheckoutSessionShippingOptionsShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateCheckoutSessionShippingOptionsShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateCheckoutSessionShippingOptionsShippingRateDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateCheckoutSessionShippingOptionsShippingRateDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateCheckoutSessionShippingOptionsShippingRateDataType",
            )
        })
    }
}
/// Updates a Checkout Session object.
///
/// Related guide: <a href="/payments/checkout/dynamic-updates">Dynamically update Checkout</a>
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCheckoutSession {
    inner: UpdateCheckoutSessionBuilder,
    session: stripe_shared::CheckoutSessionId,
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
    /// The shipping rate options to apply to this Session. Up to a maximum of 5.
    pub fn shipping_options(
        mut self,
        shipping_options: impl Into<Vec<UpdateCheckoutSessionShippingOptions>>,
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
#[derive(Clone, Debug, serde::Serialize)]
struct ExpireCheckoutSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl ExpireCheckoutSessionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// A Checkout Session can be expired when it is in one of these statuses: `open`
///
/// After it expires, a customer can’t complete a Checkout Session and customers loading the Checkout Session see a message saying the Checkout Session is expired.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ExpireCheckoutSession {
    inner: ExpireCheckoutSessionBuilder,
    session: stripe_shared::CheckoutSessionId,
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

#[derive(Clone, Debug, serde::Serialize)]
pub struct CustomTextPositionParam {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}
impl CustomTextPositionParam {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}
