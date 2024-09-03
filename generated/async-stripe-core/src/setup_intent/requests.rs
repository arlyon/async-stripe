use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListSetupIntentBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    attach_to_self: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListSetupIntentBuilder<'a> {
    fn new() -> Self {
        Self {
            attach_to_self: None,
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_method: None,
            starting_after: None,
        }
    }
}
/// Returns a list of SetupIntents.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListSetupIntent<'a> {
    inner: ListSetupIntentBuilder<'a>,
}
impl<'a> ListSetupIntent<'a> {
    /// Construct a new `ListSetupIntent`.
    pub fn new() -> Self {
        Self { inner: ListSetupIntentBuilder::new() }
    }
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    pub fn attach_to_self(mut self, attach_to_self: bool) -> Self {
        self.inner.attach_to_self = Some(attach_to_self);
        self
    }
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// Only return SetupIntents for the customer specified by this customer ID.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// Only return SetupIntents that associate with the specified payment method.
    pub fn payment_method(mut self, payment_method: &'a str) -> Self {
        self.inner.payment_method = Some(payment_method);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListSetupIntent<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListSetupIntent<'_> {
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
        stripe_client_core::ListPaginator::new_list("/setup_intents", self.inner)
    }
}

impl StripeRequest for ListSetupIntent<'_> {
    type Output = stripe_types::List<stripe_shared::SetupIntent>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/setup_intents").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveSetupIntentBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveSetupIntentBuilder<'a> {
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveSetupIntent<'a> {
    inner: RetrieveSetupIntentBuilder<'a>,
    intent: &'a stripe_shared::SetupIntentId,
}
impl<'a> RetrieveSetupIntent<'a> {
    /// Construct a new `RetrieveSetupIntent`.
    pub fn new(intent: &'a stripe_shared::SetupIntentId) -> Self {
        Self { intent, inner: RetrieveSetupIntentBuilder::new() }
    }
    /// The client secret of the SetupIntent.
    /// We require this string if you use a publishable key to retrieve the SetupIntent.
    pub fn client_secret(mut self, client_secret: &'a str) -> Self {
        self.inner.client_secret = Some(client_secret);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveSetupIntent<'_> {
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

impl StripeRequest for RetrieveSetupIntent<'_> {
    type Output = stripe_shared::SetupIntent;

    fn build(&self) -> RequestBuilder {
        let intent = self.intent;
        RequestBuilder::new(StripeMethod::Get, format!("/setup_intents/{intent}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateSetupIntentBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    attach_to_self: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_payment_methods: Option<CreateSetupIntentAutomaticPaymentMethods>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirmation_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_directions: Option<&'a [stripe_shared::SetupIntentFlowDirections]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mandate_data: Option<CreateSetupIntentMandateData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_configuration: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_data: Option<CreateSetupIntentPaymentMethodData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_options: Option<CreateSetupIntentPaymentMethodOptions<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_types: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_use: Option<CreateSetupIntentSingleUse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<CreateSetupIntentUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_stripe_sdk: Option<bool>,
}
impl<'a> CreateSetupIntentBuilder<'a> {
    fn new() -> Self {
        Self {
            attach_to_self: None,
            automatic_payment_methods: None,
            confirm: None,
            confirmation_token: None,
            customer: None,
            description: None,
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentAutomaticPaymentMethods {
    /// Controls whether this SetupIntent will accept redirect-based payment methods.
    ///
    /// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
    /// To [confirm](https://stripe.com/docs/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redirects: Option<CreateSetupIntentAutomaticPaymentMethodsAllowRedirects>,
    /// Whether this feature is enabled.
    pub enabled: bool,
}
impl CreateSetupIntentAutomaticPaymentMethods {
    pub fn new(enabled: bool) -> Self {
        Self { allow_redirects: None, enabled }
    }
}
/// Controls whether this SetupIntent will accept redirect-based payment methods.
///
/// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
/// To [confirm](https://stripe.com/docs/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentAutomaticPaymentMethodsAllowRedirects {
    Always,
    Never,
}
impl CreateSetupIntentAutomaticPaymentMethodsAllowRedirects {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentAutomaticPaymentMethodsAllowRedirects::*;
        match self {
            Always => "always",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentAutomaticPaymentMethodsAllowRedirects {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentAutomaticPaymentMethodsAllowRedirects::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentAutomaticPaymentMethodsAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentAutomaticPaymentMethodsAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentAutomaticPaymentMethodsAllowRedirects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentAutomaticPaymentMethodsAllowRedirects {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSetupIntentAutomaticPaymentMethodsAllowRedirects",
            )
        })
    }
}
/// This hash contains details about the mandate to create.
/// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/setup_intents/create#create_setup_intent-confirm).
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentMandateData<'a> {
    /// This hash contains details about the customer acceptance of the Mandate.
    pub customer_acceptance: CreateSetupIntentMandateDataCustomerAcceptance<'a>,
}
impl<'a> CreateSetupIntentMandateData<'a> {
    pub fn new(customer_acceptance: CreateSetupIntentMandateDataCustomerAcceptance<'a>) -> Self {
        Self { customer_acceptance }
    }
}
/// This hash contains details about the customer acceptance of the Mandate.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentMandateDataCustomerAcceptance<'a> {
    /// The time at which the customer accepted the Mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<stripe_types::Timestamp>,
    /// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub offline: Option<miniserde::json::Value>,
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<OnlineParam<'a>>,
    /// The type of customer acceptance information included with the Mandate.
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: CreateSetupIntentMandateDataCustomerAcceptanceType,
}
impl<'a> CreateSetupIntentMandateDataCustomerAcceptance<'a> {
    pub fn new(type_: CreateSetupIntentMandateDataCustomerAcceptanceType) -> Self {
        Self { accepted_at: None, offline: None, online: None, type_ }
    }
}
/// The type of customer acceptance information included with the Mandate.
/// One of `online` or `offline`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentMandateDataCustomerAcceptanceType {
    Offline,
    Online,
}
impl CreateSetupIntentMandateDataCustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentMandateDataCustomerAcceptanceType::*;
        match self {
            Offline => "offline",
            Online => "online",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentMandateDataCustomerAcceptanceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentMandateDataCustomerAcceptanceType::*;
        match s {
            "offline" => Ok(Offline),
            "online" => Ok(Online),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentMandateDataCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentMandateDataCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentMandateDataCustomerAcceptanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentMandateDataCustomerAcceptanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSetupIntentMandateDataCustomerAcceptanceType",
            )
        })
    }
}
/// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method).
/// value in the SetupIntent.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodData<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodParam<'a>>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub affirm: Option<miniserde::json::Value>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub afterpay_clearpay: Option<miniserde::json::Value>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub alipay: Option<miniserde::json::Value>,
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<CreateSetupIntentPaymentMethodDataAllowRedisplay>,
    /// If this is a AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreateSetupIntentPaymentMethodDataAuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreateSetupIntentPaymentMethodDataBacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub bancontact: Option<miniserde::json::Value>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetailsInnerParams<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub blik: Option<miniserde::json::Value>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreateSetupIntentPaymentMethodDataBoleto<'a>>,
    /// If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub cashapp: Option<miniserde::json::Value>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub customer_balance: Option<miniserde::json::Value>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreateSetupIntentPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreateSetupIntentPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub giropay: Option<miniserde::json::Value>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub grabpay: Option<miniserde::json::Value>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreateSetupIntentPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub interac_present: Option<miniserde::json::Value>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateSetupIntentPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub konbini: Option<miniserde::json::Value>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub link: Option<miniserde::json::Value>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub mobilepay: Option<miniserde::json::Value>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub oxxo: Option<miniserde::json::Value>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreateSetupIntentPaymentMethodDataP24>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub paynow: Option<miniserde::json::Value>,
    /// If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub paypal: Option<miniserde::json::Value>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub pix: Option<miniserde::json::Value>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub promptpay: Option<miniserde::json::Value>,
    /// Options to configure Radar.
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<RadarOptionsWithHiddenOptions<'a>>,
    /// If this is a `Revolut Pay` PaymentMethod, this hash contains details about the Revolut Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub revolut_pay: Option<miniserde::json::Value>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateSetupIntentPaymentMethodDataSepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateSetupIntentPaymentMethodDataSofort>,
    /// If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub swish: Option<miniserde::json::Value>,
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: CreateSetupIntentPaymentMethodDataType,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateSetupIntentPaymentMethodDataUsBankAccount<'a>>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub wechat_pay: Option<miniserde::json::Value>,
    /// If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub zip: Option<miniserde::json::Value>,
}
impl<'a> CreateSetupIntentPaymentMethodData<'a> {
    pub fn new(type_: CreateSetupIntentPaymentMethodDataType) -> Self {
        Self {
            acss_debit: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            allow_redisplay: None,
            amazon_pay: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            billing_details: None,
            blik: None,
            boleto: None,
            cashapp: None,
            customer_balance: None,
            eps: None,
            fpx: None,
            giropay: None,
            grabpay: None,
            ideal: None,
            interac_present: None,
            klarna: None,
            konbini: None,
            link: None,
            metadata: None,
            mobilepay: None,
            oxxo: None,
            p24: None,
            paynow: None,
            paypal: None,
            pix: None,
            promptpay: None,
            radar_options: None,
            revolut_pay: None,
            sepa_debit: None,
            sofort: None,
            swish: None,
            type_,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodDataAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl CreateSetupIntentPaymentMethodDataAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodDataAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodDataAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodDataAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodDataAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodDataAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodDataAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSetupIntentPaymentMethodDataAllowRedisplay",
            )
        })
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataAuBecsDebit<'a> {
    /// The account number for the bank account.
    pub account_number: &'a str,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: &'a str,
}
impl<'a> CreateSetupIntentPaymentMethodDataAuBecsDebit<'a> {
    pub fn new(account_number: &'a str, bsb_number: &'a str) -> Self {
        Self { account_number, bsb_number }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBacsDebit<'a> {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Sort code of the bank account. (e.g., `10-20-30`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<&'a str>,
}
impl<'a> CreateSetupIntentPaymentMethodDataBacsDebit<'a> {
    pub fn new() -> Self {
        Self { account_number: None, sort_code: None }
    }
}
impl<'a> Default for CreateSetupIntentPaymentMethodDataBacsDebit<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBoleto<'a> {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers)
    pub tax_id: &'a str,
}
impl<'a> CreateSetupIntentPaymentMethodDataBoleto<'a> {
    pub fn new(tax_id: &'a str) -> Self {
        Self { tax_id }
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateSetupIntentPaymentMethodDataEpsBank>,
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
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSetupIntentPaymentMethodDataEpsBank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateSetupIntentPaymentMethodDataEpsBank {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodDataEpsBank::*;
        match self {
            ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            BankAustria => "bank_austria",
            BankhausCarlSpangler => "bankhaus_carl_spangler",
            BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            BawagPskAg => "bawag_psk_ag",
            BksBankAg => "bks_bank_ag",
            BrullKallmusBankAg => "brull_kallmus_bank_ag",
            BtvVierLanderBank => "btv_vier_lander_bank",
            CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            DeutscheBankAg => "deutsche_bank_ag",
            Dolomitenbank => "dolomitenbank",
            EasybankAg => "easybank_ag",
            ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            HypoTirolBankAg => "hypo_tirol_bank_ag",
            HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            MarchfelderBank => "marchfelder_bank",
            OberbankAg => "oberbank_ag",
            RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            SchoellerbankAg => "schoellerbank_ag",
            SpardaBankWien => "sparda_bank_wien",
            VolksbankGruppe => "volksbank_gruppe",
            VolkskreditbankAg => "volkskreditbank_ag",
            VrBankBraunau => "vr_bank_braunau",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodDataEpsBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodDataEpsBank::*;
        match s {
            "arzte_und_apotheker_bank" => Ok(ArzteUndApothekerBank),
            "austrian_anadi_bank_ag" => Ok(AustrianAnadiBankAg),
            "bank_austria" => Ok(BankAustria),
            "bankhaus_carl_spangler" => Ok(BankhausCarlSpangler),
            "bankhaus_schelhammer_und_schattera_ag" => Ok(BankhausSchelhammerUndSchatteraAg),
            "bawag_psk_ag" => Ok(BawagPskAg),
            "bks_bank_ag" => Ok(BksBankAg),
            "brull_kallmus_bank_ag" => Ok(BrullKallmusBankAg),
            "btv_vier_lander_bank" => Ok(BtvVierLanderBank),
            "capital_bank_grawe_gruppe_ag" => Ok(CapitalBankGraweGruppeAg),
            "deutsche_bank_ag" => Ok(DeutscheBankAg),
            "dolomitenbank" => Ok(Dolomitenbank),
            "easybank_ag" => Ok(EasybankAg),
            "erste_bank_und_sparkassen" => Ok(ErsteBankUndSparkassen),
            "hypo_alpeadriabank_international_ag" => Ok(HypoAlpeadriabankInternationalAg),
            "hypo_bank_burgenland_aktiengesellschaft" => Ok(HypoBankBurgenlandAktiengesellschaft),
            "hypo_noe_lb_fur_niederosterreich_u_wien" => Ok(HypoNoeLbFurNiederosterreichUWien),
            "hypo_oberosterreich_salzburg_steiermark" => Ok(HypoOberosterreichSalzburgSteiermark),
            "hypo_tirol_bank_ag" => Ok(HypoTirolBankAg),
            "hypo_vorarlberg_bank_ag" => Ok(HypoVorarlbergBankAg),
            "marchfelder_bank" => Ok(MarchfelderBank),
            "oberbank_ag" => Ok(OberbankAg),
            "raiffeisen_bankengruppe_osterreich" => Ok(RaiffeisenBankengruppeOsterreich),
            "schoellerbank_ag" => Ok(SchoellerbankAg),
            "sparda_bank_wien" => Ok(SpardaBankWien),
            "volksbank_gruppe" => Ok(VolksbankGruppe),
            "volkskreditbank_ag" => Ok(VolkskreditbankAg),
            "vr_bank_braunau" => Ok(VrBankBraunau),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodDataEpsBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodDataEpsBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreateSetupIntentPaymentMethodDataFpxAccountHolderType>,
    /// The customer's bank.
    pub bank: CreateSetupIntentPaymentMethodDataFpxBank,
}
impl CreateSetupIntentPaymentMethodDataFpx {
    pub fn new(bank: CreateSetupIntentPaymentMethodDataFpxBank) -> Self {
        Self { account_holder_type: None, bank }
    }
}
/// Account holder type for FPX transaction
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
}
impl CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodDataFpxAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodDataFpxAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSetupIntentPaymentMethodDataFpxAccountHolderType",
            )
        })
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSetupIntentPaymentMethodDataFpxBank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateSetupIntentPaymentMethodDataFpxBank {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodDataFpxBank::*;
        match self {
            AffinBank => "affin_bank",
            Agrobank => "agrobank",
            AllianceBank => "alliance_bank",
            Ambank => "ambank",
            BankIslam => "bank_islam",
            BankMuamalat => "bank_muamalat",
            BankOfChina => "bank_of_china",
            BankRakyat => "bank_rakyat",
            Bsn => "bsn",
            Cimb => "cimb",
            DeutscheBank => "deutsche_bank",
            HongLeongBank => "hong_leong_bank",
            Hsbc => "hsbc",
            Kfh => "kfh",
            Maybank2e => "maybank2e",
            Maybank2u => "maybank2u",
            Ocbc => "ocbc",
            PbEnterprise => "pb_enterprise",
            PublicBank => "public_bank",
            Rhb => "rhb",
            StandardChartered => "standard_chartered",
            Uob => "uob",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodDataFpxBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodDataFpxBank::*;
        match s {
            "affin_bank" => Ok(AffinBank),
            "agrobank" => Ok(Agrobank),
            "alliance_bank" => Ok(AllianceBank),
            "ambank" => Ok(Ambank),
            "bank_islam" => Ok(BankIslam),
            "bank_muamalat" => Ok(BankMuamalat),
            "bank_of_china" => Ok(BankOfChina),
            "bank_rakyat" => Ok(BankRakyat),
            "bsn" => Ok(Bsn),
            "cimb" => Ok(Cimb),
            "deutsche_bank" => Ok(DeutscheBank),
            "hong_leong_bank" => Ok(HongLeongBank),
            "hsbc" => Ok(Hsbc),
            "kfh" => Ok(Kfh),
            "maybank2e" => Ok(Maybank2e),
            "maybank2u" => Ok(Maybank2u),
            "ocbc" => Ok(Ocbc),
            "pb_enterprise" => Ok(PbEnterprise),
            "public_bank" => Ok(PublicBank),
            "rhb" => Ok(Rhb),
            "standard_chartered" => Ok(StandardChartered),
            "uob" => Ok(Uob),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodDataFpxBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodDataFpxBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateSetupIntentPaymentMethodDataIdealBank>,
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
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSetupIntentPaymentMethodDataIdealBank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateSetupIntentPaymentMethodDataIdealBank {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodDataIdealBank::*;
        match self {
            AbnAmro => "abn_amro",
            AsnBank => "asn_bank",
            Bunq => "bunq",
            Handelsbanken => "handelsbanken",
            Ing => "ing",
            Knab => "knab",
            Moneyou => "moneyou",
            N26 => "n26",
            Nn => "nn",
            Rabobank => "rabobank",
            Regiobank => "regiobank",
            Revolut => "revolut",
            SnsBank => "sns_bank",
            TriodosBank => "triodos_bank",
            VanLanschot => "van_lanschot",
            Yoursafe => "yoursafe",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodDataIdealBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodDataIdealBank::*;
        match s {
            "abn_amro" => Ok(AbnAmro),
            "asn_bank" => Ok(AsnBank),
            "bunq" => Ok(Bunq),
            "handelsbanken" => Ok(Handelsbanken),
            "ing" => Ok(Ing),
            "knab" => Ok(Knab),
            "moneyou" => Ok(Moneyou),
            "n26" => Ok(N26),
            "nn" => Ok(Nn),
            "rabobank" => Ok(Rabobank),
            "regiobank" => Ok(Regiobank),
            "revolut" => Ok(Revolut),
            "sns_bank" => Ok(SnsBank),
            "triodos_bank" => Ok(TriodosBank),
            "van_lanschot" => Ok(VanLanschot),
            "yoursafe" => Ok(Yoursafe),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodDataIdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodDataIdealBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataKlarna {
    /// Customer's date of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirth>,
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
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateSetupIntentPaymentMethodDataP24Bank>,
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
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSetupIntentPaymentMethodDataP24Bank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateSetupIntentPaymentMethodDataP24Bank {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodDataP24Bank::*;
        match self {
            AliorBank => "alior_bank",
            BankMillennium => "bank_millennium",
            BankNowyBfgSa => "bank_nowy_bfg_sa",
            BankPekaoSa => "bank_pekao_sa",
            BankiSpbdzielcze => "banki_spbdzielcze",
            Blik => "blik",
            BnpParibas => "bnp_paribas",
            Boz => "boz",
            CitiHandlowy => "citi_handlowy",
            CreditAgricole => "credit_agricole",
            Envelobank => "envelobank",
            EtransferPocztowy24 => "etransfer_pocztowy24",
            GetinBank => "getin_bank",
            Ideabank => "ideabank",
            Ing => "ing",
            Inteligo => "inteligo",
            MbankMtransfer => "mbank_mtransfer",
            NestPrzelew => "nest_przelew",
            NoblePay => "noble_pay",
            PbacZIpko => "pbac_z_ipko",
            PlusBank => "plus_bank",
            SantanderPrzelew24 => "santander_przelew24",
            TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            ToyotaBank => "toyota_bank",
            Velobank => "velobank",
            VolkswagenBank => "volkswagen_bank",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodDataP24Bank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodDataP24Bank::*;
        match s {
            "alior_bank" => Ok(AliorBank),
            "bank_millennium" => Ok(BankMillennium),
            "bank_nowy_bfg_sa" => Ok(BankNowyBfgSa),
            "bank_pekao_sa" => Ok(BankPekaoSa),
            "banki_spbdzielcze" => Ok(BankiSpbdzielcze),
            "blik" => Ok(Blik),
            "bnp_paribas" => Ok(BnpParibas),
            "boz" => Ok(Boz),
            "citi_handlowy" => Ok(CitiHandlowy),
            "credit_agricole" => Ok(CreditAgricole),
            "envelobank" => Ok(Envelobank),
            "etransfer_pocztowy24" => Ok(EtransferPocztowy24),
            "getin_bank" => Ok(GetinBank),
            "ideabank" => Ok(Ideabank),
            "ing" => Ok(Ing),
            "inteligo" => Ok(Inteligo),
            "mbank_mtransfer" => Ok(MbankMtransfer),
            "nest_przelew" => Ok(NestPrzelew),
            "noble_pay" => Ok(NoblePay),
            "pbac_z_ipko" => Ok(PbacZIpko),
            "plus_bank" => Ok(PlusBank),
            "santander_przelew24" => Ok(SantanderPrzelew24),
            "tmobile_usbugi_bankowe" => Ok(TmobileUsbugiBankowe),
            "toyota_bank" => Ok(ToyotaBank),
            "velobank" => Ok(Velobank),
            "volkswagen_bank" => Ok(VolkswagenBank),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodDataP24Bank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodDataP24Bank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataSepaDebit<'a> {
    /// IBAN of the bank account.
    pub iban: &'a str,
}
impl<'a> CreateSetupIntentPaymentMethodDataSepaDebit<'a> {
    pub fn new(iban: &'a str) -> Self {
        Self { iban }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: CreateSetupIntentPaymentMethodDataSofortCountry,
}
impl CreateSetupIntentPaymentMethodDataSofort {
    pub fn new(country: CreateSetupIntentPaymentMethodDataSofortCountry) -> Self {
        Self { country }
    }
}
/// Two-letter ISO code representing the country the bank account is located in.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodDataSofortCountry {
    At,
    Be,
    De,
    Es,
    It,
    Nl,
}
impl CreateSetupIntentPaymentMethodDataSofortCountry {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodDataSofortCountry::*;
        match self {
            At => "AT",
            Be => "BE",
            De => "DE",
            Es => "ES",
            It => "IT",
            Nl => "NL",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodDataSofortCountry {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodDataSofortCountry::*;
        match s {
            "AT" => Ok(At),
            "BE" => Ok(Be),
            "DE" => Ok(De),
            "ES" => Ok(Es),
            "IT" => Ok(It),
            "NL" => Ok(Nl),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodDataSofortCountry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodDataSofortCountry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSetupIntentPaymentMethodDataSofortCountry",
            )
        })
    }
}
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSetupIntentPaymentMethodDataType {
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
impl CreateSetupIntentPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodDataType::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            AmazonPay => "amazon_pay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Blik => "blik",
            Boleto => "boleto",
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
            Mobilepay => "mobilepay",
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

impl std::str::FromStr for CreateSetupIntentPaymentMethodDataType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodDataType::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "amazon_pay" => Ok(AmazonPay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
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
            "mobilepay" => Ok(Mobilepay),
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
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<CreateSetupIntentPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> CreateSetupIntentPaymentMethodDataUsBankAccount<'a> {
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
impl<'a> Default for CreateSetupIntentPaymentMethodDataUsBankAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}
impl CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType"))
    }
}
/// Account type: checkings or savings. Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}
impl CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodDataUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodDataUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSetupIntentPaymentMethodDataUsBankAccountAccountType",
            )
        })
    }
}
/// Payment method-specific configuration for this SetupIntent.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptions<'a> {
    /// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateSetupIntentPaymentMethodOptionsAcssDebit<'a>>,
    /// If this is a `amazon_pay` SetupIntent, this sub-hash contains details about the AmazonPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// Configuration for any card setup attempted on this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateSetupIntentPaymentMethodOptionsCard<'a>>,
    /// If this is a `card_present` PaymentMethod, this sub-hash contains details about the card-present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub card_present: Option<miniserde::json::Value>,
    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<SetupIntentPaymentMethodOptionsParam<'a>>,
    /// If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodOptionsParam<'a>>,
    /// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateSetupIntentPaymentMethodOptionsSepaDebit>,
    /// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateSetupIntentPaymentMethodOptionsUsBankAccount<'a>>,
}
impl<'a> CreateSetupIntentPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self {
            acss_debit: None,
            amazon_pay: None,
            card: None,
            card_present: None,
            link: None,
            paypal: None,
            sepa_debit: None,
            us_bank_account: None,
        }
    }
}
impl<'a> Default for CreateSetupIntentPaymentMethodOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self { currency: None, mandate_options: None, verification_method: None }
    }
}
impl<'a> Default for CreateSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
/// Must be a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
}
impl CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency::*;
        match self {
            Cad => "cad",
            Usd => "usd",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency::*;
        match s {
            "cad" => Ok(Cad),
            "usd" => Ok(Usd),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency",
            )
        })
    }
}
/// Additional fields for Mandate creation
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,.
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<&'a [CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor]>,
    /// Description of the mandate interval.
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
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
impl<'a> Default for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// List of Stripe products where this mandate can be selected automatically.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}
impl CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::*;
        match self {
            Invoice => "invoice",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::*;
        match s {
            "invoice" => Ok(Invoice),
            "subscription" => Ok(Subscription),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor"))
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}
impl CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
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
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule"))
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}
impl CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
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
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType"))
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod"))
    }
}
/// Configuration for any card setup attempted on this SetupIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCard<'a> {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsCardMandateOptions<'a>>,
    /// When specified, this parameter signals that a card has been collected
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA. This
    /// parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    /// Selected network to process this SetupIntent on.
    /// Depends on the available networks of the card attached to the SetupIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<CreateSetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,
    /// If 3D Secure authentication was performed with a third-party provider,
    /// the authentication details to use for this setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<CreateSetupIntentPaymentMethodOptionsCardThreeDSecure<'a>>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsCard<'a> {
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
impl<'a> Default for CreateSetupIntentPaymentMethodOptionsCard<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCardMandateOptions<'a> {
    /// Amount to be charged for future payments.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
    /// Currency in which future payments will be charged.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// End date of the mandate or subscription.
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<stripe_types::Timestamp>,
    /// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
    /// The number of intervals between payments.
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: &'a str,
    /// Start date of the mandate or subscription. Start date should not be lesser than yesterday.
    pub start_date: stripe_types::Timestamp,
    /// Specifies the type of mandates supported. Possible values are `india`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types:
        Option<&'a [CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes]>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new(
        amount: i64,
        amount_type: CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
        currency: stripe_types::Currency,
        interval: CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
        reference: &'a str,
        start_date: stripe_types::Timestamp,
    ) -> Self {
        Self {
            amount,
            amount_type,
            currency,
            description: None,
            end_date: None,
            interval,
            interval_count: None,
            reference,
            start_date,
            supported_types: None,
        }
    }
}
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}
impl CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType"))
    }
}
/// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}
impl CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Sporadic => "sporadic",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "sporadic" => Ok(Sporadic),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval",
            )
        })
    }
}
/// Specifies the type of mandates supported. Possible values are `india`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}
impl CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes::*;
        match self {
            India => "india",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes::*;
        match s {
            "india" => Ok(India),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes"))
    }
}
/// Selected network to process this SetupIntent on.
/// Depends on the available networks of the card attached to the SetupIntent.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}
impl CreateSetupIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsCardNetwork::*;
        match self {
            Amex => "amex",
            CartesBancaires => "cartes_bancaires",
            Diners => "diners",
            Discover => "discover",
            EftposAu => "eftpos_au",
            Interac => "interac",
            Jcb => "jcb",
            Mastercard => "mastercard",
            Unionpay => "unionpay",
            Unknown => "unknown",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsCardNetwork {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsCardNetwork::*;
        match s {
            "amex" => Ok(Amex),
            "cartes_bancaires" => Ok(CartesBancaires),
            "diners" => Ok(Diners),
            "discover" => Ok(Discover),
            "eftpos_au" => Ok(EftposAu),
            "interac" => Ok(Interac),
            "jcb" => Ok(Jcb),
            "mastercard" => Ok(Mastercard),
            "unionpay" => Ok(Unionpay),
            "unknown" => Ok(Unknown),
            "visa" => Ok(Visa),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsCardNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodOptionsCardNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSetupIntentPaymentMethodOptionsCardNetwork",
            )
        })
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// If not provided, this value defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}
impl CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
            Challenge => "challenge",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            "challenge" => Ok(Challenge),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure",
            )
        })
    }
}
/// If 3D Secure authentication was performed with a third-party provider,
/// the authentication details to use for this setup.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCardThreeDSecure<'a> {
    /// The `transStatus` returned from the card Issuer’s ACS in the ARes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ares_trans_status:
        Option<CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus>,
    /// The cryptogram, also known as the "authentication value" (AAV, CAVV or
    /// AEVV). This value is 20 bytes, base64-encoded into a 28-character string.
    /// (Most 3D Secure providers will return the base64-encoded version, which
    /// is what you should specify here.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptogram: Option<&'a str>,
    /// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
    /// provider and indicates what degree of authentication was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator:
        Option<CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator>,
    /// Network specific 3DS fields. Network specific arguments require an
    /// explicit card brand choice. The parameter `payment_method_options.card.network``
    /// must be populated accordingly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options:
        Option<CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions<'a>>,
    /// The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the
    /// AReq sent to the card Issuer's ACS. A string containing 2 digits from 01-99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_challenge_indicator: Option<&'a str>,
    /// For 3D Secure 1, the XID. For 3D Secure 2, the Directory Server
    /// Transaction ID (dsTransID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<&'a str>,
    /// The version of 3D Secure that was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsCardThreeDSecure<'a> {
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
impl<'a> Default for CreateSetupIntentPaymentMethodOptionsCardThreeDSecure<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The `transStatus` returned from the card Issuer’s ACS in the ARes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    A,
    C,
    I,
    N,
    R,
    U,
    Y,
}
impl CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::*;
        match self {
            A => "A",
            C => "C",
            I => "I",
            N => "N",
            R => "R",
            U => "U",
            Y => "Y",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::*;
        match s {
            "A" => Ok(A),
            "C" => Ok(C),
            "I" => Ok(I),
            "N" => Ok(N),
            "R" => Ok(R),
            "U" => Ok(U),
            "Y" => Ok(Y),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus"))
    }
}
/// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
/// provider and indicates what degree of authentication was performed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    V01,
    V02,
    V05,
    V06,
    V07,
}
impl CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::*;
        match self {
            V01 => "01",
            V02 => "02",
            V05 => "05",
            V06 => "06",
            V07 => "07",
        }
    }
}

