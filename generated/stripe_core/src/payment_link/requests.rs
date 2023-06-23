use stripe::{Client, Response};

impl stripe_core::payment_link::PaymentLink {
    /// Returns a list of your payment links.
    pub fn list(
        client: &Client,
        params: ListPaymentLink,
    ) -> Response<stripe_types::List<stripe_core::payment_link::PaymentLink>> {
        client.get_query("/payment_links", params)
    }
    /// Retrieve a payment link.
    pub fn retrieve(
        client: &Client,
        payment_link: &stripe_core::payment_link::PaymentLinkId,
        params: RetrievePaymentLink,
    ) -> Response<stripe_core::payment_link::PaymentLink> {
        client.get_query(
            &format!("/payment_links/{payment_link}", payment_link = payment_link),
            params,
        )
    }
    /// When retrieving a payment link, there is an includable **line_items** property containing the first handful of those items.
    ///
    /// There is also a URL where you can retrieve the full (paginated) list of line items.
    pub fn list_line_items(
        client: &Client,
        payment_link: &stripe_core::payment_link::PaymentLinkId,
        params: ListLineItemsPaymentLink,
    ) -> Response<stripe_types::List<stripe_core::line_item::LineItem>> {
        client.get_query(
            &format!("/payment_links/{payment_link}/line_items", payment_link = payment_link),
            params,
        )
    }
    /// Creates a payment link.
    pub fn create(
        client: &Client,
        params: CreatePaymentLink,
    ) -> Response<stripe_core::payment_link::PaymentLink> {
        client.send_form("/payment_links", params, http_types::Method::Post)
    }
    /// Updates a payment link.
    pub fn update(
        client: &Client,
        payment_link: &stripe_core::payment_link::PaymentLinkId,
        params: UpdatePaymentLink,
    ) -> Response<stripe_core::payment_link::PaymentLink> {
        client.send_form(
            &format!("/payment_links/{payment_link}", payment_link = payment_link),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListPaymentLink<'a> {
    /// Only return payment links that are active or inactive (e.g., pass `false` to list all inactive payment links).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
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
impl<'a> ListPaymentLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePaymentLink<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListLineItemsPaymentLink<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
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
impl<'a> ListLineItemsPaymentLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLink<'a> {
    /// Behavior after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<CreatePaymentLinkAfterCompletion<'a>>,
    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// Can only be applied when there are no line items with recurring prices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Configuration for automatic tax collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreatePaymentLinkAutomaticTax>,
    /// Configuration for collecting the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<CreatePaymentLinkBillingAddressCollection>,
    /// Configure fields to gather active consent from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<CreatePaymentLinkConsentCollection>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies) and supported by each line item's price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<CreatePaymentLinkCustomerCreation>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The line items representing what is being sold.
    ///
    /// Each line item represents an item being sold.
    /// Up to 20 line items are supported.
    pub line_items: &'a [CreatePaymentLinkLineItems<'a>],
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// Metadata associated with this Payment Link will automatically be copied to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The account on behalf of which to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<CreatePaymentLinkPaymentIntentData>,
    /// Specify whether Checkout should collect a payment method.
    ///
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<CreatePaymentLinkPaymentMethodCollection>,
    /// The list of payment method types that customers can use.
    ///
    /// If no value is passed, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods) (20+ payment methods [supported](https://stripe.com/docs/payments/payment-methods/integration-options#payment-method-product-support)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [CreatePaymentLinkPaymentMethodTypes]>,
    /// Controls phone number collection settings during checkout.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<CreatePaymentLinkPhoneNumberCollection>,
    /// Configuration for collecting the customer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<CreatePaymentLinkShippingAddressCollection<'a>>,
    /// The shipping rate options to apply to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<&'a [CreatePaymentLinkShippingOptions<'a>]>,
    /// Describes the type of transaction being performed in order to customize relevant text on the page, such as the submit button.
    ///
    /// Changing this value will also affect the hostname in the [url](https://stripe.com/docs/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<CreatePaymentLinkSubmitType>,
    /// When creating a subscription, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<CreatePaymentLinkSubscriptionData<'a>>,
    /// Controls tax ID collection during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<CreatePaymentLinkTaxIdCollection>,
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreatePaymentLinkTransferData<'a>>,
}
impl<'a> CreatePaymentLink<'a> {
    pub fn new(line_items: &'a [CreatePaymentLinkLineItems<'a>]) -> Self {
        Self {
            after_completion: Default::default(),
            allow_promotion_codes: Default::default(),
            application_fee_amount: Default::default(),
            application_fee_percent: Default::default(),
            automatic_tax: Default::default(),
            billing_address_collection: Default::default(),
            consent_collection: Default::default(),
            currency: Default::default(),
            customer_creation: Default::default(),
            expand: Default::default(),
            line_items,
            metadata: Default::default(),
            on_behalf_of: Default::default(),
            payment_intent_data: Default::default(),
            payment_method_collection: Default::default(),
            payment_method_types: Default::default(),
            phone_number_collection: Default::default(),
            shipping_address_collection: Default::default(),
            shipping_options: Default::default(),
            submit_type: Default::default(),
            subscription_data: Default::default(),
            tax_id_collection: Default::default(),
            transfer_data: Default::default(),
        }
    }
}
/// Behavior after the purchase is complete.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkAfterCompletion<'a> {
    /// Configuration when `type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<CreatePaymentLinkAfterCompletionHostedConfirmation<'a>>,
    /// Configuration when `type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<CreatePaymentLinkAfterCompletionRedirect<'a>>,
    /// The specified behavior after the purchase is complete.
    ///
    /// Either `redirect` or `hosted_confirmation`.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkAfterCompletionType,
}
impl<'a> CreatePaymentLinkAfterCompletion<'a> {
    pub fn new(type_: CreatePaymentLinkAfterCompletionType) -> Self {
        Self { hosted_confirmation: Default::default(), redirect: Default::default(), type_ }
    }
}
/// Configuration when `type=hosted_confirmation`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkAfterCompletionHostedConfirmation<'a> {
    /// A custom message to display to the customer after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<&'a str>,
}
impl<'a> CreatePaymentLinkAfterCompletionHostedConfirmation<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration when `type=redirect`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkAfterCompletionRedirect<'a> {
    /// The URL the customer will be redirected to after the purchase is complete.
    ///
    /// You can embed `{CHECKOUT_SESSION_ID}` into the URL to have the `id` of the completed [checkout session](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-id) included.
    pub url: &'a str,
}
impl<'a> CreatePaymentLinkAfterCompletionRedirect<'a> {
    pub fn new(url: &'a str) -> Self {
        Self { url }
    }
}
/// The specified behavior after the purchase is complete.
///
/// Either `redirect` or `hosted_confirmation`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl CreatePaymentLinkAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HostedConfirmation => "hosted_confirmation",
            Self::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configuration for automatic tax collection.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkAutomaticTax {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,
}
impl CreatePaymentLinkAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Configuration for collecting the customer's billing address.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkBillingAddressCollection {
    Auto,
    Required,
}

