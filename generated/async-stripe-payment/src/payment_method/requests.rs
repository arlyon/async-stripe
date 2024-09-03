use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListPaymentMethodBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<ListPaymentMethodType>,
}
impl<'a> ListPaymentMethodBuilder<'a> {
    fn new() -> Self {
        Self {
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_: None,
        }
    }
}
/// An optional filter on the list, based on the object `type` field.
/// Without the filter, the list includes all current and future payment method types.
/// If your integration expects only one type of payment method in the response, make sure to provide a type value in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListPaymentMethodType {
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
impl ListPaymentMethodType {
    pub fn as_str(self) -> &'static str {
        use ListPaymentMethodType::*;
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

impl std::str::FromStr for ListPaymentMethodType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListPaymentMethodType::*;
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
impl std::fmt::Display for ListPaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListPaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListPaymentMethodType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListPaymentMethodType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Returns a list of PaymentMethods for Treasury flows.
/// If you want to list the PaymentMethods attached to a Customer for payments, you should use the [List a Customer’s PaymentMethods](https://stripe.com/docs/api/payment_methods/customer_list) API instead.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListPaymentMethod<'a> {
    inner: ListPaymentMethodBuilder<'a>,
}
impl<'a> ListPaymentMethod<'a> {
    /// Construct a new `ListPaymentMethod`.
    pub fn new() -> Self {
        Self { inner: ListPaymentMethodBuilder::new() }
    }
    /// The ID of the customer whose PaymentMethods will be retrieved.
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// An optional filter on the list, based on the object `type` field.
    /// Without the filter, the list includes all current and future payment method types.
    /// If your integration expects only one type of payment method in the response, make sure to provide a type value in the request.
    pub fn type_(mut self, type_: ListPaymentMethodType) -> Self {
        self.inner.type_ = Some(type_);
        self
    }
}
impl<'a> Default for ListPaymentMethod<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPaymentMethod<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::PaymentMethod>> {
        stripe_client_core::ListPaginator::new_list("/payment_methods", self.inner)
    }
}

impl StripeRequest for ListPaymentMethod<'_> {
    type Output = stripe_types::List<stripe_shared::PaymentMethod>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/payment_methods").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrievePaymentMethodBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentMethodBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a PaymentMethod object attached to the StripeAccount.