impl std::str::FromStr
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::*;
        match s {
            "01" => Ok(V01),
            "02" => Ok(V02),
            "05" => Ok(V05),
            "06" => Ok(V06),
            "07" => Ok(V07),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
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
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator"))
    }
}
/// Network specific 3DS fields. Network specific arguments require an
/// explicit card brand choice. The parameter `payment_method_options.card.network``
/// must be populated accordingly
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions<'a> {
    /// Cartes Bancaires-specific 3DS fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires: Option<
        CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires<'a>,
    >,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions<'a> {
    pub fn new() -> Self {
        Self { cartes_bancaires: None }
    }
}
impl<'a> Default for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Cartes Bancaires-specific 3DS fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires<'a> {
    /// The cryptogram calculation algorithm used by the card Issuer's ACS
    /// to calculate the Authentication cryptogram. Also known as `cavvAlgorithm`.
    /// messageExtension: CB-AVALGO
    pub cb_avalgo:
        CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo,
    /// The exemption indicator returned from Cartes Bancaires in the ARes.
    /// message extension: CB-EXEMPTION; string (4 characters)
    /// This is a 3 byte bitmap (low significant byte first and most significant
    /// bit first) that has been Base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_exemption: Option<&'a str>,
    /// The risk score returned from Cartes Bancaires in the ARes.
    /// message extension: CB-SCORE; numeric value 0-99
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_score: Option<i64>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires<'a> {
    pub fn new(
        cb_avalgo: CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo,
    ) -> Self {
        Self { cb_avalgo, cb_exemption: None, cb_score: None }
    }
}
/// The cryptogram calculation algorithm used by the card Issuer's ACS
/// to calculate the Authentication cryptogram. Also known as `cavvAlgorithm`.
/// messageExtension: CB-AVALGO
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    V0,
    V1,
    V2,
    V3,
    V4,
    A,
}
impl CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::*;
        match self {
            V0 => "0",
            V1 => "1",
            V2 => "2",
            V3 => "3",
            V4 => "4",
            A => "A",
        }
    }
}

