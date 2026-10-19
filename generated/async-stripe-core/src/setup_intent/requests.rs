use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListSetupIntentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    attach_to_self: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListSetupIntentBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListSetupIntentBuilder").finish_non_exhaustive()
    }
}
impl ListSetupIntentBuilder {
    fn new() -> Self {
        Self {
            attach_to_self: None,
            created: None,
            customer: None,
            customer_account: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_method: None,
            starting_after: None,
        }
    }
}
/// Returns a list of SetupIntents.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListSetupIntent {
    inner: ListSetupIntentBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListSetupIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListSetupIntent").finish_non_exhaustive()
    }
}
impl ListSetupIntent {
    /// Construct a new `ListSetupIntent`.
    pub fn new() -> Self {
        Self { inner: ListSetupIntentBuilder::new() }
    }
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    pub fn attach_to_self(mut self, attach_to_self: impl Into<bool>) -> Self {
        self.inner.attach_to_self = Some(attach_to_self.into());
        self
    }
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return SetupIntents for the customer specified by this customer ID.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// Only return SetupIntents for the account specified by this customer ID.
    pub fn customer_account(mut self, customer_account: impl Into<String>) -> Self {
        self.inner.customer_account = Some(customer_account.into());
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
    /// Only return SetupIntents that associate with the specified payment method.
    pub fn payment_method(mut self, payment_method: impl Into<String>) -> Self {
        self.inner.payment_method = Some(payment_method.into());
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
impl Default for ListSetupIntent {
    fn default() -> Self {
        Self::new()
    }
}
impl ListSetupIntent {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::SetupIntent>> {
        stripe_client_core::ListPaginator::new_list("/setup_intents", &self.inner)
    }
}

impl StripeRequest for ListSetupIntent {
    type Output = stripe_types::List<stripe_shared::SetupIntent>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/setup_intents").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveSetupIntentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveSetupIntentBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveSetupIntentBuilder").finish_non_exhaustive()
    }
}
impl RetrieveSetupIntentBuilder {
    fn new() -> Self {
        Self { client_secret: None, expand: None }
    }
}
/// Retrieves the details of a SetupIntent that has previously been created.
///
/// Client-side retrieval using a publishable key is allowed when the `client_secret` is provided in the query string.
///
///
/// When retrieved with a publishable key, only a subset of properties will be returned.
/// Please refer to the [SetupIntent](https://stripe.com/docs/api#setup_intent_object) object reference for more details.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveSetupIntent {
    inner: RetrieveSetupIntentBuilder,
    intent: stripe_shared::SetupIntentId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveSetupIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveSetupIntent").finish_non_exhaustive()
    }
}
impl RetrieveSetupIntent {
    /// Construct a new `RetrieveSetupIntent`.
    pub fn new(intent: impl Into<stripe_shared::SetupIntentId>) -> Self {
        Self { intent: intent.into(), inner: RetrieveSetupIntentBuilder::new() }
    }
    /// The client secret of the SetupIntent.
    /// We require this string if you use a publishable key to retrieve the SetupIntent.
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.inner.client_secret = Some(client_secret.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveSetupIntent {
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

impl StripeRequest for RetrieveSetupIntent {
    type Output = stripe_shared::SetupIntent;

    fn build(&self) -> RequestBuilder {
        let intent = &self.intent;
        RequestBuilder::new(StripeMethod::Get, format!("/setup_intents/{intent}"))
            .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateSetupIntentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_payment_method_types: Option<Vec<stripe_shared::AllowedPaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attach_to_self: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_payment_methods: Option<CreateSetupIntentAutomaticPaymentMethods>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirmation_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_payment_method_types: Option<Vec<stripe_shared::ExcludedPaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_directions: Option<Vec<stripe_shared::FlowDirections>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mandate_data: Option<SecretKeyParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_data: Option<CreateSetupIntentPaymentMethodData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_options: Option<CreateSetupIntentPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_use: Option<CreateSetupIntentSingleUse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<stripe_shared::CreateSetupIntentUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_stripe_sdk: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentBuilder").finish_non_exhaustive()
    }
}
impl CreateSetupIntentBuilder {
    fn new() -> Self {
        Self {
            allowed_payment_method_types: None,
            attach_to_self: None,
            automatic_payment_methods: None,
            confirm: None,
            confirmation_token: None,
            customer: None,
            customer_account: None,
            description: None,
            excluded_payment_method_types: None,
            expand: None,
            flow_directions: None,
            mandate_data: None,
            metadata: None,
            on_behalf_of: None,
            payment_method: None,
            payment_method_configuration: None,
            payment_method_data: None,
            payment_method_options: None,
            payment_method_types: None,
            return_url: None,
            single_use: None,
            usage: None,
            use_stripe_sdk: None,
        }
    }
}
/// When you enable this parameter, this SetupIntent accepts payment methods that you enable in the Dashboard and that are compatible with its other parameters.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentAutomaticPaymentMethods {
    /// Controls whether this SetupIntent will accept redirect-based payment methods.
    ///
    /// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
    /// To [confirm](https://docs.stripe.com/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redirects: Option<stripe_shared::MandatePixAmountIncludesIof>,
    /// Whether this feature is enabled.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentAutomaticPaymentMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentAutomaticPaymentMethods").finish_non_exhaustive()
    }
}
impl CreateSetupIntentAutomaticPaymentMethods {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { allow_redirects: None, enabled: enabled.into() }
    }
}
/// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-payment_method).
/// value in the SetupIntent.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodData {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodParam>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub affirm: Option<miniserde::json::Value>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub afterpay_clearpay: Option<miniserde::json::Value>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub alipay: Option<miniserde::json::Value>,
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<stripe_shared::CardAllowRedisplay>,
    /// If this is a Alma PaymentMethod, this hash contains details about the Alma payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub alma: Option<miniserde::json::Value>,
    /// If this is a AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreateSetupIntentPaymentMethodDataAuBecsDebit>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreateSetupIntentPaymentMethodDataBacsDebit>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub bancontact: Option<miniserde::json::Value>,
    /// If this is a `billie` PaymentMethod, this hash contains details about the Billie payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub billie: Option<miniserde::json::Value>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetailsInnerParams>,
    /// If this is a `bizum` PaymentMethod, this hash contains details about the Bizum payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub bizum: Option<miniserde::json::Value>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub blik: Option<miniserde::json::Value>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreateSetupIntentPaymentMethodDataBoleto>,
    /// If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub cashapp: Option<miniserde::json::Value>,
    /// If this is a Crypto PaymentMethod, this hash contains details about the Crypto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub crypto: Option<miniserde::json::Value>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub customer_balance: Option<miniserde::json::Value>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreateSetupIntentPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreateSetupIntentPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub giropay: Option<miniserde::json::Value>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub grabpay: Option<miniserde::json::Value>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreateSetupIntentPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub interac_present: Option<miniserde::json::Value>,
    /// If this is a `kakao_pay` PaymentMethod, this hash contains details about the Kakao Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub kakao_pay: Option<miniserde::json::Value>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateSetupIntentPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub konbini: Option<miniserde::json::Value>,
    /// If this is a `kr_card` PaymentMethod, this hash contains details about the Korean Card payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub kr_card: Option<miniserde::json::Value>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method (Link is also known as Onelink in the UK).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub link: Option<miniserde::json::Value>,
    /// If this is a MB WAY PaymentMethod, this hash contains details about the MB WAY payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub mb_way: Option<miniserde::json::Value>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub mobilepay: Option<miniserde::json::Value>,
    /// If this is a `multibanco` PaymentMethod, this hash contains details about the Multibanco payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub multibanco: Option<miniserde::json::Value>,
    /// If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<CreateSetupIntentPaymentMethodDataNaverPay>,
    /// If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<CreateSetupIntentPaymentMethodDataNzBankAccount>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub oxxo: Option<miniserde::json::Value>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreateSetupIntentPaymentMethodDataP24>,
    /// If this is a `pay_by_bank` PaymentMethod, this hash contains details about the PayByBank payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub pay_by_bank: Option<miniserde::json::Value>,
    /// If this is a `payco` PaymentMethod, this hash contains details about the PAYCO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub payco: Option<miniserde::json::Value>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub paynow: Option<miniserde::json::Value>,
    /// If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub paypal: Option<miniserde::json::Value>,
    /// If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto: Option<CreateSetupIntentPaymentMethodDataPayto>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub pix: Option<miniserde::json::Value>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub promptpay: Option<miniserde::json::Value>,
    /// Options to configure Radar.
    /// See [Radar Session](https://docs.stripe.com/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<RadarOptionsWithHiddenOptions>,
    /// If this is a `revolut_pay` PaymentMethod, this hash contains details about the Revolut Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub revolut_pay: Option<miniserde::json::Value>,
    /// If this is a `samsung_pay` PaymentMethod, this hash contains details about the SamsungPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub samsung_pay: Option<miniserde::json::Value>,
    /// If this is a `satispay` PaymentMethod, this hash contains details about the Satispay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub satispay: Option<miniserde::json::Value>,
    /// If this is a Scalapay PaymentMethod, this hash contains details about the Scalapay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub scalapay: Option<miniserde::json::Value>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateSetupIntentPaymentMethodDataSepaDebit>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateSetupIntentPaymentMethodDataSofort>,
    /// If this is a `sunbit` PaymentMethod, this hash contains details about the Sunbit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub sunbit: Option<miniserde::json::Value>,
    /// If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub swish: Option<miniserde::json::Value>,
    /// If this is a TWINT PaymentMethod, this hash contains details about the TWINT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub twint: Option<miniserde::json::Value>,
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateSetupIntentPaymentMethodDataType,
    /// If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi: Option<CreateSetupIntentPaymentMethodDataUpi>,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateSetupIntentPaymentMethodDataUsBankAccount>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub wechat_pay: Option<miniserde::json::Value>,
    /// If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub zip: Option<miniserde::json::Value>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodData").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodData {
    pub fn new(type_: impl Into<stripe_shared::CreateSetupIntentPaymentMethodDataType>) -> Self {
        Self {
            acss_debit: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            allow_redisplay: None,
            alma: None,
            amazon_pay: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            billie: None,
            billing_details: None,
            bizum: None,
            blik: None,
            boleto: None,
            cashapp: None,
            crypto: None,
            customer_balance: None,
            eps: None,
            fpx: None,
            giropay: None,
            grabpay: None,
            ideal: None,
            interac_present: None,
            kakao_pay: None,
            klarna: None,
            konbini: None,
            kr_card: None,
            link: None,
            mb_way: None,
            metadata: None,
            mobilepay: None,
            multibanco: None,
            naver_pay: None,
            nz_bank_account: None,
            oxxo: None,
            p24: None,
            pay_by_bank: None,
            payco: None,
            paynow: None,
            paypal: None,
            payto: None,
            pix: None,
            promptpay: None,
            radar_options: None,
            revolut_pay: None,
            samsung_pay: None,
            satispay: None,
            scalapay: None,
            sepa_debit: None,
            sofort: None,
            sunbit: None,
            swish: None,
            twint: None,
            type_: type_.into(),
            upi: None,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataAuBecsDebit {
    /// The account number for the bank account.
    pub account_number: String,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataAuBecsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataAuBecsDebit").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataAuBecsDebit {
    pub fn new(account_number: impl Into<String>, bsb_number: impl Into<String>) -> Self {
        Self { account_number: account_number.into(), bsb_number: bsb_number.into() }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBacsDebit {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Sort code of the bank account. (e.g., `10-20-30`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataBacsDebit").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataBacsDebit {
    pub fn new() -> Self {
        Self { account_number: None, sort_code: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodDataBacsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBoleto {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers)
    pub tax_id: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataBoleto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataBoleto").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataBoleto {
    pub fn new(tax_id: impl Into<String>) -> Self {
        Self { tax_id: tax_id.into() }
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<stripe_shared::Bank>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataEps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataEps").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataEps {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodDataEps {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<stripe_shared::IssuingCardholderType>,
    /// The customer's bank.
    pub bank: stripe_shared::PaymentMethodFpxBank,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataFpx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataFpx").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataFpx {
    pub fn new(bank: impl Into<stripe_shared::PaymentMethodFpxBank>) -> Self {
        Self { account_holder_type: None, bank: bank.into() }
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    /// Only use this parameter for existing customers.
    /// Don't use it for new customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<stripe_shared::PaymentMethodIdealBank>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataIdeal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataIdeal").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataIdeal {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodDataIdeal {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataKlarna {
    /// Customer's date of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirth>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataKlarna").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataKlarna {
    pub fn new() -> Self {
        Self { dob: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodDataKlarna {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataNaverPay {
    /// Whether to use Naver Pay points or a card to fund this transaction.
    /// If not provided, this defaults to `card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<stripe_shared::Funding>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataNaverPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataNaverPay").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataNaverPay {
    pub fn new() -> Self {
        Self { funding: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodDataNaverPay {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataNzBankAccount {
    /// The name on the bank account.
    /// Only required if the account holder name is different from the name of the authorized signatory collected in the PaymentMethod’s billing details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    /// The account number for the bank account.
    pub account_number: String,
    /// The numeric code for the bank account's bank.
    pub bank_code: String,
    /// The numeric code for the bank account's bank branch.
    pub branch_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// The suffix of the bank account number.
    pub suffix: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataNzBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataNzBankAccount").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataNzBankAccount {
    pub fn new(
        account_number: impl Into<String>,
        bank_code: impl Into<String>,
        branch_code: impl Into<String>,
        suffix: impl Into<String>,
    ) -> Self {
        Self {
            account_holder_name: None,
            account_number: account_number.into(),
            bank_code: bank_code.into(),
            branch_code: branch_code.into(),
            reference: None,
            suffix: suffix.into(),
        }
    }
}
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<stripe_shared::PaymentMethodP24Bank>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataP24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataP24").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataP24 {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodDataP24 {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataPayto {
    /// The account number for the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Bank-State-Branch number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsb_number: Option<String>,
    /// The PayID alias for the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataPayto").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataPayto {
    pub fn new() -> Self {
        Self { account_number: None, bsb_number: None, pay_id: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodDataPayto {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataSepaDebit {
    /// IBAN of the bank account.
    pub iban: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataSepaDebit").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataSepaDebit {
    pub fn new(iban: impl Into<String>) -> Self {
        Self { iban: iban.into() }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: stripe_shared::Country,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataSofort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataSofort").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataSofort {
    pub fn new(country: impl Into<stripe_shared::Country>) -> Self {
        Self { country: country.into() }
    }
}
/// If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataUpi {
    /// Configuration options for setting up an eMandate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodDataUpiMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataUpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataUpi").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataUpi {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodDataUpi {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration options for setting up an eMandate
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataUpiMandateOptions {
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
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataUpiMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataUpiMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataUpiMandateOptions {
    pub fn new() -> Self {
        Self { amount: None, amount_type: None, description: None, end_date: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodDataUpiMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataUsBankAccount {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<stripe_shared::IssuingCardholderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<stripe_shared::PaymentMethodUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodDataUsBankAccount").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodDataUsBankAccount {
    pub fn new() -> Self {
        Self {
            account_holder_type: None,
            account_number: None,
            account_type: None,
            financial_connections_account: None,
            routing_number: None,
        }
    }
}
impl Default for CreateSetupIntentPaymentMethodDataUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Payment method-specific configuration for this SetupIntent.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptions {
    /// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateSetupIntentPaymentMethodOptionsAcssDebit>,
    /// If this is a `amazon_pay` SetupIntent, this sub-hash contains details about the AmazonPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// If this is a `bacs_debit` SetupIntent, this sub-hash contains details about the Bacs Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreateSetupIntentPaymentMethodOptionsBacsDebit>,
    /// If this is a `bizum` SetupIntent, this sub-hash contains details about the Bizum payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub bizum: Option<miniserde::json::Value>,
    /// Configuration for any card setup attempted on this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateSetupIntentPaymentMethodOptionsCard>,
    /// If this is a `card_present` PaymentMethod, this sub-hash contains details about the card-present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub card_present: Option<miniserde::json::Value>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateSetupIntentPaymentMethodOptionsKlarna>,
    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options (Link is also known as Onelink in the UK).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<SetupIntentPaymentMethodOptionsParam>,
    /// If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodOptionsParam>,
    /// If this is a `payto` SetupIntent, this sub-hash contains details about the PayTo payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto: Option<CreateSetupIntentPaymentMethodOptionsPayto>,
    /// If this is a `pix` SetupIntent, this sub-hash contains details about the Pix payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreateSetupIntentPaymentMethodOptionsPix>,
    /// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateSetupIntentPaymentMethodOptionsSepaDebit>,
    /// If this is a `upi` SetupIntent, this sub-hash contains details about the UPI payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi: Option<CreateSetupIntentPaymentMethodOptionsUpi>,
    /// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateSetupIntentPaymentMethodOptionsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptions").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptions {
    pub fn new() -> Self {
        Self {
            acss_debit: None,
            amazon_pay: None,
            bacs_debit: None,
            bizum: None,
            card: None,
            card_present: None,
            klarna: None,
            link: None,
            paypal: None,
            payto: None,
            pix: None,
            sepa_debit: None,
            upi: None,
            us_bank_account: None,
        }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsAcssDebit {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_shared::Currency>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsMandateOptionsParam>,
    /// Bank account verification method. The default value is `automatic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsAcssDebit").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self { currency: None, mandate_options: None, verification_method: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsAcssDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `bacs_debit` SetupIntent, this sub-hash contains details about the Bacs Debit payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsBacsDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<PaymentMethodOptionsMandateOptionsParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsBacsDebit").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsBacsDebit {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsBacsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for any card setup attempted on this SetupIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCard {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<SetupIntentMandateOptionsParam>,
    /// When specified, this parameter signals that a card has been collected
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA. This
    /// parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    /// Selected network to process this SetupIntent on.
    /// Depends on the available networks of the card attached to the SetupIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<stripe_shared::RequestThreeDSecure>,
    /// If 3D Secure authentication was performed with a third-party provider,
    /// the authentication details to use for this setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<CreateSetupIntentPaymentMethodOptionsCardThreeDSecure>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsCard").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self {
            mandate_options: None,
            moto: None,
            network: None,
            request_three_d_secure: None,
            three_d_secure: None,
        }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsCard {
    fn default() -> Self {
        Self::new()
    }
}
/// If 3D Secure authentication was performed with a third-party provider,
/// the authentication details to use for this setup.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCardThreeDSecure {
    /// The `transStatus` returned from the card Issuer’s ACS in the ARes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ares_trans_status: Option<stripe_shared::AresTransStatus>,
    /// The cryptogram, also known as the "authentication value" (AAV, CAVV or
    /// AEVV). This value is 20 bytes, base64-encoded into a 28-character string.
    /// (Most 3D Secure providers will return the base64-encoded version, which
    /// is what you should specify here.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptogram: Option<String>,
    /// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
    /// provider and indicates what degree of authentication was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator: Option<stripe_shared::ElectronicCommerceIndicator>,
    /// Network specific 3DS fields. Network specific arguments require an
    /// explicit card brand choice. The parameter `payment_method_options.card.network``
    /// must be populated accordingly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options: Option<NetworkOptionsParam>,
    /// The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the
    /// AReq sent to the card Issuer's ACS. A string containing 2 digits from 01-99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_challenge_indicator: Option<String>,
    /// For 3D Secure 1, the XID. For 3D Secure 2, the Directory Server
    /// Transaction ID (dsTransID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// The version of 3D Secure that was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<stripe_shared::ThreeDSecureDetailsVersion>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsCardThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsCardThreeDSecure")
            .finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsCardThreeDSecure {
    pub fn new() -> Self {
        Self {
            ares_trans_status: None,
            cryptogram: None,
            electronic_commerce_indicator: None,
            network_options: None,
            requestor_challenge_indicator: None,
            transaction_id: None,
            version: None,
        }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsCardThreeDSecure {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsKlarna {
    /// The currency of the SetupIntent. Three letter ISO currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// On-demand details if setting up a payment method for on-demand payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand: Option<OnDemandParam>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale:
        Option<stripe_shared::CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale>,
    /// Subscription details if setting up or charging a subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<SetupIntentSubscriptionParam>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsKlarna").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsKlarna {
    pub fn new() -> Self {
        Self { currency: None, on_demand: None, preferred_locale: None, subscriptions: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsKlarna {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `payto` SetupIntent, this sub-hash contains details about the PayTo payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsPayto {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsPaytoMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsPayto").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsPayto {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsPayto {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsPaytoMandateOptions {
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
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsPaytoMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsPaytoMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsPaytoMandateOptions {
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
impl Default for CreateSetupIntentPaymentMethodOptionsPaytoMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `pix` SetupIntent, this sub-hash contains details about the Pix payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsPix {
    /// Additional fields for mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsPixMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsPix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsPix").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsPix {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsPix {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for mandate creation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsPixMandateOptions {
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
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsPixMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsPixMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsPixMandateOptions {
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
impl Default for CreateSetupIntentPaymentMethodOptionsPixMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsSepaDebit").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsSepaDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    /// Prefix used to generate the Mandate reference.
    /// Must be at most 12 characters long.
    /// Must consist of only uppercase letters, numbers, spaces, or the following special characters: '/', '_', '-', '&', '.'.
    /// Cannot begin with 'STRIPE'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    pub fn new() -> Self {
        Self { reference_prefix: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `upi` SetupIntent, this sub-hash contains details about the UPI payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUpi {
    /// Configuration options for setting up an eMandate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsUpiMandateOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::SetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsUpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsUpi").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsUpi {
    pub fn new() -> Self {
        Self { mandate_options: None, setup_future_usage: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsUpi {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration options for setting up an eMandate
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUpiMandateOptions {
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
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsUpiMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsUpiMandateOptions")
            .finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsUpiMandateOptions {
    pub fn new() -> Self {
        Self { amount: None, amount_type: None, description: None, end_date: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsUpiMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<LinkedAccountOptionsParam>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<MandateOptionsParam>,
    /// Additional fields for network related functions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<NetworksOptionsParam>,
    /// Bank account verification method. The default value is `automatic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentPaymentMethodOptionsUsBankAccount").finish_non_exhaustive()
    }
}
impl CreateSetupIntentPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self {
            financial_connections: None,
            mandate_options: None,
            networks: None,
            verification_method: None,
        }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// If you populate this hash, this SetupIntent generates a `single_use` mandate after successful completion.
///
/// Single-use mandates are only valid for the following payment methods: `acss_debit`, `alipay`, `au_becs_debit`, `bacs_debit`, `bancontact`, `boleto`, `ideal`, `link`, `sepa_debit`, and `us_bank_account`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntentSingleUse {
    /// Amount the customer is granting permission to collect later.
    /// A positive integer representing how much to charge in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://docs.stripe.com/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentSingleUse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntentSingleUse").finish_non_exhaustive()
    }
}
impl CreateSetupIntentSingleUse {
    pub fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self { amount: amount.into(), currency: currency.into() }
    }
}
/// Creates a SetupIntent object.
///
/// After you create the SetupIntent, attach a payment method and [confirm](https://stripe.com/docs/api/setup_intents/confirm).
/// it to collect any required permissions to charge the payment method later.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateSetupIntent {
    inner: CreateSetupIntentBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateSetupIntent").finish_non_exhaustive()
    }
}
impl CreateSetupIntent {
    /// Construct a new `CreateSetupIntent`.
    pub fn new() -> Self {
        Self { inner: CreateSetupIntentBuilder::new() }
    }
    /// The list of payment method types to allow for this SetupIntent.
    /// Stripe will only use methods in this list when determining the payment methods to offer.
    /// A list of valid payment method types can be found [here](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type).
    pub fn allowed_payment_method_types(
        mut self,
        allowed_payment_method_types: impl Into<Vec<stripe_shared::AllowedPaymentMethodTypes>>,
    ) -> Self {
        self.inner.allowed_payment_method_types = Some(allowed_payment_method_types.into());
        self
    }
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    pub fn attach_to_self(mut self, attach_to_self: impl Into<bool>) -> Self {
        self.inner.attach_to_self = Some(attach_to_self.into());
        self
    }
    /// When you enable this parameter, this SetupIntent accepts payment methods that you enable in the Dashboard and that are compatible with its other parameters.
    pub fn automatic_payment_methods(
        mut self,
        automatic_payment_methods: impl Into<CreateSetupIntentAutomaticPaymentMethods>,
    ) -> Self {
        self.inner.automatic_payment_methods = Some(automatic_payment_methods.into());
        self
    }
    /// Set to `true` to attempt to confirm this SetupIntent immediately.
    /// This parameter defaults to `false`.
    /// If a card is the attached payment method, you can provide a `return_url` in case further authentication is necessary.
    pub fn confirm(mut self, confirm: impl Into<bool>) -> Self {
        self.inner.confirm = Some(confirm.into());
        self
    }
    /// ID of the ConfirmationToken used to confirm this SetupIntent.
    ///
    /// If the provided ConfirmationToken contains properties that are also being provided in this request, such as `payment_method`, then the values in this request will take precedence.
    pub fn confirmation_token(mut self, confirmation_token: impl Into<String>) -> Self {
        self.inner.confirmation_token = Some(confirmation_token.into());
        self
    }
    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// ID of the Account this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Account on successful setup.
    /// Payment methods attached to other Accounts cannot be used with this SetupIntent.
    pub fn customer_account(mut self, customer_account: impl Into<String>) -> Self {
        self.inner.customer_account = Some(customer_account.into());
        self
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// The list of payment method types to exclude from use with this SetupIntent.
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
    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    pub fn flow_directions(
        mut self,
        flow_directions: impl Into<Vec<stripe_shared::FlowDirections>>,
    ) -> Self {
        self.inner.flow_directions = Some(flow_directions.into());
        self
    }
    /// This hash contains details about the mandate to create.
    /// This parameter can only be used with [`confirm=true`](https://docs.stripe.com/api/setup_intents/create#create_setup_intent-confirm).
    pub fn mandate_data(mut self, mandate_data: impl Into<SecretKeyParam>) -> Self {
        self.inner.mandate_data = Some(mandate_data.into());
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
    /// The Stripe account ID created for this SetupIntent.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    pub fn payment_method(mut self, payment_method: impl Into<String>) -> Self {
        self.inner.payment_method = Some(payment_method.into());
        self
    }
    /// The ID of the [payment method configuration](https://docs.stripe.com/api/payment_method_configurations) to use with this SetupIntent.
    pub fn payment_method_configuration(
        mut self,
        payment_method_configuration: impl Into<String>,
    ) -> Self {
        self.inner.payment_method_configuration = Some(payment_method_configuration.into());
        self
    }
    /// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-payment_method).
    /// value in the SetupIntent.
    pub fn payment_method_data(
        mut self,
        payment_method_data: impl Into<CreateSetupIntentPaymentMethodData>,
    ) -> Self {
        self.inner.payment_method_data = Some(payment_method_data.into());
        self
    }
    /// Payment method-specific configuration for this SetupIntent.
    pub fn payment_method_options(
        mut self,
        payment_method_options: impl Into<CreateSetupIntentPaymentMethodOptions>,
    ) -> Self {
        self.inner.payment_method_options = Some(payment_method_options.into());
        self
    }
    /// The list of payment method types (for example, card) that this SetupIntent can use.
    /// If you don't provide this, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    /// A list of valid payment method types can be found [here](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type).
    pub fn payment_method_types(mut self, payment_method_types: impl Into<Vec<String>>) -> Self {
        self.inner.payment_method_types = Some(payment_method_types.into());
        self
    }
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    /// To redirect to a mobile application, you can alternatively supply an application URI scheme.
    /// This parameter can only be used with [`confirm=true`](https://docs.stripe.com/api/setup_intents/create#create_setup_intent-confirm).
    pub fn return_url(mut self, return_url: impl Into<String>) -> Self {
        self.inner.return_url = Some(return_url.into());
        self
    }
    /// If you populate this hash, this SetupIntent generates a `single_use` mandate after successful completion.
    ///
    /// Single-use mandates are only valid for the following payment methods: `acss_debit`, `alipay`, `au_becs_debit`, `bacs_debit`, `bancontact`, `boleto`, `ideal`, `link`, `sepa_debit`, and `us_bank_account`.
    pub fn single_use(mut self, single_use: impl Into<CreateSetupIntentSingleUse>) -> Self {
        self.inner.single_use = Some(single_use.into());
        self
    }
    /// Indicates how the payment method is intended to be used in the future.
    /// If not provided, this value defaults to `off_session`.
    pub fn usage(mut self, usage: impl Into<stripe_shared::CreateSetupIntentUsage>) -> Self {
        self.inner.usage = Some(usage.into());
        self
    }
    /// Set to `true` when confirming server-side and using Stripe.js, iOS, or Android client-side SDKs to handle the next actions.
    pub fn use_stripe_sdk(mut self, use_stripe_sdk: impl Into<bool>) -> Self {
        self.inner.use_stripe_sdk = Some(use_stripe_sdk.into());
        self
    }
}
impl Default for CreateSetupIntent {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateSetupIntent {
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

impl StripeRequest for CreateSetupIntent {
    type Output = stripe_shared::SetupIntent;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/setup_intents").form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateSetupIntentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_payment_method_types: Option<Vec<stripe_shared::AllowedPaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attach_to_self: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_payment_method_types: Option<Vec<stripe_shared::ExcludedPaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_directions: Option<Vec<stripe_shared::FlowDirections>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_data: Option<UpdateSetupIntentPaymentMethodData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_options: Option<UpdateSetupIntentPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_types: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentBuilder").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentBuilder {
    fn new() -> Self {
        Self {
            allowed_payment_method_types: None,
            attach_to_self: None,
            customer: None,
            customer_account: None,
            description: None,
            excluded_payment_method_types: None,
            expand: None,
            flow_directions: None,
            metadata: None,
            payment_method: None,
            payment_method_configuration: None,
            payment_method_data: None,
            payment_method_options: None,
            payment_method_types: None,
        }
    }
}
/// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-payment_method).
/// value in the SetupIntent.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodData {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodParam>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub affirm: Option<miniserde::json::Value>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub afterpay_clearpay: Option<miniserde::json::Value>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub alipay: Option<miniserde::json::Value>,
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<stripe_shared::CardAllowRedisplay>,
    /// If this is a Alma PaymentMethod, this hash contains details about the Alma payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub alma: Option<miniserde::json::Value>,
    /// If this is a AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<UpdateSetupIntentPaymentMethodDataAuBecsDebit>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<UpdateSetupIntentPaymentMethodDataBacsDebit>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub bancontact: Option<miniserde::json::Value>,
    /// If this is a `billie` PaymentMethod, this hash contains details about the Billie payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub billie: Option<miniserde::json::Value>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetailsInnerParams>,
    /// If this is a `bizum` PaymentMethod, this hash contains details about the Bizum payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub bizum: Option<miniserde::json::Value>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub blik: Option<miniserde::json::Value>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<UpdateSetupIntentPaymentMethodDataBoleto>,
    /// If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub cashapp: Option<miniserde::json::Value>,
    /// If this is a Crypto PaymentMethod, this hash contains details about the Crypto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub crypto: Option<miniserde::json::Value>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub customer_balance: Option<miniserde::json::Value>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<UpdateSetupIntentPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<UpdateSetupIntentPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub giropay: Option<miniserde::json::Value>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub grabpay: Option<miniserde::json::Value>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<UpdateSetupIntentPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub interac_present: Option<miniserde::json::Value>,
    /// If this is a `kakao_pay` PaymentMethod, this hash contains details about the Kakao Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub kakao_pay: Option<miniserde::json::Value>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<UpdateSetupIntentPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub konbini: Option<miniserde::json::Value>,
    /// If this is a `kr_card` PaymentMethod, this hash contains details about the Korean Card payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub kr_card: Option<miniserde::json::Value>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method (Link is also known as Onelink in the UK).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub link: Option<miniserde::json::Value>,
    /// If this is a MB WAY PaymentMethod, this hash contains details about the MB WAY payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub mb_way: Option<miniserde::json::Value>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub mobilepay: Option<miniserde::json::Value>,
    /// If this is a `multibanco` PaymentMethod, this hash contains details about the Multibanco payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub multibanco: Option<miniserde::json::Value>,
    /// If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<UpdateSetupIntentPaymentMethodDataNaverPay>,
    /// If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<UpdateSetupIntentPaymentMethodDataNzBankAccount>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub oxxo: Option<miniserde::json::Value>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<UpdateSetupIntentPaymentMethodDataP24>,
    /// If this is a `pay_by_bank` PaymentMethod, this hash contains details about the PayByBank payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub pay_by_bank: Option<miniserde::json::Value>,
    /// If this is a `payco` PaymentMethod, this hash contains details about the PAYCO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub payco: Option<miniserde::json::Value>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub paynow: Option<miniserde::json::Value>,
    /// If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub paypal: Option<miniserde::json::Value>,
    /// If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto: Option<UpdateSetupIntentPaymentMethodDataPayto>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub pix: Option<miniserde::json::Value>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub promptpay: Option<miniserde::json::Value>,
    /// Options to configure Radar.
    /// See [Radar Session](https://docs.stripe.com/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<RadarOptionsWithHiddenOptions>,
    /// If this is a `revolut_pay` PaymentMethod, this hash contains details about the Revolut Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub revolut_pay: Option<miniserde::json::Value>,
    /// If this is a `samsung_pay` PaymentMethod, this hash contains details about the SamsungPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub samsung_pay: Option<miniserde::json::Value>,
    /// If this is a `satispay` PaymentMethod, this hash contains details about the Satispay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub satispay: Option<miniserde::json::Value>,
    /// If this is a Scalapay PaymentMethod, this hash contains details about the Scalapay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub scalapay: Option<miniserde::json::Value>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdateSetupIntentPaymentMethodDataSepaDebit>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<UpdateSetupIntentPaymentMethodDataSofort>,
    /// If this is a `sunbit` PaymentMethod, this hash contains details about the Sunbit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub sunbit: Option<miniserde::json::Value>,
    /// If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub swish: Option<miniserde::json::Value>,
    /// If this is a TWINT PaymentMethod, this hash contains details about the TWINT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub twint: Option<miniserde::json::Value>,
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateSetupIntentPaymentMethodDataType,
    /// If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi: Option<UpdateSetupIntentPaymentMethodDataUpi>,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdateSetupIntentPaymentMethodDataUsBankAccount>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub wechat_pay: Option<miniserde::json::Value>,
    /// If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub zip: Option<miniserde::json::Value>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodData").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodData {
    pub fn new(type_: impl Into<stripe_shared::CreateSetupIntentPaymentMethodDataType>) -> Self {
        Self {
            acss_debit: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            allow_redisplay: None,
            alma: None,
            amazon_pay: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            billie: None,
            billing_details: None,
            bizum: None,
            blik: None,
            boleto: None,
            cashapp: None,
            crypto: None,
            customer_balance: None,
            eps: None,
            fpx: None,
            giropay: None,
            grabpay: None,
            ideal: None,
            interac_present: None,
            kakao_pay: None,
            klarna: None,
            konbini: None,
            kr_card: None,
            link: None,
            mb_way: None,
            metadata: None,
            mobilepay: None,
            multibanco: None,
            naver_pay: None,
            nz_bank_account: None,
            oxxo: None,
            p24: None,
            pay_by_bank: None,
            payco: None,
            paynow: None,
            paypal: None,
            payto: None,
            pix: None,
            promptpay: None,
            radar_options: None,
            revolut_pay: None,
            samsung_pay: None,
            satispay: None,
            scalapay: None,
            sepa_debit: None,
            sofort: None,
            sunbit: None,
            swish: None,
            twint: None,
            type_: type_.into(),
            upi: None,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataAuBecsDebit {
    /// The account number for the bank account.
    pub account_number: String,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataAuBecsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataAuBecsDebit").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataAuBecsDebit {
    pub fn new(account_number: impl Into<String>, bsb_number: impl Into<String>) -> Self {
        Self { account_number: account_number.into(), bsb_number: bsb_number.into() }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBacsDebit {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Sort code of the bank account. (e.g., `10-20-30`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataBacsDebit").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataBacsDebit {
    pub fn new() -> Self {
        Self { account_number: None, sort_code: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodDataBacsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBoleto {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers)
    pub tax_id: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataBoleto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataBoleto").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataBoleto {
    pub fn new(tax_id: impl Into<String>) -> Self {
        Self { tax_id: tax_id.into() }
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<stripe_shared::Bank>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataEps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataEps").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataEps {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodDataEps {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<stripe_shared::IssuingCardholderType>,
    /// The customer's bank.
    pub bank: stripe_shared::PaymentMethodFpxBank,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataFpx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataFpx").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataFpx {
    pub fn new(bank: impl Into<stripe_shared::PaymentMethodFpxBank>) -> Self {
        Self { account_holder_type: None, bank: bank.into() }
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    /// Only use this parameter for existing customers.
    /// Don't use it for new customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<stripe_shared::PaymentMethodIdealBank>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataIdeal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataIdeal").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataIdeal {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodDataIdeal {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataKlarna {
    /// Customer's date of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirth>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataKlarna").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataKlarna {
    pub fn new() -> Self {
        Self { dob: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodDataKlarna {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataNaverPay {
    /// Whether to use Naver Pay points or a card to fund this transaction.
    /// If not provided, this defaults to `card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<stripe_shared::Funding>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataNaverPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataNaverPay").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataNaverPay {
    pub fn new() -> Self {
        Self { funding: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodDataNaverPay {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataNzBankAccount {
    /// The name on the bank account.
    /// Only required if the account holder name is different from the name of the authorized signatory collected in the PaymentMethod’s billing details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    /// The account number for the bank account.
    pub account_number: String,
    /// The numeric code for the bank account's bank.
    pub bank_code: String,
    /// The numeric code for the bank account's bank branch.
    pub branch_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// The suffix of the bank account number.
    pub suffix: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataNzBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataNzBankAccount").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataNzBankAccount {
    pub fn new(
        account_number: impl Into<String>,
        bank_code: impl Into<String>,
        branch_code: impl Into<String>,
        suffix: impl Into<String>,
    ) -> Self {
        Self {
            account_holder_name: None,
            account_number: account_number.into(),
            bank_code: bank_code.into(),
            branch_code: branch_code.into(),
            reference: None,
            suffix: suffix.into(),
        }
    }
}
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<stripe_shared::PaymentMethodP24Bank>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataP24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataP24").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataP24 {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodDataP24 {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataPayto {
    /// The account number for the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Bank-State-Branch number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsb_number: Option<String>,
    /// The PayID alias for the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataPayto").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataPayto {
    pub fn new() -> Self {
        Self { account_number: None, bsb_number: None, pay_id: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodDataPayto {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataSepaDebit {
    /// IBAN of the bank account.
    pub iban: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataSepaDebit").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataSepaDebit {
    pub fn new(iban: impl Into<String>) -> Self {
        Self { iban: iban.into() }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: stripe_shared::Country,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataSofort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataSofort").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataSofort {
    pub fn new(country: impl Into<stripe_shared::Country>) -> Self {
        Self { country: country.into() }
    }
}
/// If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataUpi {
    /// Configuration options for setting up an eMandate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodDataUpiMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataUpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataUpi").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataUpi {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodDataUpi {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration options for setting up an eMandate
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataUpiMandateOptions {
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
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataUpiMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataUpiMandateOptions")
            .finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataUpiMandateOptions {
    pub fn new() -> Self {
        Self { amount: None, amount_type: None, description: None, end_date: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodDataUpiMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataUsBankAccount {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<stripe_shared::IssuingCardholderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<stripe_shared::PaymentMethodUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodDataUsBankAccount").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodDataUsBankAccount {
    pub fn new() -> Self {
        Self {
            account_holder_type: None,
            account_number: None,
            account_type: None,
            financial_connections_account: None,
            routing_number: None,
        }
    }
}
impl Default for UpdateSetupIntentPaymentMethodDataUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Payment method-specific configuration for this SetupIntent.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptions {
    /// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdateSetupIntentPaymentMethodOptionsAcssDebit>,
    /// If this is a `amazon_pay` SetupIntent, this sub-hash contains details about the AmazonPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// If this is a `bacs_debit` SetupIntent, this sub-hash contains details about the Bacs Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<UpdateSetupIntentPaymentMethodOptionsBacsDebit>,
    /// If this is a `bizum` SetupIntent, this sub-hash contains details about the Bizum payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub bizum: Option<miniserde::json::Value>,
    /// Configuration for any card setup attempted on this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdateSetupIntentPaymentMethodOptionsCard>,
    /// If this is a `card_present` PaymentMethod, this sub-hash contains details about the card-present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub card_present: Option<miniserde::json::Value>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<UpdateSetupIntentPaymentMethodOptionsKlarna>,
    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options (Link is also known as Onelink in the UK).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<SetupIntentPaymentMethodOptionsParam>,
    /// If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodOptionsParam>,
    /// If this is a `payto` SetupIntent, this sub-hash contains details about the PayTo payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto: Option<UpdateSetupIntentPaymentMethodOptionsPayto>,
    /// If this is a `pix` SetupIntent, this sub-hash contains details about the Pix payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<UpdateSetupIntentPaymentMethodOptionsPix>,
    /// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdateSetupIntentPaymentMethodOptionsSepaDebit>,
    /// If this is a `upi` SetupIntent, this sub-hash contains details about the UPI payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi: Option<UpdateSetupIntentPaymentMethodOptionsUpi>,
    /// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptions").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptions {
    pub fn new() -> Self {
        Self {
            acss_debit: None,
            amazon_pay: None,
            bacs_debit: None,
            bizum: None,
            card: None,
            card_present: None,
            klarna: None,
            link: None,
            paypal: None,
            payto: None,
            pix: None,
            sepa_debit: None,
            upi: None,
            us_bank_account: None,
        }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsAcssDebit {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_shared::Currency>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsMandateOptionsParam>,
    /// Bank account verification method. The default value is `automatic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsAcssDebit").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self { currency: None, mandate_options: None, verification_method: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsAcssDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `bacs_debit` SetupIntent, this sub-hash contains details about the Bacs Debit payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsBacsDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<PaymentMethodOptionsMandateOptionsParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsBacsDebit").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsBacsDebit {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsBacsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for any card setup attempted on this SetupIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCard {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<SetupIntentMandateOptionsParam>,
    /// When specified, this parameter signals that a card has been collected
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA. This
    /// parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    /// Selected network to process this SetupIntent on.
    /// Depends on the available networks of the card attached to the SetupIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<stripe_shared::RequestThreeDSecure>,
    /// If 3D Secure authentication was performed with a third-party provider,
    /// the authentication details to use for this setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsCard").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self {
            mandate_options: None,
            moto: None,
            network: None,
            request_three_d_secure: None,
            three_d_secure: None,
        }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsCard {
    fn default() -> Self {
        Self::new()
    }
}
/// If 3D Secure authentication was performed with a third-party provider,
/// the authentication details to use for this setup.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure {
    /// The `transStatus` returned from the card Issuer’s ACS in the ARes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ares_trans_status: Option<stripe_shared::AresTransStatus>,
    /// The cryptogram, also known as the "authentication value" (AAV, CAVV or
    /// AEVV). This value is 20 bytes, base64-encoded into a 28-character string.
    /// (Most 3D Secure providers will return the base64-encoded version, which
    /// is what you should specify here.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptogram: Option<String>,
    /// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
    /// provider and indicates what degree of authentication was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator: Option<stripe_shared::ElectronicCommerceIndicator>,
    /// Network specific 3DS fields. Network specific arguments require an
    /// explicit card brand choice. The parameter `payment_method_options.card.network``
    /// must be populated accordingly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options: Option<NetworkOptionsParam>,
    /// The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the
    /// AReq sent to the card Issuer's ACS. A string containing 2 digits from 01-99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_challenge_indicator: Option<String>,
    /// For 3D Secure 1, the XID. For 3D Secure 2, the Directory Server
    /// Transaction ID (dsTransID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// The version of 3D Secure that was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<stripe_shared::ThreeDSecureDetailsVersion>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure")
            .finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure {
    pub fn new() -> Self {
        Self {
            ares_trans_status: None,
            cryptogram: None,
            electronic_commerce_indicator: None,
            network_options: None,
            requestor_challenge_indicator: None,
            transaction_id: None,
            version: None,
        }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsKlarna {
    /// The currency of the SetupIntent. Three letter ISO currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// On-demand details if setting up a payment method for on-demand payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand: Option<OnDemandParam>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale:
        Option<stripe_shared::CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale>,
    /// Subscription details if setting up or charging a subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<SetupIntentSubscriptionParam>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsKlarna").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsKlarna {
    pub fn new() -> Self {
        Self { currency: None, on_demand: None, preferred_locale: None, subscriptions: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsKlarna {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `payto` SetupIntent, this sub-hash contains details about the PayTo payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsPayto {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsPaytoMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsPayto").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsPayto {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsPayto {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsPaytoMandateOptions {
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
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsPaytoMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsPaytoMandateOptions")
            .finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsPaytoMandateOptions {
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
impl Default for UpdateSetupIntentPaymentMethodOptionsPaytoMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `pix` SetupIntent, this sub-hash contains details about the Pix payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsPix {
    /// Additional fields for mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsPixMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsPix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsPix").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsPix {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsPix {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for mandate creation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsPixMandateOptions {
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
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsPixMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsPixMandateOptions")
            .finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsPixMandateOptions {
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
impl Default for UpdateSetupIntentPaymentMethodOptionsPixMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsSepaDebit").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsSepaDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    /// Prefix used to generate the Mandate reference.
    /// Must be at most 12 characters long.
    /// Must consist of only uppercase letters, numbers, spaces, or the following special characters: '/', '_', '-', '&', '.'.
    /// Cannot begin with 'STRIPE'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions")
            .finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    pub fn new() -> Self {
        Self { reference_prefix: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `upi` SetupIntent, this sub-hash contains details about the UPI payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUpi {
    /// Configuration options for setting up an eMandate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsUpiMandateOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::SetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsUpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsUpi").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsUpi {
    pub fn new() -> Self {
        Self { mandate_options: None, setup_future_usage: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsUpi {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration options for setting up an eMandate
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUpiMandateOptions {
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
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsUpiMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsUpiMandateOptions")
            .finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsUpiMandateOptions {
    pub fn new() -> Self {
        Self { amount: None, amount_type: None, description: None, end_date: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsUpiMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<LinkedAccountOptionsParam>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<MandateOptionsParam>,
    /// Additional fields for network related functions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<NetworksOptionsParam>,
    /// Bank account verification method. The default value is `automatic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntentPaymentMethodOptionsUsBankAccount").finish_non_exhaustive()
    }
}
impl UpdateSetupIntentPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self {
            financial_connections: None,
            mandate_options: None,
            networks: None,
            verification_method: None,
        }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates a SetupIntent object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateSetupIntent {
    inner: UpdateSetupIntentBuilder,
    intent: stripe_shared::SetupIntentId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateSetupIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateSetupIntent").finish_non_exhaustive()
    }
}
impl UpdateSetupIntent {
    /// Construct a new `UpdateSetupIntent`.
    pub fn new(intent: impl Into<stripe_shared::SetupIntentId>) -> Self {
        Self { intent: intent.into(), inner: UpdateSetupIntentBuilder::new() }
    }
    /// The list of payment method types to allow for this SetupIntent.
    /// Stripe will only use methods in this list when determining the payment methods to offer.
    /// A list of valid payment method types can be found [here](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type).
    pub fn allowed_payment_method_types(
        mut self,
        allowed_payment_method_types: impl Into<Vec<stripe_shared::AllowedPaymentMethodTypes>>,
    ) -> Self {
        self.inner.allowed_payment_method_types = Some(allowed_payment_method_types.into());
        self
    }
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    pub fn attach_to_self(mut self, attach_to_self: impl Into<bool>) -> Self {
        self.inner.attach_to_self = Some(attach_to_self.into());
        self
    }
    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// ID of the Account this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Account on successful setup.
    /// Payment methods attached to other Accounts cannot be used with this SetupIntent.
    pub fn customer_account(mut self, customer_account: impl Into<String>) -> Self {
        self.inner.customer_account = Some(customer_account.into());
        self
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// The list of payment method types to exclude from use with this SetupIntent.
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
    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    pub fn flow_directions(
        mut self,
        flow_directions: impl Into<Vec<stripe_shared::FlowDirections>>,
    ) -> Self {
        self.inner.flow_directions = Some(flow_directions.into());
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
    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    /// To unset this field to null, pass in an empty string.
    pub fn payment_method(mut self, payment_method: impl Into<String>) -> Self {
        self.inner.payment_method = Some(payment_method.into());
        self
    }
    /// The ID of the [payment method configuration](https://docs.stripe.com/api/payment_method_configurations) to use with this SetupIntent.
    pub fn payment_method_configuration(
        mut self,
        payment_method_configuration: impl Into<String>,
    ) -> Self {
        self.inner.payment_method_configuration = Some(payment_method_configuration.into());
        self
    }
    /// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-payment_method).
    /// value in the SetupIntent.
    pub fn payment_method_data(
        mut self,
        payment_method_data: impl Into<UpdateSetupIntentPaymentMethodData>,
    ) -> Self {
        self.inner.payment_method_data = Some(payment_method_data.into());
        self
    }
    /// Payment method-specific configuration for this SetupIntent.
    pub fn payment_method_options(
        mut self,
        payment_method_options: impl Into<UpdateSetupIntentPaymentMethodOptions>,
    ) -> Self {
        self.inner.payment_method_options = Some(payment_method_options.into());
        self
    }
    /// The list of payment method types (for example, card) that this SetupIntent can set up.
    /// If you don't provide this, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    /// A list of valid payment method types can be found [here](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type).
    pub fn payment_method_types(mut self, payment_method_types: impl Into<Vec<String>>) -> Self {
        self.inner.payment_method_types = Some(payment_method_types.into());
        self
    }
}
impl UpdateSetupIntent {
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

impl StripeRequest for UpdateSetupIntent {
    type Output = stripe_shared::SetupIntent;

    fn build(&self) -> RequestBuilder {
        let intent = &self.intent;
        RequestBuilder::new(StripeMethod::Post, format!("/setup_intents/{intent}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CancelSetupIntentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    cancellation_reason: Option<stripe_shared::CancellationReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancelSetupIntentBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CancelSetupIntentBuilder").finish_non_exhaustive()
    }
}
impl CancelSetupIntentBuilder {
    fn new() -> Self {
        Self { cancellation_reason: None, expand: None }
    }
}
/// You can cancel a SetupIntent object when it’s in one of these statuses: `requires_payment_method`, `requires_confirmation`, or `requires_action`.
///
///
/// After you cancel it, setup is abandoned and any operations on the SetupIntent fail with an error.
/// You can’t cancel the SetupIntent for a Checkout Session.
/// [Expire the Checkout Session](https://stripe.com/docs/api/checkout/sessions/expire) instead.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CancelSetupIntent {
    inner: CancelSetupIntentBuilder,
    intent: stripe_shared::SetupIntentId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancelSetupIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CancelSetupIntent").finish_non_exhaustive()
    }
}
impl CancelSetupIntent {
    /// Construct a new `CancelSetupIntent`.
    pub fn new(intent: impl Into<stripe_shared::SetupIntentId>) -> Self {
        Self { intent: intent.into(), inner: CancelSetupIntentBuilder::new() }
    }
    /// Reason for canceling this SetupIntent.
    /// Possible values are: `abandoned`, `requested_by_customer`, or `duplicate`.
    pub fn cancellation_reason(
        mut self,
        cancellation_reason: impl Into<stripe_shared::CancellationReason>,
    ) -> Self {
        self.inner.cancellation_reason = Some(cancellation_reason.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CancelSetupIntent {
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

impl StripeRequest for CancelSetupIntent {
    type Output = stripe_shared::SetupIntent;

    fn build(&self) -> RequestBuilder {
        let intent = &self.intent;
        RequestBuilder::new(StripeMethod::Post, format!("/setup_intents/{intent}/cancel"))
            .form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ConfirmSetupIntentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_payment_method_types: Option<Vec<stripe_shared::AllowedPaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirmation_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mandate_data: Option<ConfirmSetupIntentMandateData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_data: Option<ConfirmSetupIntentPaymentMethodData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_options: Option<ConfirmSetupIntentPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_stripe_sdk: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentBuilder").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentBuilder {
    fn new() -> Self {
        Self {
            allowed_payment_method_types: None,
            confirmation_token: None,
            expand: None,
            mandate_data: None,
            payment_method: None,
            payment_method_data: None,
            payment_method_options: None,
            return_url: None,
            use_stripe_sdk: None,
        }
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentMandateData {
    #[serde(untagged)]
    SecretKeyParam(SecretKeyParam),
    #[serde(untagged)]
    ClientKeyParam(ConfirmSetupIntentClientKeyParam),
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentMandateData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentMandateData").finish_non_exhaustive()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentClientKeyParam {
    /// This hash contains details about the customer acceptance of the Mandate.
    pub customer_acceptance: ConfirmSetupIntentClientKeyParamCustomerAcceptance,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentClientKeyParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentClientKeyParam").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentClientKeyParam {
    pub fn new(
        customer_acceptance: impl Into<ConfirmSetupIntentClientKeyParamCustomerAcceptance>,
    ) -> Self {
        Self { customer_acceptance: customer_acceptance.into() }
    }
}
/// This hash contains details about the customer acceptance of the Mandate.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentClientKeyParamCustomerAcceptance {
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    pub online: ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline,
    /// The type of customer acceptance information included with the Mandate.
    #[serde(rename = "type")]
    pub type_: stripe_shared::ConfirmSetupIntentClientKeyParamCustomerAcceptanceType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentClientKeyParamCustomerAcceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentClientKeyParamCustomerAcceptance").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentClientKeyParamCustomerAcceptance {
    pub fn new(
        online: impl Into<ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline>,
        type_: impl Into<stripe_shared::ConfirmSetupIntentClientKeyParamCustomerAcceptanceType>,
    ) -> Self {
        Self { online: online.into(), type_: type_.into() }
    }
}
/// If this is a Mandate accepted online, this hash contains details about the online acceptance.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline {
    /// The IP address from which the Mandate was accepted by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline")
            .finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline {
    pub fn new() -> Self {
        Self { ip_address: None, user_agent: None }
    }
}
impl Default for ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline {
    fn default() -> Self {
        Self::new()
    }
}
/// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-payment_method).
/// value in the SetupIntent.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodData {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodParam>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub affirm: Option<miniserde::json::Value>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub afterpay_clearpay: Option<miniserde::json::Value>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub alipay: Option<miniserde::json::Value>,
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<stripe_shared::CardAllowRedisplay>,
    /// If this is a Alma PaymentMethod, this hash contains details about the Alma payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub alma: Option<miniserde::json::Value>,
    /// If this is a AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<ConfirmSetupIntentPaymentMethodDataAuBecsDebit>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<ConfirmSetupIntentPaymentMethodDataBacsDebit>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub bancontact: Option<miniserde::json::Value>,
    /// If this is a `billie` PaymentMethod, this hash contains details about the Billie payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub billie: Option<miniserde::json::Value>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetailsInnerParams>,
    /// If this is a `bizum` PaymentMethod, this hash contains details about the Bizum payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub bizum: Option<miniserde::json::Value>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub blik: Option<miniserde::json::Value>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<ConfirmSetupIntentPaymentMethodDataBoleto>,
    /// If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub cashapp: Option<miniserde::json::Value>,
    /// If this is a Crypto PaymentMethod, this hash contains details about the Crypto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub crypto: Option<miniserde::json::Value>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub customer_balance: Option<miniserde::json::Value>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<ConfirmSetupIntentPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<ConfirmSetupIntentPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub giropay: Option<miniserde::json::Value>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub grabpay: Option<miniserde::json::Value>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<ConfirmSetupIntentPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub interac_present: Option<miniserde::json::Value>,
    /// If this is a `kakao_pay` PaymentMethod, this hash contains details about the Kakao Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub kakao_pay: Option<miniserde::json::Value>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<ConfirmSetupIntentPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub konbini: Option<miniserde::json::Value>,
    /// If this is a `kr_card` PaymentMethod, this hash contains details about the Korean Card payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub kr_card: Option<miniserde::json::Value>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method (Link is also known as Onelink in the UK).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub link: Option<miniserde::json::Value>,
    /// If this is a MB WAY PaymentMethod, this hash contains details about the MB WAY payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub mb_way: Option<miniserde::json::Value>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub mobilepay: Option<miniserde::json::Value>,
    /// If this is a `multibanco` PaymentMethod, this hash contains details about the Multibanco payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub multibanco: Option<miniserde::json::Value>,
    /// If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<ConfirmSetupIntentPaymentMethodDataNaverPay>,
    /// If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<ConfirmSetupIntentPaymentMethodDataNzBankAccount>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub oxxo: Option<miniserde::json::Value>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<ConfirmSetupIntentPaymentMethodDataP24>,
    /// If this is a `pay_by_bank` PaymentMethod, this hash contains details about the PayByBank payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub pay_by_bank: Option<miniserde::json::Value>,
    /// If this is a `payco` PaymentMethod, this hash contains details about the PAYCO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub payco: Option<miniserde::json::Value>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub paynow: Option<miniserde::json::Value>,
    /// If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub paypal: Option<miniserde::json::Value>,
    /// If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto: Option<ConfirmSetupIntentPaymentMethodDataPayto>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub pix: Option<miniserde::json::Value>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub promptpay: Option<miniserde::json::Value>,
    /// Options to configure Radar.
    /// See [Radar Session](https://docs.stripe.com/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<RadarOptionsWithHiddenOptions>,
    /// If this is a `revolut_pay` PaymentMethod, this hash contains details about the Revolut Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub revolut_pay: Option<miniserde::json::Value>,
    /// If this is a `samsung_pay` PaymentMethod, this hash contains details about the SamsungPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub samsung_pay: Option<miniserde::json::Value>,
    /// If this is a `satispay` PaymentMethod, this hash contains details about the Satispay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub satispay: Option<miniserde::json::Value>,
    /// If this is a Scalapay PaymentMethod, this hash contains details about the Scalapay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub scalapay: Option<miniserde::json::Value>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<ConfirmSetupIntentPaymentMethodDataSepaDebit>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<ConfirmSetupIntentPaymentMethodDataSofort>,
    /// If this is a `sunbit` PaymentMethod, this hash contains details about the Sunbit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub sunbit: Option<miniserde::json::Value>,
    /// If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub swish: Option<miniserde::json::Value>,
    /// If this is a TWINT PaymentMethod, this hash contains details about the TWINT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub twint: Option<miniserde::json::Value>,
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateSetupIntentPaymentMethodDataType,
    /// If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi: Option<ConfirmSetupIntentPaymentMethodDataUpi>,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<ConfirmSetupIntentPaymentMethodDataUsBankAccount>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub wechat_pay: Option<miniserde::json::Value>,
    /// If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub zip: Option<miniserde::json::Value>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodData").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodData {
    pub fn new(type_: impl Into<stripe_shared::CreateSetupIntentPaymentMethodDataType>) -> Self {
        Self {
            acss_debit: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            allow_redisplay: None,
            alma: None,
            amazon_pay: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            billie: None,
            billing_details: None,
            bizum: None,
            blik: None,
            boleto: None,
            cashapp: None,
            crypto: None,
            customer_balance: None,
            eps: None,
            fpx: None,
            giropay: None,
            grabpay: None,
            ideal: None,
            interac_present: None,
            kakao_pay: None,
            klarna: None,
            konbini: None,
            kr_card: None,
            link: None,
            mb_way: None,
            metadata: None,
            mobilepay: None,
            multibanco: None,
            naver_pay: None,
            nz_bank_account: None,
            oxxo: None,
            p24: None,
            pay_by_bank: None,
            payco: None,
            paynow: None,
            paypal: None,
            payto: None,
            pix: None,
            promptpay: None,
            radar_options: None,
            revolut_pay: None,
            samsung_pay: None,
            satispay: None,
            scalapay: None,
            sepa_debit: None,
            sofort: None,
            sunbit: None,
            swish: None,
            twint: None,
            type_: type_.into(),
            upi: None,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataAuBecsDebit {
    /// The account number for the bank account.
    pub account_number: String,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataAuBecsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataAuBecsDebit").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataAuBecsDebit {
    pub fn new(account_number: impl Into<String>, bsb_number: impl Into<String>) -> Self {
        Self { account_number: account_number.into(), bsb_number: bsb_number.into() }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataBacsDebit {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Sort code of the bank account. (e.g., `10-20-30`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataBacsDebit").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataBacsDebit {
    pub fn new() -> Self {
        Self { account_number: None, sort_code: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodDataBacsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataBoleto {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers)
    pub tax_id: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataBoleto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataBoleto").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataBoleto {
    pub fn new(tax_id: impl Into<String>) -> Self {
        Self { tax_id: tax_id.into() }
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<stripe_shared::Bank>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataEps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataEps").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataEps {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodDataEps {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<stripe_shared::IssuingCardholderType>,
    /// The customer's bank.
    pub bank: stripe_shared::PaymentMethodFpxBank,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataFpx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataFpx").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataFpx {
    pub fn new(bank: impl Into<stripe_shared::PaymentMethodFpxBank>) -> Self {
        Self { account_holder_type: None, bank: bank.into() }
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    /// Only use this parameter for existing customers.
    /// Don't use it for new customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<stripe_shared::PaymentMethodIdealBank>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataIdeal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataIdeal").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataIdeal {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodDataIdeal {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataKlarna {
    /// Customer's date of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirth>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataKlarna").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataKlarna {
    pub fn new() -> Self {
        Self { dob: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodDataKlarna {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataNaverPay {
    /// Whether to use Naver Pay points or a card to fund this transaction.
    /// If not provided, this defaults to `card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<stripe_shared::Funding>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataNaverPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataNaverPay").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataNaverPay {
    pub fn new() -> Self {
        Self { funding: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodDataNaverPay {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataNzBankAccount {
    /// The name on the bank account.
    /// Only required if the account holder name is different from the name of the authorized signatory collected in the PaymentMethod’s billing details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    /// The account number for the bank account.
    pub account_number: String,
    /// The numeric code for the bank account's bank.
    pub bank_code: String,
    /// The numeric code for the bank account's bank branch.
    pub branch_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// The suffix of the bank account number.
    pub suffix: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataNzBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataNzBankAccount").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataNzBankAccount {
    pub fn new(
        account_number: impl Into<String>,
        bank_code: impl Into<String>,
        branch_code: impl Into<String>,
        suffix: impl Into<String>,
    ) -> Self {
        Self {
            account_holder_name: None,
            account_number: account_number.into(),
            bank_code: bank_code.into(),
            branch_code: branch_code.into(),
            reference: None,
            suffix: suffix.into(),
        }
    }
}
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<stripe_shared::PaymentMethodP24Bank>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataP24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataP24").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataP24 {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodDataP24 {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataPayto {
    /// The account number for the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Bank-State-Branch number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsb_number: Option<String>,
    /// The PayID alias for the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataPayto").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataPayto {
    pub fn new() -> Self {
        Self { account_number: None, bsb_number: None, pay_id: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodDataPayto {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataSepaDebit {
    /// IBAN of the bank account.
    pub iban: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataSepaDebit").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataSepaDebit {
    pub fn new(iban: impl Into<String>) -> Self {
        Self { iban: iban.into() }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: stripe_shared::Country,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataSofort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataSofort").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataSofort {
    pub fn new(country: impl Into<stripe_shared::Country>) -> Self {
        Self { country: country.into() }
    }
}
/// If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataUpi {
    /// Configuration options for setting up an eMandate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmSetupIntentPaymentMethodDataUpiMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataUpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataUpi").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataUpi {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodDataUpi {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration options for setting up an eMandate
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataUpiMandateOptions {
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
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataUpiMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataUpiMandateOptions")
            .finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataUpiMandateOptions {
    pub fn new() -> Self {
        Self { amount: None, amount_type: None, description: None, end_date: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodDataUpiMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataUsBankAccount {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<stripe_shared::IssuingCardholderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<stripe_shared::PaymentMethodUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodDataUsBankAccount").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodDataUsBankAccount {
    pub fn new() -> Self {
        Self {
            account_holder_type: None,
            account_number: None,
            account_type: None,
            financial_connections_account: None,
            routing_number: None,
        }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodDataUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Payment method-specific configuration for this SetupIntent.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptions {
    /// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebit>,
    /// If this is a `amazon_pay` SetupIntent, this sub-hash contains details about the AmazonPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// If this is a `bacs_debit` SetupIntent, this sub-hash contains details about the Bacs Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<ConfirmSetupIntentPaymentMethodOptionsBacsDebit>,
    /// If this is a `bizum` SetupIntent, this sub-hash contains details about the Bizum payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub bizum: Option<miniserde::json::Value>,
    /// Configuration for any card setup attempted on this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<ConfirmSetupIntentPaymentMethodOptionsCard>,
    /// If this is a `card_present` PaymentMethod, this sub-hash contains details about the card-present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub card_present: Option<miniserde::json::Value>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<ConfirmSetupIntentPaymentMethodOptionsKlarna>,
    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options (Link is also known as Onelink in the UK).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<SetupIntentPaymentMethodOptionsParam>,
    /// If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodOptionsParam>,
    /// If this is a `payto` SetupIntent, this sub-hash contains details about the PayTo payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto: Option<ConfirmSetupIntentPaymentMethodOptionsPayto>,
    /// If this is a `pix` SetupIntent, this sub-hash contains details about the Pix payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<ConfirmSetupIntentPaymentMethodOptionsPix>,
    /// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<ConfirmSetupIntentPaymentMethodOptionsSepaDebit>,
    /// If this is a `upi` SetupIntent, this sub-hash contains details about the UPI payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi: Option<ConfirmSetupIntentPaymentMethodOptionsUpi>,
    /// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<ConfirmSetupIntentPaymentMethodOptionsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptions").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptions {
    pub fn new() -> Self {
        Self {
            acss_debit: None,
            amazon_pay: None,
            bacs_debit: None,
            bizum: None,
            card: None,
            card_present: None,
            klarna: None,
            link: None,
            paypal: None,
            payto: None,
            pix: None,
            sepa_debit: None,
            upi: None,
            us_bank_account: None,
        }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsAcssDebit {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_shared::Currency>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsMandateOptionsParam>,
    /// Bank account verification method. The default value is `automatic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsAcssDebit").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self { currency: None, mandate_options: None, verification_method: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsAcssDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `bacs_debit` SetupIntent, this sub-hash contains details about the Bacs Debit payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsBacsDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<PaymentMethodOptionsMandateOptionsParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsBacsDebit").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsBacsDebit {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsBacsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for any card setup attempted on this SetupIntent.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsCard {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<SetupIntentMandateOptionsParam>,
    /// When specified, this parameter signals that a card has been collected
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA. This
    /// parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    /// Selected network to process this SetupIntent on.
    /// Depends on the available networks of the card attached to the SetupIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<stripe_shared::RequestThreeDSecure>,
    /// If 3D Secure authentication was performed with a third-party provider,
    /// the authentication details to use for this setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecure>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsCard").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self {
            mandate_options: None,
            moto: None,
            network: None,
            request_three_d_secure: None,
            three_d_secure: None,
        }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsCard {
    fn default() -> Self {
        Self::new()
    }
}
/// If 3D Secure authentication was performed with a third-party provider,
/// the authentication details to use for this setup.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecure {
    /// The `transStatus` returned from the card Issuer’s ACS in the ARes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ares_trans_status: Option<stripe_shared::AresTransStatus>,
    /// The cryptogram, also known as the "authentication value" (AAV, CAVV or
    /// AEVV). This value is 20 bytes, base64-encoded into a 28-character string.
    /// (Most 3D Secure providers will return the base64-encoded version, which
    /// is what you should specify here.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptogram: Option<String>,
    /// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
    /// provider and indicates what degree of authentication was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator: Option<stripe_shared::ElectronicCommerceIndicator>,
    /// Network specific 3DS fields. Network specific arguments require an
    /// explicit card brand choice. The parameter `payment_method_options.card.network``
    /// must be populated accordingly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options: Option<NetworkOptionsParam>,
    /// The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the
    /// AReq sent to the card Issuer's ACS. A string containing 2 digits from 01-99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_challenge_indicator: Option<String>,
    /// For 3D Secure 1, the XID. For 3D Secure 2, the Directory Server
    /// Transaction ID (dsTransID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// The version of 3D Secure that was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<stripe_shared::ThreeDSecureDetailsVersion>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecure")
            .finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecure {
    pub fn new() -> Self {
        Self {
            ares_trans_status: None,
            cryptogram: None,
            electronic_commerce_indicator: None,
            network_options: None,
            requestor_challenge_indicator: None,
            transaction_id: None,
            version: None,
        }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecure {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsKlarna {
    /// The currency of the SetupIntent. Three letter ISO currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// On-demand details if setting up a payment method for on-demand payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand: Option<OnDemandParam>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale:
        Option<stripe_shared::CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale>,
    /// Subscription details if setting up or charging a subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<SetupIntentSubscriptionParam>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsKlarna").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsKlarna {
    pub fn new() -> Self {
        Self { currency: None, on_demand: None, preferred_locale: None, subscriptions: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsKlarna {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `payto` SetupIntent, this sub-hash contains details about the PayTo payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsPayto {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmSetupIntentPaymentMethodOptionsPaytoMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsPayto").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsPayto {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsPayto {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsPaytoMandateOptions {
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
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsPaytoMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsPaytoMandateOptions")
            .finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsPaytoMandateOptions {
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
impl Default for ConfirmSetupIntentPaymentMethodOptionsPaytoMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `pix` SetupIntent, this sub-hash contains details about the Pix payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsPix {
    /// Additional fields for mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmSetupIntentPaymentMethodOptionsPixMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsPix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsPix").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsPix {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsPix {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for mandate creation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsPixMandateOptions {
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
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsPixMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsPixMandateOptions")
            .finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsPixMandateOptions {
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
impl Default for ConfirmSetupIntentPaymentMethodOptionsPixMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmSetupIntentPaymentMethodOptionsSepaDebitMandateOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsSepaDebit").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self { mandate_options: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsSepaDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    /// Prefix used to generate the Mandate reference.
    /// Must be at most 12 characters long.
    /// Must consist of only uppercase letters, numbers, spaces, or the following special characters: '/', '_', '-', '&', '.'.
    /// Cannot begin with 'STRIPE'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsSepaDebitMandateOptions")
            .finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    pub fn new() -> Self {
        Self { reference_prefix: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `upi` SetupIntent, this sub-hash contains details about the UPI payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsUpi {
    /// Configuration options for setting up an eMandate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmSetupIntentPaymentMethodOptionsUpiMandateOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_shared::SetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsUpi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsUpi").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsUpi {
    pub fn new() -> Self {
        Self { mandate_options: None, setup_future_usage: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsUpi {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration options for setting up an eMandate
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsUpiMandateOptions {
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
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsUpiMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsUpiMandateOptions")
            .finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsUpiMandateOptions {
    pub fn new() -> Self {
        Self { amount: None, amount_type: None, description: None, end_date: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsUpiMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<LinkedAccountOptionsParam>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<MandateOptionsParam>,
    /// Additional fields for network related functions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<NetworksOptionsParam>,
    /// Bank account verification method. The default value is `automatic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntentPaymentMethodOptionsUsBankAccount")
            .finish_non_exhaustive()
    }
}
impl ConfirmSetupIntentPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self {
            financial_connections: None,
            mandate_options: None,
            networks: None,
            verification_method: None,
        }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Confirm that your customer intends to set up the current or
/// provided payment method. For example, you would confirm a SetupIntent
/// when a customer hits the “Save” button on a payment method management
/// page on your website.
///
/// If the selected payment method does not require any additional
/// steps from the customer, the SetupIntent will transition to the
/// `succeeded` status.
///
/// Otherwise, it will transition to the `requires_action` status and
/// suggest additional actions via `next_action`. If setup fails,
/// the SetupIntent will transition to the
/// `requires_payment_method` status or the `canceled` status if the
/// confirmation limit is reached.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ConfirmSetupIntent {
    inner: ConfirmSetupIntentBuilder,
    intent: stripe_shared::SetupIntentId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmSetupIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmSetupIntent").finish_non_exhaustive()
    }
}
impl ConfirmSetupIntent {
    /// Construct a new `ConfirmSetupIntent`.
    pub fn new(intent: impl Into<stripe_shared::SetupIntentId>) -> Self {
        Self { intent: intent.into(), inner: ConfirmSetupIntentBuilder::new() }
    }
    /// The list of payment method types to allow for this SetupIntent.
    /// Stripe will only use methods in this list when determining the payment methods to offer.
    /// A list of valid payment method types can be found [here](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type).
    pub fn allowed_payment_method_types(
        mut self,
        allowed_payment_method_types: impl Into<Vec<stripe_shared::AllowedPaymentMethodTypes>>,
    ) -> Self {
        self.inner.allowed_payment_method_types = Some(allowed_payment_method_types.into());
        self
    }
    /// ID of the ConfirmationToken used to confirm this SetupIntent.
    ///
    /// If the provided ConfirmationToken contains properties that are also being provided in this request, such as `payment_method`, then the values in this request will take precedence.
    pub fn confirmation_token(mut self, confirmation_token: impl Into<String>) -> Self {
        self.inner.confirmation_token = Some(confirmation_token.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    pub fn mandate_data(mut self, mandate_data: impl Into<ConfirmSetupIntentMandateData>) -> Self {
        self.inner.mandate_data = Some(mandate_data.into());
        self
    }
    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    pub fn payment_method(mut self, payment_method: impl Into<String>) -> Self {
        self.inner.payment_method = Some(payment_method.into());
        self
    }
    /// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-payment_method).
    /// value in the SetupIntent.
    pub fn payment_method_data(
        mut self,
        payment_method_data: impl Into<ConfirmSetupIntentPaymentMethodData>,
    ) -> Self {
        self.inner.payment_method_data = Some(payment_method_data.into());
        self
    }
    /// Payment method-specific configuration for this SetupIntent.
    pub fn payment_method_options(
        mut self,
        payment_method_options: impl Into<ConfirmSetupIntentPaymentMethodOptions>,
    ) -> Self {
        self.inner.payment_method_options = Some(payment_method_options.into());
        self
    }
    /// The URL to redirect your customer back to after they authenticate on the payment method's app or site.
    /// If you'd prefer to redirect to a mobile application, you can alternatively supply an application URI scheme.
    /// This parameter is only used for cards and other redirect-based payment methods.
    pub fn return_url(mut self, return_url: impl Into<String>) -> Self {
        self.inner.return_url = Some(return_url.into());
        self
    }
    /// Set to `true` when confirming server-side and using Stripe.js, iOS, or Android client-side SDKs to handle the next actions.
    pub fn use_stripe_sdk(mut self, use_stripe_sdk: impl Into<bool>) -> Self {
        self.inner.use_stripe_sdk = Some(use_stripe_sdk.into());
        self
    }
}
impl ConfirmSetupIntent {
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

impl StripeRequest for ConfirmSetupIntent {
    type Output = stripe_shared::SetupIntent;

    fn build(&self) -> RequestBuilder {
        let intent = &self.intent;
        RequestBuilder::new(StripeMethod::Post, format!("/setup_intents/{intent}/confirm"))
            .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct VerifyMicrodepositsSetupIntentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    descriptor_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for VerifyMicrodepositsSetupIntentBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VerifyMicrodepositsSetupIntentBuilder").finish_non_exhaustive()
    }
}
impl VerifyMicrodepositsSetupIntentBuilder {
    fn new() -> Self {
        Self { amounts: None, descriptor_code: None, expand: None }
    }
}
/// Verifies microdeposits on a SetupIntent object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct VerifyMicrodepositsSetupIntent {
    inner: VerifyMicrodepositsSetupIntentBuilder,
    intent: stripe_shared::SetupIntentId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for VerifyMicrodepositsSetupIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VerifyMicrodepositsSetupIntent").finish_non_exhaustive()
    }
}
impl VerifyMicrodepositsSetupIntent {
    /// Construct a new `VerifyMicrodepositsSetupIntent`.
    pub fn new(intent: impl Into<stripe_shared::SetupIntentId>) -> Self {
        Self { intent: intent.into(), inner: VerifyMicrodepositsSetupIntentBuilder::new() }
    }
    /// Two positive integers, in *cents*, equal to the values of the microdeposits sent to the bank account.
    pub fn amounts(mut self, amounts: impl Into<Vec<i64>>) -> Self {
        self.inner.amounts = Some(amounts.into());
        self
    }
    /// A six-character code starting with SM present in the microdeposit sent to the bank account.
    pub fn descriptor_code(mut self, descriptor_code: impl Into<String>) -> Self {
        self.inner.descriptor_code = Some(descriptor_code.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl VerifyMicrodepositsSetupIntent {
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

impl StripeRequest for VerifyMicrodepositsSetupIntent {
    type Output = stripe_shared::SetupIntent;

    fn build(&self) -> RequestBuilder {
        let intent = &self.intent;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/setup_intents/{intent}/verify_microdeposits"),
        )
        .form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct OnlineParam {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: String,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OnlineParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OnlineParam").finish_non_exhaustive()
    }
}
impl OnlineParam {
    pub fn new(ip_address: impl Into<String>, user_agent: impl Into<String>) -> Self {
        Self { ip_address: ip_address.into(), user_agent: user_agent.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PaymentMethodParam {
    /// Customer's bank account number.
    pub account_number: String,
    /// Institution number of the customer's bank.
    pub institution_number: String,
    /// Transit number of the customer's bank.
    pub transit_number: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodParam").finish_non_exhaustive()
    }
}
impl PaymentMethodParam {
    pub fn new(
        account_number: impl Into<String>,
        institution_number: impl Into<String>,
        transit_number: impl Into<String>,
    ) -> Self {
        Self {
            account_number: account_number.into(),
            institution_number: institution_number.into(),
            transit_number: transit_number.into(),
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct BillingDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
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
impl std::fmt::Debug for BillingDetailsAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingDetailsAddress").finish_non_exhaustive()
    }
}
impl BillingDetailsAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for BillingDetailsAddress {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DateOfBirth {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DateOfBirth {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DateOfBirth").finish_non_exhaustive()
    }
}
impl DateOfBirth {
    pub fn new(day: impl Into<i64>, month: impl Into<i64>, year: impl Into<i64>) -> Self {
        Self { day: day.into(), month: month.into(), year: year.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RadarOptionsWithHiddenOptions {
    /// A [Radar Session](https://docs.stripe.com/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RadarOptionsWithHiddenOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RadarOptionsWithHiddenOptions").finish_non_exhaustive()
    }
}
impl RadarOptionsWithHiddenOptions {
    pub fn new() -> Self {
        Self { session: None }
    }
}
impl Default for RadarOptionsWithHiddenOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsParam {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,.
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,
    /// List of Stripe products where this mandate can be selected automatically.
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
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptionsMandateOptionsParam").finish_non_exhaustive()
    }
}
impl SetupIntentPaymentMethodOptionsMandateOptionsParam {
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
impl Default for SetupIntentPaymentMethodOptionsMandateOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PaymentMethodOptionsMandateOptionsParam {
    /// Prefix used to generate the Mandate reference.
    /// Must be at most 12 characters long.
    /// Must consist of only uppercase letters, numbers, spaces, or the following special characters: '/', '_', '-', '&', '.'.
    /// Cannot begin with 'DDIC' or 'STRIPE'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsMandateOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsMandateOptionsParam").finish_non_exhaustive()
    }
}
impl PaymentMethodOptionsMandateOptionsParam {
    pub fn new() -> Self {
        Self { reference_prefix: None }
    }
}
impl Default for PaymentMethodOptionsMandateOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SetupIntentMandateOptionsParam {
    /// Amount to be charged for future payments, specified in the presentment currency.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: stripe_shared::AmountType,
    /// Currency in which future payments will be charged.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// End date of the mandate or subscription.
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<stripe_types::Timestamp>,
    /// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: stripe_shared::PaymentMethodOptionsCardMandateOptionsInterval,
    /// The number of intervals between payments.
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: String,
    /// Start date of the mandate or subscription. Start date should not be lesser than yesterday.
    pub start_date: stripe_types::Timestamp,
    /// Specifies the type of mandates supported. Possible values are `india`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types: Option<Vec<stripe_shared::SupportedTypes>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentMandateOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentMandateOptionsParam").finish_non_exhaustive()
    }
}
impl SetupIntentMandateOptionsParam {
    pub fn new(
        amount: impl Into<i64>,
        amount_type: impl Into<stripe_shared::AmountType>,
        currency: impl Into<stripe_types::Currency>,
        interval: impl Into<stripe_shared::PaymentMethodOptionsCardMandateOptionsInterval>,
        reference: impl Into<String>,
        start_date: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        Self {
            amount: amount.into(),
            amount_type: amount_type.into(),
            currency: currency.into(),
            description: None,
            end_date: None,
            interval: interval.into(),
            interval_count: None,
            reference: reference.into(),
            start_date: start_date.into(),
            supported_types: None,
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CartesBancairesNetworkOptionsParam {
    /// The cryptogram calculation algorithm used by the card Issuer's ACS
    /// to calculate the Authentication cryptogram. Also known as `cavvAlgorithm`.
    /// messageExtension: CB-AVALGO
    pub cb_avalgo: stripe_shared::CbAvalgo,
    /// The exemption indicator returned from Cartes Bancaires in the ARes.
    /// message extension: CB-EXEMPTION; string (4 characters)
    /// This is a 3 byte bitmap (low significant byte first and most significant
    /// bit first) that has been Base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_exemption: Option<String>,
    /// The risk score returned from Cartes Bancaires in the ARes.
    /// message extension: CB-SCORE; numeric value 0-99
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_score: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CartesBancairesNetworkOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CartesBancairesNetworkOptionsParam").finish_non_exhaustive()
    }
}
impl CartesBancairesNetworkOptionsParam {
    pub fn new(cb_avalgo: impl Into<stripe_shared::CbAvalgo>) -> Self {
        Self { cb_avalgo: cb_avalgo.into(), cb_exemption: None, cb_score: None }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct OnDemandParam {
    /// Your average amount value.
    /// You can use a value across your customer base, or segment based on customer type, country, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_amount: Option<i64>,
    /// The maximum value you may charge a customer per purchase.
    /// You can use a value across your customer base, or segment based on customer type, country, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_amount: Option<i64>,
    /// The lowest or minimum value you may charge a customer per purchase.
    /// You can use a value across your customer base, or segment based on customer type, country, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
    /// Interval at which the customer is making purchases
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_interval: Option<stripe_shared::PlanInterval>,
    /// The number of `purchase_interval` between charges
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_interval_count: Option<u64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OnDemandParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OnDemandParam").finish_non_exhaustive()
    }
}
impl OnDemandParam {
    pub fn new() -> Self {
        Self {
            average_amount: None,
            maximum_amount: None,
            minimum_amount: None,
            purchase_interval: None,
            purchase_interval_count: None,
        }
    }
}
impl Default for OnDemandParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SubscriptionNextBillingParam {
    /// The amount of the next charge for the subscription.
    pub amount: i64,
    /// The date of the next charge for the subscription in YYYY-MM-DD format.
    pub date: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionNextBillingParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionNextBillingParam").finish_non_exhaustive()
    }
}
impl SubscriptionNextBillingParam {
    pub fn new(amount: impl Into<i64>, date: impl Into<String>) -> Self {
        Self { amount: amount.into(), date: date.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SetupIntentPaymentMethodOptionsParam {
    /// \[Deprecated\] This is a legacy parameter that no longer has any function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptionsParam").finish_non_exhaustive()
    }
}
impl SetupIntentPaymentMethodOptionsParam {
    pub fn new() -> Self {
        Self { persistent_token: None }
    }
}
impl Default for SetupIntentPaymentMethodOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PaymentMethodOptionsParam {
    /// The PayPal Billing Agreement ID (BAID).
    /// This is an ID generated by PayPal which represents the mandate between the merchant and the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_agreement_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsParam").finish_non_exhaustive()
    }
}
impl PaymentMethodOptionsParam {
    pub fn new() -> Self {
        Self { billing_agreement_id: None }
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
pub struct LinkedAccountOptionsFiltersParam {
    /// The account subcategories to use to filter for selectable accounts.
    /// Valid subcategories are `checking` and `savings`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_subcategories: Option<Vec<stripe_shared::PaymentMethodUsBankAccountAccountType>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LinkedAccountOptionsFiltersParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LinkedAccountOptionsFiltersParam").finish_non_exhaustive()
    }
}
impl LinkedAccountOptionsFiltersParam {
    pub fn new() -> Self {
        Self { account_subcategories: None }
    }
}
impl Default for LinkedAccountOptionsFiltersParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct MandateOptionsParam {
    /// The method used to collect offline mandate customer acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<stripe_shared::MandateUsBankAccountCollectionMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandateOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MandateOptionsParam").finish_non_exhaustive()
    }
}
impl MandateOptionsParam {
    pub fn new() -> Self {
        Self { collection_method: None }
    }
}
impl Default for MandateOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct NetworksOptionsParam {
    /// Triggers validations to run across the selected networks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Vec<stripe_shared::UsBankAccountNetworksSupported>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for NetworksOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("NetworksOptionsParam").finish_non_exhaustive()
    }
}
impl NetworksOptionsParam {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for NetworksOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CustomerAcceptanceParam {
    /// The time at which the customer accepted the Mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<stripe_types::Timestamp>,
    /// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "stripe_types::with_serde_json_opt")]
    pub offline: Option<miniserde::json::Value>,
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<OnlineParam>,
    /// The type of customer acceptance information included with the Mandate.
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::TerminalReaderStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerAcceptanceParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerAcceptanceParam").finish_non_exhaustive()
    }
}
impl CustomerAcceptanceParam {
    pub fn new(type_: impl Into<stripe_shared::TerminalReaderStatus>) -> Self {
        Self { accepted_at: None, offline: None, online: None, type_: type_.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct BillingDetailsInnerParams {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<BillingDetailsAddress>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Taxpayer identification number.
    /// Used only for transactions between LATAM buyers and non-LATAM sellers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingDetailsInnerParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingDetailsInnerParams").finish_non_exhaustive()
    }
}
impl BillingDetailsInnerParams {
    pub fn new() -> Self {
        Self { address: None, email: None, name: None, phone: None, tax_id: None }
    }
}
impl Default for BillingDetailsInnerParams {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct NetworkOptionsParam {
    /// Cartes Bancaires-specific 3DS fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires: Option<CartesBancairesNetworkOptionsParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for NetworkOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("NetworkOptionsParam").finish_non_exhaustive()
    }
}
impl NetworkOptionsParam {
    pub fn new() -> Self {
        Self { cartes_bancaires: None }
    }
}
impl Default for NetworkOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SetupIntentSubscriptionParam {
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
    pub next_billing: SubscriptionNextBillingParam,
    /// A non-customer-facing reference to correlate subscription charges in the Klarna app.
    /// Use a value that persists across subscription charges.
    pub reference: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentSubscriptionParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentSubscriptionParam").finish_non_exhaustive()
    }
}
impl SetupIntentSubscriptionParam {
    pub fn new(
        interval: impl Into<stripe_shared::PlanInterval>,
        next_billing: impl Into<SubscriptionNextBillingParam>,
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct LinkedAccountOptionsParam {
    /// Provide filters for the linked accounts that the customer can select for the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<LinkedAccountOptionsFiltersParam>,
    /// The list of permissions to request.
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<stripe_shared::Permissions>>,
    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<stripe_shared::Prefetch>>,
    /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LinkedAccountOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LinkedAccountOptionsParam").finish_non_exhaustive()
    }
}
impl LinkedAccountOptionsParam {
    pub fn new() -> Self {
        Self { filters: None, permissions: None, prefetch: None, return_url: None }
    }
}
impl Default for LinkedAccountOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SecretKeyParam {
    /// This hash contains details about the customer acceptance of the Mandate.
    pub customer_acceptance: CustomerAcceptanceParam,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SecretKeyParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SecretKeyParam").finish_non_exhaustive()
    }
}
impl SecretKeyParam {
    pub fn new(customer_acceptance: impl Into<CustomerAcceptanceParam>) -> Self {
        Self { customer_acceptance: customer_acceptance.into() }
    }
}
/// The list of payment method types to allow for this SetupIntent.
/// Stripe will only use methods in this list when determining the payment methods to offer.
/// A list of valid payment method types can be found [here](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type).
pub type CreateSetupIntentAllowedPaymentMethodTypes = stripe_shared::AllowedPaymentMethodTypes;
/// Controls whether this SetupIntent will accept redirect-based payment methods.
///
/// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
/// To [confirm](https://docs.stripe.com/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
pub type CreateSetupIntentAutomaticPaymentMethodsAllowRedirects =
    stripe_shared::MandatePixAmountIncludesIof;
/// The list of payment method types to exclude from use with this SetupIntent.
pub type CreateSetupIntentExcludedPaymentMethodTypes = stripe_shared::ExcludedPaymentMethodTypes;
/// Indicates the directions of money movement for which this payment method is intended to be used.
///
/// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
/// Include `outbound` if you intend to use the payment method as the destination to send funds to.
/// You can include both if you intend to use the payment method for both purposes.
pub type CreateSetupIntentFlowDirections = stripe_shared::FlowDirections;
/// The type of customer acceptance information included with the Mandate.
/// One of `online` or `offline`.
pub type CreateSetupIntentMandateDataCustomerAcceptanceType = stripe_shared::TerminalReaderStatus;
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to `unspecified`.
pub type CreateSetupIntentPaymentMethodDataAllowRedisplay = stripe_shared::CardAllowRedisplay;
/// The customer's bank.
pub type CreateSetupIntentPaymentMethodDataEpsBank = stripe_shared::Bank;
/// Account holder type for FPX transaction
pub type CreateSetupIntentPaymentMethodDataFpxAccountHolderType =
    stripe_shared::IssuingCardholderType;
/// The customer's bank.
pub type CreateSetupIntentPaymentMethodDataFpxBank = stripe_shared::PaymentMethodFpxBank;
/// The customer's bank.
/// Only use this parameter for existing customers.
/// Don't use it for new customers.
pub type CreateSetupIntentPaymentMethodDataIdealBank = stripe_shared::PaymentMethodIdealBank;
/// Whether to use Naver Pay points or a card to fund this transaction.
/// If not provided, this defaults to `card`.
pub type CreateSetupIntentPaymentMethodDataNaverPayFunding = stripe_shared::Funding;
/// The customer's bank.
pub type CreateSetupIntentPaymentMethodDataP24Bank = stripe_shared::PaymentMethodP24Bank;
/// Two-letter ISO code representing the country the bank account is located in.
pub type CreateSetupIntentPaymentMethodDataSofortCountry = stripe_shared::Country;
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
pub type CreateSetupIntentPaymentMethodDataType =
    stripe_shared::CreateSetupIntentPaymentMethodDataType;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type CreateSetupIntentPaymentMethodDataUpiMandateOptionsAmountType = stripe_shared::AmountType;
/// Account holder type: individual or company.
pub type CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType =
    stripe_shared::IssuingCardholderType;
/// Account type: checkings or savings. Defaults to checking if omitted.
pub type CreateSetupIntentPaymentMethodDataUsBankAccountAccountType =
    stripe_shared::PaymentMethodUsBankAccountAccountType;
/// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
/// Must be a [supported currency](https://stripe.com/docs/currencies).
pub type CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency = stripe_shared::Currency;
/// List of Stripe products where this mandate can be selected automatically.
pub type CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor =
    stripe_shared::DefaultFor;
/// Payment schedule for the mandate.
pub type CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule =
    stripe_shared::PaymentSchedule;
/// Transaction type of the mandate.
pub type CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType =
    stripe_shared::TransactionType;
/// Bank account verification method. The default value is `automatic`.
pub type CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod =
    stripe_shared::VerificationMethod;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType =
    stripe_shared::AmountType;
/// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
pub type CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval =
    stripe_shared::PaymentMethodOptionsCardMandateOptionsInterval;
/// Specifies the type of mandates supported. Possible values are `india`.
pub type CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes =
    stripe_shared::SupportedTypes;
/// Selected network to process this SetupIntent on.
/// Depends on the available networks of the card attached to the SetupIntent.
/// Can be only set confirm-time.
pub type CreateSetupIntentPaymentMethodOptionsCardNetwork =
    stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork;
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// If not provided, this value defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
pub type CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure =
    stripe_shared::RequestThreeDSecure;
/// The `transStatus` returned from the card Issuer’s ACS in the ARes.
pub type CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus =
    stripe_shared::AresTransStatus;
/// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
/// provider and indicates what degree of authentication was performed.
pub type CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator =
    stripe_shared::ElectronicCommerceIndicator;
/// The cryptogram calculation algorithm used by the card Issuer's ACS
/// to calculate the Authentication cryptogram. Also known as `cavvAlgorithm`.
/// messageExtension: CB-AVALGO
pub type CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo =
    stripe_shared::CbAvalgo;
/// The version of 3D Secure that was performed.
pub type CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion =
    stripe_shared::ThreeDSecureDetailsVersion;
/// Interval at which the customer is making purchases
pub type CreateSetupIntentPaymentMethodOptionsKlarnaOnDemandPurchaseInterval =
    stripe_shared::PlanInterval;
/// Preferred language of the Klarna authorization page that the customer is redirected to
pub type CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale =
    stripe_shared::CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale;
/// Unit of time between subscription charges.
pub type CreateSetupIntentPaymentMethodOptionsKlarnaSubscriptionsInterval =
    stripe_shared::PlanInterval;
/// The type of amount that will be collected.
/// The amount charged must be exact or up to the value of `amount` param for `fixed` or `maximum` type respectively.
/// Defaults to `maximum`.
pub type CreateSetupIntentPaymentMethodOptionsPaytoMandateOptionsAmountType =
    stripe_shared::AmountType;
/// The periodicity at which payments will be collected. Defaults to `adhoc`.
pub type CreateSetupIntentPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule =
    stripe_shared::MandatePaytoPaymentSchedule;
/// The purpose for which payments are made. Has a default value based on your merchant category code.
pub type CreateSetupIntentPaymentMethodOptionsPaytoMandateOptionsPurpose = stripe_shared::Purpose;
/// Determines if the amount includes the IOF tax. Defaults to `never`.
pub type CreateSetupIntentPaymentMethodOptionsPixMandateOptionsAmountIncludesIof =
    stripe_shared::MandatePixAmountIncludesIof;
/// Type of amount. Defaults to `maximum`.
pub type CreateSetupIntentPaymentMethodOptionsPixMandateOptionsAmountType =
    stripe_shared::AmountType;
/// Schedule at which the future payments will be charged. Defaults to `monthly`.
pub type CreateSetupIntentPaymentMethodOptionsPixMandateOptionsPaymentSchedule =
    stripe_shared::MandatePixPaymentSchedule;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type CreateSetupIntentPaymentMethodOptionsUpiMandateOptionsAmountType =
    stripe_shared::AmountType;
pub type CreateSetupIntentPaymentMethodOptionsUpiSetupFutureUsage = stripe_shared::SetupFutureUsage;
/// The account subcategories to use to filter for selectable accounts.
/// Valid subcategories are `checking` and `savings`.
pub type CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategories =
    stripe_shared::PaymentMethodUsBankAccountAccountType;
/// The list of permissions to request.
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
pub type CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions =
    stripe_shared::Permissions;
/// List of data features that you would like to retrieve upon account creation.
pub type CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch =
    stripe_shared::Prefetch;
/// The method used to collect offline mandate customer acceptance.
pub type CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod =
    stripe_shared::MandateUsBankAccountCollectionMethod;
/// Triggers validations to run across the selected networks
pub type CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested =
    stripe_shared::UsBankAccountNetworksSupported;
/// Bank account verification method. The default value is `automatic`.
pub type CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod =
    stripe_shared::VerificationMethod;
/// Indicates how the payment method is intended to be used in the future.
/// If not provided, this value defaults to `off_session`.
pub type CreateSetupIntentUsage = stripe_shared::CreateSetupIntentUsage;
/// The list of payment method types to allow for this SetupIntent.
/// Stripe will only use methods in this list when determining the payment methods to offer.
/// A list of valid payment method types can be found [here](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type).
pub type UpdateSetupIntentAllowedPaymentMethodTypes = stripe_shared::AllowedPaymentMethodTypes;
/// The list of payment method types to exclude from use with this SetupIntent.
pub type UpdateSetupIntentExcludedPaymentMethodTypes = stripe_shared::ExcludedPaymentMethodTypes;
/// Indicates the directions of money movement for which this payment method is intended to be used.
///
/// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
/// Include `outbound` if you intend to use the payment method as the destination to send funds to.
/// You can include both if you intend to use the payment method for both purposes.
pub type UpdateSetupIntentFlowDirections = stripe_shared::FlowDirections;
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to `unspecified`.
pub type UpdateSetupIntentPaymentMethodDataAllowRedisplay = stripe_shared::CardAllowRedisplay;
/// The customer's bank.
pub type UpdateSetupIntentPaymentMethodDataEpsBank = stripe_shared::Bank;
/// Account holder type for FPX transaction
pub type UpdateSetupIntentPaymentMethodDataFpxAccountHolderType =
    stripe_shared::IssuingCardholderType;
/// The customer's bank.
pub type UpdateSetupIntentPaymentMethodDataFpxBank = stripe_shared::PaymentMethodFpxBank;
/// The customer's bank.
/// Only use this parameter for existing customers.
/// Don't use it for new customers.
pub type UpdateSetupIntentPaymentMethodDataIdealBank = stripe_shared::PaymentMethodIdealBank;
/// Whether to use Naver Pay points or a card to fund this transaction.
/// If not provided, this defaults to `card`.
pub type UpdateSetupIntentPaymentMethodDataNaverPayFunding = stripe_shared::Funding;
/// The customer's bank.
pub type UpdateSetupIntentPaymentMethodDataP24Bank = stripe_shared::PaymentMethodP24Bank;
/// Two-letter ISO code representing the country the bank account is located in.
pub type UpdateSetupIntentPaymentMethodDataSofortCountry = stripe_shared::Country;
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
pub type UpdateSetupIntentPaymentMethodDataType =
    stripe_shared::CreateSetupIntentPaymentMethodDataType;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type UpdateSetupIntentPaymentMethodDataUpiMandateOptionsAmountType = stripe_shared::AmountType;
/// Account holder type: individual or company.
pub type UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType =
    stripe_shared::IssuingCardholderType;
/// Account type: checkings or savings. Defaults to checking if omitted.
pub type UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType =
    stripe_shared::PaymentMethodUsBankAccountAccountType;
/// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
/// Must be a [supported currency](https://stripe.com/docs/currencies).
pub type UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency = stripe_shared::Currency;
/// List of Stripe products where this mandate can be selected automatically.
pub type UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor =
    stripe_shared::DefaultFor;
/// Payment schedule for the mandate.
pub type UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule =
    stripe_shared::PaymentSchedule;
/// Transaction type of the mandate.
pub type UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType =
    stripe_shared::TransactionType;
/// Bank account verification method. The default value is `automatic`.
pub type UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod =
    stripe_shared::VerificationMethod;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType =
    stripe_shared::AmountType;
/// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
pub type UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval =
    stripe_shared::PaymentMethodOptionsCardMandateOptionsInterval;
/// Specifies the type of mandates supported. Possible values are `india`.
pub type UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes =
    stripe_shared::SupportedTypes;
/// Selected network to process this SetupIntent on.
/// Depends on the available networks of the card attached to the SetupIntent.
/// Can be only set confirm-time.
pub type UpdateSetupIntentPaymentMethodOptionsCardNetwork =
    stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork;
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// If not provided, this value defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
pub type UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure =
    stripe_shared::RequestThreeDSecure;
/// The `transStatus` returned from the card Issuer’s ACS in the ARes.
pub type UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus =
    stripe_shared::AresTransStatus;
/// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
/// provider and indicates what degree of authentication was performed.
pub type UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator =
    stripe_shared::ElectronicCommerceIndicator;
/// The cryptogram calculation algorithm used by the card Issuer's ACS
/// to calculate the Authentication cryptogram. Also known as `cavvAlgorithm`.
/// messageExtension: CB-AVALGO
pub type UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo =
    stripe_shared::CbAvalgo;
/// The version of 3D Secure that was performed.
pub type UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion =
    stripe_shared::ThreeDSecureDetailsVersion;
/// Interval at which the customer is making purchases
pub type UpdateSetupIntentPaymentMethodOptionsKlarnaOnDemandPurchaseInterval =
    stripe_shared::PlanInterval;
/// Preferred language of the Klarna authorization page that the customer is redirected to
pub type UpdateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale =
    stripe_shared::CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale;
/// Unit of time between subscription charges.
pub type UpdateSetupIntentPaymentMethodOptionsKlarnaSubscriptionsInterval =
    stripe_shared::PlanInterval;
/// The type of amount that will be collected.
/// The amount charged must be exact or up to the value of `amount` param for `fixed` or `maximum` type respectively.
/// Defaults to `maximum`.
pub type UpdateSetupIntentPaymentMethodOptionsPaytoMandateOptionsAmountType =
    stripe_shared::AmountType;
/// The periodicity at which payments will be collected. Defaults to `adhoc`.
pub type UpdateSetupIntentPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule =
    stripe_shared::MandatePaytoPaymentSchedule;
/// The purpose for which payments are made. Has a default value based on your merchant category code.
pub type UpdateSetupIntentPaymentMethodOptionsPaytoMandateOptionsPurpose = stripe_shared::Purpose;
/// Determines if the amount includes the IOF tax. Defaults to `never`.
pub type UpdateSetupIntentPaymentMethodOptionsPixMandateOptionsAmountIncludesIof =
    stripe_shared::MandatePixAmountIncludesIof;
/// Type of amount. Defaults to `maximum`.
pub type UpdateSetupIntentPaymentMethodOptionsPixMandateOptionsAmountType =
    stripe_shared::AmountType;
/// Schedule at which the future payments will be charged. Defaults to `monthly`.
pub type UpdateSetupIntentPaymentMethodOptionsPixMandateOptionsPaymentSchedule =
    stripe_shared::MandatePixPaymentSchedule;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type UpdateSetupIntentPaymentMethodOptionsUpiMandateOptionsAmountType =
    stripe_shared::AmountType;
pub type UpdateSetupIntentPaymentMethodOptionsUpiSetupFutureUsage = stripe_shared::SetupFutureUsage;
/// The account subcategories to use to filter for selectable accounts.
/// Valid subcategories are `checking` and `savings`.
pub type UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategories =
    stripe_shared::PaymentMethodUsBankAccountAccountType;
/// The list of permissions to request.
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
pub type UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions =
    stripe_shared::Permissions;
/// List of data features that you would like to retrieve upon account creation.
pub type UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch =
    stripe_shared::Prefetch;
/// The method used to collect offline mandate customer acceptance.
pub type UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod =
    stripe_shared::MandateUsBankAccountCollectionMethod;
/// Triggers validations to run across the selected networks
pub type UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested =
    stripe_shared::UsBankAccountNetworksSupported;
/// Bank account verification method. The default value is `automatic`.
pub type UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod =
    stripe_shared::VerificationMethod;
/// Reason for canceling this SetupIntent.
/// Possible values are: `abandoned`, `requested_by_customer`, or `duplicate`.
pub type CancelSetupIntentCancellationReason = stripe_shared::CancellationReason;
/// The list of payment method types to allow for this SetupIntent.
/// Stripe will only use methods in this list when determining the payment methods to offer.
/// A list of valid payment method types can be found [here](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type).
pub type ConfirmSetupIntentAllowedPaymentMethodTypes = stripe_shared::AllowedPaymentMethodTypes;
/// The type of customer acceptance information included with the Mandate.
/// One of `online` or `offline`.
pub type ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType =
    stripe_shared::TerminalReaderStatus;
/// The type of customer acceptance information included with the Mandate.
pub type ConfirmSetupIntentClientKeyParamCustomerAcceptanceType =
    stripe_shared::ConfirmSetupIntentClientKeyParamCustomerAcceptanceType;
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to `unspecified`.
pub type ConfirmSetupIntentPaymentMethodDataAllowRedisplay = stripe_shared::CardAllowRedisplay;
/// The customer's bank.
pub type ConfirmSetupIntentPaymentMethodDataEpsBank = stripe_shared::Bank;
/// Account holder type for FPX transaction
pub type ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType =
    stripe_shared::IssuingCardholderType;
/// The customer's bank.
pub type ConfirmSetupIntentPaymentMethodDataFpxBank = stripe_shared::PaymentMethodFpxBank;
/// The customer's bank.
/// Only use this parameter for existing customers.
/// Don't use it for new customers.
pub type ConfirmSetupIntentPaymentMethodDataIdealBank = stripe_shared::PaymentMethodIdealBank;
/// Whether to use Naver Pay points or a card to fund this transaction.
/// If not provided, this defaults to `card`.
pub type ConfirmSetupIntentPaymentMethodDataNaverPayFunding = stripe_shared::Funding;
/// The customer's bank.
pub type ConfirmSetupIntentPaymentMethodDataP24Bank = stripe_shared::PaymentMethodP24Bank;
/// Two-letter ISO code representing the country the bank account is located in.
pub type ConfirmSetupIntentPaymentMethodDataSofortCountry = stripe_shared::Country;
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
pub type ConfirmSetupIntentPaymentMethodDataType =
    stripe_shared::CreateSetupIntentPaymentMethodDataType;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type ConfirmSetupIntentPaymentMethodDataUpiMandateOptionsAmountType = stripe_shared::AmountType;
/// Account holder type: individual or company.
pub type ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType =
    stripe_shared::IssuingCardholderType;
/// Account type: checkings or savings. Defaults to checking if omitted.
pub type ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType =
    stripe_shared::PaymentMethodUsBankAccountAccountType;
/// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
/// Must be a [supported currency](https://stripe.com/docs/currencies).
pub type ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency = stripe_shared::Currency;
/// List of Stripe products where this mandate can be selected automatically.
pub type ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor =
    stripe_shared::DefaultFor;
/// Payment schedule for the mandate.
pub type ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule =
    stripe_shared::PaymentSchedule;
/// Transaction type of the mandate.
pub type ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType =
    stripe_shared::TransactionType;
/// Bank account verification method. The default value is `automatic`.
pub type ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod =
    stripe_shared::VerificationMethod;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType =
    stripe_shared::AmountType;
/// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
pub type ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval =
    stripe_shared::PaymentMethodOptionsCardMandateOptionsInterval;
/// Specifies the type of mandates supported. Possible values are `india`.
pub type ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes =
    stripe_shared::SupportedTypes;
/// Selected network to process this SetupIntent on.
/// Depends on the available networks of the card attached to the SetupIntent.
/// Can be only set confirm-time.
pub type ConfirmSetupIntentPaymentMethodOptionsCardNetwork =
    stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork;
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// If not provided, this value defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
pub type ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure =
    stripe_shared::RequestThreeDSecure;
/// The `transStatus` returned from the card Issuer’s ACS in the ARes.
pub type ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus =
    stripe_shared::AresTransStatus;
/// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
/// provider and indicates what degree of authentication was performed.
pub type ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator =
    stripe_shared::ElectronicCommerceIndicator;
/// The cryptogram calculation algorithm used by the card Issuer's ACS
/// to calculate the Authentication cryptogram. Also known as `cavvAlgorithm`.
/// messageExtension: CB-AVALGO
pub type ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo =
    stripe_shared::CbAvalgo;
/// The version of 3D Secure that was performed.
pub type ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureVersion =
    stripe_shared::ThreeDSecureDetailsVersion;
/// Interval at which the customer is making purchases
pub type ConfirmSetupIntentPaymentMethodOptionsKlarnaOnDemandPurchaseInterval =
    stripe_shared::PlanInterval;
/// Preferred language of the Klarna authorization page that the customer is redirected to
pub type ConfirmSetupIntentPaymentMethodOptionsKlarnaPreferredLocale =
    stripe_shared::CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale;
/// Unit of time between subscription charges.
pub type ConfirmSetupIntentPaymentMethodOptionsKlarnaSubscriptionsInterval =
    stripe_shared::PlanInterval;
/// The type of amount that will be collected.
/// The amount charged must be exact or up to the value of `amount` param for `fixed` or `maximum` type respectively.
/// Defaults to `maximum`.
pub type ConfirmSetupIntentPaymentMethodOptionsPaytoMandateOptionsAmountType =
    stripe_shared::AmountType;
/// The periodicity at which payments will be collected. Defaults to `adhoc`.
pub type ConfirmSetupIntentPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule =
    stripe_shared::MandatePaytoPaymentSchedule;
/// The purpose for which payments are made. Has a default value based on your merchant category code.
pub type ConfirmSetupIntentPaymentMethodOptionsPaytoMandateOptionsPurpose = stripe_shared::Purpose;
/// Determines if the amount includes the IOF tax. Defaults to `never`.
pub type ConfirmSetupIntentPaymentMethodOptionsPixMandateOptionsAmountIncludesIof =
    stripe_shared::MandatePixAmountIncludesIof;
/// Type of amount. Defaults to `maximum`.
pub type ConfirmSetupIntentPaymentMethodOptionsPixMandateOptionsAmountType =
    stripe_shared::AmountType;
/// Schedule at which the future payments will be charged. Defaults to `monthly`.
pub type ConfirmSetupIntentPaymentMethodOptionsPixMandateOptionsPaymentSchedule =
    stripe_shared::MandatePixPaymentSchedule;
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type ConfirmSetupIntentPaymentMethodOptionsUpiMandateOptionsAmountType =
    stripe_shared::AmountType;
pub type ConfirmSetupIntentPaymentMethodOptionsUpiSetupFutureUsage =
    stripe_shared::SetupFutureUsage;
/// The account subcategories to use to filter for selectable accounts.
/// Valid subcategories are `checking` and `savings`.
pub type ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategories =
    stripe_shared::PaymentMethodUsBankAccountAccountType;
/// The list of permissions to request.
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
pub type ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions =
    stripe_shared::Permissions;
/// List of data features that you would like to retrieve upon account creation.
pub type ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch =
    stripe_shared::Prefetch;
/// The method used to collect offline mandate customer acceptance.
pub type ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod =
    stripe_shared::MandateUsBankAccountCollectionMethod;
/// Triggers validations to run across the selected networks
pub type ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested =
    stripe_shared::UsBankAccountNetworksSupported;
/// Bank account verification method. The default value is `automatic`.
pub type ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod =
    stripe_shared::VerificationMethod;
/// If this is a Mandate accepted online, this hash contains details about the online acceptance.
pub type CreateSetupIntentMandateDataCustomerAcceptanceOnline = OnlineParam;
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
pub type CreateSetupIntentPaymentMethodDataAcssDebit = PaymentMethodParam;
/// Billing address.
pub type CreateSetupIntentPaymentMethodDataBillingDetailsAddress = BillingDetailsAddress;
/// Customer's date of birth
pub type CreateSetupIntentPaymentMethodDataKlarnaDob = DateOfBirth;
/// Options to configure Radar.
/// See [Radar Session](https://docs.stripe.com/radar/radar-session) for more information.
pub type CreateSetupIntentPaymentMethodDataRadarOptions = RadarOptionsWithHiddenOptions;
/// Additional fields for Mandate creation
pub type CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions =
    SetupIntentPaymentMethodOptionsMandateOptionsParam;
/// Additional fields for Mandate creation
pub type CreateSetupIntentPaymentMethodOptionsBacsDebitMandateOptions =
    PaymentMethodOptionsMandateOptionsParam;
/// Configuration options for setting up an eMandate for cards issued in India.
pub type CreateSetupIntentPaymentMethodOptionsCardMandateOptions = SetupIntentMandateOptionsParam;
/// Cartes Bancaires-specific 3DS fields.
pub type CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires =
    CartesBancairesNetworkOptionsParam;
/// On-demand details if setting up a payment method for on-demand payments.
pub type CreateSetupIntentPaymentMethodOptionsKlarnaOnDemand = OnDemandParam;
/// Describes the upcoming charge for this subscription.
pub type CreateSetupIntentPaymentMethodOptionsKlarnaSubscriptionsNextBilling =
    SubscriptionNextBillingParam;
/// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options (Link is also known as Onelink in the UK).
pub type CreateSetupIntentPaymentMethodOptionsLink = SetupIntentPaymentMethodOptionsParam;
/// If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options.
pub type CreateSetupIntentPaymentMethodOptionsPaypal = PaymentMethodOptionsParam;
/// Provide filters for the linked accounts that the customer can select for the payment method.
pub type CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters =
    LinkedAccountOptionsFiltersParam;
/// Additional fields for Mandate creation
pub type CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions = MandateOptionsParam;
/// Additional fields for network related functions
pub type CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworks = NetworksOptionsParam;
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
pub type UpdateSetupIntentPaymentMethodDataAcssDebit = PaymentMethodParam;
/// Billing address.
pub type UpdateSetupIntentPaymentMethodDataBillingDetailsAddress = BillingDetailsAddress;
/// Customer's date of birth
pub type UpdateSetupIntentPaymentMethodDataKlarnaDob = DateOfBirth;
/// Options to configure Radar.
/// See [Radar Session](https://docs.stripe.com/radar/radar-session) for more information.
pub type UpdateSetupIntentPaymentMethodDataRadarOptions = RadarOptionsWithHiddenOptions;
/// Additional fields for Mandate creation
pub type UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions =
    SetupIntentPaymentMethodOptionsMandateOptionsParam;
/// Additional fields for Mandate creation
pub type UpdateSetupIntentPaymentMethodOptionsBacsDebitMandateOptions =
    PaymentMethodOptionsMandateOptionsParam;
/// Configuration options for setting up an eMandate for cards issued in India.
pub type UpdateSetupIntentPaymentMethodOptionsCardMandateOptions = SetupIntentMandateOptionsParam;
/// Cartes Bancaires-specific 3DS fields.
pub type UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires =
    CartesBancairesNetworkOptionsParam;
/// On-demand details if setting up a payment method for on-demand payments.
pub type UpdateSetupIntentPaymentMethodOptionsKlarnaOnDemand = OnDemandParam;
/// Describes the upcoming charge for this subscription.
pub type UpdateSetupIntentPaymentMethodOptionsKlarnaSubscriptionsNextBilling =
    SubscriptionNextBillingParam;
/// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options (Link is also known as Onelink in the UK).
pub type UpdateSetupIntentPaymentMethodOptionsLink = SetupIntentPaymentMethodOptionsParam;
/// If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options.
pub type UpdateSetupIntentPaymentMethodOptionsPaypal = PaymentMethodOptionsParam;
/// Provide filters for the linked accounts that the customer can select for the payment method.
pub type UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters =
    LinkedAccountOptionsFiltersParam;
/// Additional fields for Mandate creation
pub type UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions = MandateOptionsParam;
/// Additional fields for network related functions
pub type UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworks = NetworksOptionsParam;
/// If this is a Mandate accepted online, this hash contains details about the online acceptance.
pub type ConfirmSetupIntentSecretKeyParamCustomerAcceptanceOnline = OnlineParam;
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
pub type ConfirmSetupIntentPaymentMethodDataAcssDebit = PaymentMethodParam;
/// Billing address.
pub type ConfirmSetupIntentPaymentMethodDataBillingDetailsAddress = BillingDetailsAddress;
/// Customer's date of birth
pub type ConfirmSetupIntentPaymentMethodDataKlarnaDob = DateOfBirth;
/// Options to configure Radar.
/// See [Radar Session](https://docs.stripe.com/radar/radar-session) for more information.
pub type ConfirmSetupIntentPaymentMethodDataRadarOptions = RadarOptionsWithHiddenOptions;
/// Additional fields for Mandate creation
pub type ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptions =
    SetupIntentPaymentMethodOptionsMandateOptionsParam;
/// Additional fields for Mandate creation
pub type ConfirmSetupIntentPaymentMethodOptionsBacsDebitMandateOptions =
    PaymentMethodOptionsMandateOptionsParam;
/// Configuration options for setting up an eMandate for cards issued in India.
pub type ConfirmSetupIntentPaymentMethodOptionsCardMandateOptions = SetupIntentMandateOptionsParam;
/// Cartes Bancaires-specific 3DS fields.
pub type ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires =
    CartesBancairesNetworkOptionsParam;
/// On-demand details if setting up a payment method for on-demand payments.
pub type ConfirmSetupIntentPaymentMethodOptionsKlarnaOnDemand = OnDemandParam;
/// Describes the upcoming charge for this subscription.
pub type ConfirmSetupIntentPaymentMethodOptionsKlarnaSubscriptionsNextBilling =
    SubscriptionNextBillingParam;
/// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options (Link is also known as Onelink in the UK).
pub type ConfirmSetupIntentPaymentMethodOptionsLink = SetupIntentPaymentMethodOptionsParam;
/// If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options.
pub type ConfirmSetupIntentPaymentMethodOptionsPaypal = PaymentMethodOptionsParam;
/// Provide filters for the linked accounts that the customer can select for the payment method.
pub type ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters =
    LinkedAccountOptionsFiltersParam;
/// Additional fields for Mandate creation
pub type ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions = MandateOptionsParam;
/// Additional fields for network related functions
pub type ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworks = NetworksOptionsParam;
/// This hash contains details about the customer acceptance of the Mandate.
pub type CreateSetupIntentMandateDataCustomerAcceptance = CustomerAcceptanceParam;
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
pub type CreateSetupIntentPaymentMethodDataBillingDetails = BillingDetailsInnerParams;
/// Network specific 3DS fields. Network specific arguments require an
/// explicit card brand choice. The parameter `payment_method_options.card.network``
/// must be populated accordingly
pub type CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions = NetworkOptionsParam;
/// Subscription details if setting up or charging a subscription
pub type CreateSetupIntentPaymentMethodOptionsKlarnaSubscriptions = SetupIntentSubscriptionParam;
/// Additional fields for Financial Connections Session creation
pub type CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections =
    LinkedAccountOptionsParam;
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
pub type UpdateSetupIntentPaymentMethodDataBillingDetails = BillingDetailsInnerParams;
/// Network specific 3DS fields. Network specific arguments require an
/// explicit card brand choice. The parameter `payment_method_options.card.network``
/// must be populated accordingly
pub type UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions = NetworkOptionsParam;
/// Subscription details if setting up or charging a subscription
pub type UpdateSetupIntentPaymentMethodOptionsKlarnaSubscriptions = SetupIntentSubscriptionParam;
/// Additional fields for Financial Connections Session creation
pub type UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections =
    LinkedAccountOptionsParam;
/// This hash contains details about the customer acceptance of the Mandate.
pub type ConfirmSetupIntentSecretKeyParamCustomerAcceptance = CustomerAcceptanceParam;
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
pub type ConfirmSetupIntentPaymentMethodDataBillingDetails = BillingDetailsInnerParams;
/// Network specific 3DS fields. Network specific arguments require an
/// explicit card brand choice. The parameter `payment_method_options.card.network``
/// must be populated accordingly
pub type ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions = NetworkOptionsParam;
/// Subscription details if setting up or charging a subscription
pub type ConfirmSetupIntentPaymentMethodOptionsKlarnaSubscriptions = SetupIntentSubscriptionParam;
/// Additional fields for Financial Connections Session creation
pub type ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections =
    LinkedAccountOptionsParam;
/// This hash contains details about the mandate to create.
/// This parameter can only be used with [`confirm=true`](https://docs.stripe.com/api/setup_intents/create#create_setup_intent-confirm).
pub type CreateSetupIntentMandateData = SecretKeyParam;
pub type ConfirmSetupIntentSecretKeyParam = SecretKeyParam;
