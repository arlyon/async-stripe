#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListCheckoutSession<'a> {
    /// Only return the Checkout Sessions that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// Only return the Checkout Sessions for the Customer specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Only return the Checkout Sessions for the Customer details specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<ListCheckoutSessionCustomerDetails<'a>>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return the Checkout Session for the PaymentIntent specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<&'a str>,
    /// Only return the Checkout Sessions for the Payment Link specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<&'a str>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return the Checkout Sessions matching the given status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_checkout::CheckoutSessionStatus>,
    /// Only return the Checkout Session for the subscription specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
}
impl<'a> ListCheckoutSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Only return the Checkout Sessions for the Customer details specified.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListCheckoutSessionCustomerDetails<'a> {
    /// Customer's email address.
    pub email: &'a str,
}
impl<'a> ListCheckoutSessionCustomerDetails<'a> {
    pub fn new(email: &'a str) -> Self {
        Self { email }
    }
}
impl<'a> ListCheckoutSession<'a> {
    /// Returns a list of Checkout Sessions.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_checkout::CheckoutSession>> {
        client.get_query("/checkout/sessions", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_checkout::CheckoutSession>> {
        stripe::ListPaginator::from_list_params("/checkout/sessions", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCheckoutSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCheckoutSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveCheckoutSession<'a> {
    /// Retrieves a Session object.
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &stripe_checkout::CheckoutSessionId,
    ) -> stripe::Response<stripe_checkout::CheckoutSession> {
        client.get_query(&format!("/checkout/sessions/{session}"), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListLineItemsCheckoutSession<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListLineItemsCheckoutSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListLineItemsCheckoutSession<'a> {
    /// When retrieving a Checkout Session, there is an includable **line_items** property containing the first handful of those items.
    /// There is also a URL where you can retrieve the full (paginated) list of line items.
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &stripe_checkout::CheckoutSessionId,
    ) -> stripe::Response<stripe_types::List<stripe_shared::CheckoutSessionItem>> {
        client.get_query(&format!("/checkout/sessions/{session}/line_items"), self)
    }
    pub fn paginate(
        self,
        session: &stripe_checkout::CheckoutSessionId,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_shared::CheckoutSessionItem>> {
        stripe::ListPaginator::from_list_params(
            &format!("/checkout/sessions/{session}/line_items"),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSession<'a> {
    /// Configure actions after a Checkout Session has expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<CreateCheckoutSessionAfterExpiration>,
    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    /// Settings for automatic tax lookup for this session and resulting payments, invoices, and subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateCheckoutSessionAutomaticTax<'a>>,
    /// Specify whether Checkout should collect the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection:
        Option<stripe_checkout::CheckoutSessionBillingAddressCollection>,
    /// If set, Checkout displays a back button and customers will be directed to this URL if they decide to cancel payment and return to your website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<&'a str>,
    /// A unique string to reference the Checkout Session. This can be a
    /// customer ID, a cart ID, or similar, and can be used to reconcile the
    /// session with your internal systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<&'a str>,
    /// Configure fields for the Checkout Session to gather active consent from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<CreateCheckoutSessionConsentCollection>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Required in `setup` mode when `payment_method_types` is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Collect additional information from your customer using custom fields.
    /// Up to 3 fields are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [CreateCheckoutSessionCustomFields<'a>]>,
    /// Display additional text for your customers using custom text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_text: Option<CreateCheckoutSessionCustomText<'a>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<CreateCheckoutSessionCustomerCreation>,
    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file. To access information about the customer once a session is
    /// complete, use the `customer` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<&'a str>,
    /// Controls what fields on Customer can be updated by the Checkout Session.
    /// Can only be provided when `customer` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<CreateCheckoutSessionCustomerUpdate>,
    /// The coupon or promotion code to apply to this Session. Currently, only up to one may be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [CreateCheckoutSessionDiscounts<'a>]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The Epoch time in seconds at which the Checkout Session will expire.
    /// It can be anywhere from 30 minutes to 24 hours after Checkout Session creation.
    /// By default, this value is 24 hours from creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Generate a post-purchase Invoice for one-time payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<CreateCheckoutSessionInvoiceCreation<'a>>,
    /// A list of items the customer is purchasing.
    /// Use this parameter to pass one-time or recurring [Prices](https://stripe.com/docs/api/prices).
    ///
    /// For `payment` mode, there is a maximum of 100 line items, however it is recommended to consolidate line items if there are more than a few dozen.
    ///
    /// For `subscription` mode, there is a maximum of 20 line items with recurring Prices and 20 line items with one-time Prices.
    /// Line items with one-time Prices will be on the initial invoice only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<&'a [CreateCheckoutSessionLineItems<'a>]>,
    /// The IETF language tag of the locale Checkout is displayed in.
    /// If blank or `auto`, the browser's locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<stripe_checkout::CheckoutSessionLocale>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The mode of the Checkout Session.
    /// Pass `subscription` if the Checkout Session includes at least one recurring item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<stripe_checkout::CheckoutSessionMode>,
    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<CreateCheckoutSessionPaymentIntentData<'a>>,
    /// Specify whether Checkout should collect a payment method.
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.
    /// This may occur if the Checkout Session includes a free trial or a discount.
    ///
    /// Can only be set in `subscription` mode.
    ///
    /// If you'd like information on how to collect a payment method outside of Checkout, read the guide on configuring [subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<CreateCheckoutSessionPaymentMethodCollection>,
    /// The ID of the payment method configuration to use with this Checkout session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<&'a str>,
    /// Payment-method-specific configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateCheckoutSessionPaymentMethodOptions<'a>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [CreateCheckoutSessionPaymentMethodTypes]>,
    /// Controls phone number collection settings for the session.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts
    /// before using this feature.
    /// Learn more about [collecting phone numbers with Checkout](https://stripe.com/docs/payments/checkout/phone-numbers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<CreateCheckoutSessionPhoneNumberCollection>,
    /// This parameter applies to `ui_mode: embedded`.
    /// By default, Stripe will always redirect to your return_url after a successful confirmation.
    /// If you set `redirect_on_completion: 'if_required'`, then we will only redirect if your user chooses a redirect-based payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_on_completion: Option<stripe_checkout::CheckoutSessionRedirectOnCompletion>,
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the
    /// payment method's app or site. This parameter is required if ui_mode is `embedded`
    /// and redirect-based payment methods are enabled on the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    /// A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent_data: Option<CreateCheckoutSessionSetupIntentData<'a>>,
    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<CreateCheckoutSessionShippingAddressCollection<'a>>,
    /// The shipping rate options to apply to this Session. Up to a maximum of 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<&'a [CreateCheckoutSessionShippingOptions<'a>]>,
    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button. `submit_type` can only be
    /// specified on Checkout Sessions in `payment` mode. If blank or `auto`, `pay` is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<stripe_checkout::CheckoutSessionSubmitType>,
    /// A subset of parameters to be passed to subscription creation for Checkout Sessions in `subscription` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<CreateCheckoutSessionSubscriptionData<'a>>,
    /// The URL to which Stripe should send customers when payment or setup
    /// is complete.
    /// This parameter is not allowed if ui_mode is `embedded`. If you’d like to use
    /// information from the successful Checkout Session on your page, read the
    /// guide on [customizing your success page](https://stripe.com/docs/payments/checkout/custom-success-page).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<&'a str>,
    /// Controls tax ID collection settings for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<CreateCheckoutSessionTaxIdCollection>,
    /// `ui_mode` can be `hosted` or `embedded`. The default is `hosted`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_mode: Option<stripe_checkout::CheckoutSessionUiMode>,
}
impl<'a> CreateCheckoutSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configure actions after a Checkout Session has expired.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionAfterExpiration {
    /// Configure a Checkout Session that can be used to recover an expired session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery: Option<CreateCheckoutSessionAfterExpirationRecovery>,
}
impl CreateCheckoutSessionAfterExpiration {
    pub fn new() -> Self {
        Self::default()
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
    pub fn new(enabled: bool) -> Self {
        Self { allow_promotion_codes: None, enabled }
    }
}
/// Settings for automatic tax lookup for this session and resulting payments, invoices, and subscriptions.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionAutomaticTax<'a> {
    /// Set to true to enable automatic taxes.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreateCheckoutSessionAutomaticTaxLiability<'a>>,
}
impl<'a> CreateCheckoutSessionAutomaticTax<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionAutomaticTaxLiability<'a> {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionAutomaticTaxLiabilityType,
}
impl<'a> CreateCheckoutSessionAutomaticTaxLiability<'a> {
    pub fn new(type_: CreateCheckoutSessionAutomaticTaxLiabilityType) -> Self {
        Self { account: None, type_ }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(()),
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
/// Configure fields for the Checkout Session to gather active consent from customers.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
        position: CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition,
    ) -> Self {
        Self { position }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition::*;
        match s {
            "auto" => Ok(Auto),
            "hidden" => Ok(Hidden),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionConsentCollectionPromotions::*;
        match s {
            "auto" => Ok(Auto),
            "none" => Ok(None),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionConsentCollectionTermsOfService::*;
        match s {
            "none" => Ok(None),
            "required" => Ok(Required),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionCustomFields<'a> {
    /// Configuration for `type=dropdown` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<CreateCheckoutSessionCustomFieldsDropdown<'a>>,
    /// String of your choice that your integration can use to reconcile this field.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: &'a str,
    /// The label for the field, displayed to the customer.
    pub label: CreateCheckoutSessionCustomFieldsLabel<'a>,
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
impl<'a> CreateCheckoutSessionCustomFields<'a> {
    pub fn new(
        key: &'a str,
        label: CreateCheckoutSessionCustomFieldsLabel<'a>,
        type_: CreateCheckoutSessionCustomFieldsType,
    ) -> Self {
        Self { dropdown: None, key, label, numeric: None, optional: None, text: None, type_ }
    }
}
/// Configuration for `type=dropdown` fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsDropdown<'a> {
    /// The options available for the customer to select. Up to 200 options allowed.
    pub options: &'a [CreateCheckoutSessionCustomFieldsDropdownOptions<'a>],
}
impl<'a> CreateCheckoutSessionCustomFieldsDropdown<'a> {
    pub fn new(options: &'a [CreateCheckoutSessionCustomFieldsDropdownOptions<'a>]) -> Self {
        Self { options }
    }
}
/// The options available for the customer to select. Up to 200 options allowed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsDropdownOptions<'a> {
    /// The label for the option, displayed to the customer. Up to 100 characters.
    pub label: &'a str,
    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: &'a str,
}
impl<'a> CreateCheckoutSessionCustomFieldsDropdownOptions<'a> {
    pub fn new(label: &'a str, value: &'a str) -> Self {
        Self { label, value }
    }
}
/// The label for the field, displayed to the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsLabel<'a> {
    /// Custom text for the label, displayed to the customer. Up to 50 characters.
    pub custom: &'a str,
    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionCustomFieldsLabelType,
}
impl<'a> CreateCheckoutSessionCustomFieldsLabel<'a> {
    pub fn new(custom: &'a str, type_: CreateCheckoutSessionCustomFieldsLabelType) -> Self {
        Self { custom, type_ }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionCustomFieldsLabelType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsNumeric {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
impl CreateCheckoutSessionCustomFieldsNumeric {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for `type=text` fields.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
impl CreateCheckoutSessionCustomFieldsText {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionCustomFieldsType::*;
        match s {
            "dropdown" => Ok(Dropdown),
            "numeric" => Ok(Numeric),
            "text" => Ok(Text),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionCustomText<'a> {
    /// Custom text that should be displayed after the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_submit: Option<CustomTextPositionParam<'a>>,
    /// Custom text that should be displayed alongside shipping address collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CustomTextPositionParam<'a>>,
    /// Custom text that should be displayed alongside the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<CustomTextPositionParam<'a>>,
    /// Custom text that should be displayed in place of the default terms of service agreement text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance: Option<CustomTextPositionParam<'a>>,
}
impl<'a> CreateCheckoutSessionCustomText<'a> {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionCustomerCreation::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionCustomerUpdateAddress::*;
        match s {
            "auto" => Ok(Auto),
            "never" => Ok(Never),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionCustomerUpdateName::*;
        match s {
            "auto" => Ok(Auto),
            "never" => Ok(Never),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionCustomerUpdateShipping::*;
        match s {
            "auto" => Ok(Auto),
            "never" => Ok(Never),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionDiscounts<'a> {
    /// The ID of the coupon to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// The ID of a promotion code to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
}
impl<'a> CreateCheckoutSessionDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Generate a post-purchase Invoice for one-time payments.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreation<'a> {
    /// Set to `true` to enable invoice creation.
    pub enabled: bool,
    /// Parameters passed when creating invoices for payment-mode Checkout Sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<CreateCheckoutSessionInvoiceCreationInvoiceData<'a>>,
}
impl<'a> CreateCheckoutSessionInvoiceCreation<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, invoice_data: None }
    }
}
/// Parameters passed when creating invoices for payment-mode Checkout Sessions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceData<'a> {
    /// The account tax IDs associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<&'a [&'a str]>,
    /// Default custom fields to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields:
        Option<&'a [CreateCheckoutSessionInvoiceCreationInvoiceDataCustomFields<'a>]>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions>,
}
impl<'a> CreateCheckoutSessionInvoiceCreationInvoiceData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Default custom fields to be displayed on invoices for this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceDataCustomFields<'a> {
    /// The name of the custom field. This may be up to 30 characters.
    pub name: &'a str,
    /// The value of the custom field. This may be up to 30 characters.
    pub value: &'a str,
}
impl<'a> CreateCheckoutSessionInvoiceCreationInvoiceDataCustomFields<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer<'a> {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType,
}
impl<'a> CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer<'a> {
    pub fn new(type_: CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType) -> Self {
        Self { account: None, type_ }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display:
        Option<CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay>,
}
impl CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(()),
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
///
/// For `payment` mode, there is a maximum of 100 line items, however it is recommended to consolidate line items if there are more than a few dozen.
///
/// For `subscription` mode, there is a maximum of 20 line items with recurring Prices and 20 line items with one-time Prices.
/// Line items with one-time Prices will be on the initial invoice only.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionLineItems<'a> {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreateCheckoutSessionLineItemsAdjustableQuantity>,
    /// The [tax rates](https://stripe.com/docs/api/tax_rates) that will be applied to this line item depending on the customer's billing/shipping address.
    /// We currently support the following countries: US, GB, AU, and all countries in the EU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_tax_rates: Option<&'a [&'a str]>,
    /// The ID of the [Price](https://stripe.com/docs/api/prices) or [Plan](https://stripe.com/docs/api/plans) object.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateCheckoutSessionLineItemsPriceData<'a>>,
    /// The quantity of the line item being purchased.
    /// Quantity should not be defined when `recurring.usage_type=metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The [tax rates](https://stripe.com/docs/api/tax_rates) which apply to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> CreateCheckoutSessionLineItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// When set, provides configuration for this item’s quantity to be adjusted by the customer during Checkout.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionLineItemsAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative integer.
    /// By default customers will be able to remove the line item by setting the quantity to 0.
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
    pub fn new(enabled: bool) -> Self {
        Self { enabled, maximum: None, minimum: None }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
/// One of `price` or `price_data` is required.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    /// One of `product` or `product_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// Data used to generate a new product object inline. One of `product` or `product_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<CreateCheckoutSessionLineItemsPriceDataProductData<'a>>,
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
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateCheckoutSessionLineItemsPriceData<'a> {
    pub fn new(currency: stripe_types::Currency) -> Self {
        Self {
            currency,
            product: None,
            product_data: None,
            recurring: None,
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Data used to generate a new product object inline. One of `product` or `product_data` is required.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceDataProductData<'a> {
    /// The product's description, meant to be displayable to the customer.
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    pub name: &'a str,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
}
impl<'a> CreateCheckoutSessionLineItemsPriceDataProductData<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { description: None, images: None, metadata: None, name, tax_code: None }
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
    pub fn new(interval: CreateCheckoutSessionLineItemsPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: None }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionLineItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionLineItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
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
/// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentIntentData<'a> {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    /// The amount of the application fee collected will be capped at the total payment amount.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentIntentDataCaptureMethod>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The Stripe account ID for which these funds are intended. For details,
    /// see the PaymentIntents [use case for connected
    /// accounts](/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// Email address that the receipt for the resulting payment will be sent to.
    /// If `receipt_email` is specified for a payment in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
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
    pub shipping: Option<CreateCheckoutSessionPaymentIntentDataShipping<'a>>,
    /// Extra information about the payment. This will appear on your
    /// customer's statement when this payment succeeds in creating a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// Provides information about the charge that customers see on their statements. Concatenated with the
    /// prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete.
    /// statement descriptor. Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,
    /// The parameters used to automatically create a Transfer when the payment succeeds.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateCheckoutSessionPaymentIntentDataTransferData<'a>>,
    /// A string that identifies the resulting payment as part of a group.
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}
impl<'a> CreateCheckoutSessionPaymentIntentData<'a> {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentIntentDataCaptureMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "automatic_async" => Ok(AutomaticAsync),
            "manual" => Ok(Manual),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentIntentDataSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataShipping<'a> {
    /// Shipping address.
    pub address: CreateCheckoutSessionPaymentIntentDataShippingAddress<'a>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<&'a str>,
    /// Recipient name.
    pub name: &'a str,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The tracking number for a physical product, obtained from the delivery service.
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<&'a str>,
}
impl<'a> CreateCheckoutSessionPaymentIntentDataShipping<'a> {
    pub fn new(
        address: CreateCheckoutSessionPaymentIntentDataShippingAddress<'a>,
        name: &'a str,
    ) -> Self {
        Self { address, carrier: None, name, phone: None, tracking_number: None }
    }
}
/// Shipping address.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataShippingAddress<'a> {
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
impl<'a> CreateCheckoutSessionPaymentIntentDataShippingAddress<'a> {
    pub fn new(line1: &'a str) -> Self {
        Self { city: None, country: None, line1, line2: None, postal_code: None, state: None }
    }
}
/// The parameters used to automatically create a Transfer when the payment succeeds.
/// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataTransferData<'a> {
    /// The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// If specified, successful charges will be attributed to the destination
    /// account for tax reporting, and the funds from charges will be transferred
    /// to the destination account. The ID of the resulting transfer will be
    /// returned on the successful charge's `transfer` field.
    pub destination: &'a str,
}
impl<'a> CreateCheckoutSessionPaymentIntentDataTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: None, destination }
    }
}
/// Specify whether Checkout should collect a payment method.
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.
/// This may occur if the Checkout Session includes a free trial or a discount.
///
/// Can only be set in `subscription` mode.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodCollection::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(()),
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
/// Payment-method-specific configuration.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptions<'a> {
    /// contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebit<'a>>,
    /// contains details about the Affirm payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreateCheckoutSessionPaymentMethodOptionsAffirm>,
    /// contains details about the Afterpay Clearpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay>,
    /// contains details about the Alipay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreateCheckoutSessionPaymentMethodOptionsAlipay>,
    /// contains details about the AU Becs Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit>,
    /// contains details about the Bacs Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreateCheckoutSessionPaymentMethodOptionsBacsDebit>,
    /// contains details about the Bancontact payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateCheckoutSessionPaymentMethodOptionsBancontact>,
    /// contains details about the Boleto payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreateCheckoutSessionPaymentMethodOptionsBoleto>,
    /// contains details about the Card payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateCheckoutSessionPaymentMethodOptionsCard<'a>>,
    /// contains details about the Cashapp Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<CreateCheckoutSessionPaymentMethodOptionsCashapp>,
    /// contains details about the Customer Balance payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalance<'a>>,
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
    /// contains details about the Klarna payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateCheckoutSessionPaymentMethodOptionsKlarna>,
    /// contains details about the Konbini payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreateCheckoutSessionPaymentMethodOptionsKonbini>,
    /// contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateCheckoutSessionPaymentMethodOptionsLink>,
    /// contains details about the OXXO payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreateCheckoutSessionPaymentMethodOptionsOxxo>,
    /// contains details about the P24 payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreateCheckoutSessionPaymentMethodOptionsP24>,
    /// contains details about the PayNow payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreateCheckoutSessionPaymentMethodOptionsPaynow>,
    /// contains details about the PayPal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CreateCheckoutSessionPaymentMethodOptionsPaypal<'a>>,
    /// contains details about the Pix payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreateCheckoutSessionPaymentMethodOptionsPix>,
    /// contains details about the RevolutPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<CreateCheckoutSessionPaymentMethodOptionsRevolutPay>,
    /// contains details about the Sepa Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateCheckoutSessionPaymentMethodOptionsSepaDebit>,
    /// contains details about the Sofort payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateCheckoutSessionPaymentMethodOptionsSofort>,
    /// contains details about the Swish payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<CreateCheckoutSessionPaymentMethodOptionsSwish<'a>>,
    /// contains details about the Us Bank Account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccount<'a>>,
    /// contains details about the WeChat Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreateCheckoutSessionPaymentMethodOptionsWechatPay<'a>>,
}
impl<'a> CreateCheckoutSessionPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// contains details about the ACSS Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebit<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// This is only accepted for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> CreateCheckoutSessionPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitCurrency::*;
        match s {
            "cad" => Ok(Cad),
            "usd" => Ok(Usd),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,.
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// List of Stripe products where this mandate can be selected automatically.
    /// Only usable in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<&'a [CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor]>,
    /// Description of the mandate interval.
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}
impl<'a> CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::*;
        match s {
            "invoice" => Ok(Invoice),
            "subscription" => Ok(Subscription),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(()),
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
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAffirm {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAlipay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
/// contains details about the AU Becs Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBacsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsBacsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBancontact {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
/// contains details about the Boleto payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBoleto {
    /// The number of calendar days before a Boleto voucher expires.
    /// For example, if you create a Boleto voucher on Monday and you set expires_after_days to 2, the Boleto invoice will expire on Wednesday at 23:59 America/Sao_Paulo time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsBoleto {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCard<'a> {
    /// Installment options for card payments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<CreateCheckoutSessionPaymentMethodOptionsCardInstallments>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage>,
    /// Provides information about a card payment that customers see on their statements.
    /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<&'a str>,
    /// Provides information about a card payment that customers see on their statements.
    /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 17 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<&'a str>,
}
impl<'a> CreateCheckoutSessionPaymentMethodOptionsCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Installment options for card payments
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this Checkout Session.
    /// Setting to false will prevent any installment plan from applying to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl CreateCheckoutSessionPaymentMethodOptionsCardInstallments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCashapp {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsCashapp {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer<'a>>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}
impl<'a> CreateCheckoutSessionPaymentMethodOptionsCustomerBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    /// Configuration for eu_bank_transfer funding type.
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>>,
        /// List of address types that should be returned in the financial_addresses response.
    /// If not specified, all valid types will be returned.
    ///
    /// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[serde(skip_serializing_if = "Option::is_none")]
pub requested_address_types: Option<&'a [CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes]>,
    /// The list of bank transfer types that this PaymentIntent is allowed to use for funding.
#[serde(rename = "type")]
pub type_: CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType,

}
impl<'a> CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    pub fn new(
        type_: CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType,
    ) -> Self {
        Self { eu_bank_transfer: None, requested_address_types: None, type_ }
    }
}
/// Configuration for eu_bank_transfer funding type.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a> {
    /// The desired country code of the bank account information.
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a> CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a> {
    pub fn new(country: &'a str) -> Self {
        Self { country }
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
    type Err = ();
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
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "gb_bank_transfer" => Ok(GbBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            "mx_bank_transfer" => Ok(MxBankTransfer),
            "us_bank_transfer" => Ok(UsBankTransfer),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            _ => Err(()),
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
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
/// contains details about the EPS payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsEps {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsFpx {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsFpx {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsGiropay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsGrabpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsIdeal {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
/// contains details about the Klarna payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKlarna {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
/// contains details about the Konbini payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKonbini {
    /// The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire.
    /// For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
    /// Defaults to 3 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
/// contains details about the Link payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsLink {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsLink {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(()),
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
/// contains details about the OXXO payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsOxxo {
    /// The number of calendar days before an OXXO voucher expires.
    /// For example, if you create an OXXO voucher on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsP24 {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage>,
    /// Confirm that the payer has accepted the P24 terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}
impl CreateCheckoutSessionPaymentMethodOptionsP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
/// contains details about the PayNow payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPaynow {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPaypal<'a> {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod>,
    /// [Preferred locale](https://stripe.com/docs/payments/paypal/supported-locales) of the PayPal checkout page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale>,
    /// A reference of the PayPal transaction visible to customer which is mapped to PayPal's invoice ID.
    /// This must be a globally unique ID if you have configured in your PayPal settings to block multiple payments per invoice ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
    /// The risk correlation ID for an on-session payment using a saved PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_correlation_id: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage>,
}
impl<'a> CreateCheckoutSessionPaymentMethodOptionsPaypal<'a> {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(()),
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
#[derive(Copy, Clone, Eq, PartialEq)]
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
    Unknown,
}
impl CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    pub fn as_str(self) -> &'static str {
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
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    type Err = ();
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
            _ => Err(()),
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
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
///
/// If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPix {
    /// The number of seconds (between 10 and 1209600) after which Pix payment will expire.
    /// Defaults to 86400 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
}
impl CreateCheckoutSessionPaymentMethodOptionsPix {
    pub fn new() -> Self {
        Self::default()
    }
}
/// contains details about the RevolutPay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsRevolutPay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsRevolutPay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(()),
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
/// contains details about the Sepa Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSepaDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSofort {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage>,
}
impl CreateCheckoutSessionPaymentMethodOptionsSofort {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSwish<'a> {
    /// The order reference that will be displayed to customers in the Swish application.
    /// Defaults to the `id` of the Payment Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
}
impl<'a> CreateCheckoutSessionPaymentMethodOptionsSwish<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// contains details about the Us Bank Account payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> CreateCheckoutSessionPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Financial Connections Session creation
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
        /// The list of permissions to request.
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[serde(skip_serializing_if = "Option::is_none")]
pub permissions: Option<&'a [CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions]>,
    /// List of data features that you would like to retrieve upon account creation.
#[serde(skip_serializing_if = "Option::is_none")]
pub prefetch: Option<&'a [CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch]>,

}
impl<'a> CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
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
    Transactions,
}
impl CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match self {
            Balances => "balances",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "transactions" => Ok(Transactions),
            _ => Err(()),
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
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsWechatPay<'a> {
    /// The app ID registered with WeChat Pay. Only required when client is ios or android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<&'a str>,
    /// The client type that the end customer will pay from
    pub client: CreateCheckoutSessionPaymentMethodOptionsWechatPayClient,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage>,
}
impl<'a> CreateCheckoutSessionPaymentMethodOptionsWechatPay<'a> {
    pub fn new(client: CreateCheckoutSessionPaymentMethodOptionsWechatPayClient) -> Self {
        Self { app_id: None, client, setup_future_usage: None }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsWechatPayClient::*;
        match s {
            "android" => Ok(Android),
            "ios" => Ok(Ios),
            "web" => Ok(Web),
            _ => Err(()),
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
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(()),
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
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateCheckoutSessionPaymentMethodTypes {
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
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    RevolutPay,
    SepaDebit,
    Sofort,
    Swish,
    UsBankAccount,
    WechatPay,
    Zip,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateCheckoutSessionPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use CreateCheckoutSessionPaymentMethodTypes::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Blik => "blik",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            CustomerBalance => "customer_balance",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            Klarna => "klarna",
            Konbini => "konbini",
            Link => "link",
            Oxxo => "oxxo",
            P24 => "p24",
            Paynow => "paynow",
            Paypal => "paypal",
            Pix => "pix",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            Swish => "swish",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionPaymentMethodTypes::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "customer_balance" => Ok(CustomerBalance),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "klarna" => Ok(Klarna),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "oxxo" => Ok(Oxxo),
            "p24" => Ok(P24),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "swish" => Ok(Swish),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            _ => Err(()),
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
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
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
    pub enabled: bool,
}
impl CreateCheckoutSessionPhoneNumberCollection {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in `setup` mode.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionSetupIntentData<'a> {
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The Stripe account for which the setup is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
}
impl<'a> CreateCheckoutSessionSetupIntentData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// When set, provides configuration for Checkout to collect a shipping address from a customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionShippingAddressCollection<'a> {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
    /// shipping locations.
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: &'a [CreateCheckoutSessionShippingAddressCollectionAllowedCountries],
}
impl<'a> CreateCheckoutSessionShippingAddressCollection<'a> {
    pub fn new(
        allowed_countries: &'a [CreateCheckoutSessionShippingAddressCollectionAllowedCountries],
    ) -> Self {
        Self { allowed_countries }
    }
}
/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
/// shipping locations.
/// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
#[derive(Copy, Clone, Eq, PartialEq)]
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
    Unknown,
}
impl CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
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
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    type Err = ();
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
            _ => Err(()),
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
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
/// The shipping rate options to apply to this Session. Up to a maximum of 5.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionShippingOptions<'a> {
    /// The ID of the Shipping Rate to use for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
    /// Parameters to be passed to Shipping Rate creation for this shipping option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<CreateCheckoutSessionShippingOptionsShippingRateData<'a>>,
}
impl<'a> CreateCheckoutSessionShippingOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Parameters to be passed to Shipping Rate creation for this shipping option
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateData<'a> {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate:
        Option<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    pub display_name: &'a str,
    /// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The type of calculation to use on the shipping rate. Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateCheckoutSessionShippingOptionsShippingRateDataType>,
}
impl<'a> CreateCheckoutSessionShippingOptionsShippingRateData<'a> {
    pub fn new(display_name: &'a str) -> Self {
        Self {
            delivery_estimate: None,
            display_name,
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
        unit: CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit,
        value: i64,
    ) -> Self {
        Self { unit, value }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(()),
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
        unit: CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit,
        value: i64,
    ) -> Self {
        Self { unit, value }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount<'a> {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        &'a std::collections::HashMap<
            stripe_types::Currency,
            CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions,
        >,
    >,
}
impl<'a> CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency, currency_options: None }
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
    pub fn new(amount: i64) -> Self {
        Self { amount, tax_behavior: None }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
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
/// The type of calculation to use on the shipping rate. Can only be `fixed_amount` for now.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionShippingOptionsShippingRateDataType::*;
        match s {
            "fixed_amount" => Ok(FixedAmount),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionData<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// To use an application fee percent, the request must be made on behalf of another account, using the `Stripe-Account` header or an OAuth key.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// A future timestamp to anchor the subscription's billing cycle for new subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<stripe_types::Timestamp>,
    /// The tax rates that will apply to any subscription item that does not have
    /// `tax_rates` set. Invoices created will have their `default_tax_rates` populated
    /// from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<&'a [&'a str]>,
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription
    /// for rendering in the [customer portal](https://stripe.com/docs/customer-management).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateCheckoutSessionSubscriptionDataInvoiceSettings<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The account on behalf of which to charge, for each of the subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// Determines how to handle prorations resulting from the `billing_cycle_anchor`.
    /// If no value is passed, the default is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreateCheckoutSessionSubscriptionDataProrationBehavior>,
    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateCheckoutSessionSubscriptionDataTransferData<'a>>,
    /// Unix timestamp representing the end of the trial period the customer
    /// will get before being charged for the first time. Has to be at least
    /// 48 hours in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<stripe_types::Timestamp>,
    /// Integer representing the number of trial period days before the
    /// customer is charged for the first time. Has to be at least 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<CreateCheckoutSessionSubscriptionDataTrialSettings>,
}
impl<'a> CreateCheckoutSessionSubscriptionData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataInvoiceSettings<'a> {
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer<'a>>,
}
impl<'a> CreateCheckoutSessionSubscriptionDataInvoiceSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer<'a> {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType,
}
impl<'a> CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer<'a> {
    pub fn new(type_: CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType) -> Self {
        Self { account: None, type_ }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionSubscriptionDataProrationBehavior::*;
        match s {
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTransferData<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> CreateCheckoutSessionSubscriptionDataTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount_percent: None, destination }
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
        end_behavior: CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior,
    ) -> Self {
        Self { end_behavior }
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
        missing_payment_method: CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
    ) -> Self {
        Self { missing_payment_method }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match s {
            "cancel" => Ok(Cancel),
            "create_invoice" => Ok(CreateInvoice),
            "pause" => Ok(Pause),
            _ => Err(()),
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
/// Controls tax ID collection settings for the session.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCheckoutSessionTaxIdCollection {
    /// Set to true to enable Tax ID collection.
    pub enabled: bool,
}
impl CreateCheckoutSessionTaxIdCollection {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
impl<'a> CreateCheckoutSession<'a> {
    /// Creates a Session object.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_checkout::CheckoutSession> {
        client.send_form("/checkout/sessions", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ExpireCheckoutSession<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ExpireCheckoutSession<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ExpireCheckoutSession<'a> {
    /// A Session can be expired when it is in one of these statuses: `open`
    ///
    /// After it expires, a customer can’t complete a Session and customers loading the Session see a message saying the Session is expired.
    pub fn send(
        &self,
        client: &stripe::Client,
        session: &stripe_checkout::CheckoutSessionId,
    ) -> stripe::Response<stripe_checkout::CheckoutSession> {
        client.send_form(
            &format!("/checkout/sessions/{session}/expire"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CustomTextPositionParam<'a> {
    /// Text may be up to 1200 characters in length.
    pub message: &'a str,
}
impl<'a> CustomTextPositionParam<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}