impl std::str::FromStr
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::*;
        match s {
            "0" => Ok(V0),
            "1" => Ok(V1),
            "2" => Ok(V2),
            "3" => Ok(V3),
            "4" => Ok(V4),
            "A" => Ok(A),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
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
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo"))
    }
}
/// The version of 3D Secure that was performed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    V1_0_2,
    V2_1_0,
    V2_2_0,
}
impl CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion::*;
        match self {
            V1_0_2 => "1.0.2",
            V2_1_0 => "2.1.0",
            V2_2_0 => "2.2.0",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion::*;
        match s {
            "1.0.2" => Ok(V1_0_2),
            "2.1.0" => Ok(V2_1_0),
            "2.2.0" => Ok(V2_2_0),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion",
            )
        })
    }
}
/// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub mandate_options: Option<miniserde::json::Value>,
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
/// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a>>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions>,
    /// Additional fields for network related functions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a>>,
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self {
            financial_connections: None,
            mandate_options: None,
            networks: None,
            verification_method: None,
        }
    }
}
impl<'a> Default for CreateSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Financial Connections Session creation
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        &'a [CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions],
    >,
    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<
        &'a [CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch],
    >,
    /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self { permissions: None, prefetch: None, return_url: None }
    }
}
impl<'a> Default for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The list of permissions to request.
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}
impl CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
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
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
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
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions"))
    }
}
/// List of data features that you would like to retrieve upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    Balances,
    Ownership,
    Transactions,
}
impl CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
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
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch"))
    }
}
/// Additional fields for Mandate creation
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions {
    /// The method used to collect offline mandate customer acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method:
        Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}
impl CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions {
    pub fn new() -> Self {
        Self { collection_method: None }
    }
}
impl Default for CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// The method used to collect offline mandate customer acceptance.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    Paper,
}
impl CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::*;
        match self {
            Paper => "paper",
        }
    }
}

