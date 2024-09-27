// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{CustomerId, PaymentMethodId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, Timestamp};
use crate::resources::{BillingDetails, CardDetails, CardPresent, Customer, PaymentFlowsPrivatePaymentMethodsAlipay, PaymentMethodAcssDebit, PaymentMethodAffirm, PaymentMethodAfterpayClearpay, PaymentMethodAmazonPay, PaymentMethodAuBecsDebit, PaymentMethodBacsDebit, PaymentMethodBancontact, PaymentMethodBlik, PaymentMethodBoleto, PaymentMethodCashapp, PaymentMethodCustomerBalance, PaymentMethodEps, PaymentMethodFpx, PaymentMethodGiropay, PaymentMethodGrabpay, PaymentMethodIdeal, PaymentMethodInteracPresent, PaymentMethodKlarna, PaymentMethodKonbini, PaymentMethodLink, PaymentMethodMobilepay, PaymentMethodMultibanco, PaymentMethodOxxo, PaymentMethodP24, PaymentMethodPaynow, PaymentMethodPaypal, PaymentMethodPix, PaymentMethodPromptpay, PaymentMethodRevolutPay, PaymentMethodSepaDebit, PaymentMethodSofort, PaymentMethodSwish, PaymentMethodTwint, PaymentMethodUsBankAccount, PaymentMethodWechatPay, PaymentMethodZip, RadarRadarOptions};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "PaymentMethod".
///
/// For more details see <https://stripe.com/docs/api/payment_methods/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethod {
    /// Unique identifier for the object.
    pub id: PaymentMethodId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<PaymentMethodAffirm>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<PaymentMethodAfterpayClearpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PaymentFlowsPrivatePaymentMethodsAlipay>,

    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    ///
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to “unspecified”.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<PaymentMethodAllowRedisplay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<PaymentMethodAmazonPay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<PaymentMethodAuBecsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<PaymentMethodBacsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PaymentMethodBancontact>,

    pub billing_details: BillingDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<PaymentMethodBlik>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<PaymentMethodBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<CardPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<PaymentMethodCashapp>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The ID of the Customer to which this PaymentMethod is saved.
    ///
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    pub customer: Option<Expandable<Customer>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<PaymentMethodCustomerBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<PaymentMethodEps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<PaymentMethodFpx>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<PaymentMethodGiropay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<PaymentMethodGrabpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PaymentMethodIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<PaymentMethodInteracPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PaymentMethodKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<PaymentMethodKonbini>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodLink>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<PaymentMethodMobilepay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<PaymentMethodMultibanco>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PaymentMethodOxxo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PaymentMethodP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<PaymentMethodPaynow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodPaypal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<PaymentMethodPix>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<PaymentMethodPromptpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<RadarRadarOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<PaymentMethodRevolutPay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PaymentMethodSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PaymentMethodSofort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<PaymentMethodSwish>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub twint: Option<PaymentMethodTwint>,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PaymentMethodUsBankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PaymentMethodWechatPay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<PaymentMethodZip>,
}

impl PaymentMethod {

    /// Returns a list of PaymentMethods for Treasury flows.
    ///
    /// If you want to list the PaymentMethods attached to a Customer for payments, you should use the [List a Customer’s PaymentMethods](https://stripe.com/docs/api/payment_methods/customer_list) API instead.
pub fn list(client: &Client, params: &ListPaymentMethods<'_>) -> Response<List<PaymentMethod>> {
   client.get_query("/payment_methods", params)
}


    /// Creates a PaymentMethod object.
    ///
    /// Read the [Stripe.js reference](https://stripe.com/docs/stripe-js/reference#stripe-create-payment-method) to learn how to create PaymentMethods via Stripe.js.  Instead of creating a PaymentMethod directly, we recommend using the [PaymentIntents](https://stripe.com/docs/payments/accept-a-payment) API to accept a payment immediately or the [SetupIntent](https://stripe.com/docs/payments/save-and-reuse) API to collect payment method details ahead of a future payment.
    pub fn create(client: &Client, params: CreatePaymentMethod<'_>) -> Response<PaymentMethod> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/payment_methods", &params)
    }

