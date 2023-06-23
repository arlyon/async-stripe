use stripe::{Client, Response};

impl stripe_checkout::checkout::session::Session {
    /// Returns a list of Checkout Sessions.
    pub fn list(
        client: &Client,
        params: ListSession,
    ) -> Response<stripe_types::List<stripe_checkout::checkout::session::Session>> {
        client.get_query("/checkout/sessions", params)
    }
    /// Retrieves a Session object.
    pub fn retrieve(
        client: &Client,
        session: &str,
        params: RetrieveSession,
    ) -> Response<stripe_checkout::checkout::session::Session> {
        client.get_query(&format!("/checkout/sessions/{session}", session = session), params)
    }
    /// Creates a Session object.
    pub fn create(
        client: &Client,
        params: CreateSession,
    ) -> Response<stripe_checkout::checkout::session::Session> {
        client.send_form("/checkout/sessions", params, http_types::Method::Post)
    }
    /// When retrieving a Checkout Session, there is an includable **line_items** property containing the first handful of those items.
    ///
    /// There is also a URL where you can retrieve the full (paginated) list of line items.
    pub fn list_line_items(
        client: &Client,
        session: &str,
        params: ListLineItemsSession,
    ) -> Response<stripe_types::List<stripe_core::line_item::LineItem>> {
        client.get_query(
            &format!("/checkout/sessions/{session}/line_items", session = session),
            params,
        )
    }
    /// A Session can be expired when it is in one of these statuses: `open`
    ///
    /// After it expires, a customer can’t complete a Session and customers loading the Session see a message saying the Session is expired.
    pub fn expire(
        client: &Client,
        session: &str,
        params: ExpireSession,
    ) -> Response<stripe_checkout::checkout::session::Session> {
        client.send_form(
            &format!("/checkout/sessions/{session}/expire", session = session),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListSession<'a> {
    /// Only return the Checkout Sessions for the Customer specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Only return the Checkout Sessions for the Customer details specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<ListSessionCustomerDetails<'a>>,
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
    /// Only return the Checkout Session for the PaymentIntent specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Only return the Checkout Session for the subscription specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
}
impl<'a> ListSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Only return the Checkout Sessions for the Customer details specified.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListSessionCustomerDetails<'a> {
    /// Customer's email address.
    pub email: &'a str,
}
impl<'a> ListSessionCustomerDetails<'a> {
    pub fn new(email: &'a str) -> Self {
        Self { email }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSession<'a> {
    /// Configure actions after a Checkout Session has expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<CreateSessionAfterExpiration>,
    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    /// Settings for automatic tax lookup for this session and resulting payments, invoices, and subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateSessionAutomaticTax>,
    /// Specify whether Checkout should collect the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<CreateSessionBillingAddressCollection>,
    /// The URL the customer will be directed to if they decide to cancel payment and return to your website.
    pub cancel_url: &'a str,
    /// A unique string to reference the Checkout Session.
    ///
    /// This can be a customer ID, a cart ID, or similar, and can be used to reconcile the session with your internal systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<&'a str>,
    /// Configure fields for the Checkout Session to gather active consent from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<CreateSessionConsentCollection>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of an existing Customer, if one exists.
    ///
    /// In `payment` mode, the customer’s most recent card payment method will be used to prefill the email, name, card details, and billing address on the Checkout page.
    /// In `subscription` mode, the customer’s [default payment method](https://stripe.com/docs/api/customers/update#update_customer-invoice_settings-default_payment_method) will be used if it’s a card, and otherwise the most recent card will be used.
    /// A valid billing address, billing name and billing email are required on the payment method for Checkout to prefill the customer's card details.  If the Customer already has a valid [email](https://stripe.com/docs/api/customers/object#customer_object-email) set, the email will be prefilled and not editable in Checkout. If the Customer does not have a valid `email`, Checkout will set the email entered during the session on the Customer.  If blank for Checkout Sessions in `payment` or `subscription` mode, Checkout will create a new Customer object based on information provided during the payment flow.  You can set [`payment_intent_data.setup_future_usage`](https://stripe.com/docs/api/checkout/sessions/create#create_checkout_session-payment_intent_data-setup_future_usage) to have Checkout automatically attach the payment method to the Customer you pass in for future reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Configure whether a Checkout Session creates a [Customer](https://stripe.com/docs/api/customers) during Session confirmation.
    ///
    /// When a Customer is not created, you can still retrieve email, address, and other customer data entered in Checkout
    /// with [customer_details](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-customer_details).
    ///
    /// Sessions that don't create Customers instead create [Guest Customers](https://support.stripe.com/questions/guest-customer-faq)
    /// in the Dashboard.
    ///
    /// Promotion codes limited to first time customers will return invalid for these Sessions.  Can only be set in `payment` and `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<CreateSessionCustomerCreation>,
    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file.
    ///
    /// To access information about the customer once a session is complete, use the `customer` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<&'a str>,
    /// Controls what fields on Customer can be updated by the Checkout Session.
    ///
    /// Can only be provided when `customer` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<CreateSessionCustomerUpdate>,
    /// The coupon or promotion code to apply to this Session.
    ///
    /// Currently, only up to one may be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [CreateSessionDiscounts<'a>]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The Epoch time in seconds at which the Checkout Session will expire.
    ///
    /// It can be anywhere from 30 minutes to 24 hours after Checkout Session creation.
    /// By default, this value is 24 hours from creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<stripe_types::Timestamp>,
    /// A list of items the customer is purchasing.
    ///
    /// Use this parameter to pass one-time or recurring [Prices](https://stripe.com/docs/api/prices).  For `payment` mode, there is a maximum of 100 line items, however it is recommended to consolidate line items if there are more than a few dozen.  For `subscription` mode, there is a maximum of 20 line items with recurring Prices and 20 line items with one-time Prices.
    /// Line items with one-time Prices will be on the initial invoice only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<&'a [CreateSessionLineItems<'a>]>,
    /// The IETF language tag of the locale Checkout is displayed in.
    ///
    /// If blank or `auto`, the browser's locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<CreateSessionLocale>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// The mode of the Checkout Session.
    ///
    /// Pass `subscription` if the Checkout Session includes at least one recurring item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CreateSessionMode>,
    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<CreateSessionPaymentIntentData<'a>>,
    /// Specify whether Checkout should collect a payment method.
    ///
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0. This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on configuring [subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<CreateSessionPaymentMethodCollection>,
    /// Payment-method-specific configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateSessionPaymentMethodOptions<'a>>,
    /// A list of the types of payment methods (e.g., `card`) this Checkout Session can accept.
    ///
    /// In `payment` and `subscription` mode, you can omit this attribute to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    /// It is required in `setup` mode.
    ///
    /// Read more about the supported payment methods and their requirements in our [payment
    /// method details guide](/docs/payments/checkout/payment-methods).
    ///
    /// If multiple payment methods are passed, Checkout will dynamically reorder them to
    /// prioritize the most relevant payment methods based on the customer's location and
    /// other characteristics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [CreateSessionPaymentMethodTypes]>,
    /// Controls phone number collection settings for the session.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts
    /// before using this feature.
    ///
    /// Learn more about [collecting phone numbers with Checkout](https://stripe.com/docs/payments/checkout/phone-numbers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<CreateSessionPhoneNumberCollection>,
    /// A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent_data: Option<CreateSessionSetupIntentData<'a>>,
    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<CreateSessionShippingAddressCollection<'a>>,
    /// The shipping rate options to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<&'a [CreateSessionShippingOptions<'a>]>,
    /// [Deprecated] The shipping rate to apply to this Session.
    ///
    /// Only up to one may be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rates: Option<&'a [&'a str]>,
    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button.
    ///
    /// `submit_type` can only be specified on Checkout Sessions in `payment` mode, but not Checkout Sessions in `subscription` or `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<CreateSessionSubmitType>,
    /// A subset of parameters to be passed to subscription creation for Checkout Sessions in `subscription` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<CreateSessionSubscriptionData<'a>>,
    /// The URL to which Stripe should send customers when payment or setup
    /// is complete.
    /// If you’d like to use information from the successful Checkout Session on your page,
    /// read the guide on [customizing your success page](https://stripe.com/docs/payments/checkout/custom-success-page).
    pub success_url: &'a str,
    /// Controls tax ID collection settings for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<CreateSessionTaxIdCollection>,
}
impl<'a> CreateSession<'a> {
    pub fn new(cancel_url: &'a str, success_url: &'a str) -> Self {
        Self {
            after_expiration: Default::default(),
            allow_promotion_codes: Default::default(),
            automatic_tax: Default::default(),
            billing_address_collection: Default::default(),
            cancel_url,
            client_reference_id: Default::default(),
            consent_collection: Default::default(),
            currency: Default::default(),
            customer: Default::default(),
            customer_creation: Default::default(),
            customer_email: Default::default(),
            customer_update: Default::default(),
            discounts: Default::default(),
            expand: Default::default(),
            expires_at: Default::default(),
            line_items: Default::default(),
            locale: Default::default(),
            metadata: Default::default(),
            mode: Default::default(),
            payment_intent_data: Default::default(),
            payment_method_collection: Default::default(),
            payment_method_options: Default::default(),
            payment_method_types: Default::default(),
            phone_number_collection: Default::default(),
            setup_intent_data: Default::default(),
            shipping_address_collection: Default::default(),
            shipping_options: Default::default(),
            shipping_rates: Default::default(),
            submit_type: Default::default(),
            subscription_data: Default::default(),
            success_url,
            tax_id_collection: Default::default(),
        }
    }
}
/// Configure actions after a Checkout Session has expired.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionAfterExpiration {
    /// Configure a Checkout Session that can be used to recover an expired session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery: Option<CreateSessionAfterExpirationRecovery>,
}
impl CreateSessionAfterExpiration {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configure a Checkout Session that can be used to recover an expired session.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionAfterExpirationRecovery {
    /// Enables user redeemable promotion codes on the recovered Checkout Sessions.
    ///
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    /// If `true`, a recovery URL will be generated to recover this Checkout Session if it
    /// expires before a successful transaction is completed.
    ///
    /// It will be attached to the Checkout Session object upon expiration.
    pub enabled: bool,
}
impl CreateSessionAfterExpirationRecovery {
    pub fn new(enabled: bool) -> Self {
        Self { allow_promotion_codes: Default::default(), enabled }
    }
}
/// Settings for automatic tax lookup for this session and resulting payments, invoices, and subscriptions.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionAutomaticTax {
    /// Set to true to enable automatic taxes.
    pub enabled: bool,
}
impl CreateSessionAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Specify whether Checkout should collect the customer's billing address.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionBillingAddressCollection {
    Auto,
    Required,
}

impl CreateSessionBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Required => "required",
        }
    }
}