impl std::str::FromStr
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::*;
        match s {
            "paper" => Ok(Paper),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
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
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod"))
    }
}
/// Additional fields for network related functions
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    /// Triggers validations to run across the selected networks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested:
        Option<&'a [CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested]>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl<'a> Default for CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Triggers validations to run across the selected networks
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    Ach,
    UsDomesticWire,
}
impl CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested"))
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod"))
    }
}
/// If you populate this hash, this SetupIntent generates a `single_use` mandate after successful completion.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentSingleUse {
    /// Amount the customer is granting permission to collect later.
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
}
impl CreateSetupIntentSingleUse {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency }
    }
}
/// Indicates how the payment method is intended to be used in the future.
/// If not provided, this value defaults to `off_session`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSetupIntentUsage {
    OffSession,
    OnSession,
}
impl CreateSetupIntentUsage {
    pub fn as_str(self) -> &'static str {
        use CreateSetupIntentUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreateSetupIntentUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSetupIntentUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSetupIntentUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSetupIntentUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateSetupIntentUsage"))
    }
}
/// Creates a SetupIntent object.
///
/// After you create the SetupIntent, attach a payment method and [confirm](https://stripe.com/docs/api/setup_intents/confirm).
/// it to collect any required permissions to charge the payment method later.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntent<'a> {
    inner: CreateSetupIntentBuilder<'a>,
}
impl<'a> CreateSetupIntent<'a> {
    /// Construct a new `CreateSetupIntent`.
    pub fn new() -> Self {
        Self { inner: CreateSetupIntentBuilder::new() }
    }
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    pub fn attach_to_self(mut self, attach_to_self: bool) -> Self {
        self.inner.attach_to_self = Some(attach_to_self);
        self
    }
    /// When you enable this parameter, this SetupIntent accepts payment methods that you enable in the Dashboard and that are compatible with its other parameters.
    pub fn automatic_payment_methods(
        mut self,
        automatic_payment_methods: CreateSetupIntentAutomaticPaymentMethods,
    ) -> Self {
        self.inner.automatic_payment_methods = Some(automatic_payment_methods);
        self
    }
    /// Set to `true` to attempt to confirm this SetupIntent immediately.
    /// This parameter defaults to `false`.
    /// If a card is the attached payment method, you can provide a `return_url` in case further authentication is necessary.
    pub fn confirm(mut self, confirm: bool) -> Self {
        self.inner.confirm = Some(confirm);
        self
    }
    /// ID of the ConfirmationToken used to confirm this SetupIntent.
    ///
    /// If the provided ConfirmationToken contains properties that are also being provided in this request, such as `payment_method`, then the values in this request will take precedence.
    pub fn confirmation_token(mut self, confirmation_token: &'a str) -> Self {
        self.inner.confirmation_token = Some(confirmation_token);
        self
    }
    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    pub fn flow_directions(
        mut self,
        flow_directions: &'a [stripe_shared::SetupIntentFlowDirections],
    ) -> Self {
        self.inner.flow_directions = Some(flow_directions);
        self
    }
    /// This hash contains details about the mandate to create.
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/setup_intents/create#create_setup_intent-confirm).
    pub fn mandate_data(mut self, mandate_data: CreateSetupIntentMandateData<'a>) -> Self {
        self.inner.mandate_data = Some(mandate_data);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The Stripe account ID created for this SetupIntent.
    pub fn on_behalf_of(mut self, on_behalf_of: &'a str) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of);
        self
    }
    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    pub fn payment_method(mut self, payment_method: &'a str) -> Self {
        self.inner.payment_method = Some(payment_method);
        self
    }
    /// The ID of the payment method configuration to use with this SetupIntent.
    pub fn payment_method_configuration(mut self, payment_method_configuration: &'a str) -> Self {
        self.inner.payment_method_configuration = Some(payment_method_configuration);
        self
    }
    /// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method).
    /// value in the SetupIntent.
    pub fn payment_method_data(
        mut self,
        payment_method_data: CreateSetupIntentPaymentMethodData<'a>,
    ) -> Self {
        self.inner.payment_method_data = Some(payment_method_data);
        self
    }
    /// Payment method-specific configuration for this SetupIntent.
    pub fn payment_method_options(
        mut self,
        payment_method_options: CreateSetupIntentPaymentMethodOptions<'a>,
    ) -> Self {
        self.inner.payment_method_options = Some(payment_method_options);
        self
    }
    /// The list of payment method types (for example, card) that this SetupIntent can use.
    /// If you don't provide this, it defaults to ["card"].
    pub fn payment_method_types(mut self, payment_method_types: &'a [&'a str]) -> Self {
        self.inner.payment_method_types = Some(payment_method_types);
        self
    }
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    /// To redirect to a mobile application, you can alternatively supply an application URI scheme.
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/setup_intents/create#create_setup_intent-confirm).
    pub fn return_url(mut self, return_url: &'a str) -> Self {
        self.inner.return_url = Some(return_url);
        self
    }
    /// If you populate this hash, this SetupIntent generates a `single_use` mandate after successful completion.
    pub fn single_use(mut self, single_use: CreateSetupIntentSingleUse) -> Self {
        self.inner.single_use = Some(single_use);
        self
    }
    /// Indicates how the payment method is intended to be used in the future.
    /// If not provided, this value defaults to `off_session`.
    pub fn usage(mut self, usage: CreateSetupIntentUsage) -> Self {
        self.inner.usage = Some(usage);
        self
    }
    /// Set to `true` when confirming server-side and using Stripe.js, iOS, or Android client-side SDKs to handle the next actions.
    pub fn use_stripe_sdk(mut self, use_stripe_sdk: bool) -> Self {
        self.inner.use_stripe_sdk = Some(use_stripe_sdk);
        self
    }
}
impl<'a> Default for CreateSetupIntent<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateSetupIntent<'_> {
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

impl StripeRequest for CreateSetupIntent<'_> {
    type Output = stripe_shared::SetupIntent;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/setup_intents").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateSetupIntentBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    attach_to_self: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_directions: Option<&'a [stripe_shared::SetupIntentFlowDirections]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_configuration: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_data: Option<UpdateSetupIntentPaymentMethodData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_options: Option<UpdateSetupIntentPaymentMethodOptions<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_types: Option<&'a [&'a str]>,
}
impl<'a> UpdateSetupIntentBuilder<'a> {
    fn new() -> Self {
        Self {
            attach_to_self: None,
            customer: None,
            description: None,
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
/// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method).
/// value in the SetupIntent.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodData<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodParam<'a>>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub affirm: Option<miniserde::json::Value>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub afterpay_clearpay: Option<miniserde::json::Value>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub alipay: Option<miniserde::json::Value>,
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<UpdateSetupIntentPaymentMethodDataAllowRedisplay>,
    /// If this is a AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<UpdateSetupIntentPaymentMethodDataAuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<UpdateSetupIntentPaymentMethodDataBacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub bancontact: Option<miniserde::json::Value>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetailsInnerParams<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub blik: Option<miniserde::json::Value>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<UpdateSetupIntentPaymentMethodDataBoleto<'a>>,
    /// If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub cashapp: Option<miniserde::json::Value>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub customer_balance: Option<miniserde::json::Value>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<UpdateSetupIntentPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<UpdateSetupIntentPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub giropay: Option<miniserde::json::Value>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub grabpay: Option<miniserde::json::Value>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<UpdateSetupIntentPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub interac_present: Option<miniserde::json::Value>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<UpdateSetupIntentPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub konbini: Option<miniserde::json::Value>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub link: Option<miniserde::json::Value>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub mobilepay: Option<miniserde::json::Value>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub oxxo: Option<miniserde::json::Value>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<UpdateSetupIntentPaymentMethodDataP24>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub paynow: Option<miniserde::json::Value>,
    /// If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub paypal: Option<miniserde::json::Value>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub pix: Option<miniserde::json::Value>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub promptpay: Option<miniserde::json::Value>,
    /// Options to configure Radar.
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<RadarOptionsWithHiddenOptions<'a>>,
    /// If this is a `Revolut Pay` PaymentMethod, this hash contains details about the Revolut Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub revolut_pay: Option<miniserde::json::Value>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdateSetupIntentPaymentMethodDataSepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<UpdateSetupIntentPaymentMethodDataSofort>,
    /// If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub swish: Option<miniserde::json::Value>,
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: UpdateSetupIntentPaymentMethodDataType,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdateSetupIntentPaymentMethodDataUsBankAccount<'a>>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub wechat_pay: Option<miniserde::json::Value>,
    /// If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub zip: Option<miniserde::json::Value>,
}
impl<'a> UpdateSetupIntentPaymentMethodData<'a> {
    pub fn new(type_: UpdateSetupIntentPaymentMethodDataType) -> Self {
        Self {
            acss_debit: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            allow_redisplay: None,
            amazon_pay: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            billing_details: None,
            blik: None,
            boleto: None,
            cashapp: None,
            customer_balance: None,
            eps: None,
            fpx: None,
            giropay: None,
            grabpay: None,
            ideal: None,
            interac_present: None,
            klarna: None,
            konbini: None,
            link: None,
            metadata: None,
            mobilepay: None,
            oxxo: None,
            p24: None,
            paynow: None,
            paypal: None,
            pix: None,
            promptpay: None,
            radar_options: None,
            revolut_pay: None,
            sepa_debit: None,
            sofort: None,
            swish: None,
            type_,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodDataAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl UpdateSetupIntentPaymentMethodDataAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodDataAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodDataAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodDataAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodDataAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodDataAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSetupIntentPaymentMethodDataAllowRedisplay",
            )
        })
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataAuBecsDebit<'a> {
    /// The account number for the bank account.
    pub account_number: &'a str,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: &'a str,
}
impl<'a> UpdateSetupIntentPaymentMethodDataAuBecsDebit<'a> {
    pub fn new(account_number: &'a str, bsb_number: &'a str) -> Self {
        Self { account_number, bsb_number }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBacsDebit<'a> {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Sort code of the bank account. (e.g., `10-20-30`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<&'a str>,
}
impl<'a> UpdateSetupIntentPaymentMethodDataBacsDebit<'a> {
    pub fn new() -> Self {
        Self { account_number: None, sort_code: None }
    }
}
impl<'a> Default for UpdateSetupIntentPaymentMethodDataBacsDebit<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBoleto<'a> {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers)
    pub tax_id: &'a str,
}
impl<'a> UpdateSetupIntentPaymentMethodDataBoleto<'a> {
    pub fn new(tax_id: &'a str) -> Self {
        Self { tax_id }
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<UpdateSetupIntentPaymentMethodDataEpsBank>,
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
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateSetupIntentPaymentMethodDataEpsBank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl UpdateSetupIntentPaymentMethodDataEpsBank {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodDataEpsBank::*;
        match self {
            ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            BankAustria => "bank_austria",
            BankhausCarlSpangler => "bankhaus_carl_spangler",
            BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            BawagPskAg => "bawag_psk_ag",
            BksBankAg => "bks_bank_ag",
            BrullKallmusBankAg => "brull_kallmus_bank_ag",
            BtvVierLanderBank => "btv_vier_lander_bank",
            CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            DeutscheBankAg => "deutsche_bank_ag",
            Dolomitenbank => "dolomitenbank",
            EasybankAg => "easybank_ag",
            ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            HypoTirolBankAg => "hypo_tirol_bank_ag",
            HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            MarchfelderBank => "marchfelder_bank",
            OberbankAg => "oberbank_ag",
            RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            SchoellerbankAg => "schoellerbank_ag",
            SpardaBankWien => "sparda_bank_wien",
            VolksbankGruppe => "volksbank_gruppe",
            VolkskreditbankAg => "volkskreditbank_ag",
            VrBankBraunau => "vr_bank_braunau",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodDataEpsBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodDataEpsBank::*;
        match s {
            "arzte_und_apotheker_bank" => Ok(ArzteUndApothekerBank),
            "austrian_anadi_bank_ag" => Ok(AustrianAnadiBankAg),
            "bank_austria" => Ok(BankAustria),
            "bankhaus_carl_spangler" => Ok(BankhausCarlSpangler),
            "bankhaus_schelhammer_und_schattera_ag" => Ok(BankhausSchelhammerUndSchatteraAg),
            "bawag_psk_ag" => Ok(BawagPskAg),
            "bks_bank_ag" => Ok(BksBankAg),
            "brull_kallmus_bank_ag" => Ok(BrullKallmusBankAg),
            "btv_vier_lander_bank" => Ok(BtvVierLanderBank),
            "capital_bank_grawe_gruppe_ag" => Ok(CapitalBankGraweGruppeAg),
            "deutsche_bank_ag" => Ok(DeutscheBankAg),
            "dolomitenbank" => Ok(Dolomitenbank),
            "easybank_ag" => Ok(EasybankAg),
            "erste_bank_und_sparkassen" => Ok(ErsteBankUndSparkassen),
            "hypo_alpeadriabank_international_ag" => Ok(HypoAlpeadriabankInternationalAg),
            "hypo_bank_burgenland_aktiengesellschaft" => Ok(HypoBankBurgenlandAktiengesellschaft),
            "hypo_noe_lb_fur_niederosterreich_u_wien" => Ok(HypoNoeLbFurNiederosterreichUWien),
            "hypo_oberosterreich_salzburg_steiermark" => Ok(HypoOberosterreichSalzburgSteiermark),
            "hypo_tirol_bank_ag" => Ok(HypoTirolBankAg),
            "hypo_vorarlberg_bank_ag" => Ok(HypoVorarlbergBankAg),
            "marchfelder_bank" => Ok(MarchfelderBank),
            "oberbank_ag" => Ok(OberbankAg),
            "raiffeisen_bankengruppe_osterreich" => Ok(RaiffeisenBankengruppeOsterreich),
            "schoellerbank_ag" => Ok(SchoellerbankAg),
            "sparda_bank_wien" => Ok(SpardaBankWien),
            "volksbank_gruppe" => Ok(VolksbankGruppe),
            "volkskreditbank_ag" => Ok(VolkskreditbankAg),
            "vr_bank_braunau" => Ok(VrBankBraunau),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodDataEpsBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodDataEpsBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdateSetupIntentPaymentMethodDataFpxAccountHolderType>,
    /// The customer's bank.
    pub bank: UpdateSetupIntentPaymentMethodDataFpxBank,
}
impl UpdateSetupIntentPaymentMethodDataFpx {
    pub fn new(bank: UpdateSetupIntentPaymentMethodDataFpxBank) -> Self {
        Self { account_holder_type: None, bank }
    }
}
/// Account holder type for FPX transaction
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
}
impl UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodDataFpxAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodDataFpxAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSetupIntentPaymentMethodDataFpxAccountHolderType",
            )
        })
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateSetupIntentPaymentMethodDataFpxBank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl UpdateSetupIntentPaymentMethodDataFpxBank {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodDataFpxBank::*;
        match self {
            AffinBank => "affin_bank",
            Agrobank => "agrobank",
            AllianceBank => "alliance_bank",
            Ambank => "ambank",
            BankIslam => "bank_islam",
            BankMuamalat => "bank_muamalat",
            BankOfChina => "bank_of_china",
            BankRakyat => "bank_rakyat",
            Bsn => "bsn",
            Cimb => "cimb",
            DeutscheBank => "deutsche_bank",
            HongLeongBank => "hong_leong_bank",
            Hsbc => "hsbc",
            Kfh => "kfh",
            Maybank2e => "maybank2e",
            Maybank2u => "maybank2u",
            Ocbc => "ocbc",
            PbEnterprise => "pb_enterprise",
            PublicBank => "public_bank",
            Rhb => "rhb",
            StandardChartered => "standard_chartered",
            Uob => "uob",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodDataFpxBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodDataFpxBank::*;
        match s {
            "affin_bank" => Ok(AffinBank),
            "agrobank" => Ok(Agrobank),
            "alliance_bank" => Ok(AllianceBank),
            "ambank" => Ok(Ambank),
            "bank_islam" => Ok(BankIslam),
            "bank_muamalat" => Ok(BankMuamalat),
            "bank_of_china" => Ok(BankOfChina),
            "bank_rakyat" => Ok(BankRakyat),
            "bsn" => Ok(Bsn),
            "cimb" => Ok(Cimb),
            "deutsche_bank" => Ok(DeutscheBank),
            "hong_leong_bank" => Ok(HongLeongBank),
            "hsbc" => Ok(Hsbc),
            "kfh" => Ok(Kfh),
            "maybank2e" => Ok(Maybank2e),
            "maybank2u" => Ok(Maybank2u),
            "ocbc" => Ok(Ocbc),
            "pb_enterprise" => Ok(PbEnterprise),
            "public_bank" => Ok(PublicBank),
            "rhb" => Ok(Rhb),
            "standard_chartered" => Ok(StandardChartered),
            "uob" => Ok(Uob),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodDataFpxBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodDataFpxBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<UpdateSetupIntentPaymentMethodDataIdealBank>,
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
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateSetupIntentPaymentMethodDataIdealBank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl UpdateSetupIntentPaymentMethodDataIdealBank {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodDataIdealBank::*;
        match self {
            AbnAmro => "abn_amro",
            AsnBank => "asn_bank",
            Bunq => "bunq",
            Handelsbanken => "handelsbanken",
            Ing => "ing",
            Knab => "knab",
            Moneyou => "moneyou",
            N26 => "n26",
            Nn => "nn",
            Rabobank => "rabobank",
            Regiobank => "regiobank",
            Revolut => "revolut",
            SnsBank => "sns_bank",
            TriodosBank => "triodos_bank",
            VanLanschot => "van_lanschot",
            Yoursafe => "yoursafe",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodDataIdealBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodDataIdealBank::*;
        match s {
            "abn_amro" => Ok(AbnAmro),
            "asn_bank" => Ok(AsnBank),
            "bunq" => Ok(Bunq),
            "handelsbanken" => Ok(Handelsbanken),
            "ing" => Ok(Ing),
            "knab" => Ok(Knab),
            "moneyou" => Ok(Moneyou),
            "n26" => Ok(N26),
            "nn" => Ok(Nn),
            "rabobank" => Ok(Rabobank),
            "regiobank" => Ok(Regiobank),
            "revolut" => Ok(Revolut),
            "sns_bank" => Ok(SnsBank),
            "triodos_bank" => Ok(TriodosBank),
            "van_lanschot" => Ok(VanLanschot),
            "yoursafe" => Ok(Yoursafe),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodDataIdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodDataIdealBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataKlarna {
    /// Customer's date of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirth>,
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
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<UpdateSetupIntentPaymentMethodDataP24Bank>,
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
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateSetupIntentPaymentMethodDataP24Bank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl UpdateSetupIntentPaymentMethodDataP24Bank {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodDataP24Bank::*;
        match self {
            AliorBank => "alior_bank",
            BankMillennium => "bank_millennium",
            BankNowyBfgSa => "bank_nowy_bfg_sa",
            BankPekaoSa => "bank_pekao_sa",
            BankiSpbdzielcze => "banki_spbdzielcze",
            Blik => "blik",
            BnpParibas => "bnp_paribas",
            Boz => "boz",
            CitiHandlowy => "citi_handlowy",
            CreditAgricole => "credit_agricole",
            Envelobank => "envelobank",
            EtransferPocztowy24 => "etransfer_pocztowy24",
            GetinBank => "getin_bank",
            Ideabank => "ideabank",
            Ing => "ing",
            Inteligo => "inteligo",
            MbankMtransfer => "mbank_mtransfer",
            NestPrzelew => "nest_przelew",
            NoblePay => "noble_pay",
            PbacZIpko => "pbac_z_ipko",
            PlusBank => "plus_bank",
            SantanderPrzelew24 => "santander_przelew24",
            TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            ToyotaBank => "toyota_bank",
            Velobank => "velobank",
            VolkswagenBank => "volkswagen_bank",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodDataP24Bank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodDataP24Bank::*;
        match s {
            "alior_bank" => Ok(AliorBank),
            "bank_millennium" => Ok(BankMillennium),
            "bank_nowy_bfg_sa" => Ok(BankNowyBfgSa),
            "bank_pekao_sa" => Ok(BankPekaoSa),
            "banki_spbdzielcze" => Ok(BankiSpbdzielcze),
            "blik" => Ok(Blik),
            "bnp_paribas" => Ok(BnpParibas),
            "boz" => Ok(Boz),
            "citi_handlowy" => Ok(CitiHandlowy),
            "credit_agricole" => Ok(CreditAgricole),
            "envelobank" => Ok(Envelobank),
            "etransfer_pocztowy24" => Ok(EtransferPocztowy24),
            "getin_bank" => Ok(GetinBank),
            "ideabank" => Ok(Ideabank),
            "ing" => Ok(Ing),
            "inteligo" => Ok(Inteligo),
            "mbank_mtransfer" => Ok(MbankMtransfer),
            "nest_przelew" => Ok(NestPrzelew),
            "noble_pay" => Ok(NoblePay),
            "pbac_z_ipko" => Ok(PbacZIpko),
            "plus_bank" => Ok(PlusBank),
            "santander_przelew24" => Ok(SantanderPrzelew24),
            "tmobile_usbugi_bankowe" => Ok(TmobileUsbugiBankowe),
            "toyota_bank" => Ok(ToyotaBank),
            "velobank" => Ok(Velobank),
            "volkswagen_bank" => Ok(VolkswagenBank),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodDataP24Bank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodDataP24Bank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataSepaDebit<'a> {
    /// IBAN of the bank account.
    pub iban: &'a str,
}
impl<'a> UpdateSetupIntentPaymentMethodDataSepaDebit<'a> {
    pub fn new(iban: &'a str) -> Self {
        Self { iban }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: UpdateSetupIntentPaymentMethodDataSofortCountry,
}
impl UpdateSetupIntentPaymentMethodDataSofort {
    pub fn new(country: UpdateSetupIntentPaymentMethodDataSofortCountry) -> Self {
        Self { country }
    }
}
/// Two-letter ISO code representing the country the bank account is located in.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodDataSofortCountry {
    At,
    Be,
    De,
    Es,
    It,
    Nl,
}
impl UpdateSetupIntentPaymentMethodDataSofortCountry {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodDataSofortCountry::*;
        match self {
            At => "AT",
            Be => "BE",
            De => "DE",
            Es => "ES",
            It => "IT",
            Nl => "NL",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodDataSofortCountry {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodDataSofortCountry::*;
        match s {
            "AT" => Ok(At),
            "BE" => Ok(Be),
            "DE" => Ok(De),
            "ES" => Ok(Es),
            "IT" => Ok(It),
            "NL" => Ok(Nl),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodDataSofortCountry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodDataSofortCountry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSetupIntentPaymentMethodDataSofortCountry",
            )
        })
    }
}
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateSetupIntentPaymentMethodDataType {
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
impl UpdateSetupIntentPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodDataType::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            AmazonPay => "amazon_pay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Blik => "blik",
            Boleto => "boleto",
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
            Mobilepay => "mobilepay",
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

impl std::str::FromStr for UpdateSetupIntentPaymentMethodDataType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodDataType::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "amazon_pay" => Ok(AmazonPay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
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
            "mobilepay" => Ok(Mobilepay),
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
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> UpdateSetupIntentPaymentMethodDataUsBankAccount<'a> {
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
impl<'a> Default for UpdateSetupIntentPaymentMethodDataUsBankAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}
impl UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType"))
    }
}
/// Account type: checkings or savings. Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}
impl UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType",
            )
        })
    }
}
/// Payment method-specific configuration for this SetupIntent.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptions<'a> {
    /// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdateSetupIntentPaymentMethodOptionsAcssDebit<'a>>,
    /// If this is a `amazon_pay` SetupIntent, this sub-hash contains details about the AmazonPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// Configuration for any card setup attempted on this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdateSetupIntentPaymentMethodOptionsCard<'a>>,
    /// If this is a `card_present` PaymentMethod, this sub-hash contains details about the card-present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub card_present: Option<miniserde::json::Value>,
    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<SetupIntentPaymentMethodOptionsParam<'a>>,
    /// If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodOptionsParam<'a>>,
    /// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdateSetupIntentPaymentMethodOptionsSepaDebit>,
    /// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccount<'a>>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self {
            acss_debit: None,
            amazon_pay: None,
            card: None,
            card_present: None,
            link: None,
            paypal: None,
            sepa_debit: None,
            us_bank_account: None,
        }
    }
}
impl<'a> Default for UpdateSetupIntentPaymentMethodOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self { currency: None, mandate_options: None, verification_method: None }
    }
}
impl<'a> Default for UpdateSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
/// Must be a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
}
impl UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency::*;
        match self {
            Cad => "cad",
            Usd => "usd",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency::*;
        match s {
            "cad" => Ok(Cad),
            "usd" => Ok(Usd),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency",
            )
        })
    }
}
/// Additional fields for Mandate creation
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,.
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<&'a [UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor]>,
    /// Description of the mandate interval.
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
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
impl<'a> Default for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// List of Stripe products where this mandate can be selected automatically.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}
impl UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::*;
        match self {
            Invoice => "invoice",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::*;
        match s {
            "invoice" => Ok(Invoice),
            "subscription" => Ok(Subscription),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor"))
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}
impl UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
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
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule"))
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}
impl UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
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
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType"))
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod"))
    }
}
/// Configuration for any card setup attempted on this SetupIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCard<'a> {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsCardMandateOptions<'a>>,
    /// When specified, this parameter signals that a card has been collected
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA. This
    /// parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    /// Selected network to process this SetupIntent on.
    /// Depends on the available networks of the card attached to the SetupIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<UpdateSetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,
    /// If 3D Secure authentication was performed with a third-party provider,
    /// the authentication details to use for this setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure<'a>>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsCard<'a> {
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
impl<'a> Default for UpdateSetupIntentPaymentMethodOptionsCard<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCardMandateOptions<'a> {
    /// Amount to be charged for future payments.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
    /// Currency in which future payments will be charged.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// End date of the mandate or subscription.
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<stripe_types::Timestamp>,
    /// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
    /// The number of intervals between payments.
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: &'a str,
    /// Start date of the mandate or subscription. Start date should not be lesser than yesterday.
    pub start_date: stripe_types::Timestamp,
    /// Specifies the type of mandates supported. Possible values are `india`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types:
        Option<&'a [UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes]>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new(
        amount: i64,
        amount_type: UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
        currency: stripe_types::Currency,
        interval: UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
        reference: &'a str,
        start_date: stripe_types::Timestamp,
    ) -> Self {
        Self {
            amount,
            amount_type,
            currency,
            description: None,
            end_date: None,
            interval,
            interval_count: None,
            reference,
            start_date,
            supported_types: None,
        }
    }
}
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}
impl UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType"))
    }
}
/// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}
impl UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Sporadic => "sporadic",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "sporadic" => Ok(Sporadic),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval",
            )
        })
    }
}
/// Specifies the type of mandates supported. Possible values are `india`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}
impl UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes::*;
        match self {
            India => "india",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes::*;
        match s {
            "india" => Ok(India),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes"))
    }
}
/// Selected network to process this SetupIntent on.
/// Depends on the available networks of the card attached to the SetupIntent.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}
impl UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsCardNetwork::*;
        match self {
            Amex => "amex",
            CartesBancaires => "cartes_bancaires",
            Diners => "diners",
            Discover => "discover",
            EftposAu => "eftpos_au",
            Interac => "interac",
            Jcb => "jcb",
            Mastercard => "mastercard",
            Unionpay => "unionpay",
            Unknown => "unknown",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsCardNetwork::*;
        match s {
            "amex" => Ok(Amex),
            "cartes_bancaires" => Ok(CartesBancaires),
            "diners" => Ok(Diners),
            "discover" => Ok(Discover),
            "eftpos_au" => Ok(EftposAu),
            "interac" => Ok(Interac),
            "jcb" => Ok(Jcb),
            "mastercard" => Ok(Mastercard),
            "unionpay" => Ok(Unionpay),
            "unknown" => Ok(Unknown),
            "visa" => Ok(Visa),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSetupIntentPaymentMethodOptionsCardNetwork",
            )
        })
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// If not provided, this value defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}
impl UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
            Challenge => "challenge",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            "challenge" => Ok(Challenge),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure",
            )
        })
    }
}
/// If 3D Secure authentication was performed with a third-party provider,
/// the authentication details to use for this setup.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure<'a> {
    /// The `transStatus` returned from the card Issuer’s ACS in the ARes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ares_trans_status:
        Option<UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus>,
    /// The cryptogram, also known as the "authentication value" (AAV, CAVV or
    /// AEVV). This value is 20 bytes, base64-encoded into a 28-character string.
    /// (Most 3D Secure providers will return the base64-encoded version, which
    /// is what you should specify here.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptogram: Option<&'a str>,
    /// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
    /// provider and indicates what degree of authentication was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator:
        Option<UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator>,
    /// Network specific 3DS fields. Network specific arguments require an
    /// explicit card brand choice. The parameter `payment_method_options.card.network``
    /// must be populated accordingly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options:
        Option<UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions<'a>>,
    /// The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the
    /// AReq sent to the card Issuer's ACS. A string containing 2 digits from 01-99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_challenge_indicator: Option<&'a str>,
    /// For 3D Secure 1, the XID. For 3D Secure 2, the Directory Server
    /// Transaction ID (dsTransID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<&'a str>,
    /// The version of 3D Secure that was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure<'a> {
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
impl<'a> Default for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The `transStatus` returned from the card Issuer’s ACS in the ARes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    A,
    C,
    I,
    N,
    R,
    U,
    Y,
}
impl UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::*;
        match self {
            A => "A",
            C => "C",
            I => "I",
            N => "N",
            R => "R",
            U => "U",
            Y => "Y",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::*;
        match s {
            "A" => Ok(A),
            "C" => Ok(C),
            "I" => Ok(I),
            "N" => Ok(N),
            "R" => Ok(R),
            "U" => Ok(U),
            "Y" => Ok(Y),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus"))
    }
}
/// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
/// provider and indicates what degree of authentication was performed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    V01,
    V02,
    V05,
    V06,
    V07,
}
impl UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::*;
        match self {
            V01 => "01",
            V02 => "02",
            V05 => "05",
            V06 => "06",
            V07 => "07",
        }
    }
}