/// To retrieve a payment method attached to a Customer, you should use [Retrieve a Customer’s PaymentMethods](https://stripe.com/docs/api/payment_methods/customer).
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePaymentMethod<'a> {
    inner: RetrievePaymentMethodBuilder<'a>,
    payment_method: &'a stripe_shared::PaymentMethodId,
}
impl<'a> RetrievePaymentMethod<'a> {
    /// Construct a new `RetrievePaymentMethod`.
    pub fn new(payment_method: &'a stripe_shared::PaymentMethodId) -> Self {
        Self { payment_method, inner: RetrievePaymentMethodBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrievePaymentMethod<'_> {
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

impl StripeRequest for RetrievePaymentMethod<'_> {
    type Output = stripe_shared::PaymentMethod;

    fn build(&self) -> RequestBuilder {
        let payment_method = self.payment_method;
        RequestBuilder::new(StripeMethod::Get, format!("/payment_methods/{payment_method}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreatePaymentMethodBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    acss_debit: Option<CreatePaymentMethodAcssDebit<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    affirm: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    afterpay_clearpay: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    alipay: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_redisplay: Option<CreatePaymentMethodAllowRedisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    amazon_pay: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    au_becs_debit: Option<CreatePaymentMethodAuBecsDebit<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bacs_debit: Option<CreatePaymentMethodBacsDebit<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    bancontact: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_details: Option<BillingDetailsInnerParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    blik: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boleto: Option<CreatePaymentMethodBoleto<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<CreatePaymentMethodCard<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    cashapp: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    customer_balance: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eps: Option<CreatePaymentMethodEps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fpx: Option<CreatePaymentMethodFpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    giropay: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    grabpay: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ideal: Option<CreatePaymentMethodIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    interac_present: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    klarna: Option<CreatePaymentMethodKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    konbini: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    link: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    mobilepay: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    oxxo: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    p24: Option<CreatePaymentMethodP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    paynow: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    paypal: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pix: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    promptpay: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radar_options: Option<CreatePaymentMethodRadarOptions<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    revolut_pay: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sepa_debit: Option<CreatePaymentMethodSepaDebit<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sofort: Option<CreatePaymentMethodSofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    swish: Option<miniserde::json::Value>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<CreatePaymentMethodType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    us_bank_account: Option<CreatePaymentMethodUsBankAccount<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    wechat_pay: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    zip: Option<miniserde::json::Value>,
}
impl<'a> CreatePaymentMethodBuilder<'a> {
    fn new() -> Self {
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
            card: None,
            cashapp: None,
            customer: None,
            customer_balance: None,
            eps: None,
            expand: None,
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
            payment_method: None,
            paynow: None,
            paypal: None,
            pix: None,
            promptpay: None,
            radar_options: None,
            revolut_pay: None,
            sepa_debit: None,
            sofort: None,
            swish: None,
            type_: None,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodAcssDebit<'a> {
    /// Customer's bank account number.
    pub account_number: &'a str,
    /// Institution number of the customer's bank.
    pub institution_number: &'a str,
    /// Transit number of the customer's bank.
    pub transit_number: &'a str,
}
impl<'a> CreatePaymentMethodAcssDebit<'a> {
    pub fn new(
        account_number: &'a str,
        institution_number: &'a str,
        transit_number: &'a str,
    ) -> Self {
        Self { account_number, institution_number, transit_number }
    }
}
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl CreatePaymentMethodAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentMethodAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePaymentMethodAllowRedisplay")
        })
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodAuBecsDebit<'a> {
    /// The account number for the bank account.
    pub account_number: &'a str,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: &'a str,
}
impl<'a> CreatePaymentMethodAuBecsDebit<'a> {
    pub fn new(account_number: &'a str, bsb_number: &'a str) -> Self {
        Self { account_number, bsb_number }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodBacsDebit<'a> {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Sort code of the bank account. (e.g., `10-20-30`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<&'a str>,
}
impl<'a> CreatePaymentMethodBacsDebit<'a> {
    pub fn new() -> Self {
        Self { account_number: None, sort_code: None }
    }
}
impl<'a> Default for CreatePaymentMethodBacsDebit<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodBoleto<'a> {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers)
    pub tax_id: &'a str,
}
impl<'a> CreatePaymentMethodBoleto<'a> {
    pub fn new(tax_id: &'a str) -> Self {
        Self { tax_id }
    }
}
/// If this is a `card` PaymentMethod, this hash contains the user's card details.
/// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format `card: {token: "tok_visa"}`.
/// When providing a card number, you must meet the requirements for [PCI compliance](https://stripe.com/docs/security#validating-pci-compliance).
/// We strongly recommend using Stripe.js instead of interacting with this API directly.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodCard<'a> {
    #[serde(untagged)]
    CardDetailsParams(CreatePaymentMethodCardDetailsParams<'a>),
    #[serde(untagged)]
    TokenParams(CreatePaymentMethodTokenParams<'a>),
}
/// If this is a `card` PaymentMethod, this hash contains the user's card details.
/// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format `card: {token: "tok_visa"}`.
/// When providing a card number, you must meet the requirements for [PCI compliance](https://stripe.com/docs/security#validating-pci-compliance).
/// We strongly recommend using Stripe.js instead of interacting with this API directly.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodCardDetailsParams<'a> {
    /// The card's CVC. It is highly recommended to always include this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<&'a str>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    /// Contains information about card networks used to process the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<CreatePaymentMethodCardDetailsParamsNetworks>,
    /// The card number, as a string without any separators.
    pub number: &'a str,
}
impl<'a> CreatePaymentMethodCardDetailsParams<'a> {
    pub fn new(exp_month: i64, exp_year: i64, number: &'a str) -> Self {
        Self { cvc: None, exp_month, exp_year, networks: None, number }
    }
}
/// Contains information about card networks used to process the payment.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodCardDetailsParamsNetworks {
    /// The customer's preferred card network for co-branded cards.
    /// Supports `cartes_bancaires`, `mastercard`, or `visa`.
    /// Selection of a network that does not apply to the card will be stored as `invalid_preference` on the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<CreatePaymentMethodCardDetailsParamsNetworksPreferred>,
}
impl CreatePaymentMethodCardDetailsParamsNetworks {
    pub fn new() -> Self {
        Self { preferred: None }
    }
}
impl Default for CreatePaymentMethodCardDetailsParamsNetworks {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's preferred card network for co-branded cards.
/// Supports `cartes_bancaires`, `mastercard`, or `visa`.
/// Selection of a network that does not apply to the card will be stored as `invalid_preference` on the card.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodCardDetailsParamsNetworksPreferred {
    CartesBancaires,
    Mastercard,
    Visa,
}
impl CreatePaymentMethodCardDetailsParamsNetworksPreferred {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodCardDetailsParamsNetworksPreferred::*;
        match self {
            CartesBancaires => "cartes_bancaires",
            Mastercard => "mastercard",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodCardDetailsParamsNetworksPreferred {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodCardDetailsParamsNetworksPreferred::*;
        match s {
            "cartes_bancaires" => Ok(CartesBancaires),
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodCardDetailsParamsNetworksPreferred {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodCardDetailsParamsNetworksPreferred {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodCardDetailsParamsNetworksPreferred {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentMethodCardDetailsParamsNetworksPreferred {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentMethodCardDetailsParamsNetworksPreferred",
            )
        })
    }
}
/// If this is a `card` PaymentMethod, this hash contains the user's card details.
/// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format `card: {token: "tok_visa"}`.
/// When providing a card number, you must meet the requirements for [PCI compliance](https://stripe.com/docs/security#validating-pci-compliance).
/// We strongly recommend using Stripe.js instead of interacting with this API directly.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodTokenParams<'a> {
    /// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format card: {token: "tok_visa"}.
    pub token: &'a str,
}
impl<'a> CreatePaymentMethodTokenParams<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentMethodEpsBank>,
}
impl CreatePaymentMethodEps {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for CreatePaymentMethodEps {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreatePaymentMethodEpsBank {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodEpsBank::*;
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

impl std::str::FromStr for CreatePaymentMethodEpsBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodEpsBank::*;
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
impl std::fmt::Display for CreatePaymentMethodEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodEpsBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentMethodEpsBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodFpx {
    /// Account holder type for FPX transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreatePaymentMethodFpxAccountHolderType>,
    /// The customer's bank.
    pub bank: CreatePaymentMethodFpxBank,
}
impl CreatePaymentMethodFpx {
    pub fn new(bank: CreatePaymentMethodFpxBank) -> Self {
        Self { account_holder_type: None, bank }
    }
}
/// Account holder type for FPX transaction
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodFpxAccountHolderType {
    Company,
    Individual,
}
impl CreatePaymentMethodFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodFpxAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodFpxAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodFpxAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodFpxAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentMethodFpxAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePaymentMethodFpxAccountHolderType")
        })
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreatePaymentMethodFpxBank {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodFpxBank::*;
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

impl std::str::FromStr for CreatePaymentMethodFpxBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodFpxBank::*;
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
impl std::fmt::Display for CreatePaymentMethodFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodFpxBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentMethodFpxBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentMethodIdealBank>,
}
impl CreatePaymentMethodIdeal {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for CreatePaymentMethodIdeal {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreatePaymentMethodIdealBank {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodIdealBank::*;
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

impl std::str::FromStr for CreatePaymentMethodIdealBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodIdealBank::*;
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
impl std::fmt::Display for CreatePaymentMethodIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodIdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentMethodIdealBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodKlarna {
    /// Customer's date of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreatePaymentMethodKlarnaDob>,
}
impl CreatePaymentMethodKlarna {
    pub fn new() -> Self {
        Self { dob: None }
    }
}
impl Default for CreatePaymentMethodKlarna {
    fn default() -> Self {
        Self::new()
    }
}
/// Customer's date of birth
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl CreatePaymentMethodKlarnaDob {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentMethodP24Bank>,
}
impl CreatePaymentMethodP24 {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for CreatePaymentMethodP24 {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreatePaymentMethodP24Bank {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodP24Bank::*;
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

impl std::str::FromStr for CreatePaymentMethodP24Bank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodP24Bank::*;
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
impl std::fmt::Display for CreatePaymentMethodP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodP24Bank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentMethodP24Bank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Options to configure Radar.
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodRadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> CreatePaymentMethodRadarOptions<'a> {
    pub fn new() -> Self {
        Self { session: None }
    }
}
impl<'a> Default for CreatePaymentMethodRadarOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodSepaDebit<'a> {
    /// IBAN of the bank account.
    pub iban: &'a str,
}
impl<'a> CreatePaymentMethodSepaDebit<'a> {
    pub fn new(iban: &'a str) -> Self {
        Self { iban }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: CreatePaymentMethodSofortCountry,
}
impl CreatePaymentMethodSofort {
    pub fn new(country: CreatePaymentMethodSofortCountry) -> Self {
        Self { country }
    }
}
/// Two-letter ISO code representing the country the bank account is located in.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodSofortCountry {
    At,
    Be,
    De,
    Es,
    It,
    Nl,
}
impl CreatePaymentMethodSofortCountry {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodSofortCountry::*;
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

impl std::str::FromStr for CreatePaymentMethodSofortCountry {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodSofortCountry::*;
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
impl std::fmt::Display for CreatePaymentMethodSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodSofortCountry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentMethodSofortCountry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePaymentMethodSofortCountry")
        })
    }
}
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodType {
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
impl CreatePaymentMethodType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodType::*;
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

impl std::str::FromStr for CreatePaymentMethodType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodType::*;
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
impl std::fmt::Display for CreatePaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentMethodType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreatePaymentMethodUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<CreatePaymentMethodUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> CreatePaymentMethodUsBankAccount<'a> {
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
impl<'a> Default for CreatePaymentMethodUsBankAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodUsBankAccountAccountHolderType {
    Company,
    Individual,
}
impl CreatePaymentMethodUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodUsBankAccountAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentMethodUsBankAccountAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentMethodUsBankAccountAccountHolderType",
            )
        })
    }
}
/// Account type: checkings or savings. Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodUsBankAccountAccountType {
    Checking,
    Savings,
}
impl CreatePaymentMethodUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodUsBankAccountAccountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentMethodUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentMethodUsBankAccountAccountType",
            )
        })
    }
}
/// Creates a PaymentMethod object.
/// Read the [Stripe.js reference](https://stripe.com/docs/stripe-js/reference#stripe-create-payment-method) to learn how to create PaymentMethods via Stripe.js.
///
/// Instead of creating a PaymentMethod directly, we recommend using the [PaymentIntents](https://stripe.com/docs/payments/accept-a-payment) API to accept a payment immediately or the [SetupIntent](https://stripe.com/docs/payments/save-and-reuse) API to collect payment method details ahead of a future payment.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethod<'a> {
    inner: CreatePaymentMethodBuilder<'a>,
}
impl<'a> CreatePaymentMethod<'a> {
    /// Construct a new `CreatePaymentMethod`.
    pub fn new() -> Self {
        Self { inner: CreatePaymentMethodBuilder::new() }
    }
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    pub fn acss_debit(mut self, acss_debit: CreatePaymentMethodAcssDebit<'a>) -> Self {
        self.inner.acss_debit = Some(acss_debit);
        self
    }
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    pub fn affirm(mut self, affirm: miniserde::json::Value) -> Self {
        self.inner.affirm = Some(affirm);
        self
    }
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    pub fn afterpay_clearpay(mut self, afterpay_clearpay: miniserde::json::Value) -> Self {
        self.inner.afterpay_clearpay = Some(afterpay_clearpay);
        self
    }
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    pub fn alipay(mut self, alipay: miniserde::json::Value) -> Self {
        self.inner.alipay = Some(alipay);
        self
    }
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to `unspecified`.
    pub fn allow_redisplay(mut self, allow_redisplay: CreatePaymentMethodAllowRedisplay) -> Self {
        self.inner.allow_redisplay = Some(allow_redisplay);
        self
    }
    /// If this is a AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method.
    pub fn amazon_pay(mut self, amazon_pay: miniserde::json::Value) -> Self {
        self.inner.amazon_pay = Some(amazon_pay);
        self
    }
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    pub fn au_becs_debit(mut self, au_becs_debit: CreatePaymentMethodAuBecsDebit<'a>) -> Self {
        self.inner.au_becs_debit = Some(au_becs_debit);
        self
    }
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    pub fn bacs_debit(mut self, bacs_debit: CreatePaymentMethodBacsDebit<'a>) -> Self {
        self.inner.bacs_debit = Some(bacs_debit);
        self
    }
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    pub fn bancontact(mut self, bancontact: miniserde::json::Value) -> Self {
        self.inner.bancontact = Some(bancontact);
        self
    }
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    pub fn billing_details(mut self, billing_details: BillingDetailsInnerParams<'a>) -> Self {
        self.inner.billing_details = Some(billing_details);
        self
    }
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    pub fn blik(mut self, blik: miniserde::json::Value) -> Self {
        self.inner.blik = Some(blik);
        self
    }
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    pub fn boleto(mut self, boleto: CreatePaymentMethodBoleto<'a>) -> Self {
        self.inner.boleto = Some(boleto);
        self
    }
    /// If this is a `card` PaymentMethod, this hash contains the user's card details.
    /// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format `card: {token: "tok_visa"}`.
    /// When providing a card number, you must meet the requirements for [PCI compliance](https://stripe.com/docs/security#validating-pci-compliance).
    /// We strongly recommend using Stripe.js instead of interacting with this API directly.
    pub fn card(mut self, card: CreatePaymentMethodCard<'a>) -> Self {
        self.inner.card = Some(card);
        self
    }
    /// If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method.
    pub fn cashapp(mut self, cashapp: miniserde::json::Value) -> Self {
        self.inner.cashapp = Some(cashapp);
        self
    }
    /// The `Customer` to whom the original PaymentMethod is attached.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    pub fn customer_balance(mut self, customer_balance: miniserde::json::Value) -> Self {
        self.inner.customer_balance = Some(customer_balance);
        self
    }
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    pub fn eps(mut self, eps: CreatePaymentMethodEps) -> Self {
        self.inner.eps = Some(eps);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    pub fn fpx(mut self, fpx: CreatePaymentMethodFpx) -> Self {
        self.inner.fpx = Some(fpx);
        self
    }
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    pub fn giropay(mut self, giropay: miniserde::json::Value) -> Self {
        self.inner.giropay = Some(giropay);
        self
    }
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    pub fn grabpay(mut self, grabpay: miniserde::json::Value) -> Self {
        self.inner.grabpay = Some(grabpay);
        self
    }
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    pub fn ideal(mut self, ideal: CreatePaymentMethodIdeal) -> Self {
        self.inner.ideal = Some(ideal);
        self
    }
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    pub fn interac_present(mut self, interac_present: miniserde::json::Value) -> Self {
        self.inner.interac_present = Some(interac_present);
        self
    }
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    pub fn klarna(mut self, klarna: CreatePaymentMethodKlarna) -> Self {
        self.inner.klarna = Some(klarna);
        self
    }
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    pub fn konbini(mut self, konbini: miniserde::json::Value) -> Self {
        self.inner.konbini = Some(konbini);
        self
    }
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    pub fn link(mut self, link: miniserde::json::Value) -> Self {
        self.inner.link = Some(link);
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
    /// If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method.
    pub fn mobilepay(mut self, mobilepay: miniserde::json::Value) -> Self {
        self.inner.mobilepay = Some(mobilepay);
        self
    }
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    pub fn oxxo(mut self, oxxo: miniserde::json::Value) -> Self {
        self.inner.oxxo = Some(oxxo);
        self
    }
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    pub fn p24(mut self, p24: CreatePaymentMethodP24) -> Self {
        self.inner.p24 = Some(p24);
        self
    }
    /// The PaymentMethod to share.
    pub fn payment_method(mut self, payment_method: &'a str) -> Self {
        self.inner.payment_method = Some(payment_method);
        self
    }
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    pub fn paynow(mut self, paynow: miniserde::json::Value) -> Self {
        self.inner.paynow = Some(paynow);
        self
    }
    /// If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method.
    pub fn paypal(mut self, paypal: miniserde::json::Value) -> Self {
        self.inner.paypal = Some(paypal);
        self
    }
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    pub fn pix(mut self, pix: miniserde::json::Value) -> Self {
        self.inner.pix = Some(pix);
        self
    }
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    pub fn promptpay(mut self, promptpay: miniserde::json::Value) -> Self {
        self.inner.promptpay = Some(promptpay);
        self
    }
    /// Options to configure Radar.
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    pub fn radar_options(mut self, radar_options: CreatePaymentMethodRadarOptions<'a>) -> Self {
        self.inner.radar_options = Some(radar_options);
        self
    }
    /// If this is a `Revolut Pay` PaymentMethod, this hash contains details about the Revolut Pay payment method.
    pub fn revolut_pay(mut self, revolut_pay: miniserde::json::Value) -> Self {
        self.inner.revolut_pay = Some(revolut_pay);
        self
    }
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    pub fn sepa_debit(mut self, sepa_debit: CreatePaymentMethodSepaDebit<'a>) -> Self {
        self.inner.sepa_debit = Some(sepa_debit);
        self
    }
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    pub fn sofort(mut self, sofort: CreatePaymentMethodSofort) -> Self {
        self.inner.sofort = Some(sofort);
        self
    }
    /// If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method.
    pub fn swish(mut self, swish: miniserde::json::Value) -> Self {
        self.inner.swish = Some(swish);
        self
    }
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    pub fn type_(mut self, type_: CreatePaymentMethodType) -> Self {
        self.inner.type_ = Some(type_);
        self
    }
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    pub fn us_bank_account(
        mut self,
        us_bank_account: CreatePaymentMethodUsBankAccount<'a>,
    ) -> Self {
        self.inner.us_bank_account = Some(us_bank_account);
        self
    }
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    pub fn wechat_pay(mut self, wechat_pay: miniserde::json::Value) -> Self {
        self.inner.wechat_pay = Some(wechat_pay);
        self
    }
    /// If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method.
    pub fn zip(mut self, zip: miniserde::json::Value) -> Self {
        self.inner.zip = Some(zip);
        self
    }
}
impl<'a> Default for CreatePaymentMethod<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl CreatePaymentMethod<'_> {
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

impl StripeRequest for CreatePaymentMethod<'_> {
    type Output = stripe_shared::PaymentMethod;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/payment_methods").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdatePaymentMethodBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_redisplay: Option<UpdatePaymentMethodAllowRedisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_details: Option<BillingDetailsInnerParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<UpdatePaymentMethodCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    link: Option<miniserde::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    us_bank_account: Option<UpdatePaymentMethodUsBankAccount>,
}
impl<'a> UpdatePaymentMethodBuilder<'a> {
    fn new() -> Self {
        Self {
            allow_redisplay: None,
            billing_details: None,
            card: None,
            expand: None,
            link: None,
            metadata: None,
            us_bank_account: None,
        }
    }
}
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl UpdatePaymentMethodAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentMethodAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdatePaymentMethodAllowRedisplay")
        })
    }
}
/// If this is a `card` PaymentMethod, this hash contains the user's card details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodCard {
    /// Two-digit number representing the card's expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    /// Four-digit number representing the card's expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    /// Contains information about card networks used to process the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<UpdatePaymentMethodCardNetworks>,
}
impl UpdatePaymentMethodCard {
    pub fn new() -> Self {
        Self { exp_month: None, exp_year: None, networks: None }
    }
}
impl Default for UpdatePaymentMethodCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Contains information about card networks used to process the payment.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodCardNetworks {
    /// The customer's preferred card network for co-branded cards.
    /// Supports `cartes_bancaires`, `mastercard`, or `visa`.
    /// Selection of a network that does not apply to the card will be stored as `invalid_preference` on the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<UpdatePaymentMethodCardNetworksPreferred>,
}
impl UpdatePaymentMethodCardNetworks {
    pub fn new() -> Self {
        Self { preferred: None }
    }
}
impl Default for UpdatePaymentMethodCardNetworks {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's preferred card network for co-branded cards.
/// Supports `cartes_bancaires`, `mastercard`, or `visa`.
/// Selection of a network that does not apply to the card will be stored as `invalid_preference` on the card.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodCardNetworksPreferred {
    CartesBancaires,
    Mastercard,
    Visa,
}
impl UpdatePaymentMethodCardNetworksPreferred {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodCardNetworksPreferred::*;
        match self {
            CartesBancaires => "cartes_bancaires",
            Mastercard => "mastercard",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodCardNetworksPreferred {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodCardNetworksPreferred::*;
        match s {
            "cartes_bancaires" => Ok(CartesBancaires),
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodCardNetworksPreferred {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodCardNetworksPreferred {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodCardNetworksPreferred {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentMethodCardNetworksPreferred {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdatePaymentMethodCardNetworksPreferred")
        })
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodUsBankAccount {
    /// Bank account holder type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdatePaymentMethodUsBankAccountAccountHolderType>,
    /// Bank account type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<UpdatePaymentMethodUsBankAccountAccountType>,
}
impl UpdatePaymentMethodUsBankAccount {
    pub fn new() -> Self {
        Self { account_holder_type: None, account_type: None }
    }
}
impl Default for UpdatePaymentMethodUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Bank account holder type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodUsBankAccountAccountHolderType {
    Company,
    Individual,
}
impl UpdatePaymentMethodUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodUsBankAccountAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentMethodUsBankAccountAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentMethodUsBankAccountAccountHolderType",
            )
        })
    }
}
/// Bank account type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodUsBankAccountAccountType {
    Checking,
    Savings,
}
impl UpdatePaymentMethodUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodUsBankAccountAccountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentMethodUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentMethodUsBankAccountAccountType",
            )
        })
    }
}
/// Updates a PaymentMethod object. A PaymentMethod must be attached a customer to be updated.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethod<'a> {
    inner: UpdatePaymentMethodBuilder<'a>,
    payment_method: &'a stripe_shared::PaymentMethodId,
}
impl<'a> UpdatePaymentMethod<'a> {
    /// Construct a new `UpdatePaymentMethod`.
    pub fn new(payment_method: &'a stripe_shared::PaymentMethodId) -> Self {
        Self { payment_method, inner: UpdatePaymentMethodBuilder::new() }
    }
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to `unspecified`.
    pub fn allow_redisplay(mut self, allow_redisplay: UpdatePaymentMethodAllowRedisplay) -> Self {
        self.inner.allow_redisplay = Some(allow_redisplay);
        self
    }
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    pub fn billing_details(mut self, billing_details: BillingDetailsInnerParams<'a>) -> Self {
        self.inner.billing_details = Some(billing_details);
        self
    }
    /// If this is a `card` PaymentMethod, this hash contains the user's card details.
    pub fn card(mut self, card: UpdatePaymentMethodCard) -> Self {
        self.inner.card = Some(card);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    pub fn link(mut self, link: miniserde::json::Value) -> Self {
        self.inner.link = Some(link);
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
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    pub fn us_bank_account(mut self, us_bank_account: UpdatePaymentMethodUsBankAccount) -> Self {
        self.inner.us_bank_account = Some(us_bank_account);
        self
    }
}
impl UpdatePaymentMethod<'_> {
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

impl StripeRequest for UpdatePaymentMethod<'_> {
    type Output = stripe_shared::PaymentMethod;

    fn build(&self) -> RequestBuilder {
        let payment_method = self.payment_method;
        RequestBuilder::new(StripeMethod::Post, format!("/payment_methods/{payment_method}"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct AttachPaymentMethodBuilder<'a> {
    customer: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> AttachPaymentMethodBuilder<'a> {
    fn new(customer: &'a str) -> Self {
        Self { customer, expand: None }
    }
}
/// Attaches a PaymentMethod object to a Customer.
///
/// To attach a new PaymentMethod to a customer for future payments, we recommend you use a [SetupIntent](https://stripe.com/docs/api/setup_intents).
/// or a PaymentIntent with [setup_future_usage](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-setup_future_usage).
/// These approaches will perform any necessary steps to set up the PaymentMethod for future payments.
/// Using the `/v1/payment_methods/:id/attach`.
/// endpoint without first using a SetupIntent or PaymentIntent with `setup_future_usage` does not optimize the PaymentMethod for.
/// future use, which makes later declines and payment friction more likely.
/// See [Optimizing cards for future payments](https://stripe.com/docs/payments/payment-intents#future-usage) for more information about setting up.
/// future payments.
///
/// To use this PaymentMethod as the default for invoice or subscription payments,
/// set <a href="/docs/api/customers/update#update_customer-invoice_settings-default_payment_method">`invoice_settings.default_payment_method`</a>,.
/// on the Customer to the PaymentMethod’s ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct AttachPaymentMethod<'a> {
    inner: AttachPaymentMethodBuilder<'a>,
    payment_method: &'a stripe_shared::PaymentMethodId,
}
impl<'a> AttachPaymentMethod<'a> {
    /// Construct a new `AttachPaymentMethod`.
    pub fn new(payment_method: &'a stripe_shared::PaymentMethodId, customer: &'a str) -> Self {
        Self { payment_method, inner: AttachPaymentMethodBuilder::new(customer) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl AttachPaymentMethod<'_> {
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

impl StripeRequest for AttachPaymentMethod<'_> {
    type Output = stripe_shared::PaymentMethod;

    fn build(&self) -> RequestBuilder {
        let payment_method = self.payment_method;
        RequestBuilder::new(StripeMethod::Post, format!("/payment_methods/{payment_method}/attach"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct DetachPaymentMethodBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> DetachPaymentMethodBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Detaches a PaymentMethod object from a Customer.
/// After a PaymentMethod is detached, it can no longer be used for a payment or re-attached to a Customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DetachPaymentMethod<'a> {
    inner: DetachPaymentMethodBuilder<'a>,
    payment_method: &'a stripe_shared::PaymentMethodId,
}
impl<'a> DetachPaymentMethod<'a> {
    /// Construct a new `DetachPaymentMethod`.
    pub fn new(payment_method: &'a stripe_shared::PaymentMethodId) -> Self {
        Self { payment_method, inner: DetachPaymentMethodBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl DetachPaymentMethod<'_> {
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

impl StripeRequest for DetachPaymentMethod<'_> {
    type Output = stripe_shared::PaymentMethod;

    fn build(&self) -> RequestBuilder {
        let payment_method = self.payment_method;
        RequestBuilder::new(StripeMethod::Post, format!("/payment_methods/{payment_method}/detach"))
            .form(&self.inner)
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