impl CreatePaymentLinkBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Required => "required",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkBillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configure fields to gather active consent from customers.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkConsentCollection {
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    ///
    /// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
    /// Only available to US merchants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<CreatePaymentLinkConsentCollectionPromotions>,
    /// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
    /// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<CreatePaymentLinkConsentCollectionTermsOfService>,
}
impl CreatePaymentLinkConsentCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If set to `auto`, enables the collection of customer consent for promotional communications.
///
/// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
/// Only available to US merchants.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkConsentCollectionPromotions {
    Auto,
    None,
}

impl CreatePaymentLinkConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkConsentCollectionPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
/// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkConsentCollectionTermsOfService {
    None,
    Required,
}

impl CreatePaymentLinkConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Required => "required",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkConsentCollectionTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkCustomerCreation {
    Always,
    IfRequired,
}

impl CreatePaymentLinkCustomerCreation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::IfRequired => "if_required",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkCustomerCreation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The line items representing what is being sold.
///
/// Each line item represents an item being sold.
/// Up to 20 line items are supported.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkLineItems<'a> {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreatePaymentLinkLineItemsAdjustableQuantity>,
    /// The ID of the [Price](https://stripe.com/docs/api/prices) or [Plan](https://stripe.com/docs/api/plans) object.
    pub price: &'a str,
    /// The quantity of the line item being purchased.
    pub quantity: u64,
}
impl<'a> CreatePaymentLinkLineItems<'a> {
    pub fn new(price: &'a str, quantity: u64) -> Self {
        Self { adjustable_quantity: Default::default(), price, quantity }
    }
}
/// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkLineItemsAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative Integer.
    pub enabled: bool,
    /// The maximum quantity the customer can purchase.
    ///
    /// By default this value is 99.
    /// You can specify a value up to 99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum quantity the customer can purchase.
    ///
    /// By default this value is 0.
    /// You can specify a value up to 98.
    /// If there is only one item in the cart then that item's quantity cannot go down to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}
