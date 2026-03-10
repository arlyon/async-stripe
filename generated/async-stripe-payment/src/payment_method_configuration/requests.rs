use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListPaymentMethodConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    application: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListPaymentMethodConfigurationBuilder {
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
pub struct ListPaymentMethodConfiguration {
    inner: ListPaymentMethodConfigurationBuilder,
}
impl ListPaymentMethodConfiguration {
    /// Construct a new `ListPaymentMethodConfiguration`.
    pub fn new() -> Self {
        Self { inner: ListPaymentMethodConfigurationBuilder::new() }
    }
    /// The Connect application to filter by.
    pub fn application(mut self, application: impl Into<String>) -> Self {
        self.inner.application = Some(application.into());
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListPaymentMethodConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPaymentMethodConfiguration {
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
        stripe_client_core::ListPaginator::new_list("/payment_method_configurations", &self.inner)
    }
}

impl StripeRequest for ListPaymentMethodConfiguration {
    type Output = stripe_types::List<stripe_payment::PaymentMethodConfiguration>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/payment_method_configurations").query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrievePaymentMethodConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrievePaymentMethodConfigurationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve payment method configuration
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePaymentMethodConfiguration {
    inner: RetrievePaymentMethodConfigurationBuilder,
    configuration: stripe_payment::PaymentMethodConfigurationId,
}
impl RetrievePaymentMethodConfiguration {
    /// Construct a new `RetrievePaymentMethodConfiguration`.
    pub fn new(configuration: impl Into<stripe_payment::PaymentMethodConfigurationId>) -> Self {
        Self {
            configuration: configuration.into(),
            inner: RetrievePaymentMethodConfigurationBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrievePaymentMethodConfiguration {
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

impl StripeRequest for RetrievePaymentMethodConfiguration {
    type Output = stripe_payment::PaymentMethodConfiguration;

    fn build(&self) -> RequestBuilder {
        let configuration = &self.configuration;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/payment_method_configurations/{configuration}"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct CreatePaymentMethodConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    acss_debit: Option<CreatePaymentMethodConfigurationAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    affirm: Option<CreatePaymentMethodConfigurationAffirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    afterpay_clearpay: Option<CreatePaymentMethodConfigurationAfterpayClearpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alipay: Option<CreatePaymentMethodConfigurationAlipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alma: Option<CreatePaymentMethodConfigurationAlma>,
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
    billie: Option<CreatePaymentMethodConfigurationBillie>,
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
    crypto: Option<CreatePaymentMethodConfigurationCrypto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_balance: Option<CreatePaymentMethodConfigurationCustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eps: Option<CreatePaymentMethodConfigurationEps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fpx: Option<CreatePaymentMethodConfigurationFpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fr_meal_voucher_conecs: Option<CreatePaymentMethodConfigurationFrMealVoucherConecs>,
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
    kakao_pay: Option<CreatePaymentMethodConfigurationKakaoPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    klarna: Option<CreatePaymentMethodConfigurationKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    konbini: Option<CreatePaymentMethodConfigurationKonbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kr_card: Option<CreatePaymentMethodConfigurationKrCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<CreatePaymentMethodConfigurationLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mb_way: Option<CreatePaymentMethodConfigurationMbWay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobilepay: Option<CreatePaymentMethodConfigurationMobilepay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multibanco: Option<CreatePaymentMethodConfigurationMultibanco>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    naver_pay: Option<CreatePaymentMethodConfigurationNaverPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nz_bank_account: Option<CreatePaymentMethodConfigurationNzBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oxxo: Option<CreatePaymentMethodConfigurationOxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    p24: Option<CreatePaymentMethodConfigurationP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay_by_bank: Option<CreatePaymentMethodConfigurationPayByBank>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payco: Option<CreatePaymentMethodConfigurationPayco>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paynow: Option<CreatePaymentMethodConfigurationPaynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paypal: Option<CreatePaymentMethodConfigurationPaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payto: Option<CreatePaymentMethodConfigurationPayto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pix: Option<CreatePaymentMethodConfigurationPix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promptpay: Option<CreatePaymentMethodConfigurationPromptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revolut_pay: Option<CreatePaymentMethodConfigurationRevolutPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    samsung_pay: Option<CreatePaymentMethodConfigurationSamsungPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    satispay: Option<CreatePaymentMethodConfigurationSatispay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sepa_debit: Option<CreatePaymentMethodConfigurationSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sofort: Option<CreatePaymentMethodConfigurationSofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swish: Option<CreatePaymentMethodConfigurationSwish>,
    #[serde(skip_serializing_if = "Option::is_none")]
    twint: Option<CreatePaymentMethodConfigurationTwint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    us_bank_account: Option<CreatePaymentMethodConfigurationUsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wechat_pay: Option<CreatePaymentMethodConfigurationWechatPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip: Option<CreatePaymentMethodConfigurationZip>,
}
impl CreatePaymentMethodConfigurationBuilder {
    fn new() -> Self {
        Self {
            acss_debit: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            alma: None,
            amazon_pay: None,
            apple_pay: None,
            apple_pay_later: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            billie: None,
            blik: None,
            boleto: None,
            card: None,
            cartes_bancaires: None,
            cashapp: None,
            crypto: None,
            customer_balance: None,
            eps: None,
            expand: None,
            fpx: None,
            fr_meal_voucher_conecs: None,
            giropay: None,
            google_pay: None,
            grabpay: None,
            ideal: None,
            jcb: None,
            kakao_pay: None,
            klarna: None,
            konbini: None,
            kr_card: None,
            link: None,
            mb_way: None,
            mobilepay: None,
            multibanco: None,
            name: None,
            naver_pay: None,
            nz_bank_account: None,
            oxxo: None,
            p24: None,
            parent: None,
            pay_by_bank: None,
            payco: None,
            paynow: None,
            paypal: None,
            payto: None,
            pix: None,
            promptpay: None,
            revolut_pay: None,
            samsung_pay: None,
            satispay: None,
            sepa_debit: None,
            sofort: None,
            swish: None,
            twint: None,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// Canadian pre-authorized debit payments, check this [page](https://docs.stripe.com/payments/acss-debit) for more details like country availability.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
/// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
/// Check this [page](https://docs.stripe.com/payments/affirm) for more details like country availability.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://docs.stripe.com/payments/afterpay-clearpay) for more details like country availability.
/// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Alipay is a digital wallet in China that has more than a billion active users worldwide.
/// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
/// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
/// Check this [page](https://docs.stripe.com/payments/alipay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Alma is a Buy Now, Pay Later payment method that offers customers the ability to pay in 2, 3, or 4 installments.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAlma {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAlmaDisplayPreference>,
}
impl CreatePaymentMethodConfigurationAlma {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAlma {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationAlmaDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationAlmaDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationAlmaDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users can accept [Apple Pay](https://stripe.com/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
/// There are no additional fees to process Apple Pay payments, and the [pricing](https://stripe.com/pricing) is the same as other card transactions.
/// Check this [page](https://docs.stripe.com/apple-pay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
/// Check this [page](https://docs.stripe.com/payments/au-becs-debit) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://docs.stripe.com/payments/payment-methods/bacs-debit) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
/// [Customers](https://docs.stripe.com/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
/// Check this [page](https://docs.stripe.com/payments/bancontact) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Billie is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers businesses Pay by Invoice where they offer payment terms ranging from 7-120 days.
/// Customers are redirected from your website or app, authorize the payment with Billie, then return to your website or app.
/// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationBillie {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationBillieDisplayPreference>,
}
impl CreatePaymentMethodConfigurationBillie {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationBillie {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationBillieDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationBillieDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationBillieDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationBillieDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationBillieDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationBillieDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationBillieDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationBillieDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// BLIK is a [single use](https://docs.stripe.com/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
/// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
/// Check this [page](https://docs.stripe.com/payments/blik) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationBlikDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationBlikDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationBlikDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
/// Check this [page](https://docs.stripe.com/payments/boleto) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Cards are a popular way for consumers and businesses to pay online or in person.
/// Stripe supports global and local card networks.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationCardDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationCardDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationCardDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationCardDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationCardDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationCardDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Cartes Bancaires is France's local card network.
/// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
/// Check this [page](https://docs.stripe.com/payments/cartes-bancaires) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
/// Check this [page](https://docs.stripe.com/payments/cash-app-pay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationCashappDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationCashappDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationCashappDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// [Stablecoin payments](https://docs.stripe.com/payments/stablecoin-payments) enable customers to pay in stablecoins like USDC from 100s of wallets including Phantom and Metamask.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationCrypto {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationCryptoDisplayPreference>,
}
impl CreatePaymentMethodConfigurationCrypto {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationCrypto {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationCryptoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationCryptoDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationCryptoDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationCryptoDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationCryptoDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationCryptoDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationCryptoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationCryptoDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationCryptoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationCryptoDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationCryptoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationCryptoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationCryptoDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationCryptoDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Uses a customer’s [cash balance](https://docs.stripe.com/payments/customer-balance) for the payment.
/// The cash balance can be funded via a bank transfer.
/// Check this [page](https://docs.stripe.com/payments/bank-transfers) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
/// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
/// Check this [page](https://docs.stripe.com/payments/eps) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationEpsDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationEpsDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationEpsDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
/// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
/// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
/// Check this [page](https://docs.stripe.com/payments/fpx) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationFpxDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationFpxDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationFpxDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Meal vouchers in France, or “titres-restaurant”, is a local benefits program commonly offered by employers for their employees to purchase prepared food and beverages on working days.
/// Check this [page](https://docs.stripe.com/payments/meal-vouchers/fr-meal-vouchers) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationFrMealVoucherConecs {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreference>,
}
impl CreatePaymentMethodConfigurationFrMealVoucherConecs {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationFrMealVoucherConecs {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference
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
    for CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// giropay is a German payment method based on online banking, introduced in 2006.
/// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
/// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
/// giropay accounts for 10% of online checkouts in Germany.
/// Check this [page](https://docs.stripe.com/payments/giropay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
/// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
/// Check this [page](https://docs.stripe.com/google-pay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
/// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
/// Check this [page](https://docs.stripe.com/payments/grabpay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
/// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
/// Check this [page](https://docs.stripe.com/payments/ideal) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationIdealDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationIdealDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationIdealDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// JCB is a credit card company based in Japan.
/// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
/// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationJcbDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationJcbDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationJcbDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Kakao Pay is a popular local wallet available in South Korea.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationKakaoPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationKakaoPayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationKakaoPay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationKakaoPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationKakaoPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationKakaoPayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationKakaoPayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Klarna gives customers a range of [payment options](https://docs.stripe.com/payments/klarna#payment-options) during checkout.
/// Available payment options vary depending on the customer's billing address and the transaction amount.
/// These payment options make it convenient for customers to purchase items in all price ranges.
/// Check this [page](https://docs.stripe.com/payments/klarna) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
/// Check this [page](https://docs.stripe.com/payments/konbini) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Korean cards let users pay using locally issued cards from South Korea.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationKrCard {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationKrCardDisplayPreference>,
}
impl CreatePaymentMethodConfigurationKrCard {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationKrCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationKrCardDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationKrCardDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationKrCardDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationKrCardDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationKrCardDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationKrCardDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationKrCardDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationKrCardDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationKrCardDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationKrCardDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationKrCardDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationKrCardDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationKrCardDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationKrCardDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// [Link](https://docs.stripe.com/payments/link) is a payment method network.
/// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationLinkDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationLinkDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationLinkDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// MB WAY is the most popular wallet in Portugal.
/// After entering their phone number in your checkout, customers approve the payment directly in their MB WAY app.
/// Check this [page](https://docs.stripe.com/payments/mb-way) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationMbWay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationMbWayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationMbWay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationMbWay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationMbWayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationMbWayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationMbWayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationMbWayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationMbWayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationMbWayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationMbWayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationMbWayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationMbWayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationMbWayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationMbWayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationMbWayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationMbWayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationMbWayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// MobilePay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
/// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the MobilePay app.
/// Check this [page](https://docs.stripe.com/payments/mobilepay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users in Europe and the United States can accept Multibanco payments from customers in Portugal using [Sources](https://stripe.com/docs/sources)—a single integration path for creating payments using any supported method.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationMultibanco {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationMultibancoDisplayPreference>,
}
impl CreatePaymentMethodConfigurationMultibanco {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationMultibanco {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationMultibancoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationMultibancoDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationMultibancoDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Naver Pay is a popular local wallet available in South Korea.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationNaverPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationNaverPayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationNaverPay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationNaverPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationNaverPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationNaverPayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationNaverPayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationNaverPayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationNaverPayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationNaverPayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationNaverPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationNaverPayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationNaverPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationNaverPayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationNaverPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationNaverPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationNaverPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationNaverPayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users in New Zealand can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with a New Zeland bank account.
/// Check this [page](https://docs.stripe.com/payments/nz-bank-account) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationNzBankAccount {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationNzBankAccountDisplayPreference>,
}
impl CreatePaymentMethodConfigurationNzBankAccount {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationNzBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationNzBankAccountDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationNzBankAccountDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationNzBankAccountDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
/// OXXO allows customers to pay bills and online purchases in-store with cash.
/// Check this [page](https://docs.stripe.com/payments/oxxo) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
/// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
/// Check this [page](https://docs.stripe.com/payments/p24) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationP24DisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationP24DisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationP24DisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Pay by bank is a redirect payment method backed by bank transfers.
/// A customer is redirected to their bank to authorize a bank transfer for a given amount.
/// This removes a lot of the error risks inherent in waiting for the customer to initiate a transfer themselves, and is less expensive than card payments.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPayByBank {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationPayByBankDisplayPreference>,
}
impl CreatePaymentMethodConfigurationPayByBank {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPayByBank {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPayByBankDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationPayByBankDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPayByBankDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// PAYCO is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPayco {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationPaycoDisplayPreference>,
}
impl CreatePaymentMethodConfigurationPayco {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPayco {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPaycoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationPaycoDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationPaycoDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPaycoDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationPaycoDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationPaycoDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationPaycoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationPaycoDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationPaycoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationPaycoDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationPaycoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationPaycoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationPaycoDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationPaycoDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
/// Check this [page](https://docs.stripe.com/payments/paynow) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
/// Check this [page](https://docs.stripe.com/payments/paypal) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// PayTo is a [real-time](https://docs.stripe.com/payments/real-time) payment method that enables customers in Australia to pay by providing their bank account details.
/// Customers must accept a mandate authorizing you to debit their account.
/// Check this [page](https://docs.stripe.com/payments/payto) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPayto {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationPaytoDisplayPreference>,
}
impl CreatePaymentMethodConfigurationPayto {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPayto {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPaytoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationPaytoDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationPaytoDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPaytoDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationPaytoDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationPaytoDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationPaytoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationPaytoDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationPaytoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationPaytoDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationPaytoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationPaytoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationPaytoDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationPaytoDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Pix is a payment method popular in Brazil.
/// When paying with Pix, customers authenticate and approve payments by scanning a QR code in their preferred banking app.
/// Check this [page](https://docs.stripe.com/payments/pix) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPix {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationPixDisplayPreference>,
}
impl CreatePaymentMethodConfigurationPix {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPix {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationPixDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationPixDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationPixDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationPixDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationPixDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationPixDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationPixDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationPixDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationPixDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationPixDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationPixDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationPixDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationPixDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationPixDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
/// Check this [page](https://docs.stripe.com/payments/promptpay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
/// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Samsung Pay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationSamsungPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationSamsungPayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationSamsungPay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationSamsungPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationSamsungPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationSamsungPayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationSamsungPayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Satispay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method where customers are required to [authenticate](/payments/payment-methods#customer-actions) their payment.
/// Customers pay by being redirected from your website or app, authorizing the payment with Satispay, then returning to your website or app.
/// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationSatispay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationSatispayDisplayPreference>,
}
impl CreatePaymentMethodConfigurationSatispay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationSatispay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationSatispayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationSatispayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationSatispayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
/// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://docs.stripe.com/payments/sepa-debit) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
/// Check this [page](https://docs.stripe.com/payments/sofort) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationSofortDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationSofortDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationSofortDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Swish is a [real-time](https://docs.stripe.com/payments/real-time) payment method popular in Sweden.
/// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
/// Check this [page](https://docs.stripe.com/payments/swish) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationSwishDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationSwishDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationSwishDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Twint is a payment method popular in Switzerland.
/// It allows customers to pay using their mobile phone.
/// Check this [page](https://docs.stripe.com/payments/twint) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationTwint {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationTwintDisplayPreference>,
}
impl CreatePaymentMethodConfigurationTwint {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationTwint {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreatePaymentMethodConfigurationTwintDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationTwintDisplayPreferencePreference>,
}
impl CreatePaymentMethodConfigurationTwintDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for CreatePaymentMethodConfigurationTwintDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationTwintDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationTwintDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationTwintDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentMethodConfigurationTwintDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
/// Check this [page](https://docs.stripe.com/payments/ach-direct-debit) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
/// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
/// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
/// Check this [page](https://docs.stripe.com/payments/wechat-pay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Zip gives your customers a way to split purchases over a series of payments.
/// Check this [page](https://docs.stripe.com/payments/zip) for more details like country availability.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use CreatePaymentMethodConfigurationZipDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigurationZipDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentMethodConfigurationZipDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Creates a payment method configuration
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodConfiguration {
    inner: CreatePaymentMethodConfigurationBuilder,
}
impl CreatePaymentMethodConfiguration {
    /// Construct a new `CreatePaymentMethodConfiguration`.
    pub fn new() -> Self {
        Self { inner: CreatePaymentMethodConfigurationBuilder::new() }
    }
    /// Canadian pre-authorized debit payments, check this [page](https://docs.stripe.com/payments/acss-debit) for more details like country availability.
    pub fn acss_debit(
        mut self,
        acss_debit: impl Into<CreatePaymentMethodConfigurationAcssDebit>,
    ) -> Self {
        self.inner.acss_debit = Some(acss_debit.into());
        self
    }
    /// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
    /// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
    /// Check this [page](https://docs.stripe.com/payments/affirm) for more details like country availability.
    pub fn affirm(mut self, affirm: impl Into<CreatePaymentMethodConfigurationAffirm>) -> Self {
        self.inner.affirm = Some(affirm.into());
        self
    }
    /// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://docs.stripe.com/payments/afterpay-clearpay) for more details like country availability.
    /// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
    pub fn afterpay_clearpay(
        mut self,
        afterpay_clearpay: impl Into<CreatePaymentMethodConfigurationAfterpayClearpay>,
    ) -> Self {
        self.inner.afterpay_clearpay = Some(afterpay_clearpay.into());
        self
    }
    /// Alipay is a digital wallet in China that has more than a billion active users worldwide.
    /// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
    /// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
    /// Check this [page](https://docs.stripe.com/payments/alipay) for more details.
    pub fn alipay(mut self, alipay: impl Into<CreatePaymentMethodConfigurationAlipay>) -> Self {
        self.inner.alipay = Some(alipay.into());
        self
    }
    /// Alma is a Buy Now, Pay Later payment method that offers customers the ability to pay in 2, 3, or 4 installments.
    pub fn alma(mut self, alma: impl Into<CreatePaymentMethodConfigurationAlma>) -> Self {
        self.inner.alma = Some(alma.into());
        self
    }
    /// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
    pub fn amazon_pay(
        mut self,
        amazon_pay: impl Into<CreatePaymentMethodConfigurationAmazonPay>,
    ) -> Self {
        self.inner.amazon_pay = Some(amazon_pay.into());
        self
    }
    /// Stripe users can accept [Apple Pay](https://stripe.com/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
    /// There are no additional fees to process Apple Pay payments, and the [pricing](https://stripe.com/pricing) is the same as other card transactions.
    /// Check this [page](https://docs.stripe.com/apple-pay) for more details.
    pub fn apple_pay(
        mut self,
        apple_pay: impl Into<CreatePaymentMethodConfigurationApplePay>,
    ) -> Self {
        self.inner.apple_pay = Some(apple_pay.into());
        self
    }
    /// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
    pub fn apple_pay_later(
        mut self,
        apple_pay_later: impl Into<CreatePaymentMethodConfigurationApplePayLater>,
    ) -> Self {
        self.inner.apple_pay_later = Some(apple_pay_later.into());
        self
    }
    /// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
    /// Check this [page](https://docs.stripe.com/payments/au-becs-debit) for more details.
    pub fn au_becs_debit(
        mut self,
        au_becs_debit: impl Into<CreatePaymentMethodConfigurationAuBecsDebit>,
    ) -> Self {
        self.inner.au_becs_debit = Some(au_becs_debit.into());
        self
    }
    /// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://docs.stripe.com/payments/payment-methods/bacs-debit) for more details.
    pub fn bacs_debit(
        mut self,
        bacs_debit: impl Into<CreatePaymentMethodConfigurationBacsDebit>,
    ) -> Self {
        self.inner.bacs_debit = Some(bacs_debit.into());
        self
    }
    /// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
    /// [Customers](https://docs.stripe.com/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
    /// Check this [page](https://docs.stripe.com/payments/bancontact) for more details.
    pub fn bancontact(
        mut self,
        bancontact: impl Into<CreatePaymentMethodConfigurationBancontact>,
    ) -> Self {
        self.inner.bancontact = Some(bancontact.into());
        self
    }
    /// Billie is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers businesses Pay by Invoice where they offer payment terms ranging from 7-120 days.
    /// Customers are redirected from your website or app, authorize the payment with Billie, then return to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    pub fn billie(mut self, billie: impl Into<CreatePaymentMethodConfigurationBillie>) -> Self {
        self.inner.billie = Some(billie.into());
        self
    }
    /// BLIK is a [single use](https://docs.stripe.com/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
    /// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
    /// Check this [page](https://docs.stripe.com/payments/blik) for more details.
    pub fn blik(mut self, blik: impl Into<CreatePaymentMethodConfigurationBlik>) -> Self {
        self.inner.blik = Some(blik.into());
        self
    }
    /// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
    /// Check this [page](https://docs.stripe.com/payments/boleto) for more details.
    pub fn boleto(mut self, boleto: impl Into<CreatePaymentMethodConfigurationBoleto>) -> Self {
        self.inner.boleto = Some(boleto.into());
        self
    }
    /// Cards are a popular way for consumers and businesses to pay online or in person.
    /// Stripe supports global and local card networks.
    pub fn card(mut self, card: impl Into<CreatePaymentMethodConfigurationCard>) -> Self {
        self.inner.card = Some(card.into());
        self
    }
    /// Cartes Bancaires is France's local card network.
    /// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
    /// Check this [page](https://docs.stripe.com/payments/cartes-bancaires) for more details.
    pub fn cartes_bancaires(
        mut self,
        cartes_bancaires: impl Into<CreatePaymentMethodConfigurationCartesBancaires>,
    ) -> Self {
        self.inner.cartes_bancaires = Some(cartes_bancaires.into());
        self
    }
    /// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
    /// Check this [page](https://docs.stripe.com/payments/cash-app-pay) for more details.
    pub fn cashapp(mut self, cashapp: impl Into<CreatePaymentMethodConfigurationCashapp>) -> Self {
        self.inner.cashapp = Some(cashapp.into());
        self
    }
    /// [Stablecoin payments](https://docs.stripe.com/payments/stablecoin-payments) enable customers to pay in stablecoins like USDC from 100s of wallets including Phantom and Metamask.
    pub fn crypto(mut self, crypto: impl Into<CreatePaymentMethodConfigurationCrypto>) -> Self {
        self.inner.crypto = Some(crypto.into());
        self
    }
    /// Uses a customer’s [cash balance](https://docs.stripe.com/payments/customer-balance) for the payment.
    /// The cash balance can be funded via a bank transfer.
    /// Check this [page](https://docs.stripe.com/payments/bank-transfers) for more details.
    pub fn customer_balance(
        mut self,
        customer_balance: impl Into<CreatePaymentMethodConfigurationCustomerBalance>,
    ) -> Self {
        self.inner.customer_balance = Some(customer_balance.into());
        self
    }
    /// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
    /// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
    /// Check this [page](https://docs.stripe.com/payments/eps) for more details.
    pub fn eps(mut self, eps: impl Into<CreatePaymentMethodConfigurationEps>) -> Self {
        self.inner.eps = Some(eps.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
    /// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
    /// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
    /// Check this [page](https://docs.stripe.com/payments/fpx) for more details.
    pub fn fpx(mut self, fpx: impl Into<CreatePaymentMethodConfigurationFpx>) -> Self {
        self.inner.fpx = Some(fpx.into());
        self
    }
    /// Meal vouchers in France, or “titres-restaurant”, is a local benefits program commonly offered by employers for their employees to purchase prepared food and beverages on working days.
    /// Check this [page](https://docs.stripe.com/payments/meal-vouchers/fr-meal-vouchers) for more details.
    pub fn fr_meal_voucher_conecs(
        mut self,
        fr_meal_voucher_conecs: impl Into<CreatePaymentMethodConfigurationFrMealVoucherConecs>,
    ) -> Self {
        self.inner.fr_meal_voucher_conecs = Some(fr_meal_voucher_conecs.into());
        self
    }
    /// giropay is a German payment method based on online banking, introduced in 2006.
    /// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
    /// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
    /// giropay accounts for 10% of online checkouts in Germany.
    /// Check this [page](https://docs.stripe.com/payments/giropay) for more details.
    pub fn giropay(mut self, giropay: impl Into<CreatePaymentMethodConfigurationGiropay>) -> Self {
        self.inner.giropay = Some(giropay.into());
        self
    }
    /// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
    /// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
    /// Check this [page](https://docs.stripe.com/google-pay) for more details.
    pub fn google_pay(
        mut self,
        google_pay: impl Into<CreatePaymentMethodConfigurationGooglePay>,
    ) -> Self {
        self.inner.google_pay = Some(google_pay.into());
        self
    }
    /// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
    /// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
    /// Check this [page](https://docs.stripe.com/payments/grabpay) for more details.
    pub fn grabpay(mut self, grabpay: impl Into<CreatePaymentMethodConfigurationGrabpay>) -> Self {
        self.inner.grabpay = Some(grabpay.into());
        self
    }
    /// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
    /// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
    /// Check this [page](https://docs.stripe.com/payments/ideal) for more details.
    pub fn ideal(mut self, ideal: impl Into<CreatePaymentMethodConfigurationIdeal>) -> Self {
        self.inner.ideal = Some(ideal.into());
        self
    }
    /// JCB is a credit card company based in Japan.
    /// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
    /// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
    pub fn jcb(mut self, jcb: impl Into<CreatePaymentMethodConfigurationJcb>) -> Self {
        self.inner.jcb = Some(jcb.into());
        self
    }
    /// Kakao Pay is a popular local wallet available in South Korea.
    pub fn kakao_pay(
        mut self,
        kakao_pay: impl Into<CreatePaymentMethodConfigurationKakaoPay>,
    ) -> Self {
        self.inner.kakao_pay = Some(kakao_pay.into());
        self
    }
    /// Klarna gives customers a range of [payment options](https://docs.stripe.com/payments/klarna#payment-options) during checkout.
    /// Available payment options vary depending on the customer's billing address and the transaction amount.
    /// These payment options make it convenient for customers to purchase items in all price ranges.
    /// Check this [page](https://docs.stripe.com/payments/klarna) for more details.
    pub fn klarna(mut self, klarna: impl Into<CreatePaymentMethodConfigurationKlarna>) -> Self {
        self.inner.klarna = Some(klarna.into());
        self
    }
    /// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
    /// Check this [page](https://docs.stripe.com/payments/konbini) for more details.
    pub fn konbini(mut self, konbini: impl Into<CreatePaymentMethodConfigurationKonbini>) -> Self {
        self.inner.konbini = Some(konbini.into());
        self
    }
    /// Korean cards let users pay using locally issued cards from South Korea.
    pub fn kr_card(mut self, kr_card: impl Into<CreatePaymentMethodConfigurationKrCard>) -> Self {
        self.inner.kr_card = Some(kr_card.into());
        self
    }
    /// [Link](https://docs.stripe.com/payments/link) is a payment method network.
    /// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
    pub fn link(mut self, link: impl Into<CreatePaymentMethodConfigurationLink>) -> Self {
        self.inner.link = Some(link.into());
        self
    }
    /// MB WAY is the most popular wallet in Portugal.
    /// After entering their phone number in your checkout, customers approve the payment directly in their MB WAY app.
    /// Check this [page](https://docs.stripe.com/payments/mb-way) for more details.
    pub fn mb_way(mut self, mb_way: impl Into<CreatePaymentMethodConfigurationMbWay>) -> Self {
        self.inner.mb_way = Some(mb_way.into());
        self
    }
    /// MobilePay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
    /// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the MobilePay app.
    /// Check this [page](https://docs.stripe.com/payments/mobilepay) for more details.
    pub fn mobilepay(
        mut self,
        mobilepay: impl Into<CreatePaymentMethodConfigurationMobilepay>,
    ) -> Self {
        self.inner.mobilepay = Some(mobilepay.into());
        self
    }
    /// Stripe users in Europe and the United States can accept Multibanco payments from customers in Portugal using [Sources](https://stripe.com/docs/sources)—a single integration path for creating payments using any supported method.
    pub fn multibanco(
        mut self,
        multibanco: impl Into<CreatePaymentMethodConfigurationMultibanco>,
    ) -> Self {
        self.inner.multibanco = Some(multibanco.into());
        self
    }
    /// Configuration name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
    /// Naver Pay is a popular local wallet available in South Korea.
    pub fn naver_pay(
        mut self,
        naver_pay: impl Into<CreatePaymentMethodConfigurationNaverPay>,
    ) -> Self {
        self.inner.naver_pay = Some(naver_pay.into());
        self
    }
    /// Stripe users in New Zealand can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with a New Zeland bank account.
    /// Check this [page](https://docs.stripe.com/payments/nz-bank-account) for more details.
    pub fn nz_bank_account(
        mut self,
        nz_bank_account: impl Into<CreatePaymentMethodConfigurationNzBankAccount>,
    ) -> Self {
        self.inner.nz_bank_account = Some(nz_bank_account.into());
        self
    }
    /// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
    /// OXXO allows customers to pay bills and online purchases in-store with cash.
    /// Check this [page](https://docs.stripe.com/payments/oxxo) for more details.
    pub fn oxxo(mut self, oxxo: impl Into<CreatePaymentMethodConfigurationOxxo>) -> Self {
        self.inner.oxxo = Some(oxxo.into());
        self
    }
    /// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
    /// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
    /// Check this [page](https://docs.stripe.com/payments/p24) for more details.
    pub fn p24(mut self, p24: impl Into<CreatePaymentMethodConfigurationP24>) -> Self {
        self.inner.p24 = Some(p24.into());
        self
    }
    /// Configuration's parent configuration. Specify to create a child configuration.
    pub fn parent(mut self, parent: impl Into<String>) -> Self {
        self.inner.parent = Some(parent.into());
        self
    }
    /// Pay by bank is a redirect payment method backed by bank transfers.
    /// A customer is redirected to their bank to authorize a bank transfer for a given amount.
    /// This removes a lot of the error risks inherent in waiting for the customer to initiate a transfer themselves, and is less expensive than card payments.
    pub fn pay_by_bank(
        mut self,
        pay_by_bank: impl Into<CreatePaymentMethodConfigurationPayByBank>,
    ) -> Self {
        self.inner.pay_by_bank = Some(pay_by_bank.into());
        self
    }
    /// PAYCO is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
    pub fn payco(mut self, payco: impl Into<CreatePaymentMethodConfigurationPayco>) -> Self {
        self.inner.payco = Some(payco.into());
        self
    }
    /// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
    /// Check this [page](https://docs.stripe.com/payments/paynow) for more details.
    pub fn paynow(mut self, paynow: impl Into<CreatePaymentMethodConfigurationPaynow>) -> Self {
        self.inner.paynow = Some(paynow.into());
        self
    }
    /// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
    /// Check this [page](https://docs.stripe.com/payments/paypal) for more details.
    pub fn paypal(mut self, paypal: impl Into<CreatePaymentMethodConfigurationPaypal>) -> Self {
        self.inner.paypal = Some(paypal.into());
        self
    }
    /// PayTo is a [real-time](https://docs.stripe.com/payments/real-time) payment method that enables customers in Australia to pay by providing their bank account details.
    /// Customers must accept a mandate authorizing you to debit their account.
    /// Check this [page](https://docs.stripe.com/payments/payto) for more details.
    pub fn payto(mut self, payto: impl Into<CreatePaymentMethodConfigurationPayto>) -> Self {
        self.inner.payto = Some(payto.into());
        self
    }
    /// Pix is a payment method popular in Brazil.
    /// When paying with Pix, customers authenticate and approve payments by scanning a QR code in their preferred banking app.
    /// Check this [page](https://docs.stripe.com/payments/pix) for more details.
    pub fn pix(mut self, pix: impl Into<CreatePaymentMethodConfigurationPix>) -> Self {
        self.inner.pix = Some(pix.into());
        self
    }
    /// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
    /// Check this [page](https://docs.stripe.com/payments/promptpay) for more details.
    pub fn promptpay(
        mut self,
        promptpay: impl Into<CreatePaymentMethodConfigurationPromptpay>,
    ) -> Self {
        self.inner.promptpay = Some(promptpay.into());
        self
    }
    /// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
    /// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
    pub fn revolut_pay(
        mut self,
        revolut_pay: impl Into<CreatePaymentMethodConfigurationRevolutPay>,
    ) -> Self {
        self.inner.revolut_pay = Some(revolut_pay.into());
        self
    }
    /// Samsung Pay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
    pub fn samsung_pay(
        mut self,
        samsung_pay: impl Into<CreatePaymentMethodConfigurationSamsungPay>,
    ) -> Self {
        self.inner.samsung_pay = Some(samsung_pay.into());
        self
    }
    /// Satispay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method where customers are required to [authenticate](/payments/payment-methods#customer-actions) their payment.
    /// Customers pay by being redirected from your website or app, authorizing the payment with Satispay, then returning to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    pub fn satispay(
        mut self,
        satispay: impl Into<CreatePaymentMethodConfigurationSatispay>,
    ) -> Self {
        self.inner.satispay = Some(satispay.into());
        self
    }
    /// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
    /// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://docs.stripe.com/payments/sepa-debit) for more details.
    pub fn sepa_debit(
        mut self,
        sepa_debit: impl Into<CreatePaymentMethodConfigurationSepaDebit>,
    ) -> Self {
        self.inner.sepa_debit = Some(sepa_debit.into());
        self
    }
    /// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
    /// Check this [page](https://docs.stripe.com/payments/sofort) for more details.
    pub fn sofort(mut self, sofort: impl Into<CreatePaymentMethodConfigurationSofort>) -> Self {
        self.inner.sofort = Some(sofort.into());
        self
    }
    /// Swish is a [real-time](https://docs.stripe.com/payments/real-time) payment method popular in Sweden.
    /// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
    /// Check this [page](https://docs.stripe.com/payments/swish) for more details.
    pub fn swish(mut self, swish: impl Into<CreatePaymentMethodConfigurationSwish>) -> Self {
        self.inner.swish = Some(swish.into());
        self
    }
    /// Twint is a payment method popular in Switzerland.
    /// It allows customers to pay using their mobile phone.
    /// Check this [page](https://docs.stripe.com/payments/twint) for more details.
    pub fn twint(mut self, twint: impl Into<CreatePaymentMethodConfigurationTwint>) -> Self {
        self.inner.twint = Some(twint.into());
        self
    }
    /// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
    /// Check this [page](https://docs.stripe.com/payments/ach-direct-debit) for more details.
    pub fn us_bank_account(
        mut self,
        us_bank_account: impl Into<CreatePaymentMethodConfigurationUsBankAccount>,
    ) -> Self {
        self.inner.us_bank_account = Some(us_bank_account.into());
        self
    }
    /// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
    /// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
    /// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
    /// Check this [page](https://docs.stripe.com/payments/wechat-pay) for more details.
    pub fn wechat_pay(
        mut self,
        wechat_pay: impl Into<CreatePaymentMethodConfigurationWechatPay>,
    ) -> Self {
        self.inner.wechat_pay = Some(wechat_pay.into());
        self
    }
    /// Zip gives your customers a way to split purchases over a series of payments.
    /// Check this [page](https://docs.stripe.com/payments/zip) for more details like country availability.
    pub fn zip(mut self, zip: impl Into<CreatePaymentMethodConfigurationZip>) -> Self {
        self.inner.zip = Some(zip.into());
        self
    }
}
impl Default for CreatePaymentMethodConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
impl CreatePaymentMethodConfiguration {
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

impl StripeRequest for CreatePaymentMethodConfiguration {
    type Output = stripe_payment::PaymentMethodConfiguration;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/payment_method_configurations").form(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct UpdatePaymentMethodConfigurationBuilder {
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
    alma: Option<UpdatePaymentMethodConfigurationAlma>,
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
    billie: Option<UpdatePaymentMethodConfigurationBillie>,
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
    crypto: Option<UpdatePaymentMethodConfigurationCrypto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_balance: Option<UpdatePaymentMethodConfigurationCustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eps: Option<UpdatePaymentMethodConfigurationEps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fpx: Option<UpdatePaymentMethodConfigurationFpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fr_meal_voucher_conecs: Option<UpdatePaymentMethodConfigurationFrMealVoucherConecs>,
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
    kakao_pay: Option<UpdatePaymentMethodConfigurationKakaoPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    klarna: Option<UpdatePaymentMethodConfigurationKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    konbini: Option<UpdatePaymentMethodConfigurationKonbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kr_card: Option<UpdatePaymentMethodConfigurationKrCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<UpdatePaymentMethodConfigurationLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mb_way: Option<UpdatePaymentMethodConfigurationMbWay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobilepay: Option<UpdatePaymentMethodConfigurationMobilepay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multibanco: Option<UpdatePaymentMethodConfigurationMultibanco>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    naver_pay: Option<UpdatePaymentMethodConfigurationNaverPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nz_bank_account: Option<UpdatePaymentMethodConfigurationNzBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oxxo: Option<UpdatePaymentMethodConfigurationOxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    p24: Option<UpdatePaymentMethodConfigurationP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay_by_bank: Option<UpdatePaymentMethodConfigurationPayByBank>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payco: Option<UpdatePaymentMethodConfigurationPayco>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paynow: Option<UpdatePaymentMethodConfigurationPaynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paypal: Option<UpdatePaymentMethodConfigurationPaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payto: Option<UpdatePaymentMethodConfigurationPayto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pix: Option<UpdatePaymentMethodConfigurationPix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promptpay: Option<UpdatePaymentMethodConfigurationPromptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revolut_pay: Option<UpdatePaymentMethodConfigurationRevolutPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    samsung_pay: Option<UpdatePaymentMethodConfigurationSamsungPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    satispay: Option<UpdatePaymentMethodConfigurationSatispay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sepa_debit: Option<UpdatePaymentMethodConfigurationSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sofort: Option<UpdatePaymentMethodConfigurationSofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swish: Option<UpdatePaymentMethodConfigurationSwish>,
    #[serde(skip_serializing_if = "Option::is_none")]
    twint: Option<UpdatePaymentMethodConfigurationTwint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    us_bank_account: Option<UpdatePaymentMethodConfigurationUsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wechat_pay: Option<UpdatePaymentMethodConfigurationWechatPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip: Option<UpdatePaymentMethodConfigurationZip>,
}
impl UpdatePaymentMethodConfigurationBuilder {
    fn new() -> Self {
        Self {
            acss_debit: None,
            active: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            alma: None,
            amazon_pay: None,
            apple_pay: None,
            apple_pay_later: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            billie: None,
            blik: None,
            boleto: None,
            card: None,
            cartes_bancaires: None,
            cashapp: None,
            crypto: None,
            customer_balance: None,
            eps: None,
            expand: None,
            fpx: None,
            fr_meal_voucher_conecs: None,
            giropay: None,
            google_pay: None,
            grabpay: None,
            ideal: None,
            jcb: None,
            kakao_pay: None,
            klarna: None,
            konbini: None,
            kr_card: None,
            link: None,
            mb_way: None,
            mobilepay: None,
            multibanco: None,
            name: None,
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
            revolut_pay: None,
            samsung_pay: None,
            satispay: None,
            sepa_debit: None,
            sofort: None,
            swish: None,
            twint: None,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// Canadian pre-authorized debit payments, check this [page](https://docs.stripe.com/payments/acss-debit) for more details like country availability.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
/// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
/// Check this [page](https://docs.stripe.com/payments/affirm) for more details like country availability.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://docs.stripe.com/payments/afterpay-clearpay) for more details like country availability.
/// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Alipay is a digital wallet in China that has more than a billion active users worldwide.
/// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
/// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
/// Check this [page](https://docs.stripe.com/payments/alipay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Alma is a Buy Now, Pay Later payment method that offers customers the ability to pay in 2, 3, or 4 installments.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAlma {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAlmaDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationAlma {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAlma {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationAlmaDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationAlmaDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationAlmaDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users can accept [Apple Pay](https://stripe.com/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
/// There are no additional fees to process Apple Pay payments, and the [pricing](https://stripe.com/pricing) is the same as other card transactions.
/// Check this [page](https://docs.stripe.com/apple-pay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
/// Check this [page](https://docs.stripe.com/payments/au-becs-debit) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://docs.stripe.com/payments/payment-methods/bacs-debit) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
/// [Customers](https://docs.stripe.com/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
/// Check this [page](https://docs.stripe.com/payments/bancontact) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Billie is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers businesses Pay by Invoice where they offer payment terms ranging from 7-120 days.
/// Customers are redirected from your website or app, authorize the payment with Billie, then return to your website or app.
/// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationBillie {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationBillieDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationBillie {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationBillie {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationBillieDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationBillieDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationBillieDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// BLIK is a [single use](https://docs.stripe.com/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
/// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
/// Check this [page](https://docs.stripe.com/payments/blik) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
/// Check this [page](https://docs.stripe.com/payments/boleto) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Cards are a popular way for consumers and businesses to pay online or in person.
/// Stripe supports global and local card networks.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationCardDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationCardDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationCardDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationCardDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationCardDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationCardDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Cartes Bancaires is France's local card network.
/// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
/// Check this [page](https://docs.stripe.com/payments/cartes-bancaires) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
/// Check this [page](https://docs.stripe.com/payments/cash-app-pay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// [Stablecoin payments](https://docs.stripe.com/payments/stablecoin-payments) enable customers to pay in stablecoins like USDC from 100s of wallets including Phantom and Metamask.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationCrypto {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationCryptoDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationCrypto {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationCrypto {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationCryptoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationCryptoDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationCryptoDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationCryptoDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationCryptoDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationCryptoDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationCryptoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationCryptoDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationCryptoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationCryptoDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationCryptoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationCryptoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationCryptoDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationCryptoDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Uses a customer’s [cash balance](https://docs.stripe.com/payments/customer-balance) for the payment.
/// The cash balance can be funded via a bank transfer.
/// Check this [page](https://docs.stripe.com/payments/bank-transfers) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
/// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
/// Check this [page](https://docs.stripe.com/payments/eps) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
/// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
/// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
/// Check this [page](https://docs.stripe.com/payments/fpx) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Meal vouchers in France, or “titres-restaurant”, is a local benefits program commonly offered by employers for their employees to purchase prepared food and beverages on working days.
/// Check this [page](https://docs.stripe.com/payments/meal-vouchers/fr-meal-vouchers) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationFrMealVoucherConecs {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationFrMealVoucherConecs {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationFrMealVoucherConecs {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference
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
    for UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// giropay is a German payment method based on online banking, introduced in 2006.
/// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
/// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
/// giropay accounts for 10% of online checkouts in Germany.
/// Check this [page](https://docs.stripe.com/payments/giropay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
/// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
/// Check this [page](https://docs.stripe.com/google-pay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
/// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
/// Check this [page](https://docs.stripe.com/payments/grabpay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
/// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
/// Check this [page](https://docs.stripe.com/payments/ideal) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// JCB is a credit card company based in Japan.
/// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
/// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Kakao Pay is a popular local wallet available in South Korea.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationKakaoPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationKakaoPayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationKakaoPay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationKakaoPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationKakaoPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationKakaoPayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationKakaoPayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Klarna gives customers a range of [payment options](https://docs.stripe.com/payments/klarna#payment-options) during checkout.
/// Available payment options vary depending on the customer's billing address and the transaction amount.
/// These payment options make it convenient for customers to purchase items in all price ranges.
/// Check this [page](https://docs.stripe.com/payments/klarna) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
/// Check this [page](https://docs.stripe.com/payments/konbini) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Korean cards let users pay using locally issued cards from South Korea.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationKrCard {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationKrCardDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationKrCard {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationKrCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationKrCardDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationKrCardDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationKrCardDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationKrCardDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationKrCardDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationKrCardDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationKrCardDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationKrCardDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationKrCardDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationKrCardDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationKrCardDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationKrCardDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationKrCardDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationKrCardDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// [Link](https://docs.stripe.com/payments/link) is a payment method network.
/// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// MB WAY is the most popular wallet in Portugal.
/// After entering their phone number in your checkout, customers approve the payment directly in their MB WAY app.
/// Check this [page](https://docs.stripe.com/payments/mb-way) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationMbWay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationMbWayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationMbWay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationMbWay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationMbWayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationMbWayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationMbWayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationMbWayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationMbWayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationMbWayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationMbWayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationMbWayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationMbWayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationMbWayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationMbWayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationMbWayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationMbWayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationMbWayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// MobilePay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
/// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the MobilePay app.
/// Check this [page](https://docs.stripe.com/payments/mobilepay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users in Europe and the United States can accept Multibanco payments from customers in Portugal using [Sources](https://stripe.com/docs/sources)—a single integration path for creating payments using any supported method.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationMultibanco {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationMultibancoDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationMultibanco {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationMultibanco {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationMultibancoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationMultibancoDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationMultibancoDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Naver Pay is a popular local wallet available in South Korea.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationNaverPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationNaverPayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationNaverPay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationNaverPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationNaverPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationNaverPayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationNaverPayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationNaverPayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationNaverPayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationNaverPayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationNaverPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationNaverPayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationNaverPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationNaverPayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationNaverPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationNaverPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationNaverPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationNaverPayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users in New Zealand can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with a New Zeland bank account.
/// Check this [page](https://docs.stripe.com/payments/nz-bank-account) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationNzBankAccount {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationNzBankAccountDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationNzBankAccount {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationNzBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationNzBankAccountDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference:
        Option<UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationNzBankAccountDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationNzBankAccountDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
/// OXXO allows customers to pay bills and online purchases in-store with cash.
/// Check this [page](https://docs.stripe.com/payments/oxxo) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
/// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
/// Check this [page](https://docs.stripe.com/payments/p24) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationP24DisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationP24DisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationP24DisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Pay by bank is a redirect payment method backed by bank transfers.
/// A customer is redirected to their bank to authorize a bank transfer for a given amount.
/// This removes a lot of the error risks inherent in waiting for the customer to initiate a transfer themselves, and is less expensive than card payments.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPayByBank {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationPayByBankDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationPayByBank {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPayByBank {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPayByBankDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationPayByBankDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPayByBankDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// PAYCO is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPayco {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationPaycoDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationPayco {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPayco {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPaycoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationPaycoDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationPaycoDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPaycoDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationPaycoDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationPaycoDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationPaycoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationPaycoDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationPaycoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationPaycoDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationPaycoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationPaycoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationPaycoDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationPaycoDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
/// Check this [page](https://docs.stripe.com/payments/paynow) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
/// Check this [page](https://docs.stripe.com/payments/paypal) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// PayTo is a [real-time](https://docs.stripe.com/payments/real-time) payment method that enables customers in Australia to pay by providing their bank account details.
/// Customers must accept a mandate authorizing you to debit their account.
/// Check this [page](https://docs.stripe.com/payments/payto) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPayto {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationPaytoDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationPayto {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPayto {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPaytoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationPaytoDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationPaytoDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPaytoDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationPaytoDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationPaytoDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationPaytoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationPaytoDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationPaytoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationPaytoDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationPaytoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationPaytoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationPaytoDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationPaytoDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Pix is a payment method popular in Brazil.
/// When paying with Pix, customers authenticate and approve payments by scanning a QR code in their preferred banking app.
/// Check this [page](https://docs.stripe.com/payments/pix) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPix {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationPixDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationPix {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPix {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationPixDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationPixDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationPixDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationPixDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationPixDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationPixDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationPixDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationPixDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationPixDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationPixDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationPixDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationPixDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationPixDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationPixDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
/// Check this [page](https://docs.stripe.com/payments/promptpay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
/// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Samsung Pay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationSamsungPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationSamsungPayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationSamsungPay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationSamsungPay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationSamsungPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationSamsungPayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationSamsungPayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Satispay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method where customers are required to [authenticate](/payments/payment-methods#customer-actions) their payment.
/// Customers pay by being redirected from your website or app, authorizing the payment with Satispay, then returning to your website or app.
/// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationSatispay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationSatispayDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationSatispay {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationSatispay {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationSatispayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationSatispayDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationSatispayDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
/// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://docs.stripe.com/payments/sepa-debit) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
/// Check this [page](https://docs.stripe.com/payments/sofort) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Swish is a [real-time](https://docs.stripe.com/payments/real-time) payment method popular in Sweden.
/// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
/// Check this [page](https://docs.stripe.com/payments/swish) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Twint is a payment method popular in Switzerland.
/// It allows customers to pay using their mobile phone.
/// Check this [page](https://docs.stripe.com/payments/twint) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationTwint {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationTwintDisplayPreference>,
}
impl UpdatePaymentMethodConfigurationTwint {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationTwint {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct UpdatePaymentMethodConfigurationTwintDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference>,
}
impl UpdatePaymentMethodConfigurationTwintDisplayPreference {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for UpdatePaymentMethodConfigurationTwintDisplayPreference {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
/// Check this [page](https://docs.stripe.com/payments/ach-direct-debit) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
/// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
/// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
/// Check this [page](https://docs.stripe.com/payments/wechat-pay) for more details.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Zip gives your customers a way to split purchases over a series of payments.
/// Check this [page](https://docs.stripe.com/payments/zip) for more details like country availability.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use UpdatePaymentMethodConfigurationZipDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigurationZipDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdatePaymentMethodConfigurationZipDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Update payment method configuration
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodConfiguration {
    inner: UpdatePaymentMethodConfigurationBuilder,
    configuration: stripe_payment::PaymentMethodConfigurationId,
}
impl UpdatePaymentMethodConfiguration {
    /// Construct a new `UpdatePaymentMethodConfiguration`.
    pub fn new(configuration: impl Into<stripe_payment::PaymentMethodConfigurationId>) -> Self {
        Self {
            configuration: configuration.into(),
            inner: UpdatePaymentMethodConfigurationBuilder::new(),
        }
    }
    /// Canadian pre-authorized debit payments, check this [page](https://docs.stripe.com/payments/acss-debit) for more details like country availability.
    pub fn acss_debit(
        mut self,
        acss_debit: impl Into<UpdatePaymentMethodConfigurationAcssDebit>,
    ) -> Self {
        self.inner.acss_debit = Some(acss_debit.into());
        self
    }
    /// Whether the configuration can be used for new payments.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
    /// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
    /// Check this [page](https://docs.stripe.com/payments/affirm) for more details like country availability.
    pub fn affirm(mut self, affirm: impl Into<UpdatePaymentMethodConfigurationAffirm>) -> Self {
        self.inner.affirm = Some(affirm.into());
        self
    }
    /// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://docs.stripe.com/payments/afterpay-clearpay) for more details like country availability.
    /// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
    pub fn afterpay_clearpay(
        mut self,
        afterpay_clearpay: impl Into<UpdatePaymentMethodConfigurationAfterpayClearpay>,
    ) -> Self {
        self.inner.afterpay_clearpay = Some(afterpay_clearpay.into());
        self
    }
    /// Alipay is a digital wallet in China that has more than a billion active users worldwide.
    /// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
    /// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
    /// Check this [page](https://docs.stripe.com/payments/alipay) for more details.
    pub fn alipay(mut self, alipay: impl Into<UpdatePaymentMethodConfigurationAlipay>) -> Self {
        self.inner.alipay = Some(alipay.into());
        self
    }
    /// Alma is a Buy Now, Pay Later payment method that offers customers the ability to pay in 2, 3, or 4 installments.
    pub fn alma(mut self, alma: impl Into<UpdatePaymentMethodConfigurationAlma>) -> Self {
        self.inner.alma = Some(alma.into());
        self
    }
    /// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
    pub fn amazon_pay(
        mut self,
        amazon_pay: impl Into<UpdatePaymentMethodConfigurationAmazonPay>,
    ) -> Self {
        self.inner.amazon_pay = Some(amazon_pay.into());
        self
    }
    /// Stripe users can accept [Apple Pay](https://stripe.com/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
    /// There are no additional fees to process Apple Pay payments, and the [pricing](https://stripe.com/pricing) is the same as other card transactions.
    /// Check this [page](https://docs.stripe.com/apple-pay) for more details.
    pub fn apple_pay(
        mut self,
        apple_pay: impl Into<UpdatePaymentMethodConfigurationApplePay>,
    ) -> Self {
        self.inner.apple_pay = Some(apple_pay.into());
        self
    }
    /// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
    pub fn apple_pay_later(
        mut self,
        apple_pay_later: impl Into<UpdatePaymentMethodConfigurationApplePayLater>,
    ) -> Self {
        self.inner.apple_pay_later = Some(apple_pay_later.into());
        self
    }
    /// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
    /// Check this [page](https://docs.stripe.com/payments/au-becs-debit) for more details.
    pub fn au_becs_debit(
        mut self,
        au_becs_debit: impl Into<UpdatePaymentMethodConfigurationAuBecsDebit>,
    ) -> Self {
        self.inner.au_becs_debit = Some(au_becs_debit.into());
        self
    }
    /// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://docs.stripe.com/payments/payment-methods/bacs-debit) for more details.
    pub fn bacs_debit(
        mut self,
        bacs_debit: impl Into<UpdatePaymentMethodConfigurationBacsDebit>,
    ) -> Self {
        self.inner.bacs_debit = Some(bacs_debit.into());
        self
    }
    /// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
    /// [Customers](https://docs.stripe.com/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
    /// Check this [page](https://docs.stripe.com/payments/bancontact) for more details.
    pub fn bancontact(
        mut self,
        bancontact: impl Into<UpdatePaymentMethodConfigurationBancontact>,
    ) -> Self {
        self.inner.bancontact = Some(bancontact.into());
        self
    }
    /// Billie is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers businesses Pay by Invoice where they offer payment terms ranging from 7-120 days.
    /// Customers are redirected from your website or app, authorize the payment with Billie, then return to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    pub fn billie(mut self, billie: impl Into<UpdatePaymentMethodConfigurationBillie>) -> Self {
        self.inner.billie = Some(billie.into());
        self
    }
    /// BLIK is a [single use](https://docs.stripe.com/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
    /// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
    /// Check this [page](https://docs.stripe.com/payments/blik) for more details.
    pub fn blik(mut self, blik: impl Into<UpdatePaymentMethodConfigurationBlik>) -> Self {
        self.inner.blik = Some(blik.into());
        self
    }
    /// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
    /// Check this [page](https://docs.stripe.com/payments/boleto) for more details.
    pub fn boleto(mut self, boleto: impl Into<UpdatePaymentMethodConfigurationBoleto>) -> Self {
        self.inner.boleto = Some(boleto.into());
        self
    }
    /// Cards are a popular way for consumers and businesses to pay online or in person.
    /// Stripe supports global and local card networks.
    pub fn card(mut self, card: impl Into<UpdatePaymentMethodConfigurationCard>) -> Self {
        self.inner.card = Some(card.into());
        self
    }
    /// Cartes Bancaires is France's local card network.
    /// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
    /// Check this [page](https://docs.stripe.com/payments/cartes-bancaires) for more details.
    pub fn cartes_bancaires(
        mut self,
        cartes_bancaires: impl Into<UpdatePaymentMethodConfigurationCartesBancaires>,
    ) -> Self {
        self.inner.cartes_bancaires = Some(cartes_bancaires.into());
        self
    }
    /// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
    /// Check this [page](https://docs.stripe.com/payments/cash-app-pay) for more details.
    pub fn cashapp(mut self, cashapp: impl Into<UpdatePaymentMethodConfigurationCashapp>) -> Self {
        self.inner.cashapp = Some(cashapp.into());
        self
    }
    /// [Stablecoin payments](https://docs.stripe.com/payments/stablecoin-payments) enable customers to pay in stablecoins like USDC from 100s of wallets including Phantom and Metamask.
    pub fn crypto(mut self, crypto: impl Into<UpdatePaymentMethodConfigurationCrypto>) -> Self {
        self.inner.crypto = Some(crypto.into());
        self
    }
    /// Uses a customer’s [cash balance](https://docs.stripe.com/payments/customer-balance) for the payment.
    /// The cash balance can be funded via a bank transfer.
    /// Check this [page](https://docs.stripe.com/payments/bank-transfers) for more details.
    pub fn customer_balance(
        mut self,
        customer_balance: impl Into<UpdatePaymentMethodConfigurationCustomerBalance>,
    ) -> Self {
        self.inner.customer_balance = Some(customer_balance.into());
        self
    }
    /// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
    /// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
    /// Check this [page](https://docs.stripe.com/payments/eps) for more details.
    pub fn eps(mut self, eps: impl Into<UpdatePaymentMethodConfigurationEps>) -> Self {
        self.inner.eps = Some(eps.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
    /// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
    /// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
    /// Check this [page](https://docs.stripe.com/payments/fpx) for more details.
    pub fn fpx(mut self, fpx: impl Into<UpdatePaymentMethodConfigurationFpx>) -> Self {
        self.inner.fpx = Some(fpx.into());
        self
    }
    /// Meal vouchers in France, or “titres-restaurant”, is a local benefits program commonly offered by employers for their employees to purchase prepared food and beverages on working days.
    /// Check this [page](https://docs.stripe.com/payments/meal-vouchers/fr-meal-vouchers) for more details.
    pub fn fr_meal_voucher_conecs(
        mut self,
        fr_meal_voucher_conecs: impl Into<UpdatePaymentMethodConfigurationFrMealVoucherConecs>,
    ) -> Self {
        self.inner.fr_meal_voucher_conecs = Some(fr_meal_voucher_conecs.into());
        self
    }
    /// giropay is a German payment method based on online banking, introduced in 2006.
    /// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
    /// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
    /// giropay accounts for 10% of online checkouts in Germany.
    /// Check this [page](https://docs.stripe.com/payments/giropay) for more details.
    pub fn giropay(mut self, giropay: impl Into<UpdatePaymentMethodConfigurationGiropay>) -> Self {
        self.inner.giropay = Some(giropay.into());
        self
    }
    /// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
    /// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
    /// Check this [page](https://docs.stripe.com/google-pay) for more details.
    pub fn google_pay(
        mut self,
        google_pay: impl Into<UpdatePaymentMethodConfigurationGooglePay>,
    ) -> Self {
        self.inner.google_pay = Some(google_pay.into());
        self
    }
    /// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
    /// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
    /// Check this [page](https://docs.stripe.com/payments/grabpay) for more details.
    pub fn grabpay(mut self, grabpay: impl Into<UpdatePaymentMethodConfigurationGrabpay>) -> Self {
        self.inner.grabpay = Some(grabpay.into());
        self
    }
    /// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
    /// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
    /// Check this [page](https://docs.stripe.com/payments/ideal) for more details.
    pub fn ideal(mut self, ideal: impl Into<UpdatePaymentMethodConfigurationIdeal>) -> Self {
        self.inner.ideal = Some(ideal.into());
        self
    }
    /// JCB is a credit card company based in Japan.
    /// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
    /// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
    pub fn jcb(mut self, jcb: impl Into<UpdatePaymentMethodConfigurationJcb>) -> Self {
        self.inner.jcb = Some(jcb.into());
        self
    }
    /// Kakao Pay is a popular local wallet available in South Korea.
    pub fn kakao_pay(
        mut self,
        kakao_pay: impl Into<UpdatePaymentMethodConfigurationKakaoPay>,
    ) -> Self {
        self.inner.kakao_pay = Some(kakao_pay.into());
        self
    }
    /// Klarna gives customers a range of [payment options](https://docs.stripe.com/payments/klarna#payment-options) during checkout.
    /// Available payment options vary depending on the customer's billing address and the transaction amount.
    /// These payment options make it convenient for customers to purchase items in all price ranges.
    /// Check this [page](https://docs.stripe.com/payments/klarna) for more details.
    pub fn klarna(mut self, klarna: impl Into<UpdatePaymentMethodConfigurationKlarna>) -> Self {
        self.inner.klarna = Some(klarna.into());
        self
    }
    /// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
    /// Check this [page](https://docs.stripe.com/payments/konbini) for more details.
    pub fn konbini(mut self, konbini: impl Into<UpdatePaymentMethodConfigurationKonbini>) -> Self {
        self.inner.konbini = Some(konbini.into());
        self
    }
    /// Korean cards let users pay using locally issued cards from South Korea.
    pub fn kr_card(mut self, kr_card: impl Into<UpdatePaymentMethodConfigurationKrCard>) -> Self {
        self.inner.kr_card = Some(kr_card.into());
        self
    }
    /// [Link](https://docs.stripe.com/payments/link) is a payment method network.
    /// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
    pub fn link(mut self, link: impl Into<UpdatePaymentMethodConfigurationLink>) -> Self {
        self.inner.link = Some(link.into());
        self
    }
    /// MB WAY is the most popular wallet in Portugal.
    /// After entering their phone number in your checkout, customers approve the payment directly in their MB WAY app.
    /// Check this [page](https://docs.stripe.com/payments/mb-way) for more details.
    pub fn mb_way(mut self, mb_way: impl Into<UpdatePaymentMethodConfigurationMbWay>) -> Self {
        self.inner.mb_way = Some(mb_way.into());
        self
    }
    /// MobilePay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
    /// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the MobilePay app.
    /// Check this [page](https://docs.stripe.com/payments/mobilepay) for more details.
    pub fn mobilepay(
        mut self,
        mobilepay: impl Into<UpdatePaymentMethodConfigurationMobilepay>,
    ) -> Self {
        self.inner.mobilepay = Some(mobilepay.into());
        self
    }
    /// Stripe users in Europe and the United States can accept Multibanco payments from customers in Portugal using [Sources](https://stripe.com/docs/sources)—a single integration path for creating payments using any supported method.
    pub fn multibanco(
        mut self,
        multibanco: impl Into<UpdatePaymentMethodConfigurationMultibanco>,
    ) -> Self {
        self.inner.multibanco = Some(multibanco.into());
        self
    }
    /// Configuration name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
    /// Naver Pay is a popular local wallet available in South Korea.
    pub fn naver_pay(
        mut self,
        naver_pay: impl Into<UpdatePaymentMethodConfigurationNaverPay>,
    ) -> Self {
        self.inner.naver_pay = Some(naver_pay.into());
        self
    }
    /// Stripe users in New Zealand can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with a New Zeland bank account.
    /// Check this [page](https://docs.stripe.com/payments/nz-bank-account) for more details.
    pub fn nz_bank_account(
        mut self,
        nz_bank_account: impl Into<UpdatePaymentMethodConfigurationNzBankAccount>,
    ) -> Self {
        self.inner.nz_bank_account = Some(nz_bank_account.into());
        self
    }
    /// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
    /// OXXO allows customers to pay bills and online purchases in-store with cash.
    /// Check this [page](https://docs.stripe.com/payments/oxxo) for more details.
    pub fn oxxo(mut self, oxxo: impl Into<UpdatePaymentMethodConfigurationOxxo>) -> Self {
        self.inner.oxxo = Some(oxxo.into());
        self
    }
    /// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
    /// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
    /// Check this [page](https://docs.stripe.com/payments/p24) for more details.
    pub fn p24(mut self, p24: impl Into<UpdatePaymentMethodConfigurationP24>) -> Self {
        self.inner.p24 = Some(p24.into());
        self
    }
    /// Pay by bank is a redirect payment method backed by bank transfers.
    /// A customer is redirected to their bank to authorize a bank transfer for a given amount.
    /// This removes a lot of the error risks inherent in waiting for the customer to initiate a transfer themselves, and is less expensive than card payments.
    pub fn pay_by_bank(
        mut self,
        pay_by_bank: impl Into<UpdatePaymentMethodConfigurationPayByBank>,
    ) -> Self {
        self.inner.pay_by_bank = Some(pay_by_bank.into());
        self
    }
    /// PAYCO is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
    pub fn payco(mut self, payco: impl Into<UpdatePaymentMethodConfigurationPayco>) -> Self {
        self.inner.payco = Some(payco.into());
        self
    }
    /// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
    /// Check this [page](https://docs.stripe.com/payments/paynow) for more details.
    pub fn paynow(mut self, paynow: impl Into<UpdatePaymentMethodConfigurationPaynow>) -> Self {
        self.inner.paynow = Some(paynow.into());
        self
    }
    /// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
    /// Check this [page](https://docs.stripe.com/payments/paypal) for more details.
    pub fn paypal(mut self, paypal: impl Into<UpdatePaymentMethodConfigurationPaypal>) -> Self {
        self.inner.paypal = Some(paypal.into());
        self
    }
    /// PayTo is a [real-time](https://docs.stripe.com/payments/real-time) payment method that enables customers in Australia to pay by providing their bank account details.
    /// Customers must accept a mandate authorizing you to debit their account.
    /// Check this [page](https://docs.stripe.com/payments/payto) for more details.
    pub fn payto(mut self, payto: impl Into<UpdatePaymentMethodConfigurationPayto>) -> Self {
        self.inner.payto = Some(payto.into());
        self
    }
    /// Pix is a payment method popular in Brazil.
    /// When paying with Pix, customers authenticate and approve payments by scanning a QR code in their preferred banking app.
    /// Check this [page](https://docs.stripe.com/payments/pix) for more details.
    pub fn pix(mut self, pix: impl Into<UpdatePaymentMethodConfigurationPix>) -> Self {
        self.inner.pix = Some(pix.into());
        self
    }
    /// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
    /// Check this [page](https://docs.stripe.com/payments/promptpay) for more details.
    pub fn promptpay(
        mut self,
        promptpay: impl Into<UpdatePaymentMethodConfigurationPromptpay>,
    ) -> Self {
        self.inner.promptpay = Some(promptpay.into());
        self
    }
    /// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
    /// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
    pub fn revolut_pay(
        mut self,
        revolut_pay: impl Into<UpdatePaymentMethodConfigurationRevolutPay>,
    ) -> Self {
        self.inner.revolut_pay = Some(revolut_pay.into());
        self
    }
    /// Samsung Pay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
    pub fn samsung_pay(
        mut self,
        samsung_pay: impl Into<UpdatePaymentMethodConfigurationSamsungPay>,
    ) -> Self {
        self.inner.samsung_pay = Some(samsung_pay.into());
        self
    }
    /// Satispay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method where customers are required to [authenticate](/payments/payment-methods#customer-actions) their payment.
    /// Customers pay by being redirected from your website or app, authorizing the payment with Satispay, then returning to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    pub fn satispay(
        mut self,
        satispay: impl Into<UpdatePaymentMethodConfigurationSatispay>,
    ) -> Self {
        self.inner.satispay = Some(satispay.into());
        self
    }
    /// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
    /// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://docs.stripe.com/payments/sepa-debit) for more details.
    pub fn sepa_debit(
        mut self,
        sepa_debit: impl Into<UpdatePaymentMethodConfigurationSepaDebit>,
    ) -> Self {
        self.inner.sepa_debit = Some(sepa_debit.into());
        self
    }
    /// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
    /// Check this [page](https://docs.stripe.com/payments/sofort) for more details.
    pub fn sofort(mut self, sofort: impl Into<UpdatePaymentMethodConfigurationSofort>) -> Self {
        self.inner.sofort = Some(sofort.into());
        self
    }
    /// Swish is a [real-time](https://docs.stripe.com/payments/real-time) payment method popular in Sweden.
    /// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
    /// Check this [page](https://docs.stripe.com/payments/swish) for more details.
    pub fn swish(mut self, swish: impl Into<UpdatePaymentMethodConfigurationSwish>) -> Self {
        self.inner.swish = Some(swish.into());
        self
    }
    /// Twint is a payment method popular in Switzerland.
    /// It allows customers to pay using their mobile phone.
    /// Check this [page](https://docs.stripe.com/payments/twint) for more details.
    pub fn twint(mut self, twint: impl Into<UpdatePaymentMethodConfigurationTwint>) -> Self {
        self.inner.twint = Some(twint.into());
        self
    }
    /// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
    /// Check this [page](https://docs.stripe.com/payments/ach-direct-debit) for more details.
    pub fn us_bank_account(
        mut self,
        us_bank_account: impl Into<UpdatePaymentMethodConfigurationUsBankAccount>,
    ) -> Self {
        self.inner.us_bank_account = Some(us_bank_account.into());
        self
    }
    /// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
    /// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
    /// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
    /// Check this [page](https://docs.stripe.com/payments/wechat-pay) for more details.
    pub fn wechat_pay(
        mut self,
        wechat_pay: impl Into<UpdatePaymentMethodConfigurationWechatPay>,
    ) -> Self {
        self.inner.wechat_pay = Some(wechat_pay.into());
        self
    }
    /// Zip gives your customers a way to split purchases over a series of payments.
    /// Check this [page](https://docs.stripe.com/payments/zip) for more details like country availability.
    pub fn zip(mut self, zip: impl Into<UpdatePaymentMethodConfigurationZip>) -> Self {
        self.inner.zip = Some(zip.into());
        self
    }
}
impl UpdatePaymentMethodConfiguration {
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

impl StripeRequest for UpdatePaymentMethodConfiguration {
    type Output = stripe_payment::PaymentMethodConfiguration;

    fn build(&self) -> RequestBuilder {
        let configuration = &self.configuration;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/payment_method_configurations/{configuration}"),
        )
        .form(&self.inner)
    }
}