impl AsRef<str> for CreateSessionBillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configure fields for the Checkout Session to gather active consent from customers.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionConsentCollection {
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    ///
    /// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
    /// Only available to US merchants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<CreateSessionConsentCollectionPromotions>,
    /// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
    /// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<CreateSessionConsentCollectionTermsOfService>,
}
impl CreateSessionConsentCollection {
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
pub enum CreateSessionConsentCollectionPromotions {
    Auto,
    None,
}

impl CreateSessionConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionConsentCollectionPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
/// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionConsentCollectionTermsOfService {
    None,
    Required,
}

impl CreateSessionConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Required => "required",
        }
    }
}

impl AsRef<str> for CreateSessionConsentCollectionTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configure whether a Checkout Session creates a [Customer](https://stripe.com/docs/api/customers) during Session confirmation.
///
/// When a Customer is not created, you can still retrieve email, address, and other customer data entered in Checkout
/// with [customer_details](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-customer_details).
///
/// Sessions that don't create Customers instead create [Guest Customers](https://support.stripe.com/questions/guest-customer-faq)
/// in the Dashboard.
///
/// Promotion codes limited to first time customers will return invalid for these Sessions.  Can only be set in `payment` and `setup` mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionCustomerCreation {
    Always,
    IfRequired,
}