impl CreatePaymentLinkLineItemsAdjustableQuantity {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, maximum: Default::default(), minimum: Default::default() }
    }
}
/// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkPaymentIntentData {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreatePaymentLinkPaymentIntentDataCaptureMethod>,
    /// Indicates that you intend to [make future payments](https://stripe.com/docs/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
    ///
    /// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
    ///
    /// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
    ///
    /// If a Customer has been provided or Checkout creates a new Customer,Checkout will attach the payment method to the Customer.
    ///
    /// If Checkout does not create a Customer, the payment method is not attached to a Customer.
    ///
    /// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.  When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentLinkPaymentIntentDataSetupFutureUsage>,
}
impl CreatePaymentLinkPaymentIntentData {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkPaymentIntentDataCaptureMethod {
    Automatic,
    Manual,
}

impl CreatePaymentLinkPaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to [make future payments](https://stripe.com/docs/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
///
/// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
///
/// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
///
/// If a Customer has been provided or Checkout creates a new Customer,Checkout will attach the payment method to the Customer.
///
/// If Checkout does not create a Customer, the payment method is not attached to a Customer.
///
/// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.  When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}

impl CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specify whether Checkout should collect a payment method.
///
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkPaymentMethodCollection {
    Always,
    IfRequired,
}

impl CreatePaymentLinkPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::IfRequired => "if_required",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkPaymentMethodCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The list of payment method types that customers can use.
///
/// If no value is passed, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods) (20+ payment methods [supported](https://stripe.com/docs/payments/payment-methods/integration-options#payment-method-product-support)).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkPaymentMethodTypes {
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Oxxo,
    P24,
    Paynow,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl CreatePaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Controls phone number collection settings during checkout.
///
/// We recommend that you review your privacy policy and check with your legal contacts.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkPhoneNumberCollection {
    /// Set to `true` to enable phone number collection.
    pub enabled: bool,
}
impl CreatePaymentLinkPhoneNumberCollection {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Configuration for collecting the customer's shipping address.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkShippingAddressCollection<'a> {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
    /// shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: &'a [CreatePaymentLinkShippingAddressCollectionAllowedCountries],
}
impl<'a> CreatePaymentLinkShippingAddressCollection<'a> {
    pub fn new(
        allowed_countries: &'a [CreatePaymentLinkShippingAddressCollectionAllowedCountries],
    ) -> Self {
        Self { allowed_countries }
    }
}
/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
/// shipping locations.
///
/// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    #[serde(rename = "AC")]
    Ac,
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "TA")]
    Ta,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "XK")]
    Xk,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
    #[serde(rename = "ZZ")]
    Zz,
}

