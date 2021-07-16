// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{CheckoutSessionId, CustomerId, PaymentIntentId, SubscriptionId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Timestamp};
use crate::resources::{CheckoutSessionItem, Currency, Customer, Discount, PaymentIntent, PaymentMethodOptionsBoleto, PaymentMethodOptionsOxxo, SetupIntent, Shipping, Subscription, TaxRate};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Session".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutSession {
    /// Unique identifier for the object.
    ///
    /// Used to pass to `redirectToCheckout` in Stripe.js.
    pub id: CheckoutSessionId,

    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,

    /// Total of all items before discounts or taxes are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_subtotal: Option<i64>,

    /// Total of all items after discounts and taxes are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_total: Option<i64>,

    pub automatic_tax: PaymentPagesCheckoutSessionAutomaticTax,

    /// Describes whether Checkout should collect the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<CheckoutSessionBillingAddressCollection>,

    /// The URL the customer will be directed to if they decide to cancel payment and return to your website.
    pub cancel_url: String,

    /// A unique string to reference the Checkout Session.
    ///
    /// This can be a customer ID, a cart ID, or similar, and can be used to reconcile the Session with your internal systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The ID of the customer for this Session.
    /// For Checkout Sessions in `payment` or `subscription` mode, Checkout
    /// will create a new customer object based on information provided
    /// during the payment flow unless an existing customer was provided when
    /// the Session was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// The customer details including the customer's tax exempt status and the customer's tax IDs.
    ///
    /// Only present on Sessions in `payment` or `subscription` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<PaymentPagesCheckoutSessionCustomerDetails>,

    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file.
    ///
    /// To access information about the customer once the payment flow is complete, use the `customer` attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,

    /// The line items purchased by the customer.
    #[serde(default)]
    pub line_items: List<CheckoutSessionItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The IETF language tag of the locale Checkout is displayed in.
    ///
    /// If blank or `auto`, the browser's locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<CheckoutSessionLocale>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The mode of the Checkout Session.
    pub mode: CheckoutSessionMode,

    /// The ID of the PaymentIntent for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    /// Payment-method-specific configuration for the PaymentIntent or SetupIntent of this CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CheckoutSessionPaymentMethodOptions>,

    /// A list of the types of payment methods (e.g.
    ///
    /// card) this Checkout Session is allowed to accept.
    pub payment_method_types: Vec<String>,

    /// The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
    /// You can use this value to decide when to fulfill your customer's order.
    pub payment_status: CheckoutSessionPaymentStatus,

    /// The ID of the SetupIntent for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<Expandable<SetupIntent>>,

    /// Shipping information for this Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,

    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<ShippingAddressCollection>,

    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button.
    ///
    /// `submit_type` can only be specified on Checkout Sessions in `payment` mode, but not Checkout Sessions in `subscription` or `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<CheckoutSessionSubmitType>,

    /// The ID of the subscription for Checkout Sessions in `subscription` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Expandable<Subscription>>,

    /// The URL the customer will be directed to after the payment or
    /// subscription creation is successful.
    pub success_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<PaymentPagesCheckoutSessionTaxIdCollection>,

    /// Tax and discount details for the computed total amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_details: Option<PaymentPagesCheckoutSessionTotalDetails>,

    /// The URL to the Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CheckoutSession {

    /// Returns a list of Checkout Sessions.
    pub fn list(client: &Client, params: ListCheckoutSessions<'_>) -> Response<List<CheckoutSession>> {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutSessionPaymentMethodOptions {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CheckoutAcssDebitPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<PaymentMethodOptionsBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PaymentMethodOptionsOxxo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutAcssDebitPaymentMethodOptions {

    /// Currency supported by the bank account.
    ///
    /// Returned when the Session is in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CheckoutAcssDebitMandateOptions>,

    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<CheckoutAcssDebitPaymentMethodOptionsVerificationMethod>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutAcssDebitMandateOptions {

    /// A URL for custom mandate text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    /// Description of the interval.
    ///
    /// Only required if 'payment_schedule' parmeter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<CheckoutAcssDebitMandateOptionsPaymentSchedule>,

    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<CheckoutAcssDebitMandateOptionsTransactionType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionAutomaticTax {

    /// Indicates whether automatic tax is enabled for the session.
    pub enabled: bool,

    /// The status of the most recent automated tax calculation for this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentPagesCheckoutSessionAutomaticTaxStatus>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCustomerDetails {

    /// The customer’s email at time of checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The customer’s tax exempt status at time of checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt>,

    /// The customer’s tax IDs at time of checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<PaymentPagesCheckoutSessionTaxId>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionTaxId {

    /// The type of the tax ID, one of `eu_vat`, `br_cnpj`, `br_cpf`, `gb_vat`, `nz_gst`, `au_abn`, `in_gst`, `no_vat`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ru_kpp`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `jp_rn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `my_sst`, `sg_gst`, `ae_trn`, `cl_tin`, `sa_vat`, `id_npwp`, `my_frp`, `il_vat`, or `unknown`.
    #[serde(rename = "type")]
    pub type_: PaymentPagesCheckoutSessionTaxIdType,

    /// The value of the tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionTaxIdCollection {

    /// Indicates whether tax ID collection is enabled for the session.
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionTotalDetails {

    /// This is the sum of all the line item discounts.
    pub amount_discount: i64,

    /// This is the sum of all the line item shipping amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_shipping: Option<i64>,

    /// This is the sum of all the line item tax amounts.
    pub amount_tax: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {

    /// The aggregated line item discounts.
    pub discounts: Vec<LineItemsDiscountAmount>,

    /// The aggregated line item tax amounts by rate.
    pub taxes: Vec<LineItemsTaxAmount>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LineItemsDiscountAmount {

    /// The amount discounted.
    pub amount: i64,

    pub discount: Discount,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LineItemsTaxAmount {

    /// Amount of tax applied for this rate.
    pub amount: i64,

    pub rate: TaxRate,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShippingAddressCollection {

    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
    /// shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: Vec<ShippingAddressCollectionAllowedCountries>,
}

/// The parameters for `CheckoutSession::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateCheckoutSession<'a> {

    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateCheckoutSessionAutomaticTax>,

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

    /// ID of an existing Customer, if one exists.
    ///
    /// In `payment` mode, the customer’s most recent card payment method will be used to prefill the email, name, card details, and billing address on the Checkout page.
    /// In `subscription` mode, the customer’s [default payment method](https://stripe.com/docs/api/customers/update#update_customer-invoice_settings-default_payment_method) will be used if it’s a card, and otherwise the most recent card will be used.
    /// A valid billing address is required for Checkout to prefill the customer's card details.  If the customer changes their email on the Checkout page, the Customer object will be updated with the new email.  If blank for Checkout Sessions in `payment` or `subscription` mode, Checkout will create a new Customer object based on information provided during the payment flow.  You can set [`payment_intent_data.setup_future_usage`](https://stripe.com/docs/api/checkout/sessions/create#create_checkout_session-payment_intent_data-setup_future_usage) to have Checkout automatically attach the payment method to the Customer you pass in for future reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

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
    pub customer_update: Option<CreateCheckoutSessionCustomerUpdate>,

    /// The coupon or promotion code to apply to this Session.
    ///
    /// Currently, only up to one may be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<CreateCheckoutSessionDiscounts>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A list of items the customer is purchasing.
    ///
    /// Use this parameter to pass one-time or recurring [Prices](https://stripe.com/docs/api/prices).  For `payment` mode, there is a maximum of 100 line items, however it is recommended to consolidate line items if there are more than a few dozen.  For `subscription` mode, there is a maximum of 20 line items with recurring Prices and 20 line items with one-time Prices.
    /// Line items with one-time Prices in will be on the initial invoice only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreateCheckoutSessionLineItems>>,

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
    pub payment_intent_data: Option<CreateCheckoutSessionPaymentIntentData>,

    /// Payment-method-specific configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateCheckoutSessionPaymentMethodOptions>,

    /// A list of the types of payment methods (e.g., `card`) this Checkout Session can accept.
    ///
    /// Read more about the supported payment methods and their requirements in our [payment
    /// method details guide](/docs/payments/checkout/payment-methods).
    ///
    /// If multiple payment methods are passed, Checkout will dynamically reorder them to
    /// prioritize the most relevant payment methods based on the customer's location and
    /// other characteristics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<CreateCheckoutSessionPaymentMethodTypes>>,

    /// A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent_data: Option<CreateCheckoutSessionSetupIntentData>,

    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<CreateCheckoutSessionShippingAddressCollection>,

    /// The shipping rate to apply to this Session.
    ///
    /// Currently, only up to one may be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rates: Option<Vec<String>>,

    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button.
    ///
    /// `submit_type` can only be specified on Checkout Sessions in `payment` mode, but not Checkout Sessions in `subscription` or `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<CheckoutSessionSubmitType>,

    /// A subset of parameters to be passed to subscription creation for Checkout Sessions in `subscription` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<CreateCheckoutSessionSubscriptionData>,

    /// The URL to which Stripe should send customers when payment or setup
    /// is complete.
    /// If you’d like access to the Checkout Session for the successful
    /// payment, read more about it in the guide on [fulfilling orders](https://stripe.com/docs/payments/checkout/fulfill-orders).
    pub success_url: &'a str,

    /// Controls tax ID collection settings for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<CreateCheckoutSessionTaxIdCollection>,
}

impl<'a> CreateCheckoutSession<'a> {
    pub fn new(cancel_url: &'a str, success_url: &'a str) -> Self {
        CreateCheckoutSession {
            allow_promotion_codes: Default::default(),
            automatic_tax: Default::default(),
            billing_address_collection: Default::default(),
            cancel_url,
            client_reference_id: Default::default(),
            customer: Default::default(),
            customer_email: Default::default(),
            customer_update: Default::default(),
            discounts: Default::default(),
            expand: Default::default(),
            line_items: Default::default(),
            locale: Default::default(),
            metadata: Default::default(),
            mode: Default::default(),
            payment_intent_data: Default::default(),
            payment_method_options: Default::default(),
            payment_method_types: Default::default(),
            setup_intent_data: Default::default(),
            shipping_address_collection: Default::default(),
            shipping_rates: Default::default(),
            submit_type: Default::default(),
            subscription_data: Default::default(),
            success_url,
            tax_id_collection: Default::default(),
        }
    }
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionAutomaticTax {

    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomerUpdate {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateCheckoutSessionCustomerUpdateAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CreateCheckoutSessionCustomerUpdateName>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateCheckoutSessionCustomerUpdateShipping>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionDiscounts {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItems {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreateCheckoutSessionLineItemsAdjustableQuantity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_tax_rates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateCheckoutSessionLineItemsPriceData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentData {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentIntentDataCaptureMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentIntentDataSetupFutureUsage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateCheckoutSessionPaymentIntentDataShipping>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateCheckoutSessionPaymentIntentDataTransferData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptions {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreateCheckoutSessionPaymentMethodOptionsBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreateCheckoutSessionPaymentMethodOptionsOxxo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreateCheckoutSessionPaymentMethodOptionsWechatPay>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSetupIntentData {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingAddressCollection {

    pub allowed_countries: Vec<CreateCheckoutSessionShippingAddressCollectionAllowedCountries>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionData {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreateCheckoutSessionSubscriptionDataItems>>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateCheckoutSessionSubscriptionDataTransferData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionTaxIdCollection {

    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsAdjustableQuantity {

    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceData {

    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<CreateCheckoutSessionLineItemsPriceDataProductData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreateCheckoutSessionLineItemsPriceDataRecurring>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateCheckoutSessionLineItemsPriceDataTaxBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataShipping {

    pub address: CreateCheckoutSessionPaymentIntentDataShippingAddress,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataTransferData {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    pub destination: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebit {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBoleto {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsOxxo {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsWechatPay {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,

    pub client: CreateCheckoutSessionPaymentMethodOptionsWechatPayClient,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataItems {

    pub plan: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTransferData {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    pub destination: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceDataProductData {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    #[serde(default)]
    pub metadata: Metadata,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceDataRecurring {

    pub interval: CreateCheckoutSessionLineItemsPriceDataRecurringInterval,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataShippingAddress {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    pub line1: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

/// An enum representing the possible values of an `CheckoutAcssDebitMandateOptions`'s `payment_schedule` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl CheckoutAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutAcssDebitMandateOptionsPaymentSchedule::Combined => "combined",
            CheckoutAcssDebitMandateOptionsPaymentSchedule::Interval => "interval",
            CheckoutAcssDebitMandateOptionsPaymentSchedule::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CheckoutAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl CheckoutAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutAcssDebitMandateOptionsTransactionType::Business => "business",
            CheckoutAcssDebitMandateOptionsTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str> for CheckoutAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutAcssDebitMandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CheckoutAcssDebitPaymentMethodOptions`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Automatic => "automatic",
            CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Instant => "instant",
            CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CheckoutSession`'s `billing_address_collection` field.
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

/// An enum representing the possible values of an `CheckoutSession`'s `locale` field.
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
    Fr,
    #[serde(rename = "fr-CA")]
    FrCa,
    Hu,
    Id,
    It,
    Ja,
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
            CheckoutSessionLocale::Fr => "fr",
            CheckoutSessionLocale::FrCa => "fr-CA",
            CheckoutSessionLocale::Hu => "hu",
            CheckoutSessionLocale::Id => "id",
            CheckoutSessionLocale::It => "it",
            CheckoutSessionLocale::Ja => "ja",
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

/// An enum representing the possible values of an `CheckoutSession`'s `mode` field.
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

/// An enum representing the possible values of an `CheckoutSession`'s `payment_status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionPaymentStatus {
    NoPaymentRequired,
    Paid,
    Unpaid,
}

impl CheckoutSessionPaymentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionPaymentStatus::NoPaymentRequired => "no_payment_required",
            CheckoutSessionPaymentStatus::Paid => "paid",
            CheckoutSessionPaymentStatus::Unpaid => "unpaid",
        }
    }
}

impl AsRef<str> for CheckoutSessionPaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CheckoutSession`'s `submit_type` field.
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

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
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

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
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
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
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
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
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
            CreateCheckoutSessionPaymentMethodTypes::BacsDebit => "bacs_debit",
            CreateCheckoutSessionPaymentMethodTypes::Bancontact => "bancontact",
            CreateCheckoutSessionPaymentMethodTypes::Boleto => "boleto",
            CreateCheckoutSessionPaymentMethodTypes::Card => "card",
            CreateCheckoutSessionPaymentMethodTypes::Eps => "eps",
            CreateCheckoutSessionPaymentMethodTypes::Fpx => "fpx",
            CreateCheckoutSessionPaymentMethodTypes::Giropay => "giropay",
            CreateCheckoutSessionPaymentMethodTypes::Grabpay => "grabpay",
            CreateCheckoutSessionPaymentMethodTypes::Ideal => "ideal",
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

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionAutomaticTax`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

impl PaymentPagesCheckoutSessionAutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionAutomaticTaxStatus::Complete => "complete",
            PaymentPagesCheckoutSessionAutomaticTaxStatus::Failed => "failed",
            PaymentPagesCheckoutSessionAutomaticTaxStatus::RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionCustomerDetails`'s `tax_exempt` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::Exempt => "exempt",
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::None => "none",
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionTaxId`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionTaxIdType {
    AeTrn,
    AuAbn,
    BrCnpj,
    BrCpf,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    ChVat,
    ClTin,
    EsCif,
    EuVat,
    GbVat,
    HkBr,
    IdNpwp,
    IlVat,
    InGst,
    JpCn,
    JpRn,
    KrBrn,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    ThVat,
    TwVat,
    Unknown,
    UsEin,
    ZaVat,
}

impl PaymentPagesCheckoutSessionTaxIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionTaxIdType::AeTrn => "ae_trn",
            PaymentPagesCheckoutSessionTaxIdType::AuAbn => "au_abn",
            PaymentPagesCheckoutSessionTaxIdType::BrCnpj => "br_cnpj",
            PaymentPagesCheckoutSessionTaxIdType::BrCpf => "br_cpf",
            PaymentPagesCheckoutSessionTaxIdType::CaBn => "ca_bn",
            PaymentPagesCheckoutSessionTaxIdType::CaGstHst => "ca_gst_hst",
            PaymentPagesCheckoutSessionTaxIdType::CaPstBc => "ca_pst_bc",
            PaymentPagesCheckoutSessionTaxIdType::CaPstMb => "ca_pst_mb",
            PaymentPagesCheckoutSessionTaxIdType::CaPstSk => "ca_pst_sk",
            PaymentPagesCheckoutSessionTaxIdType::CaQst => "ca_qst",
            PaymentPagesCheckoutSessionTaxIdType::ChVat => "ch_vat",
            PaymentPagesCheckoutSessionTaxIdType::ClTin => "cl_tin",
            PaymentPagesCheckoutSessionTaxIdType::EsCif => "es_cif",
            PaymentPagesCheckoutSessionTaxIdType::EuVat => "eu_vat",
            PaymentPagesCheckoutSessionTaxIdType::GbVat => "gb_vat",
            PaymentPagesCheckoutSessionTaxIdType::HkBr => "hk_br",
            PaymentPagesCheckoutSessionTaxIdType::IdNpwp => "id_npwp",
            PaymentPagesCheckoutSessionTaxIdType::IlVat => "il_vat",
            PaymentPagesCheckoutSessionTaxIdType::InGst => "in_gst",
            PaymentPagesCheckoutSessionTaxIdType::JpCn => "jp_cn",
            PaymentPagesCheckoutSessionTaxIdType::JpRn => "jp_rn",
            PaymentPagesCheckoutSessionTaxIdType::KrBrn => "kr_brn",
            PaymentPagesCheckoutSessionTaxIdType::LiUid => "li_uid",
            PaymentPagesCheckoutSessionTaxIdType::MxRfc => "mx_rfc",
            PaymentPagesCheckoutSessionTaxIdType::MyFrp => "my_frp",
            PaymentPagesCheckoutSessionTaxIdType::MyItn => "my_itn",
            PaymentPagesCheckoutSessionTaxIdType::MySst => "my_sst",
            PaymentPagesCheckoutSessionTaxIdType::NoVat => "no_vat",
            PaymentPagesCheckoutSessionTaxIdType::NzGst => "nz_gst",
            PaymentPagesCheckoutSessionTaxIdType::RuInn => "ru_inn",
            PaymentPagesCheckoutSessionTaxIdType::RuKpp => "ru_kpp",
            PaymentPagesCheckoutSessionTaxIdType::SaVat => "sa_vat",
            PaymentPagesCheckoutSessionTaxIdType::SgGst => "sg_gst",
            PaymentPagesCheckoutSessionTaxIdType::SgUen => "sg_uen",
            PaymentPagesCheckoutSessionTaxIdType::ThVat => "th_vat",
            PaymentPagesCheckoutSessionTaxIdType::TwVat => "tw_vat",
            PaymentPagesCheckoutSessionTaxIdType::Unknown => "unknown",
            PaymentPagesCheckoutSessionTaxIdType::UsEin => "us_ein",
            PaymentPagesCheckoutSessionTaxIdType::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionTaxIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionTaxIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `ShippingAddressCollection`'s `allowed_countries` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShippingAddressCollectionAllowedCountries {
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

impl ShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            ShippingAddressCollectionAllowedCountries::Ac => "AC",
            ShippingAddressCollectionAllowedCountries::Ad => "AD",
            ShippingAddressCollectionAllowedCountries::Ae => "AE",
            ShippingAddressCollectionAllowedCountries::Af => "AF",
            ShippingAddressCollectionAllowedCountries::Ag => "AG",
            ShippingAddressCollectionAllowedCountries::Ai => "AI",
            ShippingAddressCollectionAllowedCountries::Al => "AL",
            ShippingAddressCollectionAllowedCountries::Am => "AM",
            ShippingAddressCollectionAllowedCountries::Ao => "AO",
            ShippingAddressCollectionAllowedCountries::Aq => "AQ",
            ShippingAddressCollectionAllowedCountries::Ar => "AR",
            ShippingAddressCollectionAllowedCountries::At => "AT",
            ShippingAddressCollectionAllowedCountries::Au => "AU",
            ShippingAddressCollectionAllowedCountries::Aw => "AW",
            ShippingAddressCollectionAllowedCountries::Ax => "AX",
            ShippingAddressCollectionAllowedCountries::Az => "AZ",
            ShippingAddressCollectionAllowedCountries::Ba => "BA",
            ShippingAddressCollectionAllowedCountries::Bb => "BB",
            ShippingAddressCollectionAllowedCountries::Bd => "BD",
            ShippingAddressCollectionAllowedCountries::Be => "BE",
            ShippingAddressCollectionAllowedCountries::Bf => "BF",
            ShippingAddressCollectionAllowedCountries::Bg => "BG",
            ShippingAddressCollectionAllowedCountries::Bh => "BH",
            ShippingAddressCollectionAllowedCountries::Bi => "BI",
            ShippingAddressCollectionAllowedCountries::Bj => "BJ",
            ShippingAddressCollectionAllowedCountries::Bl => "BL",
            ShippingAddressCollectionAllowedCountries::Bm => "BM",
            ShippingAddressCollectionAllowedCountries::Bn => "BN",
            ShippingAddressCollectionAllowedCountries::Bo => "BO",
            ShippingAddressCollectionAllowedCountries::Bq => "BQ",
            ShippingAddressCollectionAllowedCountries::Br => "BR",
            ShippingAddressCollectionAllowedCountries::Bs => "BS",
            ShippingAddressCollectionAllowedCountries::Bt => "BT",
            ShippingAddressCollectionAllowedCountries::Bv => "BV",
            ShippingAddressCollectionAllowedCountries::Bw => "BW",
            ShippingAddressCollectionAllowedCountries::By => "BY",
            ShippingAddressCollectionAllowedCountries::Bz => "BZ",
            ShippingAddressCollectionAllowedCountries::Ca => "CA",
            ShippingAddressCollectionAllowedCountries::Cd => "CD",
            ShippingAddressCollectionAllowedCountries::Cf => "CF",
            ShippingAddressCollectionAllowedCountries::Cg => "CG",
            ShippingAddressCollectionAllowedCountries::Ch => "CH",
            ShippingAddressCollectionAllowedCountries::Ci => "CI",
            ShippingAddressCollectionAllowedCountries::Ck => "CK",
            ShippingAddressCollectionAllowedCountries::Cl => "CL",
            ShippingAddressCollectionAllowedCountries::Cm => "CM",
            ShippingAddressCollectionAllowedCountries::Cn => "CN",
            ShippingAddressCollectionAllowedCountries::Co => "CO",
            ShippingAddressCollectionAllowedCountries::Cr => "CR",
            ShippingAddressCollectionAllowedCountries::Cv => "CV",
            ShippingAddressCollectionAllowedCountries::Cw => "CW",
            ShippingAddressCollectionAllowedCountries::Cy => "CY",
            ShippingAddressCollectionAllowedCountries::Cz => "CZ",
            ShippingAddressCollectionAllowedCountries::De => "DE",
            ShippingAddressCollectionAllowedCountries::Dj => "DJ",
            ShippingAddressCollectionAllowedCountries::Dk => "DK",
            ShippingAddressCollectionAllowedCountries::Dm => "DM",
            ShippingAddressCollectionAllowedCountries::Do => "DO",
            ShippingAddressCollectionAllowedCountries::Dz => "DZ",
            ShippingAddressCollectionAllowedCountries::Ec => "EC",
            ShippingAddressCollectionAllowedCountries::Ee => "EE",
            ShippingAddressCollectionAllowedCountries::Eg => "EG",
            ShippingAddressCollectionAllowedCountries::Eh => "EH",
            ShippingAddressCollectionAllowedCountries::Er => "ER",
            ShippingAddressCollectionAllowedCountries::Es => "ES",
            ShippingAddressCollectionAllowedCountries::Et => "ET",
            ShippingAddressCollectionAllowedCountries::Fi => "FI",
            ShippingAddressCollectionAllowedCountries::Fj => "FJ",
            ShippingAddressCollectionAllowedCountries::Fk => "FK",
            ShippingAddressCollectionAllowedCountries::Fo => "FO",
            ShippingAddressCollectionAllowedCountries::Fr => "FR",
            ShippingAddressCollectionAllowedCountries::Ga => "GA",
            ShippingAddressCollectionAllowedCountries::Gb => "GB",
            ShippingAddressCollectionAllowedCountries::Gd => "GD",
            ShippingAddressCollectionAllowedCountries::Ge => "GE",
            ShippingAddressCollectionAllowedCountries::Gf => "GF",
            ShippingAddressCollectionAllowedCountries::Gg => "GG",
            ShippingAddressCollectionAllowedCountries::Gh => "GH",
            ShippingAddressCollectionAllowedCountries::Gi => "GI",
            ShippingAddressCollectionAllowedCountries::Gl => "GL",
            ShippingAddressCollectionAllowedCountries::Gm => "GM",
            ShippingAddressCollectionAllowedCountries::Gn => "GN",
            ShippingAddressCollectionAllowedCountries::Gp => "GP",
            ShippingAddressCollectionAllowedCountries::Gq => "GQ",
            ShippingAddressCollectionAllowedCountries::Gr => "GR",
            ShippingAddressCollectionAllowedCountries::Gs => "GS",
            ShippingAddressCollectionAllowedCountries::Gt => "GT",
            ShippingAddressCollectionAllowedCountries::Gu => "GU",
            ShippingAddressCollectionAllowedCountries::Gw => "GW",
            ShippingAddressCollectionAllowedCountries::Gy => "GY",
            ShippingAddressCollectionAllowedCountries::Hk => "HK",
            ShippingAddressCollectionAllowedCountries::Hn => "HN",
            ShippingAddressCollectionAllowedCountries::Hr => "HR",
            ShippingAddressCollectionAllowedCountries::Ht => "HT",
            ShippingAddressCollectionAllowedCountries::Hu => "HU",
            ShippingAddressCollectionAllowedCountries::Id => "ID",
            ShippingAddressCollectionAllowedCountries::Ie => "IE",
            ShippingAddressCollectionAllowedCountries::Il => "IL",
            ShippingAddressCollectionAllowedCountries::Im => "IM",
            ShippingAddressCollectionAllowedCountries::In => "IN",
            ShippingAddressCollectionAllowedCountries::Io => "IO",
            ShippingAddressCollectionAllowedCountries::Iq => "IQ",
            ShippingAddressCollectionAllowedCountries::Is => "IS",
            ShippingAddressCollectionAllowedCountries::It => "IT",
            ShippingAddressCollectionAllowedCountries::Je => "JE",
            ShippingAddressCollectionAllowedCountries::Jm => "JM",
            ShippingAddressCollectionAllowedCountries::Jo => "JO",
            ShippingAddressCollectionAllowedCountries::Jp => "JP",
            ShippingAddressCollectionAllowedCountries::Ke => "KE",
            ShippingAddressCollectionAllowedCountries::Kg => "KG",
            ShippingAddressCollectionAllowedCountries::Kh => "KH",
            ShippingAddressCollectionAllowedCountries::Ki => "KI",
            ShippingAddressCollectionAllowedCountries::Km => "KM",
            ShippingAddressCollectionAllowedCountries::Kn => "KN",
            ShippingAddressCollectionAllowedCountries::Kr => "KR",
            ShippingAddressCollectionAllowedCountries::Kw => "KW",
            ShippingAddressCollectionAllowedCountries::Ky => "KY",
            ShippingAddressCollectionAllowedCountries::Kz => "KZ",
            ShippingAddressCollectionAllowedCountries::La => "LA",
            ShippingAddressCollectionAllowedCountries::Lb => "LB",
            ShippingAddressCollectionAllowedCountries::Lc => "LC",
            ShippingAddressCollectionAllowedCountries::Li => "LI",
            ShippingAddressCollectionAllowedCountries::Lk => "LK",
            ShippingAddressCollectionAllowedCountries::Lr => "LR",
            ShippingAddressCollectionAllowedCountries::Ls => "LS",
            ShippingAddressCollectionAllowedCountries::Lt => "LT",
            ShippingAddressCollectionAllowedCountries::Lu => "LU",
            ShippingAddressCollectionAllowedCountries::Lv => "LV",
            ShippingAddressCollectionAllowedCountries::Ly => "LY",
            ShippingAddressCollectionAllowedCountries::Ma => "MA",
            ShippingAddressCollectionAllowedCountries::Mc => "MC",
            ShippingAddressCollectionAllowedCountries::Md => "MD",
            ShippingAddressCollectionAllowedCountries::Me => "ME",
            ShippingAddressCollectionAllowedCountries::Mf => "MF",
            ShippingAddressCollectionAllowedCountries::Mg => "MG",
            ShippingAddressCollectionAllowedCountries::Mk => "MK",
            ShippingAddressCollectionAllowedCountries::Ml => "ML",
            ShippingAddressCollectionAllowedCountries::Mm => "MM",
            ShippingAddressCollectionAllowedCountries::Mn => "MN",
            ShippingAddressCollectionAllowedCountries::Mo => "MO",
            ShippingAddressCollectionAllowedCountries::Mq => "MQ",
            ShippingAddressCollectionAllowedCountries::Mr => "MR",
            ShippingAddressCollectionAllowedCountries::Ms => "MS",
            ShippingAddressCollectionAllowedCountries::Mt => "MT",
            ShippingAddressCollectionAllowedCountries::Mu => "MU",
            ShippingAddressCollectionAllowedCountries::Mv => "MV",
            ShippingAddressCollectionAllowedCountries::Mw => "MW",
            ShippingAddressCollectionAllowedCountries::Mx => "MX",
            ShippingAddressCollectionAllowedCountries::My => "MY",
            ShippingAddressCollectionAllowedCountries::Mz => "MZ",
            ShippingAddressCollectionAllowedCountries::Na => "NA",
            ShippingAddressCollectionAllowedCountries::Nc => "NC",
            ShippingAddressCollectionAllowedCountries::Ne => "NE",
            ShippingAddressCollectionAllowedCountries::Ng => "NG",
            ShippingAddressCollectionAllowedCountries::Ni => "NI",
            ShippingAddressCollectionAllowedCountries::Nl => "NL",
            ShippingAddressCollectionAllowedCountries::No => "NO",
            ShippingAddressCollectionAllowedCountries::Np => "NP",
            ShippingAddressCollectionAllowedCountries::Nr => "NR",
            ShippingAddressCollectionAllowedCountries::Nu => "NU",
            ShippingAddressCollectionAllowedCountries::Nz => "NZ",
            ShippingAddressCollectionAllowedCountries::Om => "OM",
            ShippingAddressCollectionAllowedCountries::Pa => "PA",
            ShippingAddressCollectionAllowedCountries::Pe => "PE",
            ShippingAddressCollectionAllowedCountries::Pf => "PF",
            ShippingAddressCollectionAllowedCountries::Pg => "PG",
            ShippingAddressCollectionAllowedCountries::Ph => "PH",
            ShippingAddressCollectionAllowedCountries::Pk => "PK",
            ShippingAddressCollectionAllowedCountries::Pl => "PL",
            ShippingAddressCollectionAllowedCountries::Pm => "PM",
            ShippingAddressCollectionAllowedCountries::Pn => "PN",
            ShippingAddressCollectionAllowedCountries::Pr => "PR",
            ShippingAddressCollectionAllowedCountries::Ps => "PS",
            ShippingAddressCollectionAllowedCountries::Pt => "PT",
            ShippingAddressCollectionAllowedCountries::Py => "PY",
            ShippingAddressCollectionAllowedCountries::Qa => "QA",
            ShippingAddressCollectionAllowedCountries::Re => "RE",
            ShippingAddressCollectionAllowedCountries::Ro => "RO",
            ShippingAddressCollectionAllowedCountries::Rs => "RS",
            ShippingAddressCollectionAllowedCountries::Ru => "RU",
            ShippingAddressCollectionAllowedCountries::Rw => "RW",
            ShippingAddressCollectionAllowedCountries::Sa => "SA",
            ShippingAddressCollectionAllowedCountries::Sb => "SB",
            ShippingAddressCollectionAllowedCountries::Sc => "SC",
            ShippingAddressCollectionAllowedCountries::Se => "SE",
            ShippingAddressCollectionAllowedCountries::Sg => "SG",
            ShippingAddressCollectionAllowedCountries::Sh => "SH",
            ShippingAddressCollectionAllowedCountries::Si => "SI",
            ShippingAddressCollectionAllowedCountries::Sj => "SJ",
            ShippingAddressCollectionAllowedCountries::Sk => "SK",
            ShippingAddressCollectionAllowedCountries::Sl => "SL",
            ShippingAddressCollectionAllowedCountries::Sm => "SM",
            ShippingAddressCollectionAllowedCountries::Sn => "SN",
            ShippingAddressCollectionAllowedCountries::So => "SO",
            ShippingAddressCollectionAllowedCountries::Sr => "SR",
            ShippingAddressCollectionAllowedCountries::Ss => "SS",
            ShippingAddressCollectionAllowedCountries::St => "ST",
            ShippingAddressCollectionAllowedCountries::Sv => "SV",
            ShippingAddressCollectionAllowedCountries::Sx => "SX",
            ShippingAddressCollectionAllowedCountries::Sz => "SZ",
            ShippingAddressCollectionAllowedCountries::Ta => "TA",
            ShippingAddressCollectionAllowedCountries::Tc => "TC",
            ShippingAddressCollectionAllowedCountries::Td => "TD",
            ShippingAddressCollectionAllowedCountries::Tf => "TF",
            ShippingAddressCollectionAllowedCountries::Tg => "TG",
            ShippingAddressCollectionAllowedCountries::Th => "TH",
            ShippingAddressCollectionAllowedCountries::Tj => "TJ",
            ShippingAddressCollectionAllowedCountries::Tk => "TK",
            ShippingAddressCollectionAllowedCountries::Tl => "TL",
            ShippingAddressCollectionAllowedCountries::Tm => "TM",
            ShippingAddressCollectionAllowedCountries::Tn => "TN",
            ShippingAddressCollectionAllowedCountries::To => "TO",
            ShippingAddressCollectionAllowedCountries::Tr => "TR",
            ShippingAddressCollectionAllowedCountries::Tt => "TT",
            ShippingAddressCollectionAllowedCountries::Tv => "TV",
            ShippingAddressCollectionAllowedCountries::Tw => "TW",
            ShippingAddressCollectionAllowedCountries::Tz => "TZ",
            ShippingAddressCollectionAllowedCountries::Ua => "UA",
            ShippingAddressCollectionAllowedCountries::Ug => "UG",
            ShippingAddressCollectionAllowedCountries::Us => "US",
            ShippingAddressCollectionAllowedCountries::Uy => "UY",
            ShippingAddressCollectionAllowedCountries::Uz => "UZ",
            ShippingAddressCollectionAllowedCountries::Va => "VA",
            ShippingAddressCollectionAllowedCountries::Vc => "VC",
            ShippingAddressCollectionAllowedCountries::Ve => "VE",
            ShippingAddressCollectionAllowedCountries::Vg => "VG",
            ShippingAddressCollectionAllowedCountries::Vn => "VN",
            ShippingAddressCollectionAllowedCountries::Vu => "VU",
            ShippingAddressCollectionAllowedCountries::Wf => "WF",
            ShippingAddressCollectionAllowedCountries::Ws => "WS",
            ShippingAddressCollectionAllowedCountries::Xk => "XK",
            ShippingAddressCollectionAllowedCountries::Ye => "YE",
            ShippingAddressCollectionAllowedCountries::Yt => "YT",
            ShippingAddressCollectionAllowedCountries::Za => "ZA",
            ShippingAddressCollectionAllowedCountries::Zm => "ZM",
            ShippingAddressCollectionAllowedCountries::Zw => "ZW",
            ShippingAddressCollectionAllowedCountries::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for ShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