impl CreateSessionCustomerCreation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::IfRequired => "if_required",
        }
    }
}

impl AsRef<str> for CreateSessionCustomerCreation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Controls what fields on Customer can be updated by the Checkout Session.
///
/// Can only be provided when `customer` is provided.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionCustomerUpdate {
    /// Describes whether Checkout saves the billing address onto `customer.address`.
    /// To always collect a full billing address, use `billing_address_collection`.
    ///
    /// Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateSessionCustomerUpdateAddress>,
    /// Describes whether Checkout saves the name onto `customer.name`.
    ///
    /// Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CreateSessionCustomerUpdateName>,
    /// Describes whether Checkout saves shipping information onto `customer.shipping`.
    /// To collect shipping information, use `shipping_address_collection`.
    ///
    /// Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateSessionCustomerUpdateShipping>,
}
impl CreateSessionCustomerUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Describes whether Checkout saves the billing address onto `customer.address`.
/// To always collect a full billing address, use `billing_address_collection`.
///
/// Defaults to `never`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionCustomerUpdateAddress {
    Auto,
    Never,
}

impl CreateSessionCustomerUpdateAddress {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Never => "never",
        }
    }
}

impl AsRef<str> for CreateSessionCustomerUpdateAddress {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionCustomerUpdateAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Describes whether Checkout saves the name onto `customer.name`.
///
/// Defaults to `never`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionCustomerUpdateName {
    Auto,
    Never,
}

impl CreateSessionCustomerUpdateName {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Never => "never",
        }
    }
}

impl AsRef<str> for CreateSessionCustomerUpdateName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionCustomerUpdateName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Describes whether Checkout saves shipping information onto `customer.shipping`.
/// To collect shipping information, use `shipping_address_collection`.
///
/// Defaults to `never`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionCustomerUpdateShipping {
    Auto,
    Never,
}

impl CreateSessionCustomerUpdateShipping {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Never => "never",
        }
    }
}