impl CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ac => "AC",
            Self::Ad => "AD",
            Self::Ae => "AE",
            Self::Af => "AF",
            Self::Ag => "AG",
            Self::Ai => "AI",
            Self::Al => "AL",
            Self::Am => "AM",
            Self::Ao => "AO",
            Self::Aq => "AQ",
            Self::Ar => "AR",
            Self::At => "AT",
            Self::Au => "AU",
            Self::Aw => "AW",
            Self::Ax => "AX",
            Self::Az => "AZ",
            Self::Ba => "BA",
            Self::Bb => "BB",
            Self::Bd => "BD",
            Self::Be => "BE",
            Self::Bf => "BF",
            Self::Bg => "BG",
            Self::Bh => "BH",
            Self::Bi => "BI",
            Self::Bj => "BJ",
            Self::Bl => "BL",
            Self::Bm => "BM",
            Self::Bn => "BN",
            Self::Bo => "BO",
            Self::Bq => "BQ",
            Self::Br => "BR",
            Self::Bs => "BS",
            Self::Bt => "BT",
            Self::Bv => "BV",
            Self::Bw => "BW",
            Self::By => "BY",
            Self::Bz => "BZ",
            Self::Ca => "CA",
            Self::Cd => "CD",
            Self::Cf => "CF",
            Self::Cg => "CG",
            Self::Ch => "CH",
            Self::Ci => "CI",
            Self::Ck => "CK",
            Self::Cl => "CL",
            Self::Cm => "CM",
            Self::Cn => "CN",
            Self::Co => "CO",
            Self::Cr => "CR",
            Self::Cv => "CV",
            Self::Cw => "CW",
            Self::Cy => "CY",
            Self::Cz => "CZ",
            Self::De => "DE",
            Self::Dj => "DJ",
            Self::Dk => "DK",
            Self::Dm => "DM",
            Self::Do => "DO",
            Self::Dz => "DZ",
            Self::Ec => "EC",
            Self::Ee => "EE",
            Self::Eg => "EG",
            Self::Eh => "EH",
            Self::Er => "ER",
            Self::Es => "ES",
            Self::Et => "ET",
            Self::Fi => "FI",
            Self::Fj => "FJ",
            Self::Fk => "FK",
            Self::Fo => "FO",
            Self::Fr => "FR",
            Self::Ga => "GA",
            Self::Gb => "GB",
            Self::Gd => "GD",
            Self::Ge => "GE",
            Self::Gf => "GF",
            Self::Gg => "GG",
            Self::Gh => "GH",
            Self::Gi => "GI",
            Self::Gl => "GL",
            Self::Gm => "GM",
            Self::Gn => "GN",
            Self::Gp => "GP",
            Self::Gq => "GQ",
            Self::Gr => "GR",
            Self::Gs => "GS",
            Self::Gt => "GT",
            Self::Gu => "GU",
            Self::Gw => "GW",
            Self::Gy => "GY",
            Self::Hk => "HK",
            Self::Hn => "HN",
            Self::Hr => "HR",
            Self::Ht => "HT",
            Self::Hu => "HU",
            Self::Id => "ID",
            Self::Ie => "IE",
            Self::Il => "IL",
            Self::Im => "IM",
            Self::In => "IN",
            Self::Io => "IO",
            Self::Iq => "IQ",
            Self::Is => "IS",
            Self::It => "IT",
            Self::Je => "JE",
            Self::Jm => "JM",
            Self::Jo => "JO",
            Self::Jp => "JP",
            Self::Ke => "KE",
            Self::Kg => "KG",
            Self::Kh => "KH",
            Self::Ki => "KI",
            Self::Km => "KM",
            Self::Kn => "KN",
            Self::Kr => "KR",
            Self::Kw => "KW",
            Self::Ky => "KY",
            Self::Kz => "KZ",
            Self::La => "LA",
            Self::Lb => "LB",
            Self::Lc => "LC",
            Self::Li => "LI",
            Self::Lk => "LK",
            Self::Lr => "LR",
            Self::Ls => "LS",
            Self::Lt => "LT",
            Self::Lu => "LU",
            Self::Lv => "LV",
            Self::Ly => "LY",
            Self::Ma => "MA",
            Self::Mc => "MC",
            Self::Md => "MD",
            Self::Me => "ME",
            Self::Mf => "MF",
            Self::Mg => "MG",
            Self::Mk => "MK",
            Self::Ml => "ML",
            Self::Mm => "MM",
            Self::Mn => "MN",
            Self::Mo => "MO",
            Self::Mq => "MQ",
            Self::Mr => "MR",
            Self::Ms => "MS",
            Self::Mt => "MT",
            Self::Mu => "MU",
            Self::Mv => "MV",
            Self::Mw => "MW",
            Self::Mx => "MX",
            Self::My => "MY",
            Self::Mz => "MZ",
            Self::Na => "NA",
            Self::Nc => "NC",
            Self::Ne => "NE",
            Self::Ng => "NG",
            Self::Ni => "NI",
            Self::Nl => "NL",
            Self::No => "NO",
            Self::Np => "NP",
            Self::Nr => "NR",
            Self::Nu => "NU",
            Self::Nz => "NZ",
            Self::Om => "OM",
            Self::Pa => "PA",
            Self::Pe => "PE",
            Self::Pf => "PF",
            Self::Pg => "PG",
            Self::Ph => "PH",
            Self::Pk => "PK",
            Self::Pl => "PL",
            Self::Pm => "PM",
            Self::Pn => "PN",
            Self::Pr => "PR",
            Self::Ps => "PS",
            Self::Pt => "PT",
            Self::Py => "PY",
            Self::Qa => "QA",
            Self::Re => "RE",
            Self::Ro => "RO",
            Self::Rs => "RS",
            Self::Ru => "RU",
            Self::Rw => "RW",
            Self::Sa => "SA",
            Self::Sb => "SB",
            Self::Sc => "SC",
            Self::Se => "SE",
            Self::Sg => "SG",
            Self::Sh => "SH",
            Self::Si => "SI",
            Self::Sj => "SJ",
            Self::Sk => "SK",
            Self::Sl => "SL",
            Self::Sm => "SM",
            Self::Sn => "SN",
            Self::So => "SO",
            Self::Sr => "SR",
            Self::Ss => "SS",
            Self::St => "ST",
            Self::Sv => "SV",
            Self::Sx => "SX",
            Self::Sz => "SZ",
            Self::Ta => "TA",
            Self::Tc => "TC",
            Self::Td => "TD",
            Self::Tf => "TF",
            Self::Tg => "TG",
            Self::Th => "TH",
            Self::Tj => "TJ",
            Self::Tk => "TK",
            Self::Tl => "TL",
            Self::Tm => "TM",
            Self::Tn => "TN",
            Self::To => "TO",
            Self::Tr => "TR",
            Self::Tt => "TT",
            Self::Tv => "TV",
            Self::Tw => "TW",
            Self::Tz => "TZ",
            Self::Ua => "UA",
            Self::Ug => "UG",
            Self::Us => "US",
            Self::Uy => "UY",
            Self::Uz => "UZ",
            Self::Va => "VA",
            Self::Vc => "VC",
            Self::Ve => "VE",
            Self::Vg => "VG",
            Self::Vn => "VN",
            Self::Vu => "VU",
            Self::Wf => "WF",
            Self::Ws => "WS",
            Self::Xk => "XK",
            Self::Ye => "YE",
            Self::Yt => "YT",
            Self::Za => "ZA",
            Self::Zm => "ZM",
            Self::Zw => "ZW",
            Self::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The shipping rate options to apply to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkShippingOptions<'a> {
    /// The ID of the Shipping Rate to use for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
}
impl<'a> CreatePaymentLinkShippingOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Describes the type of transaction being performed in order to customize relevant text on the page, such as the submit button.
///
/// Changing this value will also affect the hostname in the [url](https://stripe.com/docs/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}