    /// Retrieves a PaymentMethod object attached to the StripeAccount.
    ///
    /// To retrieve a payment method attached to a Customer, you should use [Retrieve a Customer’s PaymentMethods](https://stripe.com/docs/api/payment_methods/customer).
    pub fn retrieve(client: &Client, id: &PaymentMethodId, expand: &[&str]) -> Response<PaymentMethod> {
        client.get_query(&format!("/payment_methods/{}", id), Expand { expand })
    }

    /// Updates a PaymentMethod object.
    ///
    /// A PaymentMethod must be attached a customer to be updated.
    pub fn update(client: &Client, id: &PaymentMethodId, params: UpdatePaymentMethod<'_>) -> Response<PaymentMethod> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/payment_methods/{}", id), &params)
    }
}

impl Object for PaymentMethod {
    type Id = PaymentMethodId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "payment_method"
    }
}

/// The parameters for `PaymentMethod::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreatePaymentMethod<'a> {

    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreatePaymentMethodAcssDebit>,

    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreatePaymentMethodAffirm>,

    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreatePaymentMethodAfterpayClearpay>,

    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreatePaymentMethodAlipay>,

    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    ///
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<PaymentMethodAllowRedisplay>,

    /// If this is a AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<CreatePaymentMethodAmazonPay>,

    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreatePaymentMethodAuBecsDebit>,

    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreatePaymentMethodBacsDebit>,

    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreatePaymentMethodBancontact>,

    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,

    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<CreatePaymentMethodBlik>,

    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreatePaymentMethodBoleto>,

    /// If this is a `card` PaymentMethod, this hash contains the user's card details.
    ///
    /// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format `card: {token: "tok_visa"}`.
    /// When providing a card number, you must meet the requirements for [PCI compliance](https://stripe.com/docs/security#validating-pci-compliance).
    /// We strongly recommend using Stripe.js instead of interacting with this API directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreatePaymentMethodCardUnion>,

    /// If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<CreatePaymentMethodCashapp>,

    /// The `Customer` to whom the original PaymentMethod is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreatePaymentMethodCustomerBalance>,

    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreatePaymentMethodEps>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreatePaymentMethodFpx>,

    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreatePaymentMethodGiropay>,

    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreatePaymentMethodGrabpay>,

    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreatePaymentMethodIdeal>,

    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<CreatePaymentMethodInteracPresent>,

    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreatePaymentMethodKlarna>,

    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreatePaymentMethodKonbini>,

    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreatePaymentMethodLink>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<CreatePaymentMethodMobilepay>,

    /// If this is a `multibanco` PaymentMethod, this hash contains details about the Multibanco payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<CreatePaymentMethodMultibanco>,

    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreatePaymentMethodOxxo>,

    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreatePaymentMethodP24>,

    /// The PaymentMethod to share.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodId>,

    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreatePaymentMethodPaynow>,

    /// If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CreatePaymentMethodPaypal>,

    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreatePaymentMethodPix>,

    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<CreatePaymentMethodPromptpay>,

    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<CreatePaymentMethodRadarOptions>,

    /// If this is a `Revolut Pay` PaymentMethod, this hash contains details about the Revolut Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<CreatePaymentMethodRevolutPay>,

    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreatePaymentMethodSepaDebit>,

    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreatePaymentMethodSofort>,

    /// If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<CreatePaymentMethodSwish>,

    /// If this is a TWINT PaymentMethod, this hash contains details about the TWINT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twint: Option<CreatePaymentMethodTwint>,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PaymentMethodTypeFilter>,

    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreatePaymentMethodUsBankAccount>,

    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreatePaymentMethodWechatPay>,

    /// If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<CreatePaymentMethodZip>,
}