impl std::str::FromStr
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::*;
        match s {
            "01" => Ok(V01),
            "02" => Ok(V02),
            "05" => Ok(V05),
            "06" => Ok(V06),
            "07" => Ok(V07),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
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
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator"))
    }
}
/// Network specific 3DS fields. Network specific arguments require an
/// explicit card brand choice. The parameter `payment_method_options.card.network``
/// must be populated accordingly
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions<'a> {
    /// Cartes Bancaires-specific 3DS fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires: Option<
        UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires<'a>,
    >,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions<'a> {
    pub fn new() -> Self {
        Self { cartes_bancaires: None }
    }
}
impl<'a> Default for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Cartes Bancaires-specific 3DS fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires<'a> {
    /// The cryptogram calculation algorithm used by the card Issuer's ACS
    /// to calculate the Authentication cryptogram. Also known as `cavvAlgorithm`.
    /// messageExtension: CB-AVALGO
    pub cb_avalgo:
        UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo,
    /// The exemption indicator returned from Cartes Bancaires in the ARes.
    /// message extension: CB-EXEMPTION; string (4 characters)
    /// This is a 3 byte bitmap (low significant byte first and most significant
    /// bit first) that has been Base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_exemption: Option<&'a str>,
    /// The risk score returned from Cartes Bancaires in the ARes.
    /// message extension: CB-SCORE; numeric value 0-99
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_score: Option<i64>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires<'a> {
    pub fn new(
        cb_avalgo: UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo,
    ) -> Self {
        Self { cb_avalgo, cb_exemption: None, cb_score: None }
    }
}
/// The cryptogram calculation algorithm used by the card Issuer's ACS
/// to calculate the Authentication cryptogram. Also known as `cavvAlgorithm`.
/// messageExtension: CB-AVALGO
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    V0,
    V1,
    V2,
    V3,
    V4,
    A,
}
impl UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::*;
        match self {
            V0 => "0",
            V1 => "1",
            V2 => "2",
            V3 => "3",
            V4 => "4",
            A => "A",
        }
    }
}

impl std::str::FromStr
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::*;
        match s {
            "0" => Ok(V0),
            "1" => Ok(V1),
            "2" => Ok(V2),
            "3" => Ok(V3),
            "4" => Ok(V4),
            "A" => Ok(A),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
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
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo"))
    }
}
/// The version of 3D Secure that was performed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    V1_0_2,
    V2_1_0,
    V2_2_0,
}
impl UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion::*;
        match self {
            V1_0_2 => "1.0.2",
            V2_1_0 => "2.1.0",
            V2_2_0 => "2.2.0",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion::*;
        match s {
            "1.0.2" => Ok(V1_0_2),
            "2.1.0" => Ok(V2_1_0),
            "2.2.0" => Ok(V2_2_0),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion",
            )
        })
    }
}
/// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub mandate_options: Option<miniserde::json::Value>,
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
/// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a>>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions>,
    /// Additional fields for network related functions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a>>,
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self {
            financial_connections: None,
            mandate_options: None,
            networks: None,
            verification_method: None,
        }
    }
}
impl<'a> Default for UpdateSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Financial Connections Session creation
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        &'a [UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions],
    >,
    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<
        &'a [UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch],
    >,
    /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self { permissions: None, prefetch: None, return_url: None }
    }
}
impl<'a> Default for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The list of permissions to request.
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}
impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
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
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
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
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions"))
    }
}
/// List of data features that you would like to retrieve upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    Balances,
    Ownership,
    Transactions,
}
impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
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
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch"))
    }
}
/// Additional fields for Mandate creation
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions {
    /// The method used to collect offline mandate customer acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method:
        Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}
impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions {
    pub fn new() -> Self {
        Self { collection_method: None }
    }
}
impl Default for UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// The method used to collect offline mandate customer acceptance.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    Paper,
}
impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::*;
        match self {
            Paper => "paper",
        }
    }
}

