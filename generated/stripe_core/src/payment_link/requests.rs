impl stripe_core::payment_link::PaymentLink {
    /// Returns a list of your payment links.
    pub fn list(
        client: &stripe::Client,
        params: ListPaymentLink,
    ) -> stripe::Response<stripe_types::List<stripe_core::payment_link::PaymentLink>> {
        client.get_query("/payment_links", params)
    }
    /// Retrieve a payment link.
    pub fn retrieve(
        client: &stripe::Client,
        payment_link: &stripe_core::payment_link::PaymentLinkId,
        params: RetrievePaymentLink,
    ) -> stripe::Response<stripe_core::payment_link::PaymentLink> {
        client.get_query(
            &format!("/payment_links/{payment_link}", payment_link = payment_link),
            params,
        )
    }
    /// When retrieving a payment link, there is an includable **line_items** property containing the first handful of those items.
    ///
    /// There is also a URL where you can retrieve the full (paginated) list of line items.
    pub fn list_line_items(
        client: &stripe::Client,
        payment_link: &stripe_core::payment_link::PaymentLinkId,
        params: ListLineItemsPaymentLink,
    ) -> stripe::Response<stripe_types::List<stripe_core::line_item::LineItem>> {
        client.get_query(
            &format!("/payment_links/{payment_link}/line_items", payment_link = payment_link),
            params,
        )
    }
    /// Creates a payment link.
    pub fn create(
        client: &stripe::Client,
        params: CreatePaymentLink,
    ) -> stripe::Response<stripe_core::payment_link::PaymentLink> {
        client.send_form("/payment_links", params, http_types::Method::Post)
    }
    /// Updates a payment link.
    pub fn update(
        client: &stripe::Client,
        payment_link: &stripe_core::payment_link::PaymentLinkId,
        params: UpdatePaymentLink,
    ) -> stripe::Response<stripe_core::payment_link::PaymentLink> {
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
    pub after_completion: Option<AfterCompletionParams<'a>>,
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
    pub automatic_tax: Option<AutomaticTaxParams>,
    /// Configuration for collecting the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<BillingAddressCollection>,
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
    pub payment_method_types: Option<&'a [PaymentMethodTypes]>,
    /// Controls phone number collection settings during checkout.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<CreatePaymentLinkPhoneNumberCollection>,
    /// Configuration for collecting the customer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<ShippingAddressCollectionParams<'a>>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentLinkConsentCollectionPromotions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentLinkConsentCollectionPromotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
/// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentLinkConsentCollectionTermsOfService {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "required" => Ok(Self::Required),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentLinkConsentCollectionTermsOfService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentLinkCustomerCreation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(Self::Always),
            "if_required" => Ok(Self::IfRequired),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentLinkCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub adjustable_quantity: Option<AdjustableQuantityParams>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Specify whether Checkout should collect a payment method.
///
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentLinkPaymentMethodCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(Self::Always),
            "if_required" => Ok(Self::IfRequired),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentLinkPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentLinkSubmitType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "book" => Ok(Self::Book),
            "donate" => Ok(Self::Donate),
            "pay" => Ok(Self::Pay),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentLinkSubmitType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub after_completion: Option<AfterCompletionParams<'a>>,
    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    /// Configuration for automatic tax collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<AutomaticTaxParams>,
    /// Configuration for collecting the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<BillingAddressCollection>,
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
    pub payment_method_types: Option<&'a [PaymentMethodTypes]>,
    /// Configuration for collecting the customer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<ShippingAddressCollectionParams<'a>>,
}
impl<'a> UpdatePaymentLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentLinkCustomerCreation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(Self::Always),
            "if_required" => Ok(Self::IfRequired),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentLinkCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub adjustable_quantity: Option<AdjustableQuantityParams>,
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
/// Specify whether Checkout should collect a payment method.
///
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentLinkPaymentMethodCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(Self::Always),
            "if_required" => Ok(Self::IfRequired),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentLinkPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct AfterCompletionConfirmationPageParams<'a> {
    /// A custom message to display to the customer after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<&'a str>,
}
impl<'a> AfterCompletionConfirmationPageParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AfterCompletionRedirectParams<'a> {
    /// The URL the customer will be redirected to after the purchase is complete.
    ///
    /// You can embed `{CHECKOUT_SESSION_ID}` into the URL to have the `id` of the completed [checkout session](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-id) included.
    pub url: &'a str,
}
impl<'a> AfterCompletionRedirectParams<'a> {
    pub fn new(url: &'a str) -> Self {
        Self { url }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Type {
    HostedConfirmation,
    Redirect,
}

impl Type {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HostedConfirmation => "hosted_confirmation",
            Self::Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for Type {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hosted_confirmation" => Ok(Self::HostedConfirmation),
            "redirect" => Ok(Self::Redirect),

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
        self.as_str().fmt(f)
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AutomaticTaxParams {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,
}
impl AutomaticTaxParams {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BillingAddressCollection {
    Auto,
    Required,
}

impl BillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Required => "required",
        }
    }
}

impl std::str::FromStr for BillingAddressCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "required" => Ok(Self::Required),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for BillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BillingAddressCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AdjustableQuantityParams {
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
impl AdjustableQuantityParams {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, maximum: Default::default(), minimum: Default::default() }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentMethodTypes {
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

impl PaymentMethodTypes {
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

impl std::str::FromStr for PaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "affirm" => Ok(Self::Affirm),
            "afterpay_clearpay" => Ok(Self::AfterpayClearpay),
            "alipay" => Ok(Self::Alipay),
            "au_becs_debit" => Ok(Self::AuBecsDebit),
            "bacs_debit" => Ok(Self::BacsDebit),
            "bancontact" => Ok(Self::Bancontact),
            "blik" => Ok(Self::Blik),
            "boleto" => Ok(Self::Boleto),
            "card" => Ok(Self::Card),
            "eps" => Ok(Self::Eps),
            "fpx" => Ok(Self::Fpx),
            "giropay" => Ok(Self::Giropay),
            "grabpay" => Ok(Self::Grabpay),
            "ideal" => Ok(Self::Ideal),
            "klarna" => Ok(Self::Klarna),
            "konbini" => Ok(Self::Konbini),
            "oxxo" => Ok(Self::Oxxo),
            "p24" => Ok(Self::P24),
            "paynow" => Ok(Self::Paynow),
            "pix" => Ok(Self::Pix),
            "promptpay" => Ok(Self::Promptpay),
            "sepa_debit" => Ok(Self::SepaDebit),
            "sofort" => Ok(Self::Sofort),
            "us_bank_account" => Ok(Self::UsBankAccount),
            "wechat_pay" => Ok(Self::WechatPay),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AllowedCountries {
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
}

impl AllowedCountries {
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

impl std::str::FromStr for AllowedCountries {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AC" => Ok(Self::Ac),
            "AD" => Ok(Self::Ad),
            "AE" => Ok(Self::Ae),
            "AF" => Ok(Self::Af),
            "AG" => Ok(Self::Ag),
            "AI" => Ok(Self::Ai),
            "AL" => Ok(Self::Al),
            "AM" => Ok(Self::Am),
            "AO" => Ok(Self::Ao),
            "AQ" => Ok(Self::Aq),
            "AR" => Ok(Self::Ar),
            "AT" => Ok(Self::At),
            "AU" => Ok(Self::Au),
            "AW" => Ok(Self::Aw),
            "AX" => Ok(Self::Ax),
            "AZ" => Ok(Self::Az),
            "BA" => Ok(Self::Ba),
            "BB" => Ok(Self::Bb),
            "BD" => Ok(Self::Bd),
            "BE" => Ok(Self::Be),
            "BF" => Ok(Self::Bf),
            "BG" => Ok(Self::Bg),
            "BH" => Ok(Self::Bh),
            "BI" => Ok(Self::Bi),
            "BJ" => Ok(Self::Bj),
            "BL" => Ok(Self::Bl),
            "BM" => Ok(Self::Bm),
            "BN" => Ok(Self::Bn),
            "BO" => Ok(Self::Bo),
            "BQ" => Ok(Self::Bq),
            "BR" => Ok(Self::Br),
            "BS" => Ok(Self::Bs),
            "BT" => Ok(Self::Bt),
            "BV" => Ok(Self::Bv),
            "BW" => Ok(Self::Bw),
            "BY" => Ok(Self::By),
            "BZ" => Ok(Self::Bz),
            "CA" => Ok(Self::Ca),
            "CD" => Ok(Self::Cd),
            "CF" => Ok(Self::Cf),
            "CG" => Ok(Self::Cg),
            "CH" => Ok(Self::Ch),
            "CI" => Ok(Self::Ci),
            "CK" => Ok(Self::Ck),
            "CL" => Ok(Self::Cl),
            "CM" => Ok(Self::Cm),
            "CN" => Ok(Self::Cn),
            "CO" => Ok(Self::Co),
            "CR" => Ok(Self::Cr),
            "CV" => Ok(Self::Cv),
            "CW" => Ok(Self::Cw),
            "CY" => Ok(Self::Cy),
            "CZ" => Ok(Self::Cz),
            "DE" => Ok(Self::De),
            "DJ" => Ok(Self::Dj),
            "DK" => Ok(Self::Dk),
            "DM" => Ok(Self::Dm),
            "DO" => Ok(Self::Do),
            "DZ" => Ok(Self::Dz),
            "EC" => Ok(Self::Ec),
            "EE" => Ok(Self::Ee),
            "EG" => Ok(Self::Eg),
            "EH" => Ok(Self::Eh),
            "ER" => Ok(Self::Er),
            "ES" => Ok(Self::Es),
            "ET" => Ok(Self::Et),
            "FI" => Ok(Self::Fi),
            "FJ" => Ok(Self::Fj),
            "FK" => Ok(Self::Fk),
            "FO" => Ok(Self::Fo),
            "FR" => Ok(Self::Fr),
            "GA" => Ok(Self::Ga),
            "GB" => Ok(Self::Gb),
            "GD" => Ok(Self::Gd),
            "GE" => Ok(Self::Ge),
            "GF" => Ok(Self::Gf),
            "GG" => Ok(Self::Gg),
            "GH" => Ok(Self::Gh),
            "GI" => Ok(Self::Gi),
            "GL" => Ok(Self::Gl),
            "GM" => Ok(Self::Gm),
            "GN" => Ok(Self::Gn),
            "GP" => Ok(Self::Gp),
            "GQ" => Ok(Self::Gq),
            "GR" => Ok(Self::Gr),
            "GS" => Ok(Self::Gs),
            "GT" => Ok(Self::Gt),
            "GU" => Ok(Self::Gu),
            "GW" => Ok(Self::Gw),
            "GY" => Ok(Self::Gy),
            "HK" => Ok(Self::Hk),
            "HN" => Ok(Self::Hn),
            "HR" => Ok(Self::Hr),
            "HT" => Ok(Self::Ht),
            "HU" => Ok(Self::Hu),
            "ID" => Ok(Self::Id),
            "IE" => Ok(Self::Ie),
            "IL" => Ok(Self::Il),
            "IM" => Ok(Self::Im),
            "IN" => Ok(Self::In),
            "IO" => Ok(Self::Io),
            "IQ" => Ok(Self::Iq),
            "IS" => Ok(Self::Is),
            "IT" => Ok(Self::It),
            "JE" => Ok(Self::Je),
            "JM" => Ok(Self::Jm),
            "JO" => Ok(Self::Jo),
            "JP" => Ok(Self::Jp),
            "KE" => Ok(Self::Ke),
            "KG" => Ok(Self::Kg),
            "KH" => Ok(Self::Kh),
            "KI" => Ok(Self::Ki),
            "KM" => Ok(Self::Km),
            "KN" => Ok(Self::Kn),
            "KR" => Ok(Self::Kr),
            "KW" => Ok(Self::Kw),
            "KY" => Ok(Self::Ky),
            "KZ" => Ok(Self::Kz),
            "LA" => Ok(Self::La),
            "LB" => Ok(Self::Lb),
            "LC" => Ok(Self::Lc),
            "LI" => Ok(Self::Li),
            "LK" => Ok(Self::Lk),
            "LR" => Ok(Self::Lr),
            "LS" => Ok(Self::Ls),
            "LT" => Ok(Self::Lt),
            "LU" => Ok(Self::Lu),
            "LV" => Ok(Self::Lv),
            "LY" => Ok(Self::Ly),
            "MA" => Ok(Self::Ma),
            "MC" => Ok(Self::Mc),
            "MD" => Ok(Self::Md),
            "ME" => Ok(Self::Me),
            "MF" => Ok(Self::Mf),
            "MG" => Ok(Self::Mg),
            "MK" => Ok(Self::Mk),
            "ML" => Ok(Self::Ml),
            "MM" => Ok(Self::Mm),
            "MN" => Ok(Self::Mn),
            "MO" => Ok(Self::Mo),
            "MQ" => Ok(Self::Mq),
            "MR" => Ok(Self::Mr),
            "MS" => Ok(Self::Ms),
            "MT" => Ok(Self::Mt),
            "MU" => Ok(Self::Mu),
            "MV" => Ok(Self::Mv),
            "MW" => Ok(Self::Mw),
            "MX" => Ok(Self::Mx),
            "MY" => Ok(Self::My),
            "MZ" => Ok(Self::Mz),
            "NA" => Ok(Self::Na),
            "NC" => Ok(Self::Nc),
            "NE" => Ok(Self::Ne),
            "NG" => Ok(Self::Ng),
            "NI" => Ok(Self::Ni),
            "NL" => Ok(Self::Nl),
            "NO" => Ok(Self::No),
            "NP" => Ok(Self::Np),
            "NR" => Ok(Self::Nr),
            "NU" => Ok(Self::Nu),
            "NZ" => Ok(Self::Nz),
            "OM" => Ok(Self::Om),
            "PA" => Ok(Self::Pa),
            "PE" => Ok(Self::Pe),
            "PF" => Ok(Self::Pf),
            "PG" => Ok(Self::Pg),
            "PH" => Ok(Self::Ph),
            "PK" => Ok(Self::Pk),
            "PL" => Ok(Self::Pl),
            "PM" => Ok(Self::Pm),
            "PN" => Ok(Self::Pn),
            "PR" => Ok(Self::Pr),
            "PS" => Ok(Self::Ps),
            "PT" => Ok(Self::Pt),
            "PY" => Ok(Self::Py),
            "QA" => Ok(Self::Qa),
            "RE" => Ok(Self::Re),
            "RO" => Ok(Self::Ro),
            "RS" => Ok(Self::Rs),
            "RU" => Ok(Self::Ru),
            "RW" => Ok(Self::Rw),
            "SA" => Ok(Self::Sa),
            "SB" => Ok(Self::Sb),
            "SC" => Ok(Self::Sc),
            "SE" => Ok(Self::Se),
            "SG" => Ok(Self::Sg),
            "SH" => Ok(Self::Sh),
            "SI" => Ok(Self::Si),
            "SJ" => Ok(Self::Sj),
            "SK" => Ok(Self::Sk),
            "SL" => Ok(Self::Sl),
            "SM" => Ok(Self::Sm),
            "SN" => Ok(Self::Sn),
            "SO" => Ok(Self::So),
            "SR" => Ok(Self::Sr),
            "SS" => Ok(Self::Ss),
            "ST" => Ok(Self::St),
            "SV" => Ok(Self::Sv),
            "SX" => Ok(Self::Sx),
            "SZ" => Ok(Self::Sz),
            "TA" => Ok(Self::Ta),
            "TC" => Ok(Self::Tc),
            "TD" => Ok(Self::Td),
            "TF" => Ok(Self::Tf),
            "TG" => Ok(Self::Tg),
            "TH" => Ok(Self::Th),
            "TJ" => Ok(Self::Tj),
            "TK" => Ok(Self::Tk),
            "TL" => Ok(Self::Tl),
            "TM" => Ok(Self::Tm),
            "TN" => Ok(Self::Tn),
            "TO" => Ok(Self::To),
            "TR" => Ok(Self::Tr),
            "TT" => Ok(Self::Tt),
            "TV" => Ok(Self::Tv),
            "TW" => Ok(Self::Tw),
            "TZ" => Ok(Self::Tz),
            "UA" => Ok(Self::Ua),
            "UG" => Ok(Self::Ug),
            "US" => Ok(Self::Us),
            "UY" => Ok(Self::Uy),
            "UZ" => Ok(Self::Uz),
            "VA" => Ok(Self::Va),
            "VC" => Ok(Self::Vc),
            "VE" => Ok(Self::Ve),
            "VG" => Ok(Self::Vg),
            "VN" => Ok(Self::Vn),
            "VU" => Ok(Self::Vu),
            "WF" => Ok(Self::Wf),
            "WS" => Ok(Self::Ws),
            "XK" => Ok(Self::Xk),
            "YE" => Ok(Self::Ye),
            "YT" => Ok(Self::Yt),
            "ZA" => Ok(Self::Za),
            "ZM" => Ok(Self::Zm),
            "ZW" => Ok(Self::Zw),
            "ZZ" => Ok(Self::Zz),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AllowedCountries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AfterCompletionParams<'a> {
    /// Configuration when `type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<AfterCompletionConfirmationPageParams<'a>>,
    /// Configuration when `type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<AfterCompletionRedirectParams<'a>>,
    /// The specified behavior after the purchase is complete.
    ///
    /// Either `redirect` or `hosted_confirmation`.
    #[serde(rename = "type")]
    pub type_: Type,
}
impl<'a> AfterCompletionParams<'a> {
    pub fn new(type_: Type) -> Self {
        Self { hosted_confirmation: Default::default(), redirect: Default::default(), type_ }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ShippingAddressCollectionParams<'a> {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
    /// shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: &'a [AllowedCountries],
}
impl<'a> ShippingAddressCollectionParams<'a> {
    pub fn new(allowed_countries: &'a [AllowedCountries]) -> Self {
        Self { allowed_countries }
    }
}