impl<'a> CreatePaymentMethod<'a> {
    pub fn new() -> Self {
        CreatePaymentMethod {
            acss_debit: Default::default(),
            affirm: Default::default(),
            afterpay_clearpay: Default::default(),
            alipay: Default::default(),
            allow_redisplay: Default::default(),
            amazon_pay: Default::default(),
            au_becs_debit: Default::default(),
            bacs_debit: Default::default(),
            bancontact: Default::default(),
            billing_details: Default::default(),
            blik: Default::default(),
            boleto: Default::default(),
            card: Default::default(),
            cashapp: Default::default(),
            customer: Default::default(),
            customer_balance: Default::default(),
            eps: Default::default(),
            expand: Default::default(),
            fpx: Default::default(),
            giropay: Default::default(),
            grabpay: Default::default(),
            ideal: Default::default(),
            interac_present: Default::default(),
            klarna: Default::default(),
            konbini: Default::default(),
            link: Default::default(),
            metadata: Default::default(),
            mobilepay: Default::default(),
            multibanco: Default::default(),
            oxxo: Default::default(),
            p24: Default::default(),
            payment_method: Default::default(),
            paynow: Default::default(),
            paypal: Default::default(),
            pix: Default::default(),
            promptpay: Default::default(),
            radar_options: Default::default(),
            revolut_pay: Default::default(),
            sepa_debit: Default::default(),
            sofort: Default::default(),
            swish: Default::default(),
            twint: Default::default(),
            type_: Default::default(),
            us_bank_account: Default::default(),
            wechat_pay: Default::default(),
            zip: Default::default(),
        }
    }
}

/// The parameters for `PaymentMethod::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListPaymentMethods<'a> {

    /// The ID of the customer whose PaymentMethods will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<PaymentMethodId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<PaymentMethodId>,

    /// An optional filter on the list, based on the object `type` field.
    ///
    /// Without the filter, the list includes all current and future payment method types.
    /// If your integration expects only one type of payment method in the response, make sure to provide a type value in the request.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PaymentMethodTypeFilter>,
}

impl<'a> ListPaymentMethods<'a> {
    pub fn new() -> Self {
        ListPaymentMethods {
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            type_: Default::default(),
        }
    }
}
impl Paginable for ListPaymentMethods<'_> {
    type O = PaymentMethod;
    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }}
/// The parameters for `PaymentMethod::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdatePaymentMethod<'a> {

    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    ///
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<PaymentMethodAllowRedisplay>,

    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,

    /// If this is a `card` PaymentMethod, this hash contains the user's card details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdateApiParam>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<UpdatePaymentMethodLink>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdatePaymentMethodUsBankAccount>,
}