impl AsRef<str> for CreateSessionCustomerUpdateShipping {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionCustomerUpdateShipping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The coupon or promotion code to apply to this Session.
///
/// Currently, only up to one may be specified.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionDiscounts<'a> {
    /// The ID of the coupon to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// The ID of a promotion code to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
}
impl<'a> CreateSessionDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A list of items the customer is purchasing.
///
/// Use this parameter to pass one-time or recurring [Prices](https://stripe.com/docs/api/prices).  For `payment` mode, there is a maximum of 100 line items, however it is recommended to consolidate line items if there are more than a few dozen.  For `subscription` mode, there is a maximum of 20 line items with recurring Prices and 20 line items with one-time Prices.
/// Line items with one-time Prices will be on the initial invoice only.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionLineItems<'a> {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreateSessionLineItemsAdjustableQuantity>,
    /// [Deprecated] The amount to be collected per unit of the line item.
    ///
    /// If specified, must also pass `currency` and `name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// [Deprecated] Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Required if `amount` is passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// [Deprecated] The description for the line item, to be displayed on the Checkout page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The [tax rates](https://stripe.com/docs/api/tax_rates) that will be applied to this line item depending on the customer's billing/shipping address.
    ///
    /// We currently support the following countries: US, GB, AU, and all countries in the EU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_tax_rates: Option<&'a [&'a str]>,
    /// [Deprecated] A list of image URLs representing this line item.
    ///
    /// Each image can be up to 5 MB in size.
    /// If passing `price` or `price_data`, specify images on the associated product instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<&'a [&'a str]>,
    /// [Deprecated] The name for the item to be displayed on the Checkout page.
    ///
    /// Required if `amount` is passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// The ID of the [Price](https://stripe.com/docs/api/prices) or [Plan](https://stripe.com/docs/api/plans) object.
    ///
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    ///
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateSessionLineItemsPriceData<'a>>,
    /// The quantity of the line item being purchased.
    ///
    /// Quantity should not be defined when `recurring.usage_type=metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The [tax rates](https://stripe.com/docs/api/tax_rates) which apply to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> CreateSessionLineItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// When set, provides configuration for this item’s quantity to be adjusted by the customer during Checkout.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionLineItemsAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative integer.
    ///
    /// By default customers will be able to remove the line item by setting the quantity to 0.
    pub enabled: bool,
    /// The maximum quantity the customer can purchase for the Checkout Session.
    ///
    /// By default this value is 99.
    /// You can specify a value up to 999999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum quantity the customer must purchase for the Checkout Session.
    ///
    /// By default this value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}
impl CreateSessionLineItemsAdjustableQuantity {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, maximum: Default::default(), minimum: Default::default() }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
///
/// One of `price` or `price_data` is required.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionLineItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    ///
    /// One of `product` or `product_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// Data used to generate a new product object inline.
    ///
    /// One of `product` or `product_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<CreateSessionLineItemsPriceDataProductData<'a>>,
    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreateSessionLineItemsPriceDataRecurring>,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateSessionLineItemsPriceDataTaxBehavior>,
    /// A non-negative integer in cents (or local equivalent) representing how much to charge.
    ///
    /// One of `unit_amount` or `unit_amount_decimal` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateSessionLineItemsPriceData<'a> {
    pub fn new(currency: stripe_types::Currency) -> Self {
        Self {
            currency,
            product: Default::default(),
            product_data: Default::default(),
            recurring: Default::default(),
            tax_behavior: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// Data used to generate a new product object inline.
///
/// One of `product` or `product_data` is required.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionLineItemsPriceDataProductData<'a> {
    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// The product's name, meant to be displayable to the customer.
    pub name: &'a str,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
}
impl<'a> CreateSessionLineItemsPriceDataProductData<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            description: Default::default(),
            images: Default::default(),
            metadata: Default::default(),
            name,
            tax_code: Default::default(),
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionLineItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: CreateSessionLineItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreateSessionLineItemsPriceDataRecurring {
    pub fn new(interval: CreateSessionLineItemsPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
/// Specifies billing frequency.
///
/// Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionLineItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl CreateSessionLineItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for CreateSessionLineItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionLineItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateSessionLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateSessionLineItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The IETF language tag of the locale Checkout is displayed in.
///
/// If blank or `auto`, the browser's locale is used.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    #[serde(rename = "en-GB")]
    EnMinusGb,
    Es,
    #[serde(rename = "es-419")]
    EsMinus419,
    Et,
    Fi,
    Fil,
    Fr,
    #[serde(rename = "fr-CA")]
    FrMinusCa,
    Hr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Ms,
    Mt,
    Nb,
    Nl,
    Pl,
    Pt,
    #[serde(rename = "pt-BR")]
    PtMinusBr,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Th,
    Tr,
    Vi,
    Zh,
    #[serde(rename = "zh-HK")]
    ZhMinusHk,
    #[serde(rename = "zh-TW")]
    ZhMinusTw,
}

impl CreateSessionLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Bg => "bg",
            Self::Cs => "cs",
            Self::Da => "da",
            Self::De => "de",
            Self::El => "el",
            Self::En => "en",
            Self::EnMinusGb => "en-GB",
            Self::Es => "es",
            Self::EsMinus419 => "es-419",
            Self::Et => "et",
            Self::Fi => "fi",
            Self::Fil => "fil",
            Self::Fr => "fr",
            Self::FrMinusCa => "fr-CA",
            Self::Hr => "hr",
            Self::Hu => "hu",
            Self::Id => "id",
            Self::It => "it",
            Self::Ja => "ja",
            Self::Ko => "ko",
            Self::Lt => "lt",
            Self::Lv => "lv",
            Self::Ms => "ms",
            Self::Mt => "mt",
            Self::Nb => "nb",
            Self::Nl => "nl",
            Self::Pl => "pl",
            Self::Pt => "pt",
            Self::PtMinusBr => "pt-BR",
            Self::Ro => "ro",
            Self::Ru => "ru",
            Self::Sk => "sk",
            Self::Sl => "sl",
            Self::Sv => "sv",
            Self::Th => "th",
            Self::Tr => "tr",
            Self::Vi => "vi",
            Self::Zh => "zh",
            Self::ZhMinusHk => "zh-HK",
            Self::ZhMinusTw => "zh-TW",
        }
    }
}