impl std::str::FromStr
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::*;
        match s {
            "paper" => Ok(Paper),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
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
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod"))
    }
}
/// Additional fields for network related functions
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    /// Triggers validations to run across the selected networks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested:
        Option<&'a [UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested]>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl<'a> Default for UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Triggers validations to run across the selected networks
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    Ach,
    UsDomesticWire,
}
impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested"))
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod"))
    }
}
/// Updates a SetupIntent object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntent<'a> {
    inner: UpdateSetupIntentBuilder<'a>,
    intent: &'a stripe_shared::SetupIntentId,
}
impl<'a> UpdateSetupIntent<'a> {
    /// Construct a new `UpdateSetupIntent`.
    pub fn new(intent: &'a stripe_shared::SetupIntentId) -> Self {
        Self { intent, inner: UpdateSetupIntentBuilder::new() }
    }
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    pub fn attach_to_self(mut self, attach_to_self: bool) -> Self {
        self.inner.attach_to_self = Some(attach_to_self);
        self
    }
    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    pub fn flow_directions(
        mut self,
        flow_directions: &'a [stripe_shared::SetupIntentFlowDirections],
    ) -> Self {
        self.inner.flow_directions = Some(flow_directions);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    pub fn payment_method(mut self, payment_method: &'a str) -> Self {
        self.inner.payment_method = Some(payment_method);
        self
    }
    /// The ID of the payment method configuration to use with this SetupIntent.
    pub fn payment_method_configuration(mut self, payment_method_configuration: &'a str) -> Self {
        self.inner.payment_method_configuration = Some(payment_method_configuration);
        self
    }
    /// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method).
    /// value in the SetupIntent.
    pub fn payment_method_data(
        mut self,
        payment_method_data: UpdateSetupIntentPaymentMethodData<'a>,
    ) -> Self {
        self.inner.payment_method_data = Some(payment_method_data);
        self
    }
    /// Payment method-specific configuration for this SetupIntent.
    pub fn payment_method_options(
        mut self,
        payment_method_options: UpdateSetupIntentPaymentMethodOptions<'a>,
    ) -> Self {
        self.inner.payment_method_options = Some(payment_method_options);
        self
    }
    /// The list of payment method types (for example, card) that this SetupIntent can set up.
    /// If you don't provide this array, it defaults to ["card"].
    pub fn payment_method_types(mut self, payment_method_types: &'a [&'a str]) -> Self {
        self.inner.payment_method_types = Some(payment_method_types);
        self
    }
}
impl UpdateSetupIntent<'_> {
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