impl<'a> UpdatePaymentMethod<'a> {
    pub fn new() -> Self {
        UpdatePaymentMethod {
            allow_redisplay: Default::default(),
            billing_details: Default::default(),
            card: Default::default(),
            expand: Default::default(),
            link: Default::default(),
            metadata: Default::default(),
            us_bank_account: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodAcssDebit {

    /// Customer's bank account number.
    pub account_number: String,

    /// Institution number of the customer's bank.
    pub institution_number: String,

    /// Transit number of the customer's bank.
    pub transit_number: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodAffirm {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodAfterpayClearpay {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodAlipay {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodAmazonPay {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodAuBecsDebit {

    /// The account number for the bank account.
    pub account_number: String,

    /// Bank-State-Branch number of the bank account.
    pub bsb_number: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodBacsDebit {

    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodBancontact {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodBlik {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodBoleto {

    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodCashapp {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodCustomerBalance {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodEps {

    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentMethodEpsBank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodFpx {

    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreatePaymentMethodFpxAccountHolderType>,

    /// The customer's bank.
    pub bank: CreatePaymentMethodFpxBank,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodGiropay {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodGrabpay {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodIdeal {

    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentMethodIdealBank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodInteracPresent {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodKlarna {

    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreatePaymentMethodKlarnaDob>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodKonbini {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodLink {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodMobilepay {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodMultibanco {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodOxxo {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodP24 {

    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentMethodP24Bank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodPaynow {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodPaypal {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodPix {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodPromptpay {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodRadarOptions {

    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodRevolutPay {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodSepaDebit {

    /// IBAN of the bank account.
    pub iban: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodSofort {

    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: CreatePaymentMethodSofortCountry,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodSwish {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodTwint {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodUsBankAccount {

    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreatePaymentMethodUsBankAccountAccountHolderType>,

    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<CreatePaymentMethodUsBankAccountAccountType>,

    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,

    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodWechatPay {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodZip {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodLink {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodUsBankAccount {

    /// Bank account holder type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdatePaymentMethodUsBankAccountAccountHolderType>,

    /// Bank account type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<UpdatePaymentMethodUsBankAccountAccountType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodKlarnaDob {

    /// The day of birth, between 1 and 31.
    pub day: i64,

    /// The month of birth, between 1 and 12.
    pub month: i64,

    /// The four-digit year of birth.
    pub year: i64,
}

/// An enum representing the possible values of an `CreatePaymentMethodEps`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl CreatePaymentMethodEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodEpsBank::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            CreatePaymentMethodEpsBank::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            CreatePaymentMethodEpsBank::BankAustria => "bank_austria",
            CreatePaymentMethodEpsBank::BankhausCarlSpangler => "bankhaus_carl_spangler",
            CreatePaymentMethodEpsBank::BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            CreatePaymentMethodEpsBank::BawagPskAg => "bawag_psk_ag",
            CreatePaymentMethodEpsBank::BksBankAg => "bks_bank_ag",
            CreatePaymentMethodEpsBank::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            CreatePaymentMethodEpsBank::BtvVierLanderBank => "btv_vier_lander_bank",
            CreatePaymentMethodEpsBank::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            CreatePaymentMethodEpsBank::DeutscheBankAg => "deutsche_bank_ag",
            CreatePaymentMethodEpsBank::Dolomitenbank => "dolomitenbank",
            CreatePaymentMethodEpsBank::EasybankAg => "easybank_ag",
            CreatePaymentMethodEpsBank::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            CreatePaymentMethodEpsBank::HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            CreatePaymentMethodEpsBank::HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            CreatePaymentMethodEpsBank::HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            CreatePaymentMethodEpsBank::HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            CreatePaymentMethodEpsBank::HypoTirolBankAg => "hypo_tirol_bank_ag",
            CreatePaymentMethodEpsBank::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            CreatePaymentMethodEpsBank::MarchfelderBank => "marchfelder_bank",
            CreatePaymentMethodEpsBank::OberbankAg => "oberbank_ag",
            CreatePaymentMethodEpsBank::RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            CreatePaymentMethodEpsBank::SchoellerbankAg => "schoellerbank_ag",
            CreatePaymentMethodEpsBank::SpardaBankWien => "sparda_bank_wien",
            CreatePaymentMethodEpsBank::VolksbankGruppe => "volksbank_gruppe",
            CreatePaymentMethodEpsBank::VolkskreditbankAg => "volkskreditbank_ag",
            CreatePaymentMethodEpsBank::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodEpsBank {
    fn default() -> Self {
        Self::ArzteUndApothekerBank
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodFpx`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodFpxAccountHolderType {
    Company,
    Individual,
}

impl CreatePaymentMethodFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodFpxAccountHolderType::Company => "company",
            CreatePaymentMethodFpxAccountHolderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodFpxAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodFpx`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl CreatePaymentMethodFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodFpxBank::AffinBank => "affin_bank",
            CreatePaymentMethodFpxBank::Agrobank => "agrobank",
            CreatePaymentMethodFpxBank::AllianceBank => "alliance_bank",
            CreatePaymentMethodFpxBank::Ambank => "ambank",
            CreatePaymentMethodFpxBank::BankIslam => "bank_islam",
            CreatePaymentMethodFpxBank::BankMuamalat => "bank_muamalat",
            CreatePaymentMethodFpxBank::BankOfChina => "bank_of_china",
            CreatePaymentMethodFpxBank::BankRakyat => "bank_rakyat",
            CreatePaymentMethodFpxBank::Bsn => "bsn",
            CreatePaymentMethodFpxBank::Cimb => "cimb",
            CreatePaymentMethodFpxBank::DeutscheBank => "deutsche_bank",
            CreatePaymentMethodFpxBank::HongLeongBank => "hong_leong_bank",
            CreatePaymentMethodFpxBank::Hsbc => "hsbc",
            CreatePaymentMethodFpxBank::Kfh => "kfh",
            CreatePaymentMethodFpxBank::Maybank2e => "maybank2e",
            CreatePaymentMethodFpxBank::Maybank2u => "maybank2u",
            CreatePaymentMethodFpxBank::Ocbc => "ocbc",
            CreatePaymentMethodFpxBank::PbEnterprise => "pb_enterprise",
            CreatePaymentMethodFpxBank::PublicBank => "public_bank",
            CreatePaymentMethodFpxBank::Rhb => "rhb",
            CreatePaymentMethodFpxBank::StandardChartered => "standard_chartered",
            CreatePaymentMethodFpxBank::Uob => "uob",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodFpxBank {
    fn default() -> Self {
        Self::AffinBank
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodIdeal`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
}

impl CreatePaymentMethodIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodIdealBank::AbnAmro => "abn_amro",
            CreatePaymentMethodIdealBank::AsnBank => "asn_bank",
            CreatePaymentMethodIdealBank::Bunq => "bunq",
            CreatePaymentMethodIdealBank::Handelsbanken => "handelsbanken",
            CreatePaymentMethodIdealBank::Ing => "ing",
            CreatePaymentMethodIdealBank::Knab => "knab",
            CreatePaymentMethodIdealBank::Moneyou => "moneyou",
            CreatePaymentMethodIdealBank::N26 => "n26",
            CreatePaymentMethodIdealBank::Nn => "nn",
            CreatePaymentMethodIdealBank::Rabobank => "rabobank",
            CreatePaymentMethodIdealBank::Regiobank => "regiobank",
            CreatePaymentMethodIdealBank::Revolut => "revolut",
            CreatePaymentMethodIdealBank::SnsBank => "sns_bank",
            CreatePaymentMethodIdealBank::TriodosBank => "triodos_bank",
            CreatePaymentMethodIdealBank::VanLanschot => "van_lanschot",
            CreatePaymentMethodIdealBank::Yoursafe => "yoursafe",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodIdealBank {
    fn default() -> Self {
        Self::AbnAmro
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodP24`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    Velobank,
    VolkswagenBank,
}

impl CreatePaymentMethodP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodP24Bank::AliorBank => "alior_bank",
            CreatePaymentMethodP24Bank::BankMillennium => "bank_millennium",
            CreatePaymentMethodP24Bank::BankNowyBfgSa => "bank_nowy_bfg_sa",
            CreatePaymentMethodP24Bank::BankPekaoSa => "bank_pekao_sa",
            CreatePaymentMethodP24Bank::BankiSpbdzielcze => "banki_spbdzielcze",
            CreatePaymentMethodP24Bank::Blik => "blik",
            CreatePaymentMethodP24Bank::BnpParibas => "bnp_paribas",
            CreatePaymentMethodP24Bank::Boz => "boz",
            CreatePaymentMethodP24Bank::CitiHandlowy => "citi_handlowy",
            CreatePaymentMethodP24Bank::CreditAgricole => "credit_agricole",
            CreatePaymentMethodP24Bank::Envelobank => "envelobank",
            CreatePaymentMethodP24Bank::EtransferPocztowy24 => "etransfer_pocztowy24",
            CreatePaymentMethodP24Bank::GetinBank => "getin_bank",
            CreatePaymentMethodP24Bank::Ideabank => "ideabank",
            CreatePaymentMethodP24Bank::Ing => "ing",
            CreatePaymentMethodP24Bank::Inteligo => "inteligo",
            CreatePaymentMethodP24Bank::MbankMtransfer => "mbank_mtransfer",
            CreatePaymentMethodP24Bank::NestPrzelew => "nest_przelew",
            CreatePaymentMethodP24Bank::NoblePay => "noble_pay",
            CreatePaymentMethodP24Bank::PbacZIpko => "pbac_z_ipko",
            CreatePaymentMethodP24Bank::PlusBank => "plus_bank",
            CreatePaymentMethodP24Bank::SantanderPrzelew24 => "santander_przelew24",
            CreatePaymentMethodP24Bank::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            CreatePaymentMethodP24Bank::ToyotaBank => "toyota_bank",
            CreatePaymentMethodP24Bank::Velobank => "velobank",
            CreatePaymentMethodP24Bank::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodP24Bank {
    fn default() -> Self {
        Self::AliorBank
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodSofort`'s `country` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodSofortCountry {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "NL")]
    Nl,
}

impl CreatePaymentMethodSofortCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodSofortCountry::At => "AT",
            CreatePaymentMethodSofortCountry::Be => "BE",
            CreatePaymentMethodSofortCountry::De => "DE",
            CreatePaymentMethodSofortCountry::Es => "ES",
            CreatePaymentMethodSofortCountry::It => "IT",
            CreatePaymentMethodSofortCountry::Nl => "NL",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodSofortCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodSofortCountry {
    fn default() -> Self {
        Self::At
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodUsBankAccount`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl CreatePaymentMethodUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodUsBankAccountAccountHolderType::Company => "company",
            CreatePaymentMethodUsBankAccountAccountHolderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodUsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodUsBankAccount`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodUsBankAccountAccountType {
    Checking,
    Savings,
}

impl CreatePaymentMethodUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodUsBankAccountAccountType::Checking => "checking",
            CreatePaymentMethodUsBankAccountAccountType::Savings => "savings",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodUsBankAccountAccountType {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `PaymentMethod`'s `allow_redisplay` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}

impl PaymentMethodAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodAllowRedisplay::Always => "always",
            PaymentMethodAllowRedisplay::Limited => "limited",
            PaymentMethodAllowRedisplay::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PaymentMethodAllowRedisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodAllowRedisplay {
    fn default() -> Self {
        Self::Always
    }
}

/// An enum representing the possible values of an `PaymentMethod`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    CardPresent,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    InteracPresent,
    Klarna,
    Konbini,
    Link,
    Mobilepay,
    Multibanco,
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
    Twint,
    UsBankAccount,
    WechatPay,
    Zip,
}

impl PaymentMethodType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodType::AcssDebit => "acss_debit",
            PaymentMethodType::Affirm => "affirm",
            PaymentMethodType::AfterpayClearpay => "afterpay_clearpay",
            PaymentMethodType::Alipay => "alipay",
            PaymentMethodType::AmazonPay => "amazon_pay",
            PaymentMethodType::AuBecsDebit => "au_becs_debit",
            PaymentMethodType::BacsDebit => "bacs_debit",
            PaymentMethodType::Bancontact => "bancontact",
            PaymentMethodType::Blik => "blik",
            PaymentMethodType::Boleto => "boleto",
            PaymentMethodType::Card => "card",
            PaymentMethodType::CardPresent => "card_present",
            PaymentMethodType::Cashapp => "cashapp",
            PaymentMethodType::CustomerBalance => "customer_balance",
            PaymentMethodType::Eps => "eps",
            PaymentMethodType::Fpx => "fpx",
            PaymentMethodType::Giropay => "giropay",
            PaymentMethodType::Grabpay => "grabpay",
            PaymentMethodType::Ideal => "ideal",
            PaymentMethodType::InteracPresent => "interac_present",
            PaymentMethodType::Klarna => "klarna",
            PaymentMethodType::Konbini => "konbini",
            PaymentMethodType::Link => "link",
            PaymentMethodType::Mobilepay => "mobilepay",
            PaymentMethodType::Multibanco => "multibanco",
            PaymentMethodType::Oxxo => "oxxo",
            PaymentMethodType::P24 => "p24",
            PaymentMethodType::Paynow => "paynow",
            PaymentMethodType::Paypal => "paypal",
            PaymentMethodType::Pix => "pix",
            PaymentMethodType::Promptpay => "promptpay",
            PaymentMethodType::RevolutPay => "revolut_pay",
            PaymentMethodType::SepaDebit => "sepa_debit",
            PaymentMethodType::Sofort => "sofort",
            PaymentMethodType::Swish => "swish",
            PaymentMethodType::Twint => "twint",
            PaymentMethodType::UsBankAccount => "us_bank_account",
            PaymentMethodType::WechatPay => "wechat_pay",
            PaymentMethodType::Zip => "zip",
        }
    }
}

impl AsRef<str> for PaymentMethodType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodType {
    fn default() -> Self {
        Self::AcssDebit
    }
}

/// An enum representing the possible values of an `ListPaymentMethods`'s `type_` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodTypeFilter {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AmazonPay,
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
    Mobilepay,
    Multibanco,
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
    Twint,
    UsBankAccount,
    WechatPay,
    Zip,
}

impl PaymentMethodTypeFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodTypeFilter::AcssDebit => "acss_debit",
            PaymentMethodTypeFilter::Affirm => "affirm",
            PaymentMethodTypeFilter::AfterpayClearpay => "afterpay_clearpay",
            PaymentMethodTypeFilter::Alipay => "alipay",
            PaymentMethodTypeFilter::AmazonPay => "amazon_pay",
            PaymentMethodTypeFilter::AuBecsDebit => "au_becs_debit",
            PaymentMethodTypeFilter::BacsDebit => "bacs_debit",
            PaymentMethodTypeFilter::Bancontact => "bancontact",
            PaymentMethodTypeFilter::Blik => "blik",
            PaymentMethodTypeFilter::Boleto => "boleto",
            PaymentMethodTypeFilter::Card => "card",
            PaymentMethodTypeFilter::Cashapp => "cashapp",
            PaymentMethodTypeFilter::CustomerBalance => "customer_balance",
            PaymentMethodTypeFilter::Eps => "eps",
            PaymentMethodTypeFilter::Fpx => "fpx",
            PaymentMethodTypeFilter::Giropay => "giropay",
            PaymentMethodTypeFilter::Grabpay => "grabpay",
            PaymentMethodTypeFilter::Ideal => "ideal",
            PaymentMethodTypeFilter::Klarna => "klarna",
            PaymentMethodTypeFilter::Konbini => "konbini",
            PaymentMethodTypeFilter::Link => "link",
            PaymentMethodTypeFilter::Mobilepay => "mobilepay",
            PaymentMethodTypeFilter::Multibanco => "multibanco",
            PaymentMethodTypeFilter::Oxxo => "oxxo",
            PaymentMethodTypeFilter::P24 => "p24",
            PaymentMethodTypeFilter::Paynow => "paynow",
            PaymentMethodTypeFilter::Paypal => "paypal",
            PaymentMethodTypeFilter::Pix => "pix",
            PaymentMethodTypeFilter::Promptpay => "promptpay",
            PaymentMethodTypeFilter::RevolutPay => "revolut_pay",
            PaymentMethodTypeFilter::SepaDebit => "sepa_debit",
            PaymentMethodTypeFilter::Sofort => "sofort",
            PaymentMethodTypeFilter::Swish => "swish",
            PaymentMethodTypeFilter::Twint => "twint",
            PaymentMethodTypeFilter::UsBankAccount => "us_bank_account",
            PaymentMethodTypeFilter::WechatPay => "wechat_pay",
            PaymentMethodTypeFilter::Zip => "zip",
        }
    }
}

impl AsRef<str> for PaymentMethodTypeFilter {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodTypeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodTypeFilter {
    fn default() -> Self {
        Self::AcssDebit
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodUsBankAccount`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl UpdatePaymentMethodUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodUsBankAccountAccountHolderType::Company => "company",
            UpdatePaymentMethodUsBankAccountAccountHolderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodUsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodUsBankAccount`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodUsBankAccountAccountType {
    Checking,
    Savings,
}

impl UpdatePaymentMethodUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodUsBankAccountAccountType::Checking => "checking",
            UpdatePaymentMethodUsBankAccountAccountType::Savings => "savings",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodUsBankAccountAccountType {
    fn default() -> Self {
        Self::Checking
    }
}

/// If this is a `card` PaymentMethod, this hash contains the user's card details.
///
/// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format `card: {token: "tok_visa"}`.
/// When providing a card number, you must meet the requirements for [PCI compliance](https://stripe.com/docs/security#validating-pci-compliance).
/// We strongly recommend using Stripe.js instead of interacting with this API directly.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum CreatePaymentMethodCardUnion {
    pub CardDetailsParams(CardDetailsParams),
    pub TokenParams(TokenParams),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CardDetailsParams {
    /// The card's CVC.
    ///
    /// It is highly recommended to always include this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: i32,
    /// Four-digit number representing the card's expiration year.
    pub exp_year: i32,
    /// Contains information about card networks used to process the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Networks>,
    /// The card number, as a string without any separators.
    pub number: String,
}

/// Contains information about card networks used to process the payment.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Networks {
    /// The customer's preferred card network for co-branded cards.
    ///
    /// Supports `cartes_bancaires`, `mastercard`, or `visa`.
    /// Selection of a network that does not apply to the card will be stored as `invalid_preference` on the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TokenParams {
    /// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format card: {token: "tok_visa"}.
    pub token: String,
}

/// If this is a `card` PaymentMethod, this hash contains the user's card details.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateApiParam {
    /// Two-digit number representing the card's expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i32>,
    /// Four-digit number representing the card's expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i32>,
    /// Contains information about card networks used to process the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Networks>,
}

/// Contains information about card networks used to process the payment.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Networks {
    /// The customer's preferred card network for co-branded cards.
    ///
    /// Supports `cartes_bancaires`, `mastercard`, or `visa`.
    /// Selection of a network that does not apply to the card will be stored as `invalid_preference` on the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<String>,
}
