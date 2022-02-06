// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{CustomerId, PaymentIntentId, SubscriptionId};
use crate::params::{Expand, List, Metadata, Object, Timestamp};
use crate::resources::Currency;
use crate::CheckoutSession;

impl CheckoutSession {
    /// Returns a list of Checkout Sessions.
    pub fn list(
        client: &Client,
        params: ListCheckoutSessions<'_>,
    ) -> Response<List<CheckoutSession>> {
        client.get_query("/checkout/sessions", &params)
    }

    /// Creates a Session object.
    pub fn create(client: &Client, params: CreateCheckoutSession<'_>) -> Response<CheckoutSession> {
        client.post_form("/checkout/sessions", &params)
    }
}

impl Object for CheckoutSession {
    type Id = CheckoutSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "checkout.session"
    }
}

// written at 597
/// The parameters for `CheckoutSession::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateCheckoutSession<'a> {
    /// Configure actions after a Checkout Session has expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<Box<CreateCheckoutSessionAfterExpiration>>,

    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,

    /// Settings for automatic tax lookup for this session and resulting payments, invoices, and subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<Box<CreateCheckoutSessionAutomaticTax>>,

    /// Specify whether Checkout should collect the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<CheckoutSessionBillingAddressCollection>,

    /// The URL the customer will be directed to if they decide to cancel payment and return to your website.
    pub cancel_url: &'a str,

    /// A unique string to reference the Checkout Session.
    ///
    /// This can be a customer ID, a cart ID, or similar, and can be used to reconcile the session with your internal systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<&'a str>,

    /// Configure fields for the Checkout Session to gather active consent from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<Box<CreateCheckoutSessionConsentCollection>>,

    /// ID of an existing Customer, if one exists.
    ///
    /// In `payment` mode, the customer’s most recent card payment method will be used to prefill the email, name, card details, and billing address on the Checkout page.
    /// In `subscription` mode, the customer’s [default payment method](https://stripe.com/docs/api/customers/update#update_customer-invoice_settings-default_payment_method) will be used if it’s a card, and otherwise the most recent card will be used.
    /// A valid billing address, billing name and billing email are required on the payment method for Checkout to prefill the customer's card details.  If the Customer already has a valid [email](https://stripe.com/docs/api/customers/object#customer_object-email) set, the email will be prefilled and not editable in Checkout. If the Customer does not have a valid `email`, Checkout will set the email entered during the session on the Customer.  If blank for Checkout Sessions in `payment` or `subscription` mode, Checkout will create a new Customer object based on information provided during the payment flow.  You can set [`payment_intent_data.setup_future_usage`](https://stripe.com/docs/api/checkout/sessions/create#create_checkout_session-payment_intent_data-setup_future_usage) to have Checkout automatically attach the payment method to the Customer you pass in for future reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Configure whether a Checkout Session creates a [Customer](https://stripe.com/docs/api/customers) during Session confirmation.
    ///
    /// When a Customer is not created, you can still retrieve email, address, and other customer data entered in Checkout
    /// with [customer_details](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-customer_details).
    ///
    /// Sessions that do not create Customers will instead create [Guest Customers](https://support.stripe.com/questions/guest-customer-faq) in the Dashboard.
    ///
    /// Can only be set in `payment` and `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<CheckoutSessionCustomerCreation>,

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
    pub customer_update: Option<Box<CreateCheckoutSessionCustomerUpdate>>,

    /// The coupon or promotion code to apply to this Session.
    ///
    /// Currently, only up to one may be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Box<Vec<CreateCheckoutSessionDiscounts>>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The Epoch time in seconds at which the Checkout Session will expire.
    ///
    /// It can be anywhere from 1 to 24 hours after Checkout Session creation.
    /// By default, this value is 24 hours from creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,

    /// A list of items the customer is purchasing.
    ///
    /// Use this parameter to pass one-time or recurring [Prices](https://stripe.com/docs/api/prices).  For `payment` mode, there is a maximum of 100 line items, however it is recommended to consolidate line items if there are more than a few dozen.  For `subscription` mode, there is a maximum of 20 line items with recurring Prices and 20 line items with one-time Prices.
    /// Line items with one-time Prices in will be on the initial invoice only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Box<Vec<CreateCheckoutSessionLineItems>>>,

    /// The IETF language tag of the locale Checkout is displayed in.
    ///
    /// If blank or `auto`, the browser's locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<CheckoutSessionLocale>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The mode of the Checkout Session.
    ///
    /// Required when using prices or `setup` mode.
    /// Pass `subscription` if the Checkout Session includes at least one recurring item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CheckoutSessionMode>,

    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<Box<CreateCheckoutSessionPaymentIntentData>>,

    /// Payment-method-specific configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<Box<CreateCheckoutSessionPaymentMethodOptions>>,

    /// A list of the types of payment methods (e.g., `card`) this Checkout Session can accept.
    ///
    /// Read more about the supported payment methods and their requirements in our [payment
    /// method details guide](/docs/payments/checkout/payment-methods).
    ///
    /// If multiple payment methods are passed, Checkout will dynamically reorder them to
    /// prioritize the most relevant payment methods based on the customer's location and
    /// other characteristics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Box<Vec<CreateCheckoutSessionPaymentMethodTypes>>>,

    /// Controls phone number collection settings for the session.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts
    /// before using this feature.
    ///
    /// Learn more about [collecting phone numbers with Checkout](https://stripe.com/docs/payments/checkout/phone-numbers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<Box<CreateCheckoutSessionPhoneNumberCollection>>,

    /// A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent_data: Option<Box<CreateCheckoutSessionSetupIntentData>>,

    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<Box<CreateCheckoutSessionShippingAddressCollection>>,

    /// The shipping rate options to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Box<Vec<CreateCheckoutSessionShippingOptions>>>,

    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button.
    ///
    /// `submit_type` can only be specified on Checkout Sessions in `payment` mode, but not Checkout Sessions in `subscription` or `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<CheckoutSessionSubmitType>,

    /// A subset of parameters to be passed to subscription creation for Checkout Sessions in `subscription` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<Box<CreateCheckoutSessionSubscriptionData>>,

    /// The URL to which Stripe should send customers when payment or setup
    /// is complete.
    /// If you’d like to use information from the successful Checkout Session on your page,
    /// read the guide on [customizing your success page](https://stripe.com/docs/payments/checkout/custom-success-page).
    pub success_url: &'a str,

    /// Controls tax ID collection settings for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<Box<CreateCheckoutSessionTaxIdCollection>>,
}