impl AsRef<str> for CreateSessionLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The mode of the Checkout Session.
///
/// Pass `subscription` if the Checkout Session includes at least one recurring item.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionMode {
    Payment,
    Setup,
    Subscription,
}

impl CreateSessionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Payment => "payment",
            Self::Setup => "setup",
            Self::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for CreateSessionMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentIntentData<'a> {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// The amount of the application fee collected will be capped at the total payment amount.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateSessionPaymentIntentDataCaptureMethod>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// The Stripe account ID for which these funds are intended.
    ///
    /// For details, see the PaymentIntents [use case for connected accounts](/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// Email address that the receipt for the resulting payment will be sent to.
    ///
    /// If `receipt_email` is specified for a payment in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    /// Indicates that you intend to [make future payments](https://stripe.com/docs/payments/payment-intents#future-usage) with the payment
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
    /// to a Customer.
    ///
    /// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.  When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentIntentDataSetupFutureUsage>,
    /// Shipping information for this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateSessionPaymentIntentDataShipping<'a>>,
    /// Extra information about the payment.
    ///
    /// This will appear on your customer's statement when this payment succeeds in creating a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,
    /// The parameters used to automatically create a Transfer when the payment succeeds.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateSessionPaymentIntentDataTransferData<'a>>,
    /// A string that identifies the resulting payment as part of a group.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}
impl<'a> CreateSessionPaymentIntentData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentIntentDataCaptureMethod {
    Automatic,
    Manual,
}

impl CreateSessionPaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentIntentDataCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to [make future payments](https://stripe.com/docs/payments/payment-intents#future-usage) with the payment
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
/// to a Customer.
///
/// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.  When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}

impl CreateSessionPaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentIntentDataSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Shipping information for this payment.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionPaymentIntentDataShipping<'a> {
    /// Shipping address.
    pub address: CreateSessionPaymentIntentDataShippingAddress<'a>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<&'a str>,
    /// Recipient name.
    pub name: &'a str,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<&'a str>,
}
impl<'a> CreateSessionPaymentIntentDataShipping<'a> {
    pub fn new(address: CreateSessionPaymentIntentDataShippingAddress<'a>, name: &'a str) -> Self {
        Self {
            address,
            carrier: Default::default(),
            name,
            phone: Default::default(),
            tracking_number: Default::default(),
        }
    }
}
/// Shipping address.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionPaymentIntentDataShippingAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    pub line1: &'a str,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateSessionPaymentIntentDataShippingAddress<'a> {
    pub fn new(line1: &'a str) -> Self {
        Self {
            city: Default::default(),
            country: Default::default(),
            line1,
            line2: Default::default(),
            postal_code: Default::default(),
            state: Default::default(),
        }
    }
}
/// The parameters used to automatically create a Transfer when the payment succeeds.
/// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionPaymentIntentDataTransferData<'a> {
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
impl<'a> CreateSessionPaymentIntentDataTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: Default::default(), destination }
    }
}
/// Specify whether Checkout should collect a payment method.
///
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0. This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on configuring [subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodCollection {
    Always,
    IfRequired,
}

impl CreateSessionPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::IfRequired => "if_required",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Payment-method-specific configuration.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptions<'a> {
    /// contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateSessionPaymentMethodOptionsAcssDebit<'a>>,
    /// contains details about the Affirm payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreateSessionPaymentMethodOptionsAffirm>,
    /// contains details about the Afterpay Clearpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreateSessionPaymentMethodOptionsAfterpayClearpay>,
    /// contains details about the Alipay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreateSessionPaymentMethodOptionsAlipay>,
    /// contains details about the AU Becs Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreateSessionPaymentMethodOptionsAuBecsDebit>,
    /// contains details about the Bacs Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreateSessionPaymentMethodOptionsBacsDebit>,
    /// contains details about the Bancontact payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateSessionPaymentMethodOptionsBancontact>,
    /// contains details about the Boleto payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreateSessionPaymentMethodOptionsBoleto>,
    /// contains details about the Card payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateSessionPaymentMethodOptionsCard<'a>>,
    /// contains details about the Customer Balance payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreateSessionPaymentMethodOptionsCustomerBalance<'a>>,
    /// contains details about the EPS payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreateSessionPaymentMethodOptionsEps>,
    /// contains details about the FPX payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreateSessionPaymentMethodOptionsFpx>,
    /// contains details about the Giropay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreateSessionPaymentMethodOptionsGiropay>,
    /// contains details about the Grabpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreateSessionPaymentMethodOptionsGrabpay>,
    /// contains details about the Ideal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreateSessionPaymentMethodOptionsIdeal>,
    /// contains details about the Klarna payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateSessionPaymentMethodOptionsKlarna>,
    /// contains details about the Konbini payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreateSessionPaymentMethodOptionsKonbini>,
    /// contains details about the OXXO payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreateSessionPaymentMethodOptionsOxxo>,
    /// contains details about the P24 payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreateSessionPaymentMethodOptionsP24>,
    /// contains details about the PayNow payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreateSessionPaymentMethodOptionsPaynow>,
    /// contains details about the Pix payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreateSessionPaymentMethodOptionsPix>,
    /// contains details about the Sepa Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateSessionPaymentMethodOptionsSepaDebit>,
    /// contains details about the Sofort payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateSessionPaymentMethodOptionsSofort>,
    /// contains details about the Us Bank Account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateSessionPaymentMethodOptionsUsBankAccount<'a>>,
    /// contains details about the WeChat Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreateSessionPaymentMethodOptionsWechatPay<'a>>,
}
impl<'a> CreateSessionPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// contains details about the ACSS Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsAcssDebit<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// This is only accepted for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<CreateSessionPaymentMethodOptionsAcssDebitCurrency>,
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSessionPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<CreateSessionPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> CreateSessionPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
///
/// Must be a [supported currency](https://stripe.com/docs/currencies).
/// This is only accepted for Checkout Sessions in `setup` mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
}