impl StripeRequest for UpdateSetupIntent<'_> {
    type Output = stripe_shared::SetupIntent;

    fn build(&self) -> RequestBuilder {
        let intent = self.intent;
        RequestBuilder::new(StripeMethod::Post, format!("/setup_intents/{intent}"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CancelSetupIntentBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    cancellation_reason: Option<stripe_shared::SetupIntentCancellationReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> CancelSetupIntentBuilder<'a> {
    fn new() -> Self {
        Self { cancellation_reason: None, expand: None }
    }
}
/// You can cancel a SetupIntent object when it’s in one of these statuses: `requires_payment_method`, `requires_confirmation`, or `requires_action`.
///
///
/// After you cancel it, setup is abandoned and any operations on the SetupIntent fail with an error.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelSetupIntent<'a> {
    inner: CancelSetupIntentBuilder<'a>,
    intent: &'a stripe_shared::SetupIntentId,
}
impl<'a> CancelSetupIntent<'a> {
    /// Construct a new `CancelSetupIntent`.
    pub fn new(intent: &'a stripe_shared::SetupIntentId) -> Self {
        Self { intent, inner: CancelSetupIntentBuilder::new() }
    }
    /// Reason for canceling this SetupIntent.
    /// Possible values are: `abandoned`, `requested_by_customer`, or `duplicate`.
    pub fn cancellation_reason(
        mut self,
        cancellation_reason: stripe_shared::SetupIntentCancellationReason,
    ) -> Self {
        self.inner.cancellation_reason = Some(cancellation_reason);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CancelSetupIntent<'_> {
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

impl StripeRequest for CancelSetupIntent<'_> {
    type Output = stripe_shared::SetupIntent;

    fn build(&self) -> RequestBuilder {
        let intent = self.intent;
        RequestBuilder::new(StripeMethod::Post, format!("/setup_intents/{intent}/cancel"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ConfirmSetupIntentBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    confirmation_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mandate_data: Option<ConfirmSetupIntentMandateData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_data: Option<ConfirmSetupIntentPaymentMethodData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_options: Option<ConfirmSetupIntentPaymentMethodOptions<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_stripe_sdk: Option<bool>,
}
impl<'a> ConfirmSetupIntentBuilder<'a> {
    fn new() -> Self {
        Self {
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
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentMandateData<'a> {
    #[serde(untagged)]
    SecretKeyParam(ConfirmSetupIntentSecretKeyParam<'a>),
    #[serde(untagged)]
    ClientKeyParam(ConfirmSetupIntentClientKeyParam<'a>),
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentSecretKeyParam<'a> {
    /// This hash contains details about the customer acceptance of the Mandate.
    pub customer_acceptance: ConfirmSetupIntentSecretKeyParamCustomerAcceptance<'a>,
}
impl<'a> ConfirmSetupIntentSecretKeyParam<'a> {
    pub fn new(
        customer_acceptance: ConfirmSetupIntentSecretKeyParamCustomerAcceptance<'a>,
    ) -> Self {
        Self { customer_acceptance }
    }
}
/// This hash contains details about the customer acceptance of the Mandate.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentSecretKeyParamCustomerAcceptance<'a> {
    /// The time at which the customer accepted the Mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<stripe_types::Timestamp>,
    /// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub offline: Option<miniserde::json::Value>,
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<OnlineParam<'a>>,
    /// The type of customer acceptance information included with the Mandate.
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType,
}
impl<'a> ConfirmSetupIntentSecretKeyParamCustomerAcceptance<'a> {
    pub fn new(type_: ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType) -> Self {
        Self { accepted_at: None, offline: None, online: None, type_ }
    }
}
/// The type of customer acceptance information included with the Mandate.
/// One of `online` or `offline`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType {
    Offline,
    Online,
}
impl ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType::*;
        match self {
            Offline => "offline",
            Online => "online",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType::*;
        match s {
            "offline" => Ok(Offline),
            "online" => Ok(Online),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType",
            )
        })
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentClientKeyParam<'a> {
    /// This hash contains details about the customer acceptance of the Mandate.
    pub customer_acceptance: ConfirmSetupIntentClientKeyParamCustomerAcceptance<'a>,
}
impl<'a> ConfirmSetupIntentClientKeyParam<'a> {
    pub fn new(
        customer_acceptance: ConfirmSetupIntentClientKeyParamCustomerAcceptance<'a>,
    ) -> Self {
        Self { customer_acceptance }
    }
}
/// This hash contains details about the customer acceptance of the Mandate.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentClientKeyParamCustomerAcceptance<'a> {
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    pub online: ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline<'a>,
    /// The type of customer acceptance information included with the Mandate.
    #[serde(rename = "type")]
    pub type_: ConfirmSetupIntentClientKeyParamCustomerAcceptanceType,
}
impl<'a> ConfirmSetupIntentClientKeyParamCustomerAcceptance<'a> {
    pub fn new(
        online: ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline<'a>,
        type_: ConfirmSetupIntentClientKeyParamCustomerAcceptanceType,
    ) -> Self {
        Self { online, type_ }
    }
}
/// If this is a Mandate accepted online, this hash contains details about the online acceptance.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline<'a> {
    /// The IP address from which the Mandate was accepted by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline<'a> {
    pub fn new() -> Self {
        Self { ip_address: None, user_agent: None }
    }
}
impl<'a> Default for ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The type of customer acceptance information included with the Mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    Online,
}
impl ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentClientKeyParamCustomerAcceptanceType::*;
        match self {
            Online => "online",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentClientKeyParamCustomerAcceptanceType::*;
        match s {
            "online" => Ok(Online),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType",
            )
        })
    }
}
/// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method).
/// value in the SetupIntent.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodData<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodParam<'a>>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub affirm: Option<miniserde::json::Value>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub afterpay_clearpay: Option<miniserde::json::Value>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub alipay: Option<miniserde::json::Value>,
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<ConfirmSetupIntentPaymentMethodDataAllowRedisplay>,
    /// If this is a AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<ConfirmSetupIntentPaymentMethodDataAuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<ConfirmSetupIntentPaymentMethodDataBacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub bancontact: Option<miniserde::json::Value>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetailsInnerParams<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub blik: Option<miniserde::json::Value>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<ConfirmSetupIntentPaymentMethodDataBoleto<'a>>,
    /// If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub cashapp: Option<miniserde::json::Value>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub customer_balance: Option<miniserde::json::Value>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<ConfirmSetupIntentPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<ConfirmSetupIntentPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub giropay: Option<miniserde::json::Value>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub grabpay: Option<miniserde::json::Value>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<ConfirmSetupIntentPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub interac_present: Option<miniserde::json::Value>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<ConfirmSetupIntentPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub konbini: Option<miniserde::json::Value>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub link: Option<miniserde::json::Value>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub mobilepay: Option<miniserde::json::Value>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub oxxo: Option<miniserde::json::Value>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<ConfirmSetupIntentPaymentMethodDataP24>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub paynow: Option<miniserde::json::Value>,
    /// If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub paypal: Option<miniserde::json::Value>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub pix: Option<miniserde::json::Value>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub promptpay: Option<miniserde::json::Value>,
    /// Options to configure Radar.
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<RadarOptionsWithHiddenOptions<'a>>,
    /// If this is a `Revolut Pay` PaymentMethod, this hash contains details about the Revolut Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub revolut_pay: Option<miniserde::json::Value>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<ConfirmSetupIntentPaymentMethodDataSepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<ConfirmSetupIntentPaymentMethodDataSofort>,
    /// If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub swish: Option<miniserde::json::Value>,
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: ConfirmSetupIntentPaymentMethodDataType,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<ConfirmSetupIntentPaymentMethodDataUsBankAccount<'a>>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub wechat_pay: Option<miniserde::json::Value>,
    /// If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub zip: Option<miniserde::json::Value>,
}
impl<'a> ConfirmSetupIntentPaymentMethodData<'a> {
    pub fn new(type_: ConfirmSetupIntentPaymentMethodDataType) -> Self {
        Self {
            acss_debit: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            allow_redisplay: None,
            amazon_pay: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            billing_details: None,
            blik: None,
            boleto: None,
            cashapp: None,
            customer_balance: None,
            eps: None,
            fpx: None,
            giropay: None,
            grabpay: None,
            ideal: None,
            interac_present: None,
            klarna: None,
            konbini: None,
            link: None,
            metadata: None,
            mobilepay: None,
            oxxo: None,
            p24: None,
            paynow: None,
            paypal: None,
            pix: None,
            promptpay: None,
            radar_options: None,
            revolut_pay: None,
            sepa_debit: None,
            sofort: None,
            swish: None,
            type_,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodDataAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl ConfirmSetupIntentPaymentMethodDataAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodDataAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodDataAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodDataAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodDataAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentPaymentMethodDataAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ConfirmSetupIntentPaymentMethodDataAllowRedisplay",
            )
        })
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataAuBecsDebit<'a> {
    /// The account number for the bank account.
    pub account_number: &'a str,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: &'a str,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataAuBecsDebit<'a> {
    pub fn new(account_number: &'a str, bsb_number: &'a str) -> Self {
        Self { account_number, bsb_number }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataBacsDebit<'a> {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Sort code of the bank account. (e.g., `10-20-30`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<&'a str>,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataBacsDebit<'a> {
    pub fn new() -> Self {
        Self { account_number: None, sort_code: None }
    }
}
impl<'a> Default for ConfirmSetupIntentPaymentMethodDataBacsDebit<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataBoleto<'a> {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers)
    pub tax_id: &'a str,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataBoleto<'a> {
    pub fn new(tax_id: &'a str) -> Self {
        Self { tax_id }
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<ConfirmSetupIntentPaymentMethodDataEpsBank>,
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
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfirmSetupIntentPaymentMethodDataEpsBank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl ConfirmSetupIntentPaymentMethodDataEpsBank {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodDataEpsBank::*;
        match self {
            ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            BankAustria => "bank_austria",
            BankhausCarlSpangler => "bankhaus_carl_spangler",
            BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            BawagPskAg => "bawag_psk_ag",
            BksBankAg => "bks_bank_ag",
            BrullKallmusBankAg => "brull_kallmus_bank_ag",
            BtvVierLanderBank => "btv_vier_lander_bank",
            CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            DeutscheBankAg => "deutsche_bank_ag",
            Dolomitenbank => "dolomitenbank",
            EasybankAg => "easybank_ag",
            ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            HypoTirolBankAg => "hypo_tirol_bank_ag",
            HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            MarchfelderBank => "marchfelder_bank",
            OberbankAg => "oberbank_ag",
            RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            SchoellerbankAg => "schoellerbank_ag",
            SpardaBankWien => "sparda_bank_wien",
            VolksbankGruppe => "volksbank_gruppe",
            VolkskreditbankAg => "volkskreditbank_ag",
            VrBankBraunau => "vr_bank_braunau",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodDataEpsBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodDataEpsBank::*;
        match s {
            "arzte_und_apotheker_bank" => Ok(ArzteUndApothekerBank),
            "austrian_anadi_bank_ag" => Ok(AustrianAnadiBankAg),
            "bank_austria" => Ok(BankAustria),
            "bankhaus_carl_spangler" => Ok(BankhausCarlSpangler),
            "bankhaus_schelhammer_und_schattera_ag" => Ok(BankhausSchelhammerUndSchatteraAg),
            "bawag_psk_ag" => Ok(BawagPskAg),
            "bks_bank_ag" => Ok(BksBankAg),
            "brull_kallmus_bank_ag" => Ok(BrullKallmusBankAg),
            "btv_vier_lander_bank" => Ok(BtvVierLanderBank),
            "capital_bank_grawe_gruppe_ag" => Ok(CapitalBankGraweGruppeAg),
            "deutsche_bank_ag" => Ok(DeutscheBankAg),
            "dolomitenbank" => Ok(Dolomitenbank),
            "easybank_ag" => Ok(EasybankAg),
            "erste_bank_und_sparkassen" => Ok(ErsteBankUndSparkassen),
            "hypo_alpeadriabank_international_ag" => Ok(HypoAlpeadriabankInternationalAg),
            "hypo_bank_burgenland_aktiengesellschaft" => Ok(HypoBankBurgenlandAktiengesellschaft),
            "hypo_noe_lb_fur_niederosterreich_u_wien" => Ok(HypoNoeLbFurNiederosterreichUWien),
            "hypo_oberosterreich_salzburg_steiermark" => Ok(HypoOberosterreichSalzburgSteiermark),
            "hypo_tirol_bank_ag" => Ok(HypoTirolBankAg),
            "hypo_vorarlberg_bank_ag" => Ok(HypoVorarlbergBankAg),
            "marchfelder_bank" => Ok(MarchfelderBank),
            "oberbank_ag" => Ok(OberbankAg),
            "raiffeisen_bankengruppe_osterreich" => Ok(RaiffeisenBankengruppeOsterreich),
            "schoellerbank_ag" => Ok(SchoellerbankAg),
            "sparda_bank_wien" => Ok(SpardaBankWien),
            "volksbank_gruppe" => Ok(VolksbankGruppe),
            "volkskreditbank_ag" => Ok(VolkskreditbankAg),
            "vr_bank_braunau" => Ok(VrBankBraunau),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodDataEpsBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentPaymentMethodDataEpsBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType>,
    /// The customer's bank.
    pub bank: ConfirmSetupIntentPaymentMethodDataFpxBank,
}
impl ConfirmSetupIntentPaymentMethodDataFpx {
    pub fn new(bank: ConfirmSetupIntentPaymentMethodDataFpxBank) -> Self {
        Self { account_holder_type: None, bank }
    }
}
/// Account holder type for FPX transaction
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
}
impl ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType",
            )
        })
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfirmSetupIntentPaymentMethodDataFpxBank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl ConfirmSetupIntentPaymentMethodDataFpxBank {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodDataFpxBank::*;
        match self {
            AffinBank => "affin_bank",
            Agrobank => "agrobank",
            AllianceBank => "alliance_bank",
            Ambank => "ambank",
            BankIslam => "bank_islam",
            BankMuamalat => "bank_muamalat",
            BankOfChina => "bank_of_china",
            BankRakyat => "bank_rakyat",
            Bsn => "bsn",
            Cimb => "cimb",
            DeutscheBank => "deutsche_bank",
            HongLeongBank => "hong_leong_bank",
            Hsbc => "hsbc",
            Kfh => "kfh",
            Maybank2e => "maybank2e",
            Maybank2u => "maybank2u",
            Ocbc => "ocbc",
            PbEnterprise => "pb_enterprise",
            PublicBank => "public_bank",
            Rhb => "rhb",
            StandardChartered => "standard_chartered",
            Uob => "uob",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodDataFpxBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodDataFpxBank::*;
        match s {
            "affin_bank" => Ok(AffinBank),
            "agrobank" => Ok(Agrobank),
            "alliance_bank" => Ok(AllianceBank),
            "ambank" => Ok(Ambank),
            "bank_islam" => Ok(BankIslam),
            "bank_muamalat" => Ok(BankMuamalat),
            "bank_of_china" => Ok(BankOfChina),
            "bank_rakyat" => Ok(BankRakyat),
            "bsn" => Ok(Bsn),
            "cimb" => Ok(Cimb),
            "deutsche_bank" => Ok(DeutscheBank),
            "hong_leong_bank" => Ok(HongLeongBank),
            "hsbc" => Ok(Hsbc),
            "kfh" => Ok(Kfh),
            "maybank2e" => Ok(Maybank2e),
            "maybank2u" => Ok(Maybank2u),
            "ocbc" => Ok(Ocbc),
            "pb_enterprise" => Ok(PbEnterprise),
            "public_bank" => Ok(PublicBank),
            "rhb" => Ok(Rhb),
            "standard_chartered" => Ok(StandardChartered),
            "uob" => Ok(Uob),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodDataFpxBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentPaymentMethodDataFpxBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<ConfirmSetupIntentPaymentMethodDataIdealBank>,
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
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfirmSetupIntentPaymentMethodDataIdealBank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl ConfirmSetupIntentPaymentMethodDataIdealBank {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodDataIdealBank::*;
        match self {
            AbnAmro => "abn_amro",
            AsnBank => "asn_bank",
            Bunq => "bunq",
            Handelsbanken => "handelsbanken",
            Ing => "ing",
            Knab => "knab",
            Moneyou => "moneyou",
            N26 => "n26",
            Nn => "nn",
            Rabobank => "rabobank",
            Regiobank => "regiobank",
            Revolut => "revolut",
            SnsBank => "sns_bank",
            TriodosBank => "triodos_bank",
            VanLanschot => "van_lanschot",
            Yoursafe => "yoursafe",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodDataIdealBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodDataIdealBank::*;
        match s {
            "abn_amro" => Ok(AbnAmro),
            "asn_bank" => Ok(AsnBank),
            "bunq" => Ok(Bunq),
            "handelsbanken" => Ok(Handelsbanken),
            "ing" => Ok(Ing),
            "knab" => Ok(Knab),
            "moneyou" => Ok(Moneyou),
            "n26" => Ok(N26),
            "nn" => Ok(Nn),
            "rabobank" => Ok(Rabobank),
            "regiobank" => Ok(Regiobank),
            "revolut" => Ok(Revolut),
            "sns_bank" => Ok(SnsBank),
            "triodos_bank" => Ok(TriodosBank),
            "van_lanschot" => Ok(VanLanschot),
            "yoursafe" => Ok(Yoursafe),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodDataIdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentPaymentMethodDataIdealBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataKlarna {
    /// Customer's date of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirth>,
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
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<ConfirmSetupIntentPaymentMethodDataP24Bank>,
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
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfirmSetupIntentPaymentMethodDataP24Bank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl ConfirmSetupIntentPaymentMethodDataP24Bank {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodDataP24Bank::*;
        match self {
            AliorBank => "alior_bank",
            BankMillennium => "bank_millennium",
            BankNowyBfgSa => "bank_nowy_bfg_sa",
            BankPekaoSa => "bank_pekao_sa",
            BankiSpbdzielcze => "banki_spbdzielcze",
            Blik => "blik",
            BnpParibas => "bnp_paribas",
            Boz => "boz",
            CitiHandlowy => "citi_handlowy",
            CreditAgricole => "credit_agricole",
            Envelobank => "envelobank",
            EtransferPocztowy24 => "etransfer_pocztowy24",
            GetinBank => "getin_bank",
            Ideabank => "ideabank",
            Ing => "ing",
            Inteligo => "inteligo",
            MbankMtransfer => "mbank_mtransfer",
            NestPrzelew => "nest_przelew",
            NoblePay => "noble_pay",
            PbacZIpko => "pbac_z_ipko",
            PlusBank => "plus_bank",
            SantanderPrzelew24 => "santander_przelew24",
            TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            ToyotaBank => "toyota_bank",
            Velobank => "velobank",
            VolkswagenBank => "volkswagen_bank",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodDataP24Bank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodDataP24Bank::*;
        match s {
            "alior_bank" => Ok(AliorBank),
            "bank_millennium" => Ok(BankMillennium),
            "bank_nowy_bfg_sa" => Ok(BankNowyBfgSa),
            "bank_pekao_sa" => Ok(BankPekaoSa),
            "banki_spbdzielcze" => Ok(BankiSpbdzielcze),
            "blik" => Ok(Blik),
            "bnp_paribas" => Ok(BnpParibas),
            "boz" => Ok(Boz),
            "citi_handlowy" => Ok(CitiHandlowy),
            "credit_agricole" => Ok(CreditAgricole),
            "envelobank" => Ok(Envelobank),
            "etransfer_pocztowy24" => Ok(EtransferPocztowy24),
            "getin_bank" => Ok(GetinBank),
            "ideabank" => Ok(Ideabank),
            "ing" => Ok(Ing),
            "inteligo" => Ok(Inteligo),
            "mbank_mtransfer" => Ok(MbankMtransfer),
            "nest_przelew" => Ok(NestPrzelew),
            "noble_pay" => Ok(NoblePay),
            "pbac_z_ipko" => Ok(PbacZIpko),
            "plus_bank" => Ok(PlusBank),
            "santander_przelew24" => Ok(SantanderPrzelew24),
            "tmobile_usbugi_bankowe" => Ok(TmobileUsbugiBankowe),
            "toyota_bank" => Ok(ToyotaBank),
            "velobank" => Ok(Velobank),
            "volkswagen_bank" => Ok(VolkswagenBank),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodDataP24Bank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentPaymentMethodDataP24Bank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataSepaDebit<'a> {
    /// IBAN of the bank account.
    pub iban: &'a str,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataSepaDebit<'a> {
    pub fn new(iban: &'a str) -> Self {
        Self { iban }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: ConfirmSetupIntentPaymentMethodDataSofortCountry,
}
impl ConfirmSetupIntentPaymentMethodDataSofort {
    pub fn new(country: ConfirmSetupIntentPaymentMethodDataSofortCountry) -> Self {
        Self { country }
    }
}
/// Two-letter ISO code representing the country the bank account is located in.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodDataSofortCountry {
    At,
    Be,
    De,
    Es,
    It,
    Nl,
}
impl ConfirmSetupIntentPaymentMethodDataSofortCountry {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodDataSofortCountry::*;
        match self {
            At => "AT",
            Be => "BE",
            De => "DE",
            Es => "ES",
            It => "IT",
            Nl => "NL",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodDataSofortCountry {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodDataSofortCountry::*;
        match s {
            "AT" => Ok(At),
            "BE" => Ok(Be),
            "DE" => Ok(De),
            "ES" => Ok(Es),
            "IT" => Ok(It),
            "NL" => Ok(Nl),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodDataSofortCountry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentPaymentMethodDataSofortCountry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ConfirmSetupIntentPaymentMethodDataSofortCountry",
            )
        })
    }
}
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfirmSetupIntentPaymentMethodDataType {
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
impl ConfirmSetupIntentPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodDataType::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            AmazonPay => "amazon_pay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Blik => "blik",
            Boleto => "boleto",
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
            Mobilepay => "mobilepay",
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

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodDataType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodDataType::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "amazon_pay" => Ok(AmazonPay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
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
            "mobilepay" => Ok(Mobilepay),
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
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentPaymentMethodDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataUsBankAccount<'a> {
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
impl<'a> Default for ConfirmSetupIntentPaymentMethodDataUsBankAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}
impl ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType"))
    }
}
/// Account type: checkings or savings. Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}
impl ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType",
            )
        })
    }
}
/// Payment method-specific configuration for this SetupIntent.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptions<'a> {
    /// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebit<'a>>,
    /// If this is a `amazon_pay` SetupIntent, this sub-hash contains details about the AmazonPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// Configuration for any card setup attempted on this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<ConfirmSetupIntentPaymentMethodOptionsCard<'a>>,
    /// If this is a `card_present` PaymentMethod, this sub-hash contains details about the card-present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub card_present: Option<miniserde::json::Value>,
    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<SetupIntentPaymentMethodOptionsParam<'a>>,
    /// If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodOptionsParam<'a>>,
    /// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<ConfirmSetupIntentPaymentMethodOptionsSepaDebit>,
    /// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<ConfirmSetupIntentPaymentMethodOptionsUsBankAccount<'a>>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self {
            acss_debit: None,
            amazon_pay: None,
            card: None,
            card_present: None,
            link: None,
            paypal: None,
            sepa_debit: None,
            us_bank_account: None,
        }
    }
}
impl<'a> Default for ConfirmSetupIntentPaymentMethodOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self { currency: None, mandate_options: None, verification_method: None }
    }
}
impl<'a> Default for ConfirmSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
/// Must be a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
}
impl ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency::*;
        match self {
            Cad => "cad",
            Usd => "usd",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency::*;
        match s {
            "cad" => Ok(Cad),
            "usd" => Ok(Usd),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency",
            )
        })
    }
}
/// Additional fields for Mandate creation
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,.
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<&'a [ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor]>,
    /// Description of the mandate interval.
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
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
impl<'a> Default for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// List of Stripe products where this mandate can be selected automatically.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}
impl ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::*;
        match self {
            Invoice => "invoice",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::*;
        match s {
            "invoice" => Ok(Invoice),
            "subscription" => Ok(Subscription),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor"))
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}
impl ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
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
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule"))
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}
impl ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
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
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType"))
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod"))
    }
}
/// Configuration for any card setup attempted on this SetupIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsCard<'a> {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmSetupIntentPaymentMethodOptionsCardMandateOptions<'a>>,
    /// When specified, this parameter signals that a card has been collected
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA. This
    /// parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    /// Selected network to process this SetupIntent on.
    /// Depends on the available networks of the card attached to the SetupIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<ConfirmSetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,
    /// If 3D Secure authentication was performed with a third-party provider,
    /// the authentication details to use for this setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecure<'a>>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsCard<'a> {
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
impl<'a> Default for ConfirmSetupIntentPaymentMethodOptionsCard<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsCardMandateOptions<'a> {
    /// Amount to be charged for future payments.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
    /// Currency in which future payments will be charged.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// End date of the mandate or subscription.
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<stripe_types::Timestamp>,
    /// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
    /// The number of intervals between payments.
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: &'a str,
    /// Start date of the mandate or subscription. Start date should not be lesser than yesterday.
    pub start_date: stripe_types::Timestamp,
    /// Specifies the type of mandates supported. Possible values are `india`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types:
        Option<&'a [ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes]>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new(
        amount: i64,
        amount_type: ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
        currency: stripe_types::Currency,
        interval: ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
        reference: &'a str,
        start_date: stripe_types::Timestamp,
    ) -> Self {
        Self {
            amount,
            amount_type,
            currency,
            description: None,
            end_date: None,
            interval,
            interval_count: None,
            reference,
            start_date,
            supported_types: None,
        }
    }
}
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}
impl ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType"))
    }
}
/// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}
impl ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Sporadic => "sporadic",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "sporadic" => Ok(Sporadic),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval"))
    }
}
/// Specifies the type of mandates supported. Possible values are `india`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}
impl ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes::*;
        match self {
            India => "india",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes::*;
        match s {
            "india" => Ok(India),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes"))
    }
}
/// Selected network to process this SetupIntent on.
/// Depends on the available networks of the card attached to the SetupIntent.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}
impl ConfirmSetupIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsCardNetwork::*;
        match self {
            Amex => "amex",
            CartesBancaires => "cartes_bancaires",
            Diners => "diners",
            Discover => "discover",
            EftposAu => "eftpos_au",
            Interac => "interac",
            Jcb => "jcb",
            Mastercard => "mastercard",
            Unionpay => "unionpay",
            Unknown => "unknown",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodOptionsCardNetwork {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsCardNetwork::*;
        match s {
            "amex" => Ok(Amex),
            "cartes_bancaires" => Ok(CartesBancaires),
            "diners" => Ok(Diners),
            "discover" => Ok(Discover),
            "eftpos_au" => Ok(EftposAu),
            "interac" => Ok(Interac),
            "jcb" => Ok(Jcb),
            "mastercard" => Ok(Mastercard),
            "unionpay" => Ok(Unionpay),
            "unknown" => Ok(Unknown),
            "visa" => Ok(Visa),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodOptionsCardNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmSetupIntentPaymentMethodOptionsCardNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ConfirmSetupIntentPaymentMethodOptionsCardNetwork",
            )
        })
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// If not provided, this value defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}
impl ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
            Challenge => "challenge",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            "challenge" => Ok(Challenge),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure",
            )
        })
    }
}
/// If 3D Secure authentication was performed with a third-party provider,
/// the authentication details to use for this setup.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecure<'a> {
    /// The `transStatus` returned from the card Issuer’s ACS in the ARes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ares_trans_status:
        Option<ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus>,
    /// The cryptogram, also known as the "authentication value" (AAV, CAVV or
    /// AEVV). This value is 20 bytes, base64-encoded into a 28-character string.
    /// (Most 3D Secure providers will return the base64-encoded version, which
    /// is what you should specify here.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptogram: Option<&'a str>,
    /// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
    /// provider and indicates what degree of authentication was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator:
        Option<ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator>,
    /// Network specific 3DS fields. Network specific arguments require an
    /// explicit card brand choice. The parameter `payment_method_options.card.network``
    /// must be populated accordingly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options:
        Option<ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions<'a>>,
    /// The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the
    /// AReq sent to the card Issuer's ACS. A string containing 2 digits from 01-99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_challenge_indicator: Option<&'a str>,
    /// For 3D Secure 1, the XID. For 3D Secure 2, the Directory Server
    /// Transaction ID (dsTransID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<&'a str>,
    /// The version of 3D Secure that was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureVersion>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecure<'a> {
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
impl<'a> Default for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecure<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The `transStatus` returned from the card Issuer’s ACS in the ARes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    A,
    C,
    I,
    N,
    R,
    U,
    Y,
}
impl ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::*;
        match self {
            A => "A",
            C => "C",
            I => "I",
            N => "N",
            R => "R",
            U => "U",
            Y => "Y",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::*;
        match s {
            "A" => Ok(A),
            "C" => Ok(C),
            "I" => Ok(I),
            "N" => Ok(N),
            "R" => Ok(R),
            "U" => Ok(U),
            "Y" => Ok(Y),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus"))
    }
}
/// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
/// provider and indicates what degree of authentication was performed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    V01,
    V02,
    V05,
    V06,
    V07,
}
impl ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::*;
        match self {
            V01 => "01",
            V02 => "02",
            V05 => "05",
            V06 => "06",
            V07 => "07",
        }
    }
}