impl CreatePaymentLinkSubmitType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Book => "book",
            Self::Donate => "donate",
            Self::Pay => "pay",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkSubmitType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// When creating a subscription, the specified configuration data will be used.
///
/// There must be at least one line item with a recurring price to use `subscription_data`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentLinkSubscriptionData<'a> {
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    ///
    /// Has to be at least 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}
impl<'a> CreatePaymentLinkSubscriptionData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls tax ID collection during checkout.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkTaxIdCollection {
    /// Set to `true` to enable tax ID collection.
    pub enabled: bool,
}
impl CreatePaymentLinkTaxIdCollection {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkTransferData<'a> {
    /// The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// If specified, successful charges will be attributed to the destination
    /// account for tax reporting, and the funds from charges will be transferred
    /// to the destination account.
    ///
    /// The ID of the resulting transfer will be returned on the successful charge's `transfer` field.
    pub destination: &'a str,
}
impl<'a> CreatePaymentLinkTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: Default::default(), destination }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentLink<'a> {
    /// Whether the payment link's `url` is active.
    ///
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Behavior after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<UpdatePaymentLinkAfterCompletion<'a>>,
    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    /// Configuration for automatic tax collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdatePaymentLinkAutomaticTax>,
    /// Configuration for collecting the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<UpdatePaymentLinkBillingAddressCollection>,
    /// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<UpdatePaymentLinkCustomerCreation>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The line items representing what is being sold.
    ///
    /// Each line item represents an item being sold.
    /// Up to 20 line items are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<&'a [UpdatePaymentLinkLineItems<'a>]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// Metadata associated with this Payment Link will automatically be copied to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Specify whether Checkout should collect a payment method.
    ///
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<UpdatePaymentLinkPaymentMethodCollection>,
    /// The list of payment method types that customers can use.
    ///
    /// Pass an empty string to enable automatic payment methods that use your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [UpdatePaymentLinkPaymentMethodTypes]>,
    /// Configuration for collecting the customer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<UpdatePaymentLinkShippingAddressCollection<'a>>,
}
impl<'a> UpdatePaymentLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Behavior after the purchase is complete.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkAfterCompletion<'a> {
    /// Configuration when `type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<UpdatePaymentLinkAfterCompletionHostedConfirmation<'a>>,
    /// Configuration when `type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<UpdatePaymentLinkAfterCompletionRedirect<'a>>,
    /// The specified behavior after the purchase is complete.
    ///
    /// Either `redirect` or `hosted_confirmation`.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkAfterCompletionType,
}
impl<'a> UpdatePaymentLinkAfterCompletion<'a> {
    pub fn new(type_: UpdatePaymentLinkAfterCompletionType) -> Self {
        Self { hosted_confirmation: Default::default(), redirect: Default::default(), type_ }
    }
}
/// Configuration when `type=hosted_confirmation`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentLinkAfterCompletionHostedConfirmation<'a> {
    /// A custom message to display to the customer after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<&'a str>,
}
impl<'a> UpdatePaymentLinkAfterCompletionHostedConfirmation<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration when `type=redirect`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkAfterCompletionRedirect<'a> {
    /// The URL the customer will be redirected to after the purchase is complete.
    ///
    /// You can embed `{CHECKOUT_SESSION_ID}` into the URL to have the `id` of the completed [checkout session](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-id) included.
    pub url: &'a str,
}
impl<'a> UpdatePaymentLinkAfterCompletionRedirect<'a> {
    pub fn new(url: &'a str) -> Self {
        Self { url }
    }
}
/// The specified behavior after the purchase is complete.
///
/// Either `redirect` or `hosted_confirmation`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl UpdatePaymentLinkAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HostedConfirmation => "hosted_confirmation",
            Self::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configuration for automatic tax collection.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkAutomaticTax {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,
}
impl UpdatePaymentLinkAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Configuration for collecting the customer's billing address.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkBillingAddressCollection {
    Auto,
    Required,
}