impl CreateSessionPaymentMethodOptionsAcssDebitCurrency {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cad => "cad",
            Self::Usd => "usd",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsAcssDebitCurrency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// List of Stripe products where this mandate can be selected automatically.
    ///
    /// Only usable in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<&'a [CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor]>,
    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}
impl<'a> CreateSessionPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// List of Stripe products where this mandate can be selected automatically.
///
/// Only usable in `setup` mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}

impl CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
            Self::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Affirm payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsAffirm {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsAffirmSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Afterpay Clearpay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsAfterpayClearpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsAfterpayClearpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Alipay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsAlipay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsAlipaySetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the AU Becs Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsAuBecsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsAuBecsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Bacs Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsBacsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsBacsDebitSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsBacsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Bancontact payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsBancontact {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsBancontactSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Boleto payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsBoleto {
    /// The number of calendar days before a Boleto voucher expires.
    ///
    /// For example, if you create a Boleto voucher on Monday and you set expires_after_days to 2, the Boleto invoice will expire on Wednesday at 23:59 America/Sao_Paulo time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsBoletoSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsBoleto {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Card payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsCard<'a> {
    /// Installment options for card payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<CreateSessionPaymentMethodOptionsCardInstallments>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsCardSetupFutureUsage>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<&'a str>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 17 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<&'a str>,
}
impl<'a> CreateSessionPaymentMethodOptionsCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Installment options for card payments.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this Checkout Session.
    /// Setting to false will prevent any installment plan from applying to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl CreateSessionPaymentMethodOptionsCardInstallments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsCardSetupFutureUsage {
    OffSession,
    OnSession,
}

impl CreateSessionPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsCardSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Customer Balance payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<CreateSessionPaymentMethodOptionsCustomerBalanceBankTransfer<'a>>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<CreateSessionPaymentMethodOptionsCustomerBalanceFundingType>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}
impl<'a> CreateSessionPaymentMethodOptionsCustomerBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer:
        Option<CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<
        &'a [CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes],
    >,
    /// The list of bank transfer types that this PaymentIntent is allowed to use for funding.
    ///
    /// Permitted values include: `us_bank_account`, `eu_bank_account`, `id_bank_account`, `gb_bank_account`, `jp_bank_account`, `mx_bank_account`, `eu_bank_transfer`, `gb_bank_transfer`, `id_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    pub type_: CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferType,
}
impl<'a> CreateSessionPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    pub fn new(type_: CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferType) -> Self {
        Self {
            eu_bank_transfer: Default::default(),
            requested_address_types: Default::default(),
            type_,
        }
    }
}
/// Configuration for eu_bank_transfer funding type.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a> CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a> {
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// List of address types that should be returned in the financial_addresses response.
///
/// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    Iban,
    Sepa,
    SortCode,
    Spei,
    Zengin,
}

impl CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iban => "iban",
            Self::Sepa => "sepa",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str>
    for CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The list of bank transfer types that this PaymentIntent is allowed to use for funding.
