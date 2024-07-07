use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListPaymentMethodConfigurationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    application: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListPaymentMethodConfigurationBuilder<'a> {
    fn new() -> Self {
        Self {
            application: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// List payment method configurations
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListPaymentMethodConfiguration<'a> {
    inner: ListPaymentMethodConfigurationBuilder<'a>,
}
impl<'a> ListPaymentMethodConfiguration<'a> {
    /// Construct a new `ListPaymentMethodConfiguration`.
    pub fn new() -> Self {
        Self { inner: ListPaymentMethodConfigurationBuilder::new() }
    }
    /// The Connect application to filter by.
    pub fn application(mut self, application: &'a str) -> Self {
        self.inner.application = Some(application);
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
}
impl<'a> Default for ListPaymentMethodConfiguration<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPaymentMethodConfiguration<'_> {
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
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_payment::PaymentMethodConfiguration>,
    > {
        stripe_client_core::ListPaginator::new_list("/payment_method_configurations", self.inner)
    }
}

impl StripeRequest for ListPaymentMethodConfiguration<'_> {
    type Output = stripe_types::List<stripe_payment::PaymentMethodConfiguration>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/payment_method_configurations").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrievePaymentMethodConfigurationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentMethodConfigurationBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve payment method configuration
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePaymentMethodConfiguration<'a> {
    inner: RetrievePaymentMethodConfigurationBuilder<'a>,
    configuration: &'a stripe_payment::PaymentMethodConfigurationId,
}
impl<'a> RetrievePaymentMethodConfiguration<'a> {
    /// Construct a new `RetrievePaymentMethodConfiguration`.
    pub fn new(configuration: &'a stripe_payment::PaymentMethodConfigurationId) -> Self {
        Self { configuration, inner: RetrievePaymentMethodConfigurationBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrievePaymentMethodConfiguration<'_> {
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

impl StripeRequest for RetrievePaymentMethodConfiguration<'_> {
    type Output = stripe_payment::PaymentMethodConfiguration;

    fn build(&self) -> RequestBuilder {
        let configuration = self.configuration;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/payment_method_configurations/{configuration}"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreatePaymentMethodConfigurationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    acss_debit: Option<CreatePaymentMethodConfigurationAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    affirm: Option<CreatePaymentMethodConfigurationAffirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    afterpay_clearpay: Option<CreatePaymentMethodConfigurationAfterpayClearpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alipay: Option<CreatePaymentMethodConfigurationAlipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_pay: Option<CreatePaymentMethodConfigurationAmazonPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_pay: Option<CreatePaymentMethodConfigurationApplePay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_pay_later: Option<CreatePaymentMethodConfigurationApplePayLater>,
    #[serde(skip_serializing_if = "Option::is_none")]
    au_becs_debit: Option<CreatePaymentMethodConfigurationAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bacs_debit: Option<CreatePaymentMethodConfigurationBacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bancontact: Option<CreatePaymentMethodConfigurationBancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blik: Option<CreatePaymentMethodConfigurationBlik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boleto: Option<CreatePaymentMethodConfigurationBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<CreatePaymentMethodConfigurationCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cartes_bancaires: Option<CreatePaymentMethodConfigurationCartesBancaires>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cashapp: Option<CreatePaymentMethodConfigurationCashapp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_balance: Option<CreatePaymentMethodConfigurationCustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eps: Option<CreatePaymentMethodConfigurationEps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fpx: Option<CreatePaymentMethodConfigurationFpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    giropay: Option<CreatePaymentMethodConfigurationGiropay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_pay: Option<CreatePaymentMethodConfigurationGooglePay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grabpay: Option<CreatePaymentMethodConfigurationGrabpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ideal: Option<CreatePaymentMethodConfigurationIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jcb: Option<CreatePaymentMethodConfigurationJcb>,
    #[serde(skip_serializing_if = "Option::is_none")]
    klarna: Option<CreatePaymentMethodConfigurationKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    konbini: Option<CreatePaymentMethodConfigurationKonbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<CreatePaymentMethodConfigurationLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobilepay: Option<CreatePaymentMethodConfigurationMobilepay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oxxo: Option<CreatePaymentMethodConfigurationOxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    p24: Option<CreatePaymentMethodConfigurationP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paynow: Option<CreatePaymentMethodConfigurationPaynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paypal: Option<CreatePaymentMethodConfigurationPaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promptpay: Option<CreatePaymentMethodConfigurationPromptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revolut_pay: Option<CreatePaymentMethodConfigurationRevolutPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sepa_debit: Option<CreatePaymentMethodConfigurationSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sofort: Option<CreatePaymentMethodConfigurationSofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swish: Option<CreatePaymentMethodConfigurationSwish>,
    #[serde(skip_serializing_if = "Option::is_none")]
    us_bank_account: Option<CreatePaymentMethodConfigurationUsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wechat_pay: Option<CreatePaymentMethodConfigurationWechatPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip: Option<CreatePaymentMethodConfigurationZip>,
}
impl<'a> CreatePaymentMethodConfigurationBuilder<'a> {
    fn new() -> Self {
        Self {
            acss_debit: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            amazon_pay: None,
            apple_pay: None,
            apple_pay_later: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            blik: None,
            boleto: None,
            card: None,
            cartes_bancaires: None,
            cashapp: None,
            customer_balance: None,
            eps: None,
            expand: None,
            fpx: None,
            giropay: None,
            google_pay: None,
            grabpay: None,
            ideal: None,
            jcb: None,
            klarna: None,
            konbini: None,
            link: None,
            mobilepay: None,
            name: None,
            oxxo: None,
            p24: None,
            parent: None,
            paynow: None,
            paypal: None,
            promptpay: None,
            revolut_pay: None,
            sepa_debit: None,
            sofort: None,
            swish: None,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// Canadian pre-authorized debit payments, check this [page](https://stripe.com/docs/payments/acss-debit) for more details like country availability.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAcssDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAcssDebitDisplayPreference>,
}
impl CreatePaymentMethodConfigurationAcssDebit {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAcssDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAcssDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationAcssDebitDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAcssDebitDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference"))
    }
}
/// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
/// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
/// Check this [page](https://stripe.com/docs/payments/affirm) for more details like country availability.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAffirm {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAffirmDisplayPreference>,
}
impl CreatePaymentMethodConfigurationAffirm {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAffirm {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAffirmDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationAffirmDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAffirmDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference"))
    }
}
/// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://stripe.com/docs/payments/afterpay-clearpay) for more details like country availability.
/// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAfterpayClearpay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationAfterpayClearpay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAfterpayClearpay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
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
    for CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference"))
    }
}
/// Alipay is a digital wallet in China that has more than a billion active users worldwide.
/// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
/// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
/// Check this [page](https://stripe.com/docs/payments/alipay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAlipay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAlipayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationAlipay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAlipay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAlipayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationAlipayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAlipayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference"))
    }
}
/// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAmazonPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAmazonPayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationAmazonPay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAmazonPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAmazonPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationAmazonPayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAmazonPayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference"))
    }
}
/// Stripe users can accept [Apple Pay](/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
/// There are no additional fees to process Apple Pay payments, and the [pricing](/pricing) is the same as other card transactions.
/// Check this [page](https://stripe.com/docs/apple-pay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationApplePay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationApplePayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationApplePay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationApplePay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationApplePayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationApplePayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationApplePayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference"))
    }
}
/// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationApplePayLater {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationApplePayLaterDisplayPreference>,
}
impl CreatePaymentMethodConfigurationApplePayLater {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationApplePayLater {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationApplePayLaterDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationApplePayLaterDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationApplePayLaterDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference"))
    }
}
/// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
/// Check this [page](https://stripe.com/docs/payments/au-becs-debit) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAuBecsDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAuBecsDebitDisplayPreference>,
}
impl CreatePaymentMethodConfigurationAuBecsDebit {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAuBecsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAuBecsDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationAuBecsDebitDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAuBecsDebitDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference"))
    }
}
/// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://stripe.com/docs/payments/payment-methods/bacs-debit) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationBacsDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationBacsDebitDisplayPreference>,
}
impl CreatePaymentMethodConfigurationBacsDebit {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationBacsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationBacsDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationBacsDebitDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationBacsDebitDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference"))
    }
}
/// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
/// [Customers](https://stripe.com/docs/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
/// Check this [page](https://stripe.com/docs/payments/bancontact) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationBancontact {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationBancontactDisplayPreference>,
}
impl CreatePaymentMethodConfigurationBancontact {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationBancontact {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationBancontactDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationBancontactDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationBancontactDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference"))
    }
}
/// BLIK is a [single use](https://stripe.com/docs/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
/// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
/// Check this [page](https://stripe.com/docs/payments/blik) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationBlik {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationBlikDisplayPreference>,
}
impl CreatePaymentMethodConfigurationBlik {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationBlik {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationBlikDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationBlikDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationBlikDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationBlikDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationBlikDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationBlikDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentMethodConfigurationBlikDisplayPreferencePreference",
            )
        })
    }
}
/// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
/// Check this [page](https://stripe.com/docs/payments/boleto) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationBoleto {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationBoletoDisplayPreference>,
}
impl CreatePaymentMethodConfigurationBoleto {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationBoleto {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationBoletoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationBoletoDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationBoletoDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference"))
    }
}
/// Cards are a popular way for consumers and businesses to pay online or in person.
/// Stripe supports global and local card networks.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationCard {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationCardDisplayPreference>,
}
impl CreatePaymentMethodConfigurationCard {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationCardDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationCardDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationCardDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationCardDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationCardDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationCardDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationCardDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationCardDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationCardDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationCardDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationCardDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationCardDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationCardDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentMethodConfigurationCardDisplayPreferencePreference",
            )
        })
    }
}
/// Cartes Bancaires is France's local card network.
/// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
/// Check this [page](https://stripe.com/docs/payments/cartes-bancaires) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationCartesBancaires {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigurationCartesBancairesDisplayPreference>,
}
impl CreatePaymentMethodConfigurationCartesBancaires {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationCartesBancaires {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationCartesBancairesDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationCartesBancairesDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationCartesBancairesDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
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
    for CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference"))
    }
}
/// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
/// Check this [page](https://stripe.com/docs/payments/cash-app-pay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationCashapp {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationCashappDisplayPreference>,
}
impl CreatePaymentMethodConfigurationCashapp {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationCashapp {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationCashappDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationCashappDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationCashappDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationCashappDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationCashappDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationCashappDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationCashappDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationCashappDisplayPreferencePreference"))
    }
}
/// Uses a customer’s [cash balance](https://stripe.com/docs/payments/customer-balance) for the payment.
/// The cash balance can be funded via a bank transfer.
/// Check this [page](https://stripe.com/docs/payments/bank-transfers) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationCustomerBalance {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigurationCustomerBalanceDisplayPreference>,
}
impl CreatePaymentMethodConfigurationCustomerBalance {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationCustomerBalance {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationCustomerBalanceDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationCustomerBalanceDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationCustomerBalanceDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference
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
    for CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference"))
    }
}
/// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
/// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
/// Check this [page](https://stripe.com/docs/payments/eps) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationEps {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationEpsDisplayPreference>,
}
impl CreatePaymentMethodConfigurationEps {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationEps {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationEpsDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationEpsDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationEpsDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationEpsDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationEpsDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationEpsDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationEpsDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentMethodConfigurationEpsDisplayPreferencePreference",
            )
        })
    }
}
/// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
/// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
/// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
/// Check this [page](https://stripe.com/docs/payments/fpx) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationFpx {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationFpxDisplayPreference>,
}
impl CreatePaymentMethodConfigurationFpx {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationFpx {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationFpxDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationFpxDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationFpxDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationFpxDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationFpxDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationFpxDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationFpxDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentMethodConfigurationFpxDisplayPreferencePreference",
            )
        })
    }
}
/// giropay is a German payment method based on online banking, introduced in 2006.
/// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
/// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
/// giropay accounts for 10% of online checkouts in Germany.
/// Check this [page](https://stripe.com/docs/payments/giropay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationGiropay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationGiropayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationGiropay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationGiropay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationGiropayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationGiropayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationGiropayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference"))
    }
}
/// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
/// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
/// Check this [page](https://stripe.com/docs/google-pay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationGooglePay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationGooglePayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationGooglePay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationGooglePay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationGooglePayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationGooglePayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationGooglePayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference"))
    }
}
/// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
/// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
/// Check this [page](https://stripe.com/docs/payments/grabpay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationGrabpay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationGrabpayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationGrabpay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationGrabpay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationGrabpayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationGrabpayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationGrabpayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference"))
    }
}
/// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
/// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
/// Check this [page](https://stripe.com/docs/payments/ideal) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationIdeal {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationIdealDisplayPreference>,
}
impl CreatePaymentMethodConfigurationIdeal {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationIdeal {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationIdealDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationIdealDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationIdealDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationIdealDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationIdealDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationIdealDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationIdealDisplayPreferencePreference"))
    }
}
/// JCB is a credit card company based in Japan.
/// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
/// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationJcb {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationJcbDisplayPreference>,
}
impl CreatePaymentMethodConfigurationJcb {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationJcb {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationJcbDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationJcbDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationJcbDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationJcbDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationJcbDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationJcbDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationJcbDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentMethodConfigurationJcbDisplayPreferencePreference",
            )
        })
    }
}
/// Klarna gives customers a range of [payment options](https://stripe.com/docs/payments/klarna#payment-options) during checkout.
/// Available payment options vary depending on the customer's billing address and the transaction amount.
/// These payment options make it convenient for customers to purchase items in all price ranges.
/// Check this [page](https://stripe.com/docs/payments/klarna) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationKlarna {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationKlarnaDisplayPreference>,
}
impl CreatePaymentMethodConfigurationKlarna {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationKlarna {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationKlarnaDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationKlarnaDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationKlarnaDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference"))
    }
}
/// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
/// Check this [page](https://stripe.com/docs/payments/konbini) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationKonbini {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationKonbiniDisplayPreference>,
}
impl CreatePaymentMethodConfigurationKonbini {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationKonbini {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationKonbiniDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationKonbiniDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationKonbiniDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference"))
    }
}
/// [Link](https://stripe.com/docs/payments/link) is a payment method network.
/// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationLink {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationLinkDisplayPreference>,
}
impl CreatePaymentMethodConfigurationLink {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationLink {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationLinkDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationLinkDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationLinkDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationLinkDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationLinkDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationLinkDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentMethodConfigurationLinkDisplayPreferencePreference",
            )
        })
    }
}
/// MobilePay is a [single-use](https://stripe.com/docs/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
/// It allows customers to [authenticate and approve](https://stripe.com/docs/payments/payment-methods#customer-actions) payments using the MobilePay app.
/// Check this [page](https://stripe.com/docs/payments/mobilepay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationMobilepay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationMobilepayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationMobilepay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationMobilepay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationMobilepayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationMobilepayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationMobilepayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference"))
    }
}
/// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
/// OXXO allows customers to pay bills and online purchases in-store with cash.
/// Check this [page](https://stripe.com/docs/payments/oxxo) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationOxxo {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationOxxoDisplayPreference>,
}
impl CreatePaymentMethodConfigurationOxxo {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationOxxo {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationOxxoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationOxxoDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationOxxoDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference",
            )
        })
    }
}
/// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
/// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
/// Check this [page](https://stripe.com/docs/payments/p24) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationP24 {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationP24DisplayPreference>,
}
impl CreatePaymentMethodConfigurationP24 {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationP24 {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationP24DisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationP24DisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationP24DisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationP24DisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationP24DisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationP24DisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationP24DisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentMethodConfigurationP24DisplayPreferencePreference",
            )
        })
    }
}
/// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
/// Check this [page](https://stripe.com/docs/payments/paynow) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPaynow {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationPaynowDisplayPreference>,
}
impl CreatePaymentMethodConfigurationPaynow {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPaynow {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPaynowDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationPaynowDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPaynowDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference"))
    }
}
/// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
/// Check this [page](https://stripe.com/docs/payments/paypal) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPaypal {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationPaypalDisplayPreference>,
}
impl CreatePaymentMethodConfigurationPaypal {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPaypal {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPaypalDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationPaypalDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPaypalDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference"))
    }
}
/// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
/// Check this [page](https://stripe.com/docs/payments/promptpay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPromptpay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationPromptpayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationPromptpay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPromptpay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPromptpayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationPromptpayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPromptpayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference"))
    }
}
/// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
/// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationRevolutPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationRevolutPayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationRevolutPay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationRevolutPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationRevolutPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationRevolutPayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationRevolutPayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference"))
    }
}
/// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
/// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://stripe.com/docs/payments/sepa-debit) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationSepaDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationSepaDebitDisplayPreference>,
}
impl CreatePaymentMethodConfigurationSepaDebit {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationSepaDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationSepaDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationSepaDebitDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationSepaDebitDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference"))
    }
}
/// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
/// Check this [page](https://stripe.com/docs/payments/sofort) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationSofort {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationSofortDisplayPreference>,
}
impl CreatePaymentMethodConfigurationSofort {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationSofort {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationSofortDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationSofortDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationSofortDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationSofortDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationSofortDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationSofortDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationSofortDisplayPreferencePreference"))
    }
}
/// Swish is a [real-time](https://stripe.com/docs/payments/real-time) payment method popular in Sweden.
/// It allows customers to [authenticate and approve](https://stripe.com/docs/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
/// Check this [page](https://stripe.com/docs/payments/swish) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationSwish {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationSwishDisplayPreference>,
}
impl CreatePaymentMethodConfigurationSwish {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationSwish {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationSwishDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationSwishDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationSwishDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationSwishDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationSwishDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationSwishDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationSwishDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationSwishDisplayPreferencePreference"))
    }
}
/// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
/// Check this [page](https://stripe.com/docs/payments/ach-debit) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationUsBankAccount {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationUsBankAccountDisplayPreference>,
}
impl CreatePaymentMethodConfigurationUsBankAccount {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationUsBankAccountDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationUsBankAccountDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationUsBankAccountDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference"))
    }
}
/// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
/// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
/// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
/// Check this [page](https://stripe.com/docs/payments/wechat-pay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationWechatPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationWechatPayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationWechatPay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationWechatPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationWechatPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationWechatPayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationWechatPayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference"))
    }
}
/// Zip gives your customers a way to split purchases over a series of payments.
/// Check this [page](https://stripe.com/docs/payments/zip) for more details like country availability.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationZip {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationZipDisplayPreference>,
}
impl CreatePaymentMethodConfigurationZip {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationZip {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationZipDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationZipDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationZipDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationZipDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigurationZipDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationZipDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationZipDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentMethodConfigurationZipDisplayPreferencePreference",
            )
        })
    }
}
/// Creates a payment method configuration
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfiguration<'a> {
    inner: CreatePaymentMethodConfigurationBuilder<'a>,
}
impl<'a> CreatePaymentMethodConfiguration<'a> {
    /// Construct a new `CreatePaymentMethodConfiguration`.
    pub fn new() -> Self {
        Self { inner: CreatePaymentMethodConfigurationBuilder::new() }
    }
    /// Canadian pre-authorized debit payments, check this [page](https://stripe.com/docs/payments/acss-debit) for more details like country availability.
    pub fn acss_debit(mut self, acss_debit: CreatePaymentMethodConfigurationAcssDebit) -> Self {
        self.inner.acss_debit = Some(acss_debit);
        self
    }
    /// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
    /// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
    /// Check this [page](https://stripe.com/docs/payments/affirm) for more details like country availability.
    pub fn affirm(mut self, affirm: CreatePaymentMethodConfigurationAffirm) -> Self {
        self.inner.affirm = Some(affirm);
        self
    }
    /// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://stripe.com/docs/payments/afterpay-clearpay) for more details like country availability.
    /// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
    pub fn afterpay_clearpay(
        mut self,
        afterpay_clearpay: CreatePaymentMethodConfigurationAfterpayClearpay,
    ) -> Self {
        self.inner.afterpay_clearpay = Some(afterpay_clearpay);
        self
    }
    /// Alipay is a digital wallet in China that has more than a billion active users worldwide.
    /// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
    /// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
    /// Check this [page](https://stripe.com/docs/payments/alipay) for more details.
    pub fn alipay(mut self, alipay: CreatePaymentMethodConfigurationAlipay) -> Self {
        self.inner.alipay = Some(alipay);
        self
    }
    /// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
    pub fn amazon_pay(mut self, amazon_pay: CreatePaymentMethodConfigurationAmazonPay) -> Self {
        self.inner.amazon_pay = Some(amazon_pay);
        self
    }
    /// Stripe users can accept [Apple Pay](/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
    /// There are no additional fees to process Apple Pay payments, and the [pricing](/pricing) is the same as other card transactions.
    /// Check this [page](https://stripe.com/docs/apple-pay) for more details.
    pub fn apple_pay(mut self, apple_pay: CreatePaymentMethodConfigurationApplePay) -> Self {
        self.inner.apple_pay = Some(apple_pay);
        self
    }
    /// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
    pub fn apple_pay_later(
        mut self,
        apple_pay_later: CreatePaymentMethodConfigurationApplePayLater,
    ) -> Self {
        self.inner.apple_pay_later = Some(apple_pay_later);
        self
    }
    /// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
    /// Check this [page](https://stripe.com/docs/payments/au-becs-debit) for more details.
    pub fn au_becs_debit(
        mut self,
        au_becs_debit: CreatePaymentMethodConfigurationAuBecsDebit,
    ) -> Self {
        self.inner.au_becs_debit = Some(au_becs_debit);
        self
    }
    /// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://stripe.com/docs/payments/payment-methods/bacs-debit) for more details.
    pub fn bacs_debit(mut self, bacs_debit: CreatePaymentMethodConfigurationBacsDebit) -> Self {
        self.inner.bacs_debit = Some(bacs_debit);
        self
    }
    /// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
    /// [Customers](https://stripe.com/docs/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
    /// Check this [page](https://stripe.com/docs/payments/bancontact) for more details.
    pub fn bancontact(mut self, bancontact: CreatePaymentMethodConfigurationBancontact) -> Self {
        self.inner.bancontact = Some(bancontact);
        self
    }
    /// BLIK is a [single use](https://stripe.com/docs/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
    /// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
    /// Check this [page](https://stripe.com/docs/payments/blik) for more details.
    pub fn blik(mut self, blik: CreatePaymentMethodConfigurationBlik) -> Self {
        self.inner.blik = Some(blik);
        self
    }
    /// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
    /// Check this [page](https://stripe.com/docs/payments/boleto) for more details.
    pub fn boleto(mut self, boleto: CreatePaymentMethodConfigurationBoleto) -> Self {
        self.inner.boleto = Some(boleto);
        self
    }
    /// Cards are a popular way for consumers and businesses to pay online or in person.
    /// Stripe supports global and local card networks.
    pub fn card(mut self, card: CreatePaymentMethodConfigurationCard) -> Self {
        self.inner.card = Some(card);
        self
    }
    /// Cartes Bancaires is France's local card network.
    /// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
    /// Check this [page](https://stripe.com/docs/payments/cartes-bancaires) for more details.
    pub fn cartes_bancaires(
        mut self,
        cartes_bancaires: CreatePaymentMethodConfigurationCartesBancaires,
    ) -> Self {
        self.inner.cartes_bancaires = Some(cartes_bancaires);
        self
    }
    /// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
    /// Check this [page](https://stripe.com/docs/payments/cash-app-pay) for more details.
    pub fn cashapp(mut self, cashapp: CreatePaymentMethodConfigurationCashapp) -> Self {
        self.inner.cashapp = Some(cashapp);
        self
    }
    /// Uses a customer’s [cash balance](https://stripe.com/docs/payments/customer-balance) for the payment.
    /// The cash balance can be funded via a bank transfer.
    /// Check this [page](https://stripe.com/docs/payments/bank-transfers) for more details.
    pub fn customer_balance(
        mut self,
        customer_balance: CreatePaymentMethodConfigurationCustomerBalance,
    ) -> Self {
        self.inner.customer_balance = Some(customer_balance);
        self
    }
    /// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
    /// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
    /// Check this [page](https://stripe.com/docs/payments/eps) for more details.
    pub fn eps(mut self, eps: CreatePaymentMethodConfigurationEps) -> Self {
        self.inner.eps = Some(eps);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
    /// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
    /// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
    /// Check this [page](https://stripe.com/docs/payments/fpx) for more details.
    pub fn fpx(mut self, fpx: CreatePaymentMethodConfigurationFpx) -> Self {
        self.inner.fpx = Some(fpx);
        self
    }
    /// giropay is a German payment method based on online banking, introduced in 2006.
    /// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
    /// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
    /// giropay accounts for 10% of online checkouts in Germany.
    /// Check this [page](https://stripe.com/docs/payments/giropay) for more details.
    pub fn giropay(mut self, giropay: CreatePaymentMethodConfigurationGiropay) -> Self {
        self.inner.giropay = Some(giropay);
        self
    }
    /// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
    /// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
    /// Check this [page](https://stripe.com/docs/google-pay) for more details.
    pub fn google_pay(mut self, google_pay: CreatePaymentMethodConfigurationGooglePay) -> Self {
        self.inner.google_pay = Some(google_pay);
        self
    }
    /// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
    /// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
    /// Check this [page](https://stripe.com/docs/payments/grabpay) for more details.
    pub fn grabpay(mut self, grabpay: CreatePaymentMethodConfigurationGrabpay) -> Self {
        self.inner.grabpay = Some(grabpay);
        self
    }
    /// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
    /// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
    /// Check this [page](https://stripe.com/docs/payments/ideal) for more details.
    pub fn ideal(mut self, ideal: CreatePaymentMethodConfigurationIdeal) -> Self {
        self.inner.ideal = Some(ideal);
        self
    }
    /// JCB is a credit card company based in Japan.
    /// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
    /// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
    pub fn jcb(mut self, jcb: CreatePaymentMethodConfigurationJcb) -> Self {
        self.inner.jcb = Some(jcb);
        self
    }
    /// Klarna gives customers a range of [payment options](https://stripe.com/docs/payments/klarna#payment-options) during checkout.
    /// Available payment options vary depending on the customer's billing address and the transaction amount.
    /// These payment options make it convenient for customers to purchase items in all price ranges.
    /// Check this [page](https://stripe.com/docs/payments/klarna) for more details.
    pub fn klarna(mut self, klarna: CreatePaymentMethodConfigurationKlarna) -> Self {
        self.inner.klarna = Some(klarna);
        self
    }
    /// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
    /// Check this [page](https://stripe.com/docs/payments/konbini) for more details.
    pub fn konbini(mut self, konbini: CreatePaymentMethodConfigurationKonbini) -> Self {
        self.inner.konbini = Some(konbini);
        self
    }
    /// [Link](https://stripe.com/docs/payments/link) is a payment method network.
    /// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
    pub fn link(mut self, link: CreatePaymentMethodConfigurationLink) -> Self {
        self.inner.link = Some(link);
        self
    }
    /// MobilePay is a [single-use](https://stripe.com/docs/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
    /// It allows customers to [authenticate and approve](https://stripe.com/docs/payments/payment-methods#customer-actions) payments using the MobilePay app.
    /// Check this [page](https://stripe.com/docs/payments/mobilepay) for more details.
    pub fn mobilepay(mut self, mobilepay: CreatePaymentMethodConfigurationMobilepay) -> Self {
        self.inner.mobilepay = Some(mobilepay);
        self
    }
    /// Configuration name.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
    /// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
    /// OXXO allows customers to pay bills and online purchases in-store with cash.
    /// Check this [page](https://stripe.com/docs/payments/oxxo) for more details.
    pub fn oxxo(mut self, oxxo: CreatePaymentMethodConfigurationOxxo) -> Self {
        self.inner.oxxo = Some(oxxo);
        self
    }
    /// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
    /// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
    /// Check this [page](https://stripe.com/docs/payments/p24) for more details.
    pub fn p24(mut self, p24: CreatePaymentMethodConfigurationP24) -> Self {
        self.inner.p24 = Some(p24);
        self
    }
    /// Configuration's parent configuration. Specify to create a child configuration.
    pub fn parent(mut self, parent: &'a str) -> Self {
        self.inner.parent = Some(parent);
        self
    }
    /// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
    /// Check this [page](https://stripe.com/docs/payments/paynow) for more details.
    pub fn paynow(mut self, paynow: CreatePaymentMethodConfigurationPaynow) -> Self {
        self.inner.paynow = Some(paynow);
        self
    }
    /// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
    /// Check this [page](https://stripe.com/docs/payments/paypal) for more details.
    pub fn paypal(mut self, paypal: CreatePaymentMethodConfigurationPaypal) -> Self {
        self.inner.paypal = Some(paypal);
        self
    }
    /// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
    /// Check this [page](https://stripe.com/docs/payments/promptpay) for more details.
    pub fn promptpay(mut self, promptpay: CreatePaymentMethodConfigurationPromptpay) -> Self {
        self.inner.promptpay = Some(promptpay);
        self
    }
    /// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
    /// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
    pub fn revolut_pay(mut self, revolut_pay: CreatePaymentMethodConfigurationRevolutPay) -> Self {
        self.inner.revolut_pay = Some(revolut_pay);
        self
    }
    /// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
    /// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://stripe.com/docs/payments/sepa-debit) for more details.
    pub fn sepa_debit(mut self, sepa_debit: CreatePaymentMethodConfigurationSepaDebit) -> Self {
        self.inner.sepa_debit = Some(sepa_debit);
        self
    }
    /// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
    /// Check this [page](https://stripe.com/docs/payments/sofort) for more details.
    pub fn sofort(mut self, sofort: CreatePaymentMethodConfigurationSofort) -> Self {
        self.inner.sofort = Some(sofort);
        self
    }
    /// Swish is a [real-time](https://stripe.com/docs/payments/real-time) payment method popular in Sweden.
    /// It allows customers to [authenticate and approve](https://stripe.com/docs/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
    /// Check this [page](https://stripe.com/docs/payments/swish) for more details.
    pub fn swish(mut self, swish: CreatePaymentMethodConfigurationSwish) -> Self {
        self.inner.swish = Some(swish);
        self
    }
    /// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
    /// Check this [page](https://stripe.com/docs/payments/ach-debit) for more details.
    pub fn us_bank_account(
        mut self,
        us_bank_account: CreatePaymentMethodConfigurationUsBankAccount,
    ) -> Self {
        self.inner.us_bank_account = Some(us_bank_account);
        self
    }
    /// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
    /// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
    /// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
    /// Check this [page](https://stripe.com/docs/payments/wechat-pay) for more details.
    pub fn wechat_pay(mut self, wechat_pay: CreatePaymentMethodConfigurationWechatPay) -> Self {
        self.inner.wechat_pay = Some(wechat_pay);
        self
    }
    /// Zip gives your customers a way to split purchases over a series of payments.
    /// Check this [page](https://stripe.com/docs/payments/zip) for more details like country availability.
    pub fn zip(mut self, zip: CreatePaymentMethodConfigurationZip) -> Self {
        self.inner.zip = Some(zip);
        self
    }
}
impl<'a> Default for CreatePaymentMethodConfiguration<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl CreatePaymentMethodConfiguration<'_> {
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

impl StripeRequest for CreatePaymentMethodConfiguration<'_> {
    type Output = stripe_payment::PaymentMethodConfiguration;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/payment_method_configurations").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdatePaymentMethodConfigurationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    acss_debit: Option<UpdatePaymentMethodConfigurationAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    affirm: Option<UpdatePaymentMethodConfigurationAffirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    afterpay_clearpay: Option<UpdatePaymentMethodConfigurationAfterpayClearpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alipay: Option<UpdatePaymentMethodConfigurationAlipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_pay: Option<UpdatePaymentMethodConfigurationAmazonPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_pay: Option<UpdatePaymentMethodConfigurationApplePay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_pay_later: Option<UpdatePaymentMethodConfigurationApplePayLater>,
    #[serde(skip_serializing_if = "Option::is_none")]
    au_becs_debit: Option<UpdatePaymentMethodConfigurationAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bacs_debit: Option<UpdatePaymentMethodConfigurationBacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bancontact: Option<UpdatePaymentMethodConfigurationBancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blik: Option<UpdatePaymentMethodConfigurationBlik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boleto: Option<UpdatePaymentMethodConfigurationBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<UpdatePaymentMethodConfigurationCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cartes_bancaires: Option<UpdatePaymentMethodConfigurationCartesBancaires>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cashapp: Option<UpdatePaymentMethodConfigurationCashapp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_balance: Option<UpdatePaymentMethodConfigurationCustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eps: Option<UpdatePaymentMethodConfigurationEps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fpx: Option<UpdatePaymentMethodConfigurationFpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    giropay: Option<UpdatePaymentMethodConfigurationGiropay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_pay: Option<UpdatePaymentMethodConfigurationGooglePay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grabpay: Option<UpdatePaymentMethodConfigurationGrabpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ideal: Option<UpdatePaymentMethodConfigurationIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jcb: Option<UpdatePaymentMethodConfigurationJcb>,
    #[serde(skip_serializing_if = "Option::is_none")]
    klarna: Option<UpdatePaymentMethodConfigurationKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    konbini: Option<UpdatePaymentMethodConfigurationKonbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<UpdatePaymentMethodConfigurationLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobilepay: Option<UpdatePaymentMethodConfigurationMobilepay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oxxo: Option<UpdatePaymentMethodConfigurationOxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    p24: Option<UpdatePaymentMethodConfigurationP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paynow: Option<UpdatePaymentMethodConfigurationPaynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paypal: Option<UpdatePaymentMethodConfigurationPaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promptpay: Option<UpdatePaymentMethodConfigurationPromptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revolut_pay: Option<UpdatePaymentMethodConfigurationRevolutPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sepa_debit: Option<UpdatePaymentMethodConfigurationSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sofort: Option<UpdatePaymentMethodConfigurationSofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swish: Option<UpdatePaymentMethodConfigurationSwish>,
    #[serde(skip_serializing_if = "Option::is_none")]
    us_bank_account: Option<UpdatePaymentMethodConfigurationUsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wechat_pay: Option<UpdatePaymentMethodConfigurationWechatPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip: Option<UpdatePaymentMethodConfigurationZip>,
}
impl<'a> UpdatePaymentMethodConfigurationBuilder<'a> {
    fn new() -> Self {
        Self {
            acss_debit: None,
            active: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            amazon_pay: None,
            apple_pay: None,
            apple_pay_later: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            blik: None,
            boleto: None,
            card: None,
            cartes_bancaires: None,
            cashapp: None,
            customer_balance: None,
            eps: None,
            expand: None,
            fpx: None,
            giropay: None,
            google_pay: None,
            grabpay: None,
            ideal: None,
            jcb: None,
            klarna: None,
            konbini: None,
            link: None,
            mobilepay: None,
            name: None,
            oxxo: None,
            p24: None,
            paynow: None,
            paypal: None,
            promptpay: None,
            revolut_pay: None,
            sepa_debit: None,
            sofort: None,
            swish: None,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// Canadian pre-authorized debit payments, check this [page](https://stripe.com/docs/payments/acss-debit) for more details like country availability.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAcssDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAcssDebitDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationAcssDebit {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAcssDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAcssDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationAcssDebitDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAcssDebitDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference"))
    }
}
/// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
/// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
/// Check this [page](https://stripe.com/docs/payments/affirm) for more details like country availability.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAffirm {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAffirmDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationAffirm {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAffirm {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAffirmDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationAffirmDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAffirmDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference"))
    }
}
/// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://stripe.com/docs/payments/afterpay-clearpay) for more details like country availability.
/// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAfterpayClearpay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationAfterpayClearpay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAfterpayClearpay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
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
    for UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference"))
    }
}
/// Alipay is a digital wallet in China that has more than a billion active users worldwide.
/// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
/// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
/// Check this [page](https://stripe.com/docs/payments/alipay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAlipay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAlipayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationAlipay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAlipay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAlipayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationAlipayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAlipayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference"))
    }
}
/// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAmazonPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAmazonPayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationAmazonPay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAmazonPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAmazonPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationAmazonPayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAmazonPayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference"))
    }
}
/// Stripe users can accept [Apple Pay](/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
/// There are no additional fees to process Apple Pay payments, and the [pricing](/pricing) is the same as other card transactions.
/// Check this [page](https://stripe.com/docs/apple-pay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationApplePay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationApplePayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationApplePay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationApplePay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationApplePayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationApplePayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationApplePayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference"))
    }
}
/// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationApplePayLater {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationApplePayLaterDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationApplePayLater {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationApplePayLater {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationApplePayLaterDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationApplePayLaterDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationApplePayLaterDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference"))
    }
}
/// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
/// Check this [page](https://stripe.com/docs/payments/au-becs-debit) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAuBecsDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationAuBecsDebit {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAuBecsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference"))
    }
}
/// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://stripe.com/docs/payments/payment-methods/bacs-debit) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationBacsDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationBacsDebitDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationBacsDebit {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationBacsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationBacsDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationBacsDebitDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationBacsDebitDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference"))
    }
}
/// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
/// [Customers](https://stripe.com/docs/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
/// Check this [page](https://stripe.com/docs/payments/bancontact) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationBancontact {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationBancontactDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationBancontact {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationBancontact {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationBancontactDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationBancontactDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationBancontactDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference"))
    }
}
/// BLIK is a [single use](https://stripe.com/docs/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
/// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
/// Check this [page](https://stripe.com/docs/payments/blik) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationBlik {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationBlikDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationBlik {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationBlik {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationBlikDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationBlikDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationBlikDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference",
            )
        })
    }
}
/// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
/// Check this [page](https://stripe.com/docs/payments/boleto) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationBoleto {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationBoletoDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationBoleto {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationBoleto {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationBoletoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationBoletoDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationBoletoDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference"))
    }
}
/// Cards are a popular way for consumers and businesses to pay online or in person.
/// Stripe supports global and local card networks.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationCard {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationCardDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationCard {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationCardDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationCardDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationCardDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationCardDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationCardDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationCardDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationCardDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationCardDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationCardDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationCardDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationCardDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationCardDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationCardDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentMethodConfigurationCardDisplayPreferencePreference",
            )
        })
    }
}
/// Cartes Bancaires is France's local card network.
/// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
/// Check this [page](https://stripe.com/docs/payments/cartes-bancaires) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationCartesBancaires {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigurationCartesBancairesDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationCartesBancaires {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationCartesBancaires {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationCartesBancairesDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationCartesBancairesDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationCartesBancairesDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
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
    for UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference"))
    }
}
/// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
/// Check this [page](https://stripe.com/docs/payments/cash-app-pay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationCashapp {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationCashappDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationCashapp {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationCashapp {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationCashappDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationCashappDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationCashappDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference"))
    }
}
/// Uses a customer’s [cash balance](https://stripe.com/docs/payments/customer-balance) for the payment.
/// The cash balance can be funded via a bank transfer.
/// Check this [page](https://stripe.com/docs/payments/bank-transfers) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationCustomerBalance {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationCustomerBalance {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationCustomerBalance {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference
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
    for UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference"))
    }
}
/// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
/// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
/// Check this [page](https://stripe.com/docs/payments/eps) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationEps {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationEpsDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationEps {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationEps {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationEpsDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationEpsDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationEpsDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference",
            )
        })
    }
}
/// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
/// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
/// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
/// Check this [page](https://stripe.com/docs/payments/fpx) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationFpx {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationFpxDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationFpx {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationFpx {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationFpxDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationFpxDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationFpxDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference",
            )
        })
    }
}
/// giropay is a German payment method based on online banking, introduced in 2006.
/// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
/// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
/// giropay accounts for 10% of online checkouts in Germany.
/// Check this [page](https://stripe.com/docs/payments/giropay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationGiropay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationGiropayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationGiropay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationGiropay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationGiropayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationGiropayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationGiropayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference"))
    }
}
/// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
/// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
/// Check this [page](https://stripe.com/docs/google-pay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationGooglePay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationGooglePayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationGooglePay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationGooglePay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationGooglePayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationGooglePayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationGooglePayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference"))
    }
}
/// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
/// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
/// Check this [page](https://stripe.com/docs/payments/grabpay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationGrabpay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationGrabpayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationGrabpay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationGrabpay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationGrabpayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationGrabpayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationGrabpayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference"))
    }
}
/// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
/// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
/// Check this [page](https://stripe.com/docs/payments/ideal) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationIdeal {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationIdealDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationIdeal {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationIdeal {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationIdealDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationIdealDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationIdealDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference"))
    }
}
/// JCB is a credit card company based in Japan.
/// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
/// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationJcb {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationJcbDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationJcb {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationJcb {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationJcbDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationJcbDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationJcbDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference",
            )
        })
    }
}
/// Klarna gives customers a range of [payment options](https://stripe.com/docs/payments/klarna#payment-options) during checkout.
/// Available payment options vary depending on the customer's billing address and the transaction amount.
/// These payment options make it convenient for customers to purchase items in all price ranges.
/// Check this [page](https://stripe.com/docs/payments/klarna) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationKlarna {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationKlarnaDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationKlarna {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationKlarna {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationKlarnaDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationKlarnaDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationKlarnaDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference"))
    }
}
/// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
/// Check this [page](https://stripe.com/docs/payments/konbini) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationKonbini {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationKonbiniDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationKonbini {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationKonbini {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationKonbiniDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationKonbiniDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationKonbiniDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference"))
    }
}
/// [Link](https://stripe.com/docs/payments/link) is a payment method network.
/// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationLink {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationLinkDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationLink {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationLink {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationLinkDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationLinkDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationLinkDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference",
            )
        })
    }
}
/// MobilePay is a [single-use](https://stripe.com/docs/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
/// It allows customers to [authenticate and approve](https://stripe.com/docs/payments/payment-methods#customer-actions) payments using the MobilePay app.
/// Check this [page](https://stripe.com/docs/payments/mobilepay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationMobilepay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationMobilepayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationMobilepay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationMobilepay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationMobilepayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationMobilepayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationMobilepayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference"))
    }
}
/// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
/// OXXO allows customers to pay bills and online purchases in-store with cash.
/// Check this [page](https://stripe.com/docs/payments/oxxo) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationOxxo {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationOxxoDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationOxxo {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationOxxo {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationOxxoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationOxxoDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationOxxoDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference",
            )
        })
    }
}
/// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
/// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
/// Check this [page](https://stripe.com/docs/payments/p24) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationP24 {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationP24DisplayPreference>,
}
impl UpdatePaymentMethodConfigurationP24 {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationP24 {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationP24DisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationP24DisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationP24DisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationP24DisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationP24DisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationP24DisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationP24DisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentMethodConfigurationP24DisplayPreferencePreference",
            )
        })
    }
}
/// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
/// Check this [page](https://stripe.com/docs/payments/paynow) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPaynow {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationPaynowDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationPaynow {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPaynow {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPaynowDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationPaynowDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPaynowDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference"))
    }
}
/// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
/// Check this [page](https://stripe.com/docs/payments/paypal) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPaypal {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationPaypalDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationPaypal {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPaypal {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPaypalDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationPaypalDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPaypalDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference"))
    }
}
/// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
/// Check this [page](https://stripe.com/docs/payments/promptpay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPromptpay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationPromptpayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationPromptpay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPromptpay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPromptpayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationPromptpayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPromptpayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference"))
    }
}
/// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
/// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationRevolutPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationRevolutPayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationRevolutPay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationRevolutPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationRevolutPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationRevolutPayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationRevolutPayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference"))
    }
}
/// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
/// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://stripe.com/docs/payments/sepa-debit) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationSepaDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationSepaDebitDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationSepaDebit {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationSepaDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationSepaDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationSepaDebitDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationSepaDebitDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference"))
    }
}
/// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
/// Check this [page](https://stripe.com/docs/payments/sofort) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationSofort {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationSofortDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationSofort {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationSofort {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationSofortDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationSofortDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationSofortDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference"))
    }
}
/// Swish is a [real-time](https://stripe.com/docs/payments/real-time) payment method popular in Sweden.
/// It allows customers to [authenticate and approve](https://stripe.com/docs/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
/// Check this [page](https://stripe.com/docs/payments/swish) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationSwish {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationSwishDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationSwish {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationSwish {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationSwishDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationSwishDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationSwishDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference"))
    }
}
/// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
/// Check this [page](https://stripe.com/docs/payments/ach-debit) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationUsBankAccount {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationUsBankAccountDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationUsBankAccount {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationUsBankAccountDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationUsBankAccountDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationUsBankAccountDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference"))
    }
}
/// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
/// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
/// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
/// Check this [page](https://stripe.com/docs/payments/wechat-pay) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationWechatPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationWechatPayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationWechatPay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationWechatPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationWechatPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationWechatPayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationWechatPayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference"))
    }
}
/// Zip gives your customers a way to split purchases over a series of payments.
/// Check this [page](https://stripe.com/docs/payments/zip) for more details like country availability.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationZip {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationZipDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationZip {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationZip {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationZipDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationZipDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationZipDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationZipDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigurationZipDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationZipDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationZipDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentMethodConfigurationZipDisplayPreferencePreference",
            )
        })
    }
}
/// Update payment method configuration
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfiguration<'a> {
    inner: UpdatePaymentMethodConfigurationBuilder<'a>,
    configuration: &'a stripe_payment::PaymentMethodConfigurationId,
}
impl<'a> UpdatePaymentMethodConfiguration<'a> {
    /// Construct a new `UpdatePaymentMethodConfiguration`.
    pub fn new(configuration: &'a stripe_payment::PaymentMethodConfigurationId) -> Self {
        Self { configuration, inner: UpdatePaymentMethodConfigurationBuilder::new() }
    }
    /// Canadian pre-authorized debit payments, check this [page](https://stripe.com/docs/payments/acss-debit) for more details like country availability.
    pub fn acss_debit(mut self, acss_debit: UpdatePaymentMethodConfigurationAcssDebit) -> Self {
        self.inner.acss_debit = Some(acss_debit);
        self
    }
    /// Whether the configuration can be used for new payments.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
    /// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
    /// Check this [page](https://stripe.com/docs/payments/affirm) for more details like country availability.
    pub fn affirm(mut self, affirm: UpdatePaymentMethodConfigurationAffirm) -> Self {
        self.inner.affirm = Some(affirm);
        self
    }
    /// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://stripe.com/docs/payments/afterpay-clearpay) for more details like country availability.
    /// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
    pub fn afterpay_clearpay(
        mut self,
        afterpay_clearpay: UpdatePaymentMethodConfigurationAfterpayClearpay,
    ) -> Self {
        self.inner.afterpay_clearpay = Some(afterpay_clearpay);
        self
    }
    /// Alipay is a digital wallet in China that has more than a billion active users worldwide.
    /// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
    /// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
    /// Check this [page](https://stripe.com/docs/payments/alipay) for more details.
    pub fn alipay(mut self, alipay: UpdatePaymentMethodConfigurationAlipay) -> Self {
        self.inner.alipay = Some(alipay);
        self
    }
    /// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
    pub fn amazon_pay(mut self, amazon_pay: UpdatePaymentMethodConfigurationAmazonPay) -> Self {
        self.inner.amazon_pay = Some(amazon_pay);
        self
    }
    /// Stripe users can accept [Apple Pay](/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
    /// There are no additional fees to process Apple Pay payments, and the [pricing](/pricing) is the same as other card transactions.
    /// Check this [page](https://stripe.com/docs/apple-pay) for more details.
    pub fn apple_pay(mut self, apple_pay: UpdatePaymentMethodConfigurationApplePay) -> Self {
        self.inner.apple_pay = Some(apple_pay);
        self
    }
    /// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
    pub fn apple_pay_later(
        mut self,
        apple_pay_later: UpdatePaymentMethodConfigurationApplePayLater,
    ) -> Self {
        self.inner.apple_pay_later = Some(apple_pay_later);
        self
    }
    /// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
    /// Check this [page](https://stripe.com/docs/payments/au-becs-debit) for more details.
    pub fn au_becs_debit(
        mut self,
        au_becs_debit: UpdatePaymentMethodConfigurationAuBecsDebit,
    ) -> Self {
        self.inner.au_becs_debit = Some(au_becs_debit);
        self
    }
    /// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://stripe.com/docs/payments/payment-methods/bacs-debit) for more details.
    pub fn bacs_debit(mut self, bacs_debit: UpdatePaymentMethodConfigurationBacsDebit) -> Self {
        self.inner.bacs_debit = Some(bacs_debit);
        self
    }
    /// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
    /// [Customers](https://stripe.com/docs/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
    /// Check this [page](https://stripe.com/docs/payments/bancontact) for more details.
    pub fn bancontact(mut self, bancontact: UpdatePaymentMethodConfigurationBancontact) -> Self {
        self.inner.bancontact = Some(bancontact);
        self
    }
    /// BLIK is a [single use](https://stripe.com/docs/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
    /// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
    /// Check this [page](https://stripe.com/docs/payments/blik) for more details.
    pub fn blik(mut self, blik: UpdatePaymentMethodConfigurationBlik) -> Self {
        self.inner.blik = Some(blik);
        self
    }
    /// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
    /// Check this [page](https://stripe.com/docs/payments/boleto) for more details.
    pub fn boleto(mut self, boleto: UpdatePaymentMethodConfigurationBoleto) -> Self {
        self.inner.boleto = Some(boleto);
        self
    }
    /// Cards are a popular way for consumers and businesses to pay online or in person.
    /// Stripe supports global and local card networks.
    pub fn card(mut self, card: UpdatePaymentMethodConfigurationCard) -> Self {
        self.inner.card = Some(card);
        self
    }
    /// Cartes Bancaires is France's local card network.
    /// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
    /// Check this [page](https://stripe.com/docs/payments/cartes-bancaires) for more details.
    pub fn cartes_bancaires(
        mut self,
        cartes_bancaires: UpdatePaymentMethodConfigurationCartesBancaires,
    ) -> Self {
        self.inner.cartes_bancaires = Some(cartes_bancaires);
        self
    }
    /// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
    /// Check this [page](https://stripe.com/docs/payments/cash-app-pay) for more details.
    pub fn cashapp(mut self, cashapp: UpdatePaymentMethodConfigurationCashapp) -> Self {
        self.inner.cashapp = Some(cashapp);
        self
    }
    /// Uses a customer’s [cash balance](https://stripe.com/docs/payments/customer-balance) for the payment.
    /// The cash balance can be funded via a bank transfer.
    /// Check this [page](https://stripe.com/docs/payments/bank-transfers) for more details.
    pub fn customer_balance(
        mut self,
        customer_balance: UpdatePaymentMethodConfigurationCustomerBalance,
    ) -> Self {
        self.inner.customer_balance = Some(customer_balance);
        self
    }
    /// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
    /// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
    /// Check this [page](https://stripe.com/docs/payments/eps) for more details.
    pub fn eps(mut self, eps: UpdatePaymentMethodConfigurationEps) -> Self {
        self.inner.eps = Some(eps);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
    /// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
    /// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
    /// Check this [page](https://stripe.com/docs/payments/fpx) for more details.
    pub fn fpx(mut self, fpx: UpdatePaymentMethodConfigurationFpx) -> Self {
        self.inner.fpx = Some(fpx);
        self
    }
    /// giropay is a German payment method based on online banking, introduced in 2006.
    /// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
    /// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
    /// giropay accounts for 10% of online checkouts in Germany.
    /// Check this [page](https://stripe.com/docs/payments/giropay) for more details.
    pub fn giropay(mut self, giropay: UpdatePaymentMethodConfigurationGiropay) -> Self {
        self.inner.giropay = Some(giropay);
        self
    }
    /// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
    /// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
    /// Check this [page](https://stripe.com/docs/google-pay) for more details.
    pub fn google_pay(mut self, google_pay: UpdatePaymentMethodConfigurationGooglePay) -> Self {
        self.inner.google_pay = Some(google_pay);
        self
    }
    /// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
    /// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
    /// Check this [page](https://stripe.com/docs/payments/grabpay) for more details.
    pub fn grabpay(mut self, grabpay: UpdatePaymentMethodConfigurationGrabpay) -> Self {
        self.inner.grabpay = Some(grabpay);
        self
    }
    /// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
    /// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
    /// Check this [page](https://stripe.com/docs/payments/ideal) for more details.
    pub fn ideal(mut self, ideal: UpdatePaymentMethodConfigurationIdeal) -> Self {
        self.inner.ideal = Some(ideal);
        self
    }
    /// JCB is a credit card company based in Japan.
    /// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
    /// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
    pub fn jcb(mut self, jcb: UpdatePaymentMethodConfigurationJcb) -> Self {
        self.inner.jcb = Some(jcb);
        self
    }
    /// Klarna gives customers a range of [payment options](https://stripe.com/docs/payments/klarna#payment-options) during checkout.
    /// Available payment options vary depending on the customer's billing address and the transaction amount.
    /// These payment options make it convenient for customers to purchase items in all price ranges.
    /// Check this [page](https://stripe.com/docs/payments/klarna) for more details.
    pub fn klarna(mut self, klarna: UpdatePaymentMethodConfigurationKlarna) -> Self {
        self.inner.klarna = Some(klarna);
        self
    }
    /// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
    /// Check this [page](https://stripe.com/docs/payments/konbini) for more details.
    pub fn konbini(mut self, konbini: UpdatePaymentMethodConfigurationKonbini) -> Self {
        self.inner.konbini = Some(konbini);
        self
    }
    /// [Link](https://stripe.com/docs/payments/link) is a payment method network.
    /// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
    pub fn link(mut self, link: UpdatePaymentMethodConfigurationLink) -> Self {
        self.inner.link = Some(link);
        self
    }
    /// MobilePay is a [single-use](https://stripe.com/docs/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
    /// It allows customers to [authenticate and approve](https://stripe.com/docs/payments/payment-methods#customer-actions) payments using the MobilePay app.
    /// Check this [page](https://stripe.com/docs/payments/mobilepay) for more details.
    pub fn mobilepay(mut self, mobilepay: UpdatePaymentMethodConfigurationMobilepay) -> Self {
        self.inner.mobilepay = Some(mobilepay);
        self
    }
    /// Configuration name.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
    /// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
    /// OXXO allows customers to pay bills and online purchases in-store with cash.
    /// Check this [page](https://stripe.com/docs/payments/oxxo) for more details.
    pub fn oxxo(mut self, oxxo: UpdatePaymentMethodConfigurationOxxo) -> Self {
        self.inner.oxxo = Some(oxxo);
        self
    }
    /// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
    /// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
    /// Check this [page](https://stripe.com/docs/payments/p24) for more details.
    pub fn p24(mut self, p24: UpdatePaymentMethodConfigurationP24) -> Self {
        self.inner.p24 = Some(p24);
        self
    }
    /// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
    /// Check this [page](https://stripe.com/docs/payments/paynow) for more details.
    pub fn paynow(mut self, paynow: UpdatePaymentMethodConfigurationPaynow) -> Self {
        self.inner.paynow = Some(paynow);
        self
    }
    /// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
    /// Check this [page](https://stripe.com/docs/payments/paypal) for more details.
    pub fn paypal(mut self, paypal: UpdatePaymentMethodConfigurationPaypal) -> Self {
        self.inner.paypal = Some(paypal);
        self
    }
    /// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
    /// Check this [page](https://stripe.com/docs/payments/promptpay) for more details.
    pub fn promptpay(mut self, promptpay: UpdatePaymentMethodConfigurationPromptpay) -> Self {
        self.inner.promptpay = Some(promptpay);
        self
    }
    /// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
    /// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
    pub fn revolut_pay(mut self, revolut_pay: UpdatePaymentMethodConfigurationRevolutPay) -> Self {
        self.inner.revolut_pay = Some(revolut_pay);
        self
    }
    /// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
    /// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://stripe.com/docs/payments/sepa-debit) for more details.
    pub fn sepa_debit(mut self, sepa_debit: UpdatePaymentMethodConfigurationSepaDebit) -> Self {
        self.inner.sepa_debit = Some(sepa_debit);
        self
    }
    /// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
    /// Check this [page](https://stripe.com/docs/payments/sofort) for more details.
    pub fn sofort(mut self, sofort: UpdatePaymentMethodConfigurationSofort) -> Self {
        self.inner.sofort = Some(sofort);
        self
    }
    /// Swish is a [real-time](https://stripe.com/docs/payments/real-time) payment method popular in Sweden.
    /// It allows customers to [authenticate and approve](https://stripe.com/docs/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
    /// Check this [page](https://stripe.com/docs/payments/swish) for more details.
    pub fn swish(mut self, swish: UpdatePaymentMethodConfigurationSwish) -> Self {
        self.inner.swish = Some(swish);
        self
    }
    /// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
    /// Check this [page](https://stripe.com/docs/payments/ach-debit) for more details.
    pub fn us_bank_account(
        mut self,
        us_bank_account: UpdatePaymentMethodConfigurationUsBankAccount,
    ) -> Self {
        self.inner.us_bank_account = Some(us_bank_account);
        self
    }
    /// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
    /// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
    /// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
    /// Check this [page](https://stripe.com/docs/payments/wechat-pay) for more details.
    pub fn wechat_pay(mut self, wechat_pay: UpdatePaymentMethodConfigurationWechatPay) -> Self {
        self.inner.wechat_pay = Some(wechat_pay);
        self
    }
    /// Zip gives your customers a way to split purchases over a series of payments.
    /// Check this [page](https://stripe.com/docs/payments/zip) for more details like country availability.
    pub fn zip(mut self, zip: UpdatePaymentMethodConfigurationZip) -> Self {
        self.inner.zip = Some(zip);
        self
    }
}
impl UpdatePaymentMethodConfiguration<'_> {
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

impl StripeRequest for UpdatePaymentMethodConfiguration<'_> {
    type Output = stripe_payment::PaymentMethodConfiguration;

    fn build(&self) -> RequestBuilder {
        let configuration = self.configuration;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/payment_method_configurations/{configuration}"),
        )
        .form(&self.inner)
    }
}