impl UpdatePaymentLinkBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Required => "required",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkBillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkCustomerCreation {
    Always,
    IfRequired,
}

impl UpdatePaymentLinkCustomerCreation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::IfRequired => "if_required",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkCustomerCreation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The line items representing what is being sold.
///
/// Each line item represents an item being sold.
/// Up to 20 line items are supported.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkLineItems<'a> {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<UpdatePaymentLinkLineItemsAdjustableQuantity>,
    /// The ID of an existing line item on the payment link.
    pub id: &'a str,
    /// The quantity of the line item being purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}
impl<'a> UpdatePaymentLinkLineItems<'a> {
    pub fn new(id: &'a str) -> Self {
        Self { adjustable_quantity: Default::default(), id, quantity: Default::default() }
    }
}
/// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkLineItemsAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative Integer.
    pub enabled: bool,
    /// The maximum quantity the customer can purchase.
    ///
    /// By default this value is 99.
    /// You can specify a value up to 99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum quantity the customer can purchase.
    ///
    /// By default this value is 0.
    /// You can specify a value up to 98.
    /// If there is only one item in the cart then that item's quantity cannot go down to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}
impl UpdatePaymentLinkLineItemsAdjustableQuantity {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, maximum: Default::default(), minimum: Default::default() }
    }
}
/// Specify whether Checkout should collect a payment method.
///
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkPaymentMethodCollection {
    Always,
    IfRequired,
}