impl std::str::FromStr
    for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::*;
        match s {
            "01" => Ok(V01),
            "02" => Ok(V02),
            "05" => Ok(V05),
            "06" => Ok(V06),
            "07" => Ok(V07),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
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
    for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator"))
    }
}
/// Network specific 3DS fields. Network specific arguments require an
/// explicit card brand choice. The parameter `payment_method_options.card.network``
/// must be populated accordingly
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions<'a> {
    /// Cartes Bancaires-specific 3DS fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires: Option<
        ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires<'a>,
    >,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions<'a> {
    pub fn new() -> Self {
        Self { cartes_bancaires: None }
    }
}
impl<'a> Default for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Cartes Bancaires-specific 3DS fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires<'a> {
    /// The cryptogram calculation algorithm used by the card Issuer's ACS
    /// to calculate the Authentication cryptogram. Also known as `cavvAlgorithm`.
    /// messageExtension: CB-AVALGO
    pub cb_avalgo:
        ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo,
    /// The exemption indicator returned from Cartes Bancaires in the ARes.
    /// message extension: CB-EXEMPTION; string (4 characters)
    /// This is a 3 byte bitmap (low significant byte first and most significant
    /// bit first) that has been Base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_exemption: Option<&'a str>,
    /// The risk score returned from Cartes Bancaires in the ARes.
    /// message extension: CB-SCORE; numeric value 0-99
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_score: Option<i64>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires<'a> {
    pub fn new(
        cb_avalgo: ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo,
    ) -> Self {
        Self { cb_avalgo, cb_exemption: None, cb_score: None }
    }
}
/// The cryptogram calculation algorithm used by the card Issuer's ACS
/// to calculate the Authentication cryptogram. Also known as `cavvAlgorithm`.
/// messageExtension: CB-AVALGO
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    V0,
    V1,
    V2,
    V3,
    V4,
    A,
}
impl ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::*;
        match self {
            V0 => "0",
            V1 => "1",
            V2 => "2",
            V3 => "3",
            V4 => "4",
            A => "A",
        }
    }
}

impl std::str::FromStr
    for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::*;
        match s {
            "0" => Ok(V0),
            "1" => Ok(V1),
            "2" => Ok(V2),
            "3" => Ok(V3),
            "4" => Ok(V4),
            "A" => Ok(A),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
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
    for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo"))
    }
}
/// The version of 3D Secure that was performed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    V1_0_2,
    V2_1_0,
    V2_2_0,
}
impl ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureVersion::*;
        match self {
            V1_0_2 => "1.0.2",
            V2_1_0 => "2.1.0",
            V2_2_0 => "2.2.0",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureVersion::*;
        match s {
            "1.0.2" => Ok(V1_0_2),
            "2.1.0" => Ok(V2_1_0),
            "2.2.0" => Ok(V2_2_0),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureVersion
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ConfirmSetupIntentPaymentMethodOptionsCardThreeDSecureVersion",
            )
        })
    }
}
/// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub mandate_options: Option<miniserde::json::Value>,
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
/// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a>>,
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions>,
    /// Additional fields for network related functions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a>>,
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self {
            financial_connections: None,
            mandate_options: None,
            networks: None,
            verification_method: None,
        }
    }
}
impl<'a> Default for ConfirmSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Financial Connections Session creation
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        &'a [ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions],
    >,
    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<
        &'a [ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch],
    >,
    /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self { permissions: None, prefetch: None, return_url: None }
    }
}
impl<'a> Default for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The list of permissions to request.
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}
impl ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
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
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
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
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions"))
    }
}
/// List of data features that you would like to retrieve upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    Balances,
    Ownership,
    Transactions,
}
impl ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
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
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch"))
    }
}
/// Additional fields for Mandate creation
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions {
    /// The method used to collect offline mandate customer acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method:
        Option<ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}
impl ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions {
    pub fn new() -> Self {
        Self { collection_method: None }
    }
}
impl Default for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// The method used to collect offline mandate customer acceptance.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    Paper,
}
impl ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::*;
        match self {
            Paper => "paper",
        }
    }
}

impl std::str::FromStr
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::*;
        match s {
            "paper" => Ok(Paper),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
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
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod"))
    }
}
/// Additional fields for network related functions
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    /// Triggers validations to run across the selected networks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested:
        Option<&'a [ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested]>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl<'a> Default for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Triggers validations to run across the selected networks
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    Ach,
    UsDomesticWire,
}
impl ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested"))
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod"))
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntent<'a> {
    inner: ConfirmSetupIntentBuilder<'a>,
    intent: &'a stripe_shared::SetupIntentId,
}
impl<'a> ConfirmSetupIntent<'a> {
    /// Construct a new `ConfirmSetupIntent`.
    pub fn new(intent: &'a stripe_shared::SetupIntentId) -> Self {
        Self { intent, inner: ConfirmSetupIntentBuilder::new() }
    }
    /// ID of the ConfirmationToken used to confirm this SetupIntent.
    ///
    /// If the provided ConfirmationToken contains properties that are also being provided in this request, such as `payment_method`, then the values in this request will take precedence.
    pub fn confirmation_token(mut self, confirmation_token: &'a str) -> Self {
        self.inner.confirmation_token = Some(confirmation_token);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    pub fn mandate_data(mut self, mandate_data: ConfirmSetupIntentMandateData<'a>) -> Self {
        self.inner.mandate_data = Some(mandate_data);
        self
    }
    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    pub fn payment_method(mut self, payment_method: &'a str) -> Self {
        self.inner.payment_method = Some(payment_method);
        self
    }
    /// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method).
    /// value in the SetupIntent.
    pub fn payment_method_data(
        mut self,
        payment_method_data: ConfirmSetupIntentPaymentMethodData<'a>,
    ) -> Self {
        self.inner.payment_method_data = Some(payment_method_data);
        self
    }
    /// Payment method-specific configuration for this SetupIntent.
    pub fn payment_method_options(
        mut self,
        payment_method_options: ConfirmSetupIntentPaymentMethodOptions<'a>,
    ) -> Self {
        self.inner.payment_method_options = Some(payment_method_options);
        self
    }
    /// The URL to redirect your customer back to after they authenticate on the payment method's app or site.
    /// If you'd prefer to redirect to a mobile application, you can alternatively supply an application URI scheme.
    /// This parameter is only used for cards and other redirect-based payment methods.
    pub fn return_url(mut self, return_url: &'a str) -> Self {
        self.inner.return_url = Some(return_url);
        self
    }
    /// Set to `true` when confirming server-side and using Stripe.js, iOS, or Android client-side SDKs to handle the next actions.
    pub fn use_stripe_sdk(mut self, use_stripe_sdk: bool) -> Self {
        self.inner.use_stripe_sdk = Some(use_stripe_sdk);
        self
    }
}
impl ConfirmSetupIntent<'_> {
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

impl StripeRequest for ConfirmSetupIntent<'_> {
    type Output = stripe_shared::SetupIntent;

    fn build(&self) -> RequestBuilder {
        let intent = self.intent;
        RequestBuilder::new(StripeMethod::Post, format!("/setup_intents/{intent}/confirm"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct VerifyMicrodepositsSetupIntentBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amounts: Option<&'a [i64]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    descriptor_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> VerifyMicrodepositsSetupIntentBuilder<'a> {
    fn new() -> Self {
        Self { amounts: None, descriptor_code: None, expand: None }
    }
}
/// Verifies microdeposits on a SetupIntent object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct VerifyMicrodepositsSetupIntent<'a> {
    inner: VerifyMicrodepositsSetupIntentBuilder<'a>,
    intent: &'a stripe_shared::SetupIntentId,
}
impl<'a> VerifyMicrodepositsSetupIntent<'a> {
    /// Construct a new `VerifyMicrodepositsSetupIntent`.
    pub fn new(intent: &'a stripe_shared::SetupIntentId) -> Self {
        Self { intent, inner: VerifyMicrodepositsSetupIntentBuilder::new() }
    }
    /// Two positive integers, in *cents*, equal to the values of the microdeposits sent to the bank account.
    pub fn amounts(mut self, amounts: &'a [i64]) -> Self {
        self.inner.amounts = Some(amounts);
        self
    }
    /// A six-character code starting with SM present in the microdeposit sent to the bank account.
    pub fn descriptor_code(mut self, descriptor_code: &'a str) -> Self {
        self.inner.descriptor_code = Some(descriptor_code);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl VerifyMicrodepositsSetupIntent<'_> {
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

impl StripeRequest for VerifyMicrodepositsSetupIntent<'_> {
    type Output = stripe_shared::SetupIntent;

    fn build(&self) -> RequestBuilder {
        let intent = self.intent;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/setup_intents/{intent}/verify_microdeposits"),
        )
        .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct OnlineParam<'a> {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: &'a str,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: &'a str,
}
impl<'a> OnlineParam<'a> {
    pub fn new(ip_address: &'a str, user_agent: &'a str) -> Self {
        Self { ip_address, user_agent }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PaymentMethodParam<'a> {
    /// Customer's bank account number.
    pub account_number: &'a str,
    /// Institution number of the customer's bank.
    pub institution_number: &'a str,
    /// Transit number of the customer's bank.
    pub transit_number: &'a str,
}
impl<'a> PaymentMethodParam<'a> {
    pub fn new(
        account_number: &'a str,
        institution_number: &'a str,
        transit_number: &'a str,
    ) -> Self {
        Self { account_number, institution_number, transit_number }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct BillingDetailsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
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
impl<'a> BillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl<'a> Default for BillingDetailsAddress<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DateOfBirth {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl DateOfBirth {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RadarOptionsWithHiddenOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> RadarOptionsWithHiddenOptions<'a> {
    pub fn new() -> Self {
        Self { session: None }
    }
}
impl<'a> Default for RadarOptionsWithHiddenOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SetupIntentPaymentMethodOptionsParam<'a> {
    /// \[Deprecated\] This is a legacy parameter that no longer has any function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<&'a str>,
}
impl<'a> SetupIntentPaymentMethodOptionsParam<'a> {
    pub fn new() -> Self {
        Self { persistent_token: None }
    }
}
impl<'a> Default for SetupIntentPaymentMethodOptionsParam<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PaymentMethodOptionsParam<'a> {
    /// The PayPal Billing Agreement ID (BAID).
    /// This is an ID generated by PayPal which represents the mandate between the merchant and the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_agreement_id: Option<&'a str>,
}
impl<'a> PaymentMethodOptionsParam<'a> {
    pub fn new() -> Self {
        Self { billing_agreement_id: None }
    }
}
impl<'a> Default for PaymentMethodOptionsParam<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct BillingDetailsInnerParams<'a> {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<BillingDetailsAddress<'a>>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> BillingDetailsInnerParams<'a> {
    pub fn new() -> Self {
        Self { address: None, email: None, name: None, phone: None }
    }
}
impl<'a> Default for BillingDetailsInnerParams<'a> {
    fn default() -> Self {
        Self::new()
    }
}