///
/// Permitted values include: `us_bank_account`, `eu_bank_account`, `id_bank_account`, `gb_bank_account`, `jp_bank_account`, `mx_bank_account`, `eu_bank_transfer`, `gb_bank_transfer`, `id_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The funding method type to be used when there are not enough funds in the customer balance.
///
/// Permitted values include: `bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl CreateSessionPaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsCustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the EPS payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsEps {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsEpsSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsEpsSetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsEpsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsEpsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsEpsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the FPX payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsFpx {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsFpxSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsFpx {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsFpxSetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsFpxSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsFpxSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsFpxSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Giropay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsGiropay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsGiropaySetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Grabpay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsGrabpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsGrabpaySetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Ideal payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsIdeal {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsIdealSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsIdealSetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsIdealSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsIdealSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsIdealSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Klarna payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsKlarna {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsKlarnaSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Konbini payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsKonbini {
    /// The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire.
    ///
    /// For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
    /// Defaults to 3 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsKonbiniSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the OXXO payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsOxxo {
    /// The number of calendar days before an OXXO voucher expires.
    ///
    /// For example, if you create an OXXO voucher on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsOxxoSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the P24 payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsP24 {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsP24SetupFutureUsage>,
    /// Confirm that the payer has accepted the P24 terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}
impl CreateSessionPaymentMethodOptionsP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsP24SetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsP24SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsP24SetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsP24SetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the PayNow payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsPaynow {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsPaynowSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Pix payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsPix {
    /// The number of seconds (between 10 and 1209600) after which Pix payment will expire.
    ///
    /// Defaults to 86400 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
}
impl CreateSessionPaymentMethodOptionsPix {
    pub fn new() -> Self {
        Self::default()
    }
}
/// contains details about the Sepa Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsSepaDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Sofort payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsSofort {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsSofortSetupFutureUsage>,
}
impl CreateSessionPaymentMethodOptionsSofort {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsSofortSetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsSofortSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsSofortSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the Us Bank Account payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<CreateSessionPaymentMethodOptionsUsBankAccountFinancialConnections<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateSessionPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> CreateSessionPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Financial Connections Session creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions:
        Option<&'a [CreateSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions]>,
}
impl<'a> CreateSessionPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The list of permissions to request.
///
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl CreateSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
}

impl CreateSessionPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// contains details about the WeChat Pay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionPaymentMethodOptionsWechatPay<'a> {
    /// The app ID registered with WeChat Pay.
    ///
    /// Only required when client is ios or android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<&'a str>,
    /// The client type that the end customer will pay from.
    pub client: CreateSessionPaymentMethodOptionsWechatPayClient,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateSessionPaymentMethodOptionsWechatPaySetupFutureUsage>,
}
impl<'a> CreateSessionPaymentMethodOptionsWechatPay<'a> {
    pub fn new(client: CreateSessionPaymentMethodOptionsWechatPayClient) -> Self {
        Self { app_id: Default::default(), client, setup_future_usage: Default::default() }
    }
}
/// The client type that the end customer will pay from.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl CreateSessionPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::Ios => "ios",
            Self::Web => "web",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsWechatPayClient {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

impl CreateSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// A list of the types of payment methods (e.g., `card`) this Checkout Session can accept.
///
/// In `payment` and `subscription` mode, you can omit this attribute to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
/// It is required in `setup` mode.
///
/// Read more about the supported payment methods and their requirements in our [payment
/// method details guide](/docs/payments/checkout/payment-methods).
///
/// If multiple payment methods are passed, Checkout will dynamically reorder them to
/// prioritize the most relevant payment methods based on the customer's location and
/// other characteristics.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionPaymentMethodTypes {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    CustomerBalance,
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

impl CreateSessionPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
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

impl AsRef<str> for CreateSessionPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Controls phone number collection settings for the session.
///
/// We recommend that you review your privacy policy and check with your legal contacts
/// before using this feature.
///
/// Learn more about [collecting phone numbers with Checkout](https://stripe.com/docs/payments/checkout/phone-numbers).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionPhoneNumberCollection {
    /// Set to `true` to enable phone number collection.
    pub enabled: bool,
}
impl CreateSessionPhoneNumberCollection {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in `setup` mode.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionSetupIntentData<'a> {
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// The Stripe account for which the setup is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
}
impl<'a> CreateSessionSetupIntentData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// When set, provides configuration for Checkout to collect a shipping address from a customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionShippingAddressCollection<'a> {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
    /// shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: &'a [CreateSessionShippingAddressCollectionAllowedCountries],
}
impl<'a> CreateSessionShippingAddressCollection<'a> {
    pub fn new(
        allowed_countries: &'a [CreateSessionShippingAddressCollectionAllowedCountries],
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
pub enum CreateSessionShippingAddressCollectionAllowedCountries {
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

impl CreateSessionShippingAddressCollectionAllowedCountries {
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

impl AsRef<str> for CreateSessionShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The shipping rate options to apply to this Session.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionShippingOptions<'a> {
    /// The ID of the Shipping Rate to use for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
    /// Parameters to be passed to Shipping Rate creation for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<CreateSessionShippingOptionsShippingRateData<'a>>,
}
impl<'a> CreateSessionShippingOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Parameters to be passed to Shipping Rate creation for this shipping option.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionShippingOptionsShippingRateData<'a> {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<CreateSessionShippingOptionsShippingRateDataDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub display_name: &'a str,
    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateSessionShippingOptionsShippingRateDataFixedAmount>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateSessionShippingOptionsShippingRateDataTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateSessionShippingOptionsShippingRateDataType>,
}
impl<'a> CreateSessionShippingOptionsShippingRateData<'a> {
    pub fn new(display_name: &'a str) -> Self {
        Self {
            delivery_estimate: Default::default(),
            display_name,
            fixed_amount: Default::default(),
            metadata: Default::default(),
            tax_behavior: Default::default(),
            tax_code: Default::default(),
            type_: Default::default(),
        }
    }
}
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
///
/// This will appear on CheckoutSessions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionShippingOptionsShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum>,
    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum>,
}
impl CreateSessionShippingOptionsShippingRateDataDeliveryEstimate {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The upper bound of the estimated range.
///
/// If empty, represents no upper bound i.e., infinite.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum {
    pub fn new(
        unit: CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit,
        value: i64,
    ) -> Self {
        Self { unit, value }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl AsRef<str> for CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The lower bound of the estimated range.
///
/// If empty, represents no lower bound.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum {
    pub fn new(
        unit: CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit,
        value: i64,
    ) -> Self {
        Self { unit, value }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl AsRef<str> for CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Describes a fixed amount to charge for shipping.
///
/// Must be present if type is `fixed_amount`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionShippingOptionsShippingRateDataFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options:
        Option<CreateSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions>,
}
impl CreateSessionShippingOptionsShippingRateDataFixedAmount {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency, currency_options: Default::default() }
    }
}
/// Shipping rates defined in each available currency option.
///
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<CreateSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior>,
}
impl CreateSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions {
    pub fn new(amount: i64) -> Self {
        Self { amount, tax_behavior: Default::default() }
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str>
    for CreateSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionShippingOptionsShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateSessionShippingOptionsShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateSessionShippingOptionsShippingRateDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionShippingOptionsShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of calculation to use on the shipping rate.
///
/// Can only be `fixed_amount` for now.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionShippingOptionsShippingRateDataType {
    FixedAmount,
}

impl CreateSessionShippingOptionsShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedAmount => "fixed_amount",
        }
    }
}