impl UpdatePaymentLinkPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::IfRequired => "if_required",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkPaymentMethodCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The list of payment method types that customers can use.
///
/// Pass an empty string to enable automatic payment methods that use your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkPaymentMethodTypes {
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Oxxo,
    P24,
    Paynow,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl UpdatePaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configuration for collecting the customer's shipping address.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkShippingAddressCollection<'a> {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
    /// shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: &'a [UpdatePaymentLinkShippingAddressCollectionAllowedCountries],
}
impl<'a> UpdatePaymentLinkShippingAddressCollection<'a> {
    pub fn new(
        allowed_countries: &'a [UpdatePaymentLinkShippingAddressCollectionAllowedCountries],
    ) -> Self {
        Self { allowed_countries }
    }
}
/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
/// shipping locations.
///
/// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    #[serde(rename = "AC")]
    Ac,
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "TA")]
    Ta,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "XK")]
    Xk,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
    #[serde(rename = "ZZ")]
    Zz,
}

impl UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ac => "AC",
            Self::Ad => "AD",
            Self::Ae => "AE",
            Self::Af => "AF",
            Self::Ag => "AG",
            Self::Ai => "AI",
            Self::Al => "AL",
            Self::Am => "AM",
            Self::Ao => "AO",
            Self::Aq => "AQ",
            Self::Ar => "AR",
            Self::At => "AT",
            Self::Au => "AU",
            Self::Aw => "AW",
            Self::Ax => "AX",
            Self::Az => "AZ",
            Self::Ba => "BA",
            Self::Bb => "BB",
            Self::Bd => "BD",
            Self::Be => "BE",
            Self::Bf => "BF",
            Self::Bg => "BG",
            Self::Bh => "BH",
            Self::Bi => "BI",
            Self::Bj => "BJ",
            Self::Bl => "BL",
            Self::Bm => "BM",
            Self::Bn => "BN",
            Self::Bo => "BO",
            Self::Bq => "BQ",
            Self::Br => "BR",
            Self::Bs => "BS",
            Self::Bt => "BT",
            Self::Bv => "BV",
            Self::Bw => "BW",
            Self::By => "BY",
            Self::Bz => "BZ",
            Self::Ca => "CA",
            Self::Cd => "CD",
            Self::Cf => "CF",
            Self::Cg => "CG",
            Self::Ch => "CH",
            Self::Ci => "CI",
            Self::Ck => "CK",
            Self::Cl => "CL",
            Self::Cm => "CM",
            Self::Cn => "CN",
            Self::Co => "CO",
            Self::Cr => "CR",
            Self::Cv => "CV",
            Self::Cw => "CW",
            Self::Cy => "CY",
            Self::Cz => "CZ",
            Self::De => "DE",
            Self::Dj => "DJ",
            Self::Dk => "DK",
            Self::Dm => "DM",
            Self::Do => "DO",
            Self::Dz => "DZ",
            Self::Ec => "EC",
            Self::Ee => "EE",
            Self::Eg => "EG",
            Self::Eh => "EH",
            Self::Er => "ER",
            Self::Es => "ES",
            Self::Et => "ET",
            Self::Fi => "FI",
            Self::Fj => "FJ",
            Self::Fk => "FK",
            Self::Fo => "FO",
            Self::Fr => "FR",
            Self::Ga => "GA",
            Self::Gb => "GB",
            Self::Gd => "GD",
            Self::Ge => "GE",
            Self::Gf => "GF",
            Self::Gg => "GG",
            Self::Gh => "GH",
            Self::Gi => "GI",
            Self::Gl => "GL",
            Self::Gm => "GM",
            Self::Gn => "GN",
            Self::Gp => "GP",
            Self::Gq => "GQ",
            Self::Gr => "GR",
            Self::Gs => "GS",
            Self::Gt => "GT",
            Self::Gu => "GU",
            Self::Gw => "GW",
            Self::Gy => "GY",
            Self::Hk => "HK",
            Self::Hn => "HN",
            Self::Hr => "HR",
            Self::Ht => "HT",
            Self::Hu => "HU",
            Self::Id => "ID",
            Self::Ie => "IE",
            Self::Il => "IL",
            Self::Im => "IM",
            Self::In => "IN",
            Self::Io => "IO",
            Self::Iq => "IQ",
            Self::Is => "IS",
            Self::It => "IT",
            Self::Je => "JE",
            Self::Jm => "JM",
            Self::Jo => "JO",
            Self::Jp => "JP",
            Self::Ke => "KE",
            Self::Kg => "KG",
            Self::Kh => "KH",
            Self::Ki => "KI",
            Self::Km => "KM",
            Self::Kn => "KN",
            Self::Kr => "KR",
            Self::Kw => "KW",
            Self::Ky => "KY",
            Self::Kz => "KZ",
            Self::La => "LA",
            Self::Lb => "LB",
            Self::Lc => "LC",
            Self::Li => "LI",
            Self::Lk => "LK",
            Self::Lr => "LR",
            Self::Ls => "LS",
            Self::Lt => "LT",
            Self::Lu => "LU",
            Self::Lv => "LV",
            Self::Ly => "LY",
            Self::Ma => "MA",
            Self::Mc => "MC",
            Self::Md => "MD",
            Self::Me => "ME",
            Self::Mf => "MF",
            Self::Mg => "MG",
            Self::Mk => "MK",
            Self::Ml => "ML",
            Self::Mm => "MM",
            Self::Mn => "MN",
            Self::Mo => "MO",
            Self::Mq => "MQ",
            Self::Mr => "MR",
            Self::Ms => "MS",
            Self::Mt => "MT",
            Self::Mu => "MU",
            Self::Mv => "MV",
            Self::Mw => "MW",
            Self::Mx => "MX",
            Self::My => "MY",
            Self::Mz => "MZ",
            Self::Na => "NA",
            Self::Nc => "NC",
            Self::Ne => "NE",
            Self::Ng => "NG",
            Self::Ni => "NI",
            Self::Nl => "NL",
            Self::No => "NO",
            Self::Np => "NP",
            Self::Nr => "NR",
            Self::Nu => "NU",
            Self::Nz => "NZ",
            Self::Om => "OM",
            Self::Pa => "PA",
            Self::Pe => "PE",
            Self::Pf => "PF",
            Self::Pg => "PG",
            Self::Ph => "PH",
            Self::Pk => "PK",
            Self::Pl => "PL",
            Self::Pm => "PM",
            Self::Pn => "PN",
            Self::Pr => "PR",
            Self::Ps => "PS",
            Self::Pt => "PT",
            Self::Py => "PY",
            Self::Qa => "QA",
            Self::Re => "RE",
            Self::Ro => "RO",
            Self::Rs => "RS",
            Self::Ru => "RU",
            Self::Rw => "RW",
            Self::Sa => "SA",
            Self::Sb => "SB",
            Self::Sc => "SC",
            Self::Se => "SE",
            Self::Sg => "SG",
            Self::Sh => "SH",
            Self::Si => "SI",
            Self::Sj => "SJ",
            Self::Sk => "SK",
            Self::Sl => "SL",
            Self::Sm => "SM",
            Self::Sn => "SN",
            Self::So => "SO",
            Self::Sr => "SR",
            Self::Ss => "SS",
            Self::St => "ST",
            Self::Sv => "SV",
            Self::Sx => "SX",
            Self::Sz => "SZ",
            Self::Ta => "TA",
            Self::Tc => "TC",
            Self::Td => "TD",
            Self::Tf => "TF",
            Self::Tg => "TG",
            Self::Th => "TH",
            Self::Tj => "TJ",
            Self::Tk => "TK",
            Self::Tl => "TL",
            Self::Tm => "TM",
            Self::Tn => "TN",
            Self::To => "TO",
            Self::Tr => "TR",
            Self::Tt => "TT",
            Self::Tv => "TV",
            Self::Tw => "TW",
            Self::Tz => "TZ",
            Self::Ua => "UA",
            Self::Ug => "UG",
            Self::Us => "US",
            Self::Uy => "UY",
            Self::Uz => "UZ",
            Self::Va => "VA",
            Self::Vc => "VC",
            Self::Ve => "VE",
            Self::Vg => "VG",
            Self::Vn => "VN",
            Self::Vu => "VU",
            Self::Wf => "WF",
            Self::Ws => "WS",
            Self::Xk => "XK",
            Self::Ye => "YE",
            Self::Yt => "YT",
            Self::Za => "ZA",
            Self::Zm => "ZM",
            Self::Zw => "ZW",
            Self::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