impl<'a> CreateCheckoutSession<'a> {
    pub fn new(cancel_url: &'a str, success_url: &'a str) -> Self {
        CreateCheckoutSession {
            after_expiration: Default::default(),
            allow_promotion_codes: Default::default(),
            automatic_tax: Default::default(),
            billing_address_collection: Default::default(),
            cancel_url,
            client_reference_id: Default::default(),
            consent_collection: Default::default(),
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
            payment_method_options: Default::default(),
            payment_method_types: Default::default(),
            phone_number_collection: Default::default(),
            setup_intent_data: Default::default(),
            shipping_address_collection: Default::default(),
            shipping_options: Default::default(),
            submit_type: Default::default(),
            subscription_data: Default::default(),
            success_url,
            tax_id_collection: Default::default(),
        }
    }
}

// written at 597
/// The parameters for `CheckoutSession::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListCheckoutSessions<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<CheckoutSessionId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return the Checkout Session for the PaymentIntent specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<PaymentIntentId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<CheckoutSessionId>,

    /// Only return the Checkout Session for the subscription specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionId>,
}

impl<'a> ListCheckoutSessions<'a> {
    pub fn new() -> Self {
        ListCheckoutSessions {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            payment_intent: Default::default(),
            starting_after: Default::default(),
            subscription: Default::default(),
        }
    }
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionAfterExpiration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery: Option<Box<CreateCheckoutSessionAfterExpirationRecovery>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionAutomaticTax {
    pub enabled: bool,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionConsentCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<Box<CreateCheckoutSessionConsentCollectionPromotions>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomerUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<CreateCheckoutSessionCustomerUpdateAddress>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<CreateCheckoutSessionCustomerUpdateName>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Box<CreateCheckoutSessionCustomerUpdateShipping>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionDiscounts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<Box<CreateCheckoutSessionLineItemsAdjustableQuantity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_tax_rates: Option<Box<Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<Box<CreateCheckoutSessionLineItemsPriceData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Box<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<Box<CreateCheckoutSessionPaymentIntentDataCaptureMethod>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<Box<CreateCheckoutSessionPaymentIntentDataSetupFutureUsage>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Box<CreateCheckoutSessionPaymentIntentDataShipping>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<Box<CreateCheckoutSessionPaymentIntentDataTransferData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<Box<CreateCheckoutSessionPaymentMethodOptionsAcssDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<Box<CreateCheckoutSessionPaymentMethodOptionsBoleto>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<Box<CreateCheckoutSessionPaymentMethodOptionsOxxo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<Box<CreateCheckoutSessionPaymentMethodOptionsWechatPay>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPhoneNumberCollection {
    pub enabled: bool,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSetupIntentData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingAddressCollection {
    pub allowed_countries: Vec<CreateCheckoutSessionShippingAddressCollectionAllowedCountries>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<Box<CreateCheckoutSessionShippingOptionsShippingRateData>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<Box<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Box<Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Vec<CreateCheckoutSessionSubscriptionDataItems>>>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<Box<CreateCheckoutSessionSubscriptionDataTransferData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Box<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<Box<u32>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionTaxIdCollection {
    pub enabled: bool,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionAfterExpirationRecovery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<Box<bool>>,

    pub enabled: bool,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsAdjustableQuantity {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<Box<i64>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceData {
    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<Box<CreateCheckoutSessionLineItemsPriceDataProductData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Box<CreateCheckoutSessionLineItemsPriceDataRecurring>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<Box<CreateCheckoutSessionLineItemsPriceDataTaxBehavior>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataShipping {
    pub address: CreateCheckoutSessionPaymentIntentDataShippingAddress,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<Box<String>>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<i64>>,

    pub destination: String,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<Box<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<Box<CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBoleto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<Box<u32>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsOxxo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<Box<u32>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsWechatPay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<Box<String>>,

    pub client: CreateCheckoutSessionPaymentMethodOptionsWechatPayClient,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate:
        Option<Box<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate>>,

    pub display_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<Box<CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount>>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<Box<CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<Box<String>>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Box<CreateCheckoutSessionShippingOptionsShippingRateDataType>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataItems {
    pub plan: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Box<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<Box<f64>>,

    pub destination: String,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceDataProductData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Box<Vec<String>>>,

    #[serde(default)]
    pub metadata: Metadata,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceDataRecurring {
    pub interval: CreateCheckoutSessionLineItemsPriceDataRecurringInterval,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<Box<u64>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataShippingAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    pub line1: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for: Option<
        Box<Vec<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor>>,
    >,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<
        Box<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    >,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        Box<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
    >,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum:
        Option<Box<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum:
        Option<Box<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount {
    pub amount: i64,

    pub currency: Currency,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum {
    pub unit: CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit,

    pub value: i64,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum {
    pub unit: CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit,

    pub value: i64,
}

/// An enum representing the possible values of an `CreateCheckoutSession`'s `billing_address_collection` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionBillingAddressCollection {
    Auto,
    Required,
}

impl CheckoutSessionBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionBillingAddressCollection::Auto => "auto",
            CheckoutSessionBillingAddressCollection::Required => "required",
        }
    }
}

impl AsRef<str> for CheckoutSessionBillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSession`'s `customer_creation` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionCustomerCreation {
    Always,
    IfRequired,
}

impl CheckoutSessionCustomerCreation {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionCustomerCreation::Always => "always",
            CheckoutSessionCustomerCreation::IfRequired => "if_required",
        }
    }
}

impl AsRef<str> for CheckoutSessionCustomerCreation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSession`'s `locale` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    #[serde(rename = "en-GB")]
    EnGb,
    Es,
    #[serde(rename = "es-419")]
    Es419,
    Et,
    Fi,
    Fil,
    Fr,
    #[serde(rename = "fr-CA")]
    FrCa,
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
    PtBr,
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
    ZhHk,
    #[serde(rename = "zh-TW")]
    ZhTw,
}

impl CheckoutSessionLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionLocale::Auto => "auto",
            CheckoutSessionLocale::Bg => "bg",
            CheckoutSessionLocale::Cs => "cs",
            CheckoutSessionLocale::Da => "da",
            CheckoutSessionLocale::De => "de",
            CheckoutSessionLocale::El => "el",
            CheckoutSessionLocale::En => "en",
            CheckoutSessionLocale::EnGb => "en-GB",
            CheckoutSessionLocale::Es => "es",
            CheckoutSessionLocale::Es419 => "es-419",
            CheckoutSessionLocale::Et => "et",
            CheckoutSessionLocale::Fi => "fi",
            CheckoutSessionLocale::Fil => "fil",
            CheckoutSessionLocale::Fr => "fr",
            CheckoutSessionLocale::FrCa => "fr-CA",
            CheckoutSessionLocale::Hr => "hr",
            CheckoutSessionLocale::Hu => "hu",
            CheckoutSessionLocale::Id => "id",
            CheckoutSessionLocale::It => "it",
            CheckoutSessionLocale::Ja => "ja",
            CheckoutSessionLocale::Ko => "ko",
            CheckoutSessionLocale::Lt => "lt",
            CheckoutSessionLocale::Lv => "lv",
            CheckoutSessionLocale::Ms => "ms",
            CheckoutSessionLocale::Mt => "mt",
            CheckoutSessionLocale::Nb => "nb",
            CheckoutSessionLocale::Nl => "nl",
            CheckoutSessionLocale::Pl => "pl",
            CheckoutSessionLocale::Pt => "pt",
            CheckoutSessionLocale::PtBr => "pt-BR",
            CheckoutSessionLocale::Ro => "ro",
            CheckoutSessionLocale::Ru => "ru",
            CheckoutSessionLocale::Sk => "sk",
            CheckoutSessionLocale::Sl => "sl",
            CheckoutSessionLocale::Sv => "sv",
            CheckoutSessionLocale::Th => "th",
            CheckoutSessionLocale::Tr => "tr",
            CheckoutSessionLocale::Vi => "vi",
            CheckoutSessionLocale::Zh => "zh",
            CheckoutSessionLocale::ZhHk => "zh-HK",
            CheckoutSessionLocale::ZhTw => "zh-TW",
        }
    }
}

impl AsRef<str> for CheckoutSessionLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSession`'s `mode` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionMode {
    Payment,
    Setup,
    Subscription,
}

impl CheckoutSessionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionMode::Payment => "payment",
            CheckoutSessionMode::Setup => "setup",
            CheckoutSessionMode::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for CheckoutSessionMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSession`'s `submit_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}

impl CheckoutSessionSubmitType {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionSubmitType::Auto => "auto",
            CheckoutSessionSubmitType::Book => "book",
            CheckoutSessionSubmitType::Donate => "donate",
            CheckoutSessionSubmitType::Pay => "pay",
        }
    }
}

impl AsRef<str> for CheckoutSessionSubmitType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionConsentCollection`'s `promotions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionConsentCollectionPromotions {
    Auto,
}

impl CreateCheckoutSessionConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionConsentCollectionPromotions::Auto => "auto",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionConsentCollectionPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionCustomerUpdate`'s `address` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionCustomerUpdateAddress {
    Auto,
    Never,
}

impl CreateCheckoutSessionCustomerUpdateAddress {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionCustomerUpdateAddress::Auto => "auto",
            CreateCheckoutSessionCustomerUpdateAddress::Never => "never",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionCustomerUpdateAddress {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionCustomerUpdateAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionCustomerUpdate`'s `name` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionCustomerUpdateName {
    Auto,
    Never,
}

impl CreateCheckoutSessionCustomerUpdateName {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionCustomerUpdateName::Auto => "auto",
            CreateCheckoutSessionCustomerUpdateName::Never => "never",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionCustomerUpdateName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionCustomerUpdateName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionCustomerUpdate`'s `shipping` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionCustomerUpdateShipping {
    Auto,
    Never,
}

impl CreateCheckoutSessionCustomerUpdateShipping {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionCustomerUpdateShipping::Auto => "auto",
            CreateCheckoutSessionCustomerUpdateShipping::Never => "never",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionCustomerUpdateShipping {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionCustomerUpdateShipping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionLineItemsPriceDataRecurring`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionLineItemsPriceDataRecurringInterval::Day => "day",
            CreateCheckoutSessionLineItemsPriceDataRecurringInterval::Month => "month",
            CreateCheckoutSessionLineItemsPriceDataRecurringInterval::Week => "week",
            CreateCheckoutSessionLineItemsPriceDataRecurringInterval::Year => "year",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionLineItemsPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionLineItemsPriceDataTaxBehavior::Exclusive => "exclusive",
            CreateCheckoutSessionLineItemsPriceDataTaxBehavior::Inclusive => "inclusive",
            CreateCheckoutSessionLineItemsPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentIntentData`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    Automatic,
    Manual,
}

impl CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentIntentDataCaptureMethod::Automatic => "automatic",
            CreateCheckoutSessionPaymentIntentDataCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentIntentData`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}

impl CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentIntentDataSetupFutureUsage::OffSession => "off_session",
            CreateCheckoutSessionPaymentIntentDataSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions`'s `default_for` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}

impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::Invoice => "invoice",
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions`'s `payment_schedule` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Combined => "combined",
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Interval => "interval",
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => {
                "automatic"
            }
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod::Instant => {
                "instant"
            }
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => {
                "microdeposits"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsWechatPay`'s `client` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsWechatPayClient::Android => "android",
            CreateCheckoutSessionPaymentMethodOptionsWechatPayClient::Ios => "ios",
            CreateCheckoutSessionPaymentMethodOptionsWechatPayClient::Web => "web",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSession`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodTypes {
    AcssDebit,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Oxxo,
    P24,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl CreateCheckoutSessionPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodTypes::AcssDebit => "acss_debit",
            CreateCheckoutSessionPaymentMethodTypes::AfterpayClearpay => "afterpay_clearpay",
            CreateCheckoutSessionPaymentMethodTypes::Alipay => "alipay",
            CreateCheckoutSessionPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            CreateCheckoutSessionPaymentMethodTypes::BacsDebit => "bacs_debit",
            CreateCheckoutSessionPaymentMethodTypes::Bancontact => "bancontact",
            CreateCheckoutSessionPaymentMethodTypes::Boleto => "boleto",
            CreateCheckoutSessionPaymentMethodTypes::Card => "card",
            CreateCheckoutSessionPaymentMethodTypes::Eps => "eps",
            CreateCheckoutSessionPaymentMethodTypes::Fpx => "fpx",
            CreateCheckoutSessionPaymentMethodTypes::Giropay => "giropay",
            CreateCheckoutSessionPaymentMethodTypes::Grabpay => "grabpay",
            CreateCheckoutSessionPaymentMethodTypes::Ideal => "ideal",
            CreateCheckoutSessionPaymentMethodTypes::Klarna => "klarna",
            CreateCheckoutSessionPaymentMethodTypes::Oxxo => "oxxo",
            CreateCheckoutSessionPaymentMethodTypes::P24 => "p24",
            CreateCheckoutSessionPaymentMethodTypes::SepaDebit => "sepa_debit",
            CreateCheckoutSessionPaymentMethodTypes::Sofort => "sofort",
            CreateCheckoutSessionPaymentMethodTypes::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionShippingAddressCollection`'s `allowed_countries` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
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

impl CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ac => "AC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ad => "AD",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ae => "AE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Af => "AF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ag => "AG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ai => "AI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Al => "AL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Am => "AM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ao => "AO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Aq => "AQ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ar => "AR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::At => "AT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Au => "AU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Aw => "AW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ax => "AX",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Az => "AZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ba => "BA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bb => "BB",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bd => "BD",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Be => "BE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bf => "BF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bg => "BG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bh => "BH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bi => "BI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bj => "BJ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bl => "BL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bm => "BM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bn => "BN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bo => "BO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bq => "BQ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Br => "BR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bs => "BS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bt => "BT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bv => "BV",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bw => "BW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::By => "BY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bz => "BZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ca => "CA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cd => "CD",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cf => "CF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cg => "CG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ch => "CH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ci => "CI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ck => "CK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cl => "CL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cm => "CM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cn => "CN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Co => "CO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cr => "CR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cv => "CV",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cw => "CW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cy => "CY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cz => "CZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::De => "DE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Dj => "DJ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Dk => "DK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Dm => "DM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Do => "DO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Dz => "DZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ec => "EC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ee => "EE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Eg => "EG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Eh => "EH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Er => "ER",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Es => "ES",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Et => "ET",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Fi => "FI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Fj => "FJ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Fk => "FK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Fo => "FO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Fr => "FR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ga => "GA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gb => "GB",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gd => "GD",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ge => "GE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gf => "GF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gg => "GG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gh => "GH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gi => "GI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gl => "GL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gm => "GM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gn => "GN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gp => "GP",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gq => "GQ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gr => "GR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gs => "GS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gt => "GT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gu => "GU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gw => "GW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gy => "GY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Hk => "HK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Hn => "HN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Hr => "HR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ht => "HT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Hu => "HU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Id => "ID",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ie => "IE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Il => "IL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Im => "IM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::In => "IN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Io => "IO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Iq => "IQ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Is => "IS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::It => "IT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Je => "JE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Jm => "JM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Jo => "JO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Jp => "JP",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ke => "KE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Kg => "KG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Kh => "KH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ki => "KI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Km => "KM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Kn => "KN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Kr => "KR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Kw => "KW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ky => "KY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Kz => "KZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::La => "LA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lb => "LB",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lc => "LC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Li => "LI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lk => "LK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lr => "LR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ls => "LS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lt => "LT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lu => "LU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lv => "LV",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ly => "LY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ma => "MA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mc => "MC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Md => "MD",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Me => "ME",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mf => "MF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mg => "MG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mk => "MK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ml => "ML",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mm => "MM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mn => "MN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mo => "MO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mq => "MQ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mr => "MR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ms => "MS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mt => "MT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mu => "MU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mv => "MV",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mw => "MW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mx => "MX",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::My => "MY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mz => "MZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Na => "NA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Nc => "NC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ne => "NE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ng => "NG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ni => "NI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Nl => "NL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::No => "NO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Np => "NP",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Nr => "NR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Nu => "NU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Nz => "NZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Om => "OM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pa => "PA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pe => "PE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pf => "PF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pg => "PG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ph => "PH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pk => "PK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pl => "PL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pm => "PM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pn => "PN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pr => "PR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ps => "PS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pt => "PT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Py => "PY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Qa => "QA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Re => "RE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ro => "RO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Rs => "RS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ru => "RU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Rw => "RW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sa => "SA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sb => "SB",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sc => "SC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Se => "SE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sg => "SG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sh => "SH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Si => "SI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sj => "SJ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sk => "SK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sl => "SL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sm => "SM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sn => "SN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::So => "SO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sr => "SR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ss => "SS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::St => "ST",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sv => "SV",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sx => "SX",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sz => "SZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ta => "TA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tc => "TC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Td => "TD",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tf => "TF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tg => "TG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Th => "TH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tj => "TJ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tk => "TK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tl => "TL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tm => "TM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tn => "TN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::To => "TO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tr => "TR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tt => "TT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tv => "TV",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tw => "TW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tz => "TZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ua => "UA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ug => "UG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Us => "US",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Uy => "UY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Uz => "UZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Va => "VA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Vc => "VC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ve => "VE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Vg => "VG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Vn => "VN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Vu => "VU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Wf => "WF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ws => "WS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Xk => "XK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ye => "YE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Yt => "YT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Za => "ZA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Zm => "ZM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Zw => "ZW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::BusinessDay => "business_day",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::Day => "day",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::Hour => "hour",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::Month => "month",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::Week => "week",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::BusinessDay => "business_day",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::Day => "day",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::Hour => "hour",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::Month => "month",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::Week => "week",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionShippingOptionsShippingRateData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior::Exclusive => {
                "exclusive"
            }
            CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior::Inclusive => {
                "inclusive"
            }
            CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior::Unspecified => {
                "unspecified"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionShippingOptionsShippingRateData`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataType {
    FixedAmount,
}

impl CreateCheckoutSessionShippingOptionsShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionShippingOptionsShippingRateDataType::FixedAmount => "fixed_amount",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionShippingOptionsShippingRateDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionShippingOptionsShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