impl AsRef<str> for CreateSessionShippingOptionsShippingRateDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionShippingOptionsShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Describes the type of transaction being performed by Checkout in order to customize
/// relevant text on the page, such as the submit button.
///
/// `submit_type` can only be specified on Checkout Sessions in `payment` mode, but not Checkout Sessions in `subscription` or `setup` mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}

impl CreateSessionSubmitType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Book => "book",
            Self::Donate => "donate",
            Self::Pay => "pay",
        }
    }
}

impl AsRef<str> for CreateSessionSubmitType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// A subset of parameters to be passed to subscription creation for Checkout Sessions in `subscription` mode.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSessionSubscriptionData<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// To use an application fee percent, the request must be made on behalf of another account, using the `Stripe-Account` header or an OAuth key.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// The ID of the coupon to apply to this subscription.
    ///
    /// A coupon applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// The tax rates that will apply to any subscription item that does not have
    /// `tax_rates` set.
    ///
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<&'a [&'a str]>,
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription
    /// for rendering in Stripe hosted surfaces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// This parameter is deprecated.
    ///
    /// Use the line_items parameter on the Session instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<&'a [CreateSessionSubscriptionDataItems<'a>]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// The account on behalf of which to charge, for each of the subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateSessionSubscriptionDataTransferData<'a>>,
    /// Unix timestamp representing the end of the trial period the customer
    /// will get before being charged for the first time.
    ///
    /// Has to be at least 48 hours in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<stripe_types::Timestamp>,
    /// Indicates if a plan’s `trial_period_days` should be applied to the subscription.
    ///
    /// Setting `trial_end` on `subscription_data` is preferred.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_from_plan: Option<bool>,
    /// Integer representing the number of trial period days before the
    /// customer is charged for the first time.
    ///
    /// Has to be at least 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}
impl<'a> CreateSessionSubscriptionData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This parameter is deprecated.
///
/// Use the line_items parameter on the Session instead.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionSubscriptionDataItems<'a> {
    /// Plan ID for this item.
    pub plan: &'a str,
    /// The quantity of the subscription item being purchased.
    ///
    /// Quantity should not be defined when `recurring.usage_type=metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to this item.
    ///
    /// When set, the `default_tax_rates` on `subscription_data` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> CreateSessionSubscriptionDataItems<'a> {
    pub fn new(plan: &'a str) -> Self {
        Self { plan, quantity: Default::default(), tax_rates: Default::default() }
    }
}
/// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionSubscriptionDataTransferData<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> CreateSessionSubscriptionDataTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount_percent: Default::default(), destination }
    }
}
/// Controls tax ID collection settings for the session.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSessionTaxIdCollection {
    /// Set to true to enable Tax ID collection.
    pub enabled: bool,
}
impl CreateSessionTaxIdCollection {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListLineItemsSession<'a> {
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
impl<'a> ListLineItemsSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ExpireSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ExpireSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
