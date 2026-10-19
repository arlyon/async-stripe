use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListPaymentMethodConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListPaymentMethodConfigurationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListPaymentMethodConfigurationBuilder").finish_non_exhaustive()
    }
}
impl ListPaymentMethodConfigurationBuilder {
    fn new() -> Self {
        Self {
            active: None,
            application: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// List payment method configurations
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListPaymentMethodConfiguration {
    inner: ListPaymentMethodConfigurationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListPaymentMethodConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListPaymentMethodConfiguration").finish_non_exhaustive()
    }
}
impl ListPaymentMethodConfiguration {
    /// Construct a new `ListPaymentMethodConfiguration`.
    pub fn new() -> Self {
        Self { inner: ListPaymentMethodConfigurationBuilder::new() }
    }
    /// Whether the configuration is active.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrievePaymentMethodConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrievePaymentMethodConfigurationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrievePaymentMethodConfigurationBuilder").finish_non_exhaustive()
    }
}
impl RetrievePaymentMethodConfigurationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve payment method configuration
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrievePaymentMethodConfiguration {
    inner: RetrievePaymentMethodConfigurationBuilder,
    configuration: stripe_payment::PaymentMethodConfigurationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrievePaymentMethodConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrievePaymentMethodConfiguration").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreatePaymentMethodConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    acss_debit: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    affirm: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    afterpay_clearpay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alipay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alma: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_pay_later: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    au_becs_debit: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bacs_debit: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bancontact: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billie: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bizum: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blik: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boleto: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cartes_bancaires: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cashapp: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crypto: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_balance: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eps: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fpx: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fr_meal_voucher_conecs: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    giropay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grabpay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ideal: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jcb: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kakao_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    klarna: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    konbini: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kr_card: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mb_way: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobilepay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multibanco: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    naver_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nz_bank_account: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oxxo: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    p24: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay_by_bank: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payco: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paynow: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paypal: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payto: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pix: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promptpay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revolut_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    samsung_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    satispay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scalapay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sepa_debit: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sofort: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sunbit: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swish: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    twint: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upi: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    us_bank_account: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wechat_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip: Option<PaymentMethodParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentMethodConfigurationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentMethodConfigurationBuilder").finish_non_exhaustive()
    }
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
            bizum: None,
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
            scalapay: None,
            sepa_debit: None,
            sofort: None,
            sunbit: None,
            swish: None,
            twint: None,
            upi: None,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// Creates a payment method configuration
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreatePaymentMethodConfiguration {
    inner: CreatePaymentMethodConfigurationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentMethodConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreatePaymentMethodConfiguration").finish_non_exhaustive()
    }
}
impl CreatePaymentMethodConfiguration {
    /// Construct a new `CreatePaymentMethodConfiguration`.
    pub fn new() -> Self {
        Self { inner: CreatePaymentMethodConfigurationBuilder::new() }
    }
    /// Canadian pre-authorized debit payments, check this [page](https://docs.stripe.com/payments/acss-debit) for more details like country availability.
    pub fn acss_debit(mut self, acss_debit: impl Into<PaymentMethodParam>) -> Self {
        self.inner.acss_debit = Some(acss_debit.into());
        self
    }
    /// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
    /// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
    /// Check this [page](https://docs.stripe.com/payments/affirm) for more details like country availability.
    pub fn affirm(mut self, affirm: impl Into<PaymentMethodParam>) -> Self {
        self.inner.affirm = Some(affirm.into());
        self
    }
    /// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://docs.stripe.com/payments/afterpay-clearpay) for more details like country availability.
    /// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
    pub fn afterpay_clearpay(mut self, afterpay_clearpay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.afterpay_clearpay = Some(afterpay_clearpay.into());
        self
    }
    /// Alipay is a digital wallet in China that has more than a billion active users worldwide.
    /// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
    /// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
    /// Check this [page](https://docs.stripe.com/payments/alipay) for more details.
    pub fn alipay(mut self, alipay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.alipay = Some(alipay.into());
        self
    }
    /// Alma is a Buy Now, Pay Later payment method that offers customers the ability to pay in 2, 3, or 4 installments.
    pub fn alma(mut self, alma: impl Into<PaymentMethodParam>) -> Self {
        self.inner.alma = Some(alma.into());
        self
    }
    /// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
    pub fn amazon_pay(mut self, amazon_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.amazon_pay = Some(amazon_pay.into());
        self
    }
    /// Stripe users can accept [Apple Pay](https://stripe.com/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
    /// There are no additional fees to process Apple Pay payments, and the [pricing](https://stripe.com/pricing) is the same as other card transactions.
    /// Check this [page](https://docs.stripe.com/apple-pay) for more details.
    pub fn apple_pay(mut self, apple_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.apple_pay = Some(apple_pay.into());
        self
    }
    /// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
    pub fn apple_pay_later(mut self, apple_pay_later: impl Into<PaymentMethodParam>) -> Self {
        self.inner.apple_pay_later = Some(apple_pay_later.into());
        self
    }
    /// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
    /// Check this [page](https://docs.stripe.com/payments/au-becs-debit) for more details.
    pub fn au_becs_debit(mut self, au_becs_debit: impl Into<PaymentMethodParam>) -> Self {
        self.inner.au_becs_debit = Some(au_becs_debit.into());
        self
    }
    /// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://docs.stripe.com/payments/payment-methods/bacs-debit) for more details.
    pub fn bacs_debit(mut self, bacs_debit: impl Into<PaymentMethodParam>) -> Self {
        self.inner.bacs_debit = Some(bacs_debit.into());
        self
    }
    /// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
    /// [Customers](https://docs.stripe.com/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
    /// Check this [page](https://docs.stripe.com/payments/bancontact) for more details.
    pub fn bancontact(mut self, bancontact: impl Into<PaymentMethodParam>) -> Self {
        self.inner.bancontact = Some(bancontact.into());
        self
    }
    /// Billie is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers businesses Pay by Invoice where they offer payment terms ranging from 7-120 days.
    /// Customers are redirected from your website or app, authorize the payment with Billie, then return to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    pub fn billie(mut self, billie: impl Into<PaymentMethodParam>) -> Self {
        self.inner.billie = Some(billie.into());
        self
    }
    /// To enable Bizum, buyers need a Spanish IBAN from a bank connected to Bizum.
    /// Within their banking app, they can enable Bizum and link their mobile number to their IBAN.
    pub fn bizum(mut self, bizum: impl Into<PaymentMethodParam>) -> Self {
        self.inner.bizum = Some(bizum.into());
        self
    }
    /// BLIK is a [single use](https://docs.stripe.com/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
    /// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
    /// Check this [page](https://docs.stripe.com/payments/blik) for more details.
    pub fn blik(mut self, blik: impl Into<PaymentMethodParam>) -> Self {
        self.inner.blik = Some(blik.into());
        self
    }
    /// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
    /// Check this [page](https://docs.stripe.com/payments/boleto) for more details.
    pub fn boleto(mut self, boleto: impl Into<PaymentMethodParam>) -> Self {
        self.inner.boleto = Some(boleto.into());
        self
    }
    /// Cards are a popular way for consumers and businesses to pay online or in person.
    /// Stripe supports global and local card networks.
    pub fn card(mut self, card: impl Into<PaymentMethodParam>) -> Self {
        self.inner.card = Some(card.into());
        self
    }
    /// Cartes Bancaires is France's local card network.
    /// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
    /// Check this [page](https://docs.stripe.com/payments/cartes-bancaires) for more details.
    pub fn cartes_bancaires(mut self, cartes_bancaires: impl Into<PaymentMethodParam>) -> Self {
        self.inner.cartes_bancaires = Some(cartes_bancaires.into());
        self
    }
    /// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
    /// Check this [page](https://docs.stripe.com/payments/cash-app-pay) for more details.
    pub fn cashapp(mut self, cashapp: impl Into<PaymentMethodParam>) -> Self {
        self.inner.cashapp = Some(cashapp.into());
        self
    }
    /// [Stablecoin payments](https://docs.stripe.com/payments/stablecoin-payments) enable customers to pay in stablecoins like USDC from 100s of wallets including Phantom and Metamask.
    pub fn crypto(mut self, crypto: impl Into<PaymentMethodParam>) -> Self {
        self.inner.crypto = Some(crypto.into());
        self
    }
    /// Uses a customer’s [cash balance](https://docs.stripe.com/payments/customer-balance) for the payment.
    /// The cash balance can be funded via a bank transfer.
    /// Check this [page](https://docs.stripe.com/payments/bank-transfers) for more details.
    pub fn customer_balance(mut self, customer_balance: impl Into<PaymentMethodParam>) -> Self {
        self.inner.customer_balance = Some(customer_balance.into());
        self
    }
    /// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
    /// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
    /// Check this [page](https://docs.stripe.com/payments/eps) for more details.
    pub fn eps(mut self, eps: impl Into<PaymentMethodParam>) -> Self {
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
    pub fn fpx(mut self, fpx: impl Into<PaymentMethodParam>) -> Self {
        self.inner.fpx = Some(fpx.into());
        self
    }
    /// Meal vouchers in France, or “titres-restaurant”, is a local benefits program commonly offered by employers for their employees to purchase prepared food and beverages on working days.
    /// Check this [page](https://docs.stripe.com/payments/meal-vouchers/fr-meal-vouchers) for more details.
    pub fn fr_meal_voucher_conecs(
        mut self,
        fr_meal_voucher_conecs: impl Into<PaymentMethodParam>,
    ) -> Self {
        self.inner.fr_meal_voucher_conecs = Some(fr_meal_voucher_conecs.into());
        self
    }
    /// giropay is a German payment method based on online banking, introduced in 2006.
    /// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
    /// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
    /// giropay accounts for 10% of online checkouts in Germany.
    /// Check this [page](https://docs.stripe.com/payments/giropay) for more details.
    pub fn giropay(mut self, giropay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.giropay = Some(giropay.into());
        self
    }
    /// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
    /// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
    /// Check this [page](https://docs.stripe.com/google-pay) for more details.
    pub fn google_pay(mut self, google_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.google_pay = Some(google_pay.into());
        self
    }
    /// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
    /// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
    /// Check this [page](https://docs.stripe.com/payments/grabpay) for more details.
    pub fn grabpay(mut self, grabpay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.grabpay = Some(grabpay.into());
        self
    }
    /// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
    /// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
    /// Check this [page](https://docs.stripe.com/payments/ideal) for more details.
    pub fn ideal(mut self, ideal: impl Into<PaymentMethodParam>) -> Self {
        self.inner.ideal = Some(ideal.into());
        self
    }
    /// JCB is a credit card company based in Japan.
    /// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
    /// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
    pub fn jcb(mut self, jcb: impl Into<PaymentMethodParam>) -> Self {
        self.inner.jcb = Some(jcb.into());
        self
    }
    /// Kakao Pay is a popular local wallet available in South Korea.
    pub fn kakao_pay(mut self, kakao_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.kakao_pay = Some(kakao_pay.into());
        self
    }
    /// Klarna gives customers a range of [payment options](https://docs.stripe.com/payments/klarna#payment-options) during checkout.
    /// Available payment options vary depending on the customer's billing address and the transaction amount.
    /// These payment options make it convenient for customers to purchase items in all price ranges.
    /// Check this [page](https://docs.stripe.com/payments/klarna) for more details.
    pub fn klarna(mut self, klarna: impl Into<PaymentMethodParam>) -> Self {
        self.inner.klarna = Some(klarna.into());
        self
    }
    /// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
    /// Check this [page](https://docs.stripe.com/payments/konbini) for more details.
    pub fn konbini(mut self, konbini: impl Into<PaymentMethodParam>) -> Self {
        self.inner.konbini = Some(konbini.into());
        self
    }
    /// Korean cards let users pay using locally issued cards from South Korea.
    pub fn kr_card(mut self, kr_card: impl Into<PaymentMethodParam>) -> Self {
        self.inner.kr_card = Some(kr_card.into());
        self
    }
    /// [Link](https://docs.stripe.com/payments/link) is a payment method network.
    /// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
    pub fn link(mut self, link: impl Into<PaymentMethodParam>) -> Self {
        self.inner.link = Some(link.into());
        self
    }
    /// MB WAY is the most popular wallet in Portugal.
    /// After entering their phone number in your checkout, customers approve the payment directly in their MB WAY app.
    /// Check this [page](https://docs.stripe.com/payments/mb-way) for more details.
    pub fn mb_way(mut self, mb_way: impl Into<PaymentMethodParam>) -> Self {
        self.inner.mb_way = Some(mb_way.into());
        self
    }
    /// MobilePay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
    /// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the MobilePay app.
    /// Check this [page](https://docs.stripe.com/payments/mobilepay) for more details.
    pub fn mobilepay(mut self, mobilepay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.mobilepay = Some(mobilepay.into());
        self
    }
    /// Stripe users in Europe and the United States can accept Multibanco payments from customers in Portugal using [Sources](https://stripe.com/docs/sources)—a single integration path for creating payments using any supported method.
    pub fn multibanco(mut self, multibanco: impl Into<PaymentMethodParam>) -> Self {
        self.inner.multibanco = Some(multibanco.into());
        self
    }
    /// Configuration name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
    /// Naver Pay is a popular local wallet available in South Korea.
    pub fn naver_pay(mut self, naver_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.naver_pay = Some(naver_pay.into());
        self
    }
    /// Stripe users in New Zealand can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with a New Zeland bank account.
    /// Check this [page](https://docs.stripe.com/payments/nz-bank-account) for more details.
    pub fn nz_bank_account(mut self, nz_bank_account: impl Into<PaymentMethodParam>) -> Self {
        self.inner.nz_bank_account = Some(nz_bank_account.into());
        self
    }
    /// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
    /// OXXO allows customers to pay bills and online purchases in-store with cash.
    /// Check this [page](https://docs.stripe.com/payments/oxxo) for more details.
    pub fn oxxo(mut self, oxxo: impl Into<PaymentMethodParam>) -> Self {
        self.inner.oxxo = Some(oxxo.into());
        self
    }
    /// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
    /// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
    /// Check this [page](https://docs.stripe.com/payments/p24) for more details.
    pub fn p24(mut self, p24: impl Into<PaymentMethodParam>) -> Self {
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
    pub fn pay_by_bank(mut self, pay_by_bank: impl Into<PaymentMethodParam>) -> Self {
        self.inner.pay_by_bank = Some(pay_by_bank.into());
        self
    }
    /// PAYCO is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
    pub fn payco(mut self, payco: impl Into<PaymentMethodParam>) -> Self {
        self.inner.payco = Some(payco.into());
        self
    }
    /// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
    /// Check this [page](https://docs.stripe.com/payments/paynow) for more details.
    pub fn paynow(mut self, paynow: impl Into<PaymentMethodParam>) -> Self {
        self.inner.paynow = Some(paynow.into());
        self
    }
    /// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
    /// Check this [page](https://docs.stripe.com/payments/paypal) for more details.
    pub fn paypal(mut self, paypal: impl Into<PaymentMethodParam>) -> Self {
        self.inner.paypal = Some(paypal.into());
        self
    }
    /// PayTo is a [real-time](https://docs.stripe.com/payments/real-time) payment method that enables customers in Australia to pay by providing their bank account details.
    /// Customers must accept a mandate authorizing you to debit their account.
    /// Check this [page](https://docs.stripe.com/payments/payto) for more details.
    pub fn payto(mut self, payto: impl Into<PaymentMethodParam>) -> Self {
        self.inner.payto = Some(payto.into());
        self
    }
    /// Pix is a payment method popular in Brazil.
    /// When paying with Pix, customers authenticate and approve payments by scanning a QR code in their preferred banking app.
    /// Check this [page](https://docs.stripe.com/payments/pix) for more details.
    pub fn pix(mut self, pix: impl Into<PaymentMethodParam>) -> Self {
        self.inner.pix = Some(pix.into());
        self
    }
    /// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
    /// Check this [page](https://docs.stripe.com/payments/promptpay) for more details.
    pub fn promptpay(mut self, promptpay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.promptpay = Some(promptpay.into());
        self
    }
    /// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
    /// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
    pub fn revolut_pay(mut self, revolut_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.revolut_pay = Some(revolut_pay.into());
        self
    }
    /// Samsung Pay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
    pub fn samsung_pay(mut self, samsung_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.samsung_pay = Some(samsung_pay.into());
        self
    }
    /// Satispay is a [single-use](/payments/payment-methods#usage) payment method where customers are required to [authenticate](/payments/payment-methods#customer-actions) their payment.
    /// Customers pay by being redirected from your website or app, authorizing the payment with Satispay, then returning to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    pub fn satispay(mut self, satispay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.satispay = Some(satispay.into());
        self
    }
    /// Scalapay is a [single-use](/payments/payment-methods#usage) payment method that lets customers pay in 3 or 4 installments.
    /// Customers are redirected from your website or app, authorize the payment with Scalapay, then return to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    pub fn scalapay(mut self, scalapay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.scalapay = Some(scalapay.into());
        self
    }
    /// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
    /// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://docs.stripe.com/payments/sepa-debit) for more details.
    pub fn sepa_debit(mut self, sepa_debit: impl Into<PaymentMethodParam>) -> Self {
        self.inner.sepa_debit = Some(sepa_debit.into());
        self
    }
    /// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
    /// Check this [page](https://docs.stripe.com/payments/sofort) for more details.
    pub fn sofort(mut self, sofort: impl Into<PaymentMethodParam>) -> Self {
        self.inner.sofort = Some(sofort.into());
        self
    }
    /// Sunbit is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method where customers choose to pay in 3, 6, or 12 installments.
    /// Customers are redirected from your website or app, authorize the payment with Sunbit, then return to your website or app.
    /// You get [immediate notification](https://docs.stripe.com/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    pub fn sunbit(mut self, sunbit: impl Into<PaymentMethodParam>) -> Self {
        self.inner.sunbit = Some(sunbit.into());
        self
    }
    /// Swish is a [real-time](https://docs.stripe.com/payments/real-time) payment method popular in Sweden.
    /// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
    /// Check this [page](https://docs.stripe.com/payments/swish) for more details.
    pub fn swish(mut self, swish: impl Into<PaymentMethodParam>) -> Self {
        self.inner.swish = Some(swish.into());
        self
    }
    /// Twint is a payment method popular in Switzerland.
    /// It allows customers to pay using their mobile phone.
    /// Check this [page](https://docs.stripe.com/payments/twint) for more details.
    pub fn twint(mut self, twint: impl Into<PaymentMethodParam>) -> Self {
        self.inner.twint = Some(twint.into());
        self
    }
    /// Unified Payment Interface (UPI) is India's leading payment method with exponential growth since it launched in 2016.
    pub fn upi(mut self, upi: impl Into<PaymentMethodParam>) -> Self {
        self.inner.upi = Some(upi.into());
        self
    }
    /// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
    /// Check this [page](https://docs.stripe.com/payments/ach-direct-debit) for more details.
    pub fn us_bank_account(mut self, us_bank_account: impl Into<PaymentMethodParam>) -> Self {
        self.inner.us_bank_account = Some(us_bank_account.into());
        self
    }
    /// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
    /// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
    /// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
    /// Check this [page](https://docs.stripe.com/payments/wechat-pay) for more details.
    pub fn wechat_pay(mut self, wechat_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.wechat_pay = Some(wechat_pay.into());
        self
    }
    /// Zip gives your customers a way to split purchases over a series of payments.
    /// Check this [page](https://docs.stripe.com/payments/zip) for more details like country availability.
    pub fn zip(mut self, zip: impl Into<PaymentMethodParam>) -> Self {
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdatePaymentMethodConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    acss_debit: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    affirm: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    afterpay_clearpay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alipay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alma: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apple_pay_later: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    au_becs_debit: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bacs_debit: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bancontact: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billie: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bizum: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blik: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boleto: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cartes_bancaires: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cashapp: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crypto: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_balance: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eps: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fpx: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fr_meal_voucher_conecs: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    giropay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grabpay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ideal: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jcb: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kakao_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    klarna: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    konbini: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kr_card: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mb_way: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobilepay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multibanco: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    naver_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nz_bank_account: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oxxo: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    p24: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay_by_bank: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payco: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paynow: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paypal: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payto: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pix: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promptpay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revolut_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    samsung_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    satispay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scalapay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sepa_debit: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sofort: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sunbit: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swish: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    twint: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upi: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    us_bank_account: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wechat_pay: Option<PaymentMethodParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip: Option<PaymentMethodParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentMethodConfigurationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentMethodConfigurationBuilder").finish_non_exhaustive()
    }
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
            bizum: None,
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
            scalapay: None,
            sepa_debit: None,
            sofort: None,
            sunbit: None,
            swish: None,
            twint: None,
            upi: None,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// Update payment method configuration
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdatePaymentMethodConfiguration {
    inner: UpdatePaymentMethodConfigurationBuilder,
    configuration: stripe_payment::PaymentMethodConfigurationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdatePaymentMethodConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdatePaymentMethodConfiguration").finish_non_exhaustive()
    }
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
    pub fn acss_debit(mut self, acss_debit: impl Into<PaymentMethodParam>) -> Self {
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
    pub fn affirm(mut self, affirm: impl Into<PaymentMethodParam>) -> Self {
        self.inner.affirm = Some(affirm.into());
        self
    }
    /// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://docs.stripe.com/payments/afterpay-clearpay) for more details like country availability.
    /// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
    pub fn afterpay_clearpay(mut self, afterpay_clearpay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.afterpay_clearpay = Some(afterpay_clearpay.into());
        self
    }
    /// Alipay is a digital wallet in China that has more than a billion active users worldwide.
    /// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
    /// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
    /// Check this [page](https://docs.stripe.com/payments/alipay) for more details.
    pub fn alipay(mut self, alipay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.alipay = Some(alipay.into());
        self
    }
    /// Alma is a Buy Now, Pay Later payment method that offers customers the ability to pay in 2, 3, or 4 installments.
    pub fn alma(mut self, alma: impl Into<PaymentMethodParam>) -> Self {
        self.inner.alma = Some(alma.into());
        self
    }
    /// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
    pub fn amazon_pay(mut self, amazon_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.amazon_pay = Some(amazon_pay.into());
        self
    }
    /// Stripe users can accept [Apple Pay](https://stripe.com/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
    /// There are no additional fees to process Apple Pay payments, and the [pricing](https://stripe.com/pricing) is the same as other card transactions.
    /// Check this [page](https://docs.stripe.com/apple-pay) for more details.
    pub fn apple_pay(mut self, apple_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.apple_pay = Some(apple_pay.into());
        self
    }
    /// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
    pub fn apple_pay_later(mut self, apple_pay_later: impl Into<PaymentMethodParam>) -> Self {
        self.inner.apple_pay_later = Some(apple_pay_later.into());
        self
    }
    /// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
    /// Check this [page](https://docs.stripe.com/payments/au-becs-debit) for more details.
    pub fn au_becs_debit(mut self, au_becs_debit: impl Into<PaymentMethodParam>) -> Self {
        self.inner.au_becs_debit = Some(au_becs_debit.into());
        self
    }
    /// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://docs.stripe.com/payments/payment-methods/bacs-debit) for more details.
    pub fn bacs_debit(mut self, bacs_debit: impl Into<PaymentMethodParam>) -> Self {
        self.inner.bacs_debit = Some(bacs_debit.into());
        self
    }
    /// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
    /// [Customers](https://docs.stripe.com/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
    /// Check this [page](https://docs.stripe.com/payments/bancontact) for more details.
    pub fn bancontact(mut self, bancontact: impl Into<PaymentMethodParam>) -> Self {
        self.inner.bancontact = Some(bancontact.into());
        self
    }
    /// Billie is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers businesses Pay by Invoice where they offer payment terms ranging from 7-120 days.
    /// Customers are redirected from your website or app, authorize the payment with Billie, then return to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    pub fn billie(mut self, billie: impl Into<PaymentMethodParam>) -> Self {
        self.inner.billie = Some(billie.into());
        self
    }
    /// To enable Bizum, buyers need a Spanish IBAN from a bank connected to Bizum.
    /// Within their banking app, they can enable Bizum and link their mobile number to their IBAN.
    pub fn bizum(mut self, bizum: impl Into<PaymentMethodParam>) -> Self {
        self.inner.bizum = Some(bizum.into());
        self
    }
    /// BLIK is a [single use](https://docs.stripe.com/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
    /// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
    /// Check this [page](https://docs.stripe.com/payments/blik) for more details.
    pub fn blik(mut self, blik: impl Into<PaymentMethodParam>) -> Self {
        self.inner.blik = Some(blik.into());
        self
    }
    /// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
    /// Check this [page](https://docs.stripe.com/payments/boleto) for more details.
    pub fn boleto(mut self, boleto: impl Into<PaymentMethodParam>) -> Self {
        self.inner.boleto = Some(boleto.into());
        self
    }
    /// Cards are a popular way for consumers and businesses to pay online or in person.
    /// Stripe supports global and local card networks.
    pub fn card(mut self, card: impl Into<PaymentMethodParam>) -> Self {
        self.inner.card = Some(card.into());
        self
    }
    /// Cartes Bancaires is France's local card network.
    /// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
    /// Check this [page](https://docs.stripe.com/payments/cartes-bancaires) for more details.
    pub fn cartes_bancaires(mut self, cartes_bancaires: impl Into<PaymentMethodParam>) -> Self {
        self.inner.cartes_bancaires = Some(cartes_bancaires.into());
        self
    }
    /// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
    /// Check this [page](https://docs.stripe.com/payments/cash-app-pay) for more details.
    pub fn cashapp(mut self, cashapp: impl Into<PaymentMethodParam>) -> Self {
        self.inner.cashapp = Some(cashapp.into());
        self
    }
    /// [Stablecoin payments](https://docs.stripe.com/payments/stablecoin-payments) enable customers to pay in stablecoins like USDC from 100s of wallets including Phantom and Metamask.
    pub fn crypto(mut self, crypto: impl Into<PaymentMethodParam>) -> Self {
        self.inner.crypto = Some(crypto.into());
        self
    }
    /// Uses a customer’s [cash balance](https://docs.stripe.com/payments/customer-balance) for the payment.
    /// The cash balance can be funded via a bank transfer.
    /// Check this [page](https://docs.stripe.com/payments/bank-transfers) for more details.
    pub fn customer_balance(mut self, customer_balance: impl Into<PaymentMethodParam>) -> Self {
        self.inner.customer_balance = Some(customer_balance.into());
        self
    }
    /// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
    /// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
    /// Check this [page](https://docs.stripe.com/payments/eps) for more details.
    pub fn eps(mut self, eps: impl Into<PaymentMethodParam>) -> Self {
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
    pub fn fpx(mut self, fpx: impl Into<PaymentMethodParam>) -> Self {
        self.inner.fpx = Some(fpx.into());
        self
    }
    /// Meal vouchers in France, or “titres-restaurant”, is a local benefits program commonly offered by employers for their employees to purchase prepared food and beverages on working days.
    /// Check this [page](https://docs.stripe.com/payments/meal-vouchers/fr-meal-vouchers) for more details.
    pub fn fr_meal_voucher_conecs(
        mut self,
        fr_meal_voucher_conecs: impl Into<PaymentMethodParam>,
    ) -> Self {
        self.inner.fr_meal_voucher_conecs = Some(fr_meal_voucher_conecs.into());
        self
    }
    /// giropay is a German payment method based on online banking, introduced in 2006.
    /// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
    /// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
    /// giropay accounts for 10% of online checkouts in Germany.
    /// Check this [page](https://docs.stripe.com/payments/giropay) for more details.
    pub fn giropay(mut self, giropay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.giropay = Some(giropay.into());
        self
    }
    /// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
    /// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
    /// Check this [page](https://docs.stripe.com/google-pay) for more details.
    pub fn google_pay(mut self, google_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.google_pay = Some(google_pay.into());
        self
    }
    /// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
    /// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
    /// Check this [page](https://docs.stripe.com/payments/grabpay) for more details.
    pub fn grabpay(mut self, grabpay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.grabpay = Some(grabpay.into());
        self
    }
    /// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
    /// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
    /// Check this [page](https://docs.stripe.com/payments/ideal) for more details.
    pub fn ideal(mut self, ideal: impl Into<PaymentMethodParam>) -> Self {
        self.inner.ideal = Some(ideal.into());
        self
    }
    /// JCB is a credit card company based in Japan.
    /// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
    /// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
    pub fn jcb(mut self, jcb: impl Into<PaymentMethodParam>) -> Self {
        self.inner.jcb = Some(jcb.into());
        self
    }
    /// Kakao Pay is a popular local wallet available in South Korea.
    pub fn kakao_pay(mut self, kakao_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.kakao_pay = Some(kakao_pay.into());
        self
    }
    /// Klarna gives customers a range of [payment options](https://docs.stripe.com/payments/klarna#payment-options) during checkout.
    /// Available payment options vary depending on the customer's billing address and the transaction amount.
    /// These payment options make it convenient for customers to purchase items in all price ranges.
    /// Check this [page](https://docs.stripe.com/payments/klarna) for more details.
    pub fn klarna(mut self, klarna: impl Into<PaymentMethodParam>) -> Self {
        self.inner.klarna = Some(klarna.into());
        self
    }
    /// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
    /// Check this [page](https://docs.stripe.com/payments/konbini) for more details.
    pub fn konbini(mut self, konbini: impl Into<PaymentMethodParam>) -> Self {
        self.inner.konbini = Some(konbini.into());
        self
    }
    /// Korean cards let users pay using locally issued cards from South Korea.
    pub fn kr_card(mut self, kr_card: impl Into<PaymentMethodParam>) -> Self {
        self.inner.kr_card = Some(kr_card.into());
        self
    }
    /// [Link](https://docs.stripe.com/payments/link) is a payment method network.
    /// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
    pub fn link(mut self, link: impl Into<PaymentMethodParam>) -> Self {
        self.inner.link = Some(link.into());
        self
    }
    /// MB WAY is the most popular wallet in Portugal.
    /// After entering their phone number in your checkout, customers approve the payment directly in their MB WAY app.
    /// Check this [page](https://docs.stripe.com/payments/mb-way) for more details.
    pub fn mb_way(mut self, mb_way: impl Into<PaymentMethodParam>) -> Self {
        self.inner.mb_way = Some(mb_way.into());
        self
    }
    /// MobilePay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
    /// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the MobilePay app.
    /// Check this [page](https://docs.stripe.com/payments/mobilepay) for more details.
    pub fn mobilepay(mut self, mobilepay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.mobilepay = Some(mobilepay.into());
        self
    }
    /// Stripe users in Europe and the United States can accept Multibanco payments from customers in Portugal using [Sources](https://stripe.com/docs/sources)—a single integration path for creating payments using any supported method.
    pub fn multibanco(mut self, multibanco: impl Into<PaymentMethodParam>) -> Self {
        self.inner.multibanco = Some(multibanco.into());
        self
    }
    /// Configuration name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
    /// Naver Pay is a popular local wallet available in South Korea.
    pub fn naver_pay(mut self, naver_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.naver_pay = Some(naver_pay.into());
        self
    }
    /// Stripe users in New Zealand can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with a New Zeland bank account.
    /// Check this [page](https://docs.stripe.com/payments/nz-bank-account) for more details.
    pub fn nz_bank_account(mut self, nz_bank_account: impl Into<PaymentMethodParam>) -> Self {
        self.inner.nz_bank_account = Some(nz_bank_account.into());
        self
    }
    /// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
    /// OXXO allows customers to pay bills and online purchases in-store with cash.
    /// Check this [page](https://docs.stripe.com/payments/oxxo) for more details.
    pub fn oxxo(mut self, oxxo: impl Into<PaymentMethodParam>) -> Self {
        self.inner.oxxo = Some(oxxo.into());
        self
    }
    /// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
    /// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
    /// Check this [page](https://docs.stripe.com/payments/p24) for more details.
    pub fn p24(mut self, p24: impl Into<PaymentMethodParam>) -> Self {
        self.inner.p24 = Some(p24.into());
        self
    }
    /// Pay by bank is a redirect payment method backed by bank transfers.
    /// A customer is redirected to their bank to authorize a bank transfer for a given amount.
    /// This removes a lot of the error risks inherent in waiting for the customer to initiate a transfer themselves, and is less expensive than card payments.
    pub fn pay_by_bank(mut self, pay_by_bank: impl Into<PaymentMethodParam>) -> Self {
        self.inner.pay_by_bank = Some(pay_by_bank.into());
        self
    }
    /// PAYCO is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
    pub fn payco(mut self, payco: impl Into<PaymentMethodParam>) -> Self {
        self.inner.payco = Some(payco.into());
        self
    }
    /// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
    /// Check this [page](https://docs.stripe.com/payments/paynow) for more details.
    pub fn paynow(mut self, paynow: impl Into<PaymentMethodParam>) -> Self {
        self.inner.paynow = Some(paynow.into());
        self
    }
    /// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
    /// Check this [page](https://docs.stripe.com/payments/paypal) for more details.
    pub fn paypal(mut self, paypal: impl Into<PaymentMethodParam>) -> Self {
        self.inner.paypal = Some(paypal.into());
        self
    }
    /// PayTo is a [real-time](https://docs.stripe.com/payments/real-time) payment method that enables customers in Australia to pay by providing their bank account details.
    /// Customers must accept a mandate authorizing you to debit their account.
    /// Check this [page](https://docs.stripe.com/payments/payto) for more details.
    pub fn payto(mut self, payto: impl Into<PaymentMethodParam>) -> Self {
        self.inner.payto = Some(payto.into());
        self
    }
    /// Pix is a payment method popular in Brazil.
    /// When paying with Pix, customers authenticate and approve payments by scanning a QR code in their preferred banking app.
    /// Check this [page](https://docs.stripe.com/payments/pix) for more details.
    pub fn pix(mut self, pix: impl Into<PaymentMethodParam>) -> Self {
        self.inner.pix = Some(pix.into());
        self
    }
    /// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
    /// Check this [page](https://docs.stripe.com/payments/promptpay) for more details.
    pub fn promptpay(mut self, promptpay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.promptpay = Some(promptpay.into());
        self
    }
    /// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
    /// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
    pub fn revolut_pay(mut self, revolut_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.revolut_pay = Some(revolut_pay.into());
        self
    }
    /// Samsung Pay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
    pub fn samsung_pay(mut self, samsung_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.samsung_pay = Some(samsung_pay.into());
        self
    }
    /// Satispay is a [single-use](/payments/payment-methods#usage) payment method where customers are required to [authenticate](/payments/payment-methods#customer-actions) their payment.
    /// Customers pay by being redirected from your website or app, authorizing the payment with Satispay, then returning to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    pub fn satispay(mut self, satispay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.satispay = Some(satispay.into());
        self
    }
    /// Scalapay is a [single-use](/payments/payment-methods#usage) payment method that lets customers pay in 3 or 4 installments.
    /// Customers are redirected from your website or app, authorize the payment with Scalapay, then return to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    pub fn scalapay(mut self, scalapay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.scalapay = Some(scalapay.into());
        self
    }
    /// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
    /// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://docs.stripe.com/payments/sepa-debit) for more details.
    pub fn sepa_debit(mut self, sepa_debit: impl Into<PaymentMethodParam>) -> Self {
        self.inner.sepa_debit = Some(sepa_debit.into());
        self
    }
    /// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
    /// Check this [page](https://docs.stripe.com/payments/sofort) for more details.
    pub fn sofort(mut self, sofort: impl Into<PaymentMethodParam>) -> Self {
        self.inner.sofort = Some(sofort.into());
        self
    }
    /// Sunbit is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method where customers choose to pay in 3, 6, or 12 installments.
    /// Customers are redirected from your website or app, authorize the payment with Sunbit, then return to your website or app.
    /// You get [immediate notification](https://docs.stripe.com/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    pub fn sunbit(mut self, sunbit: impl Into<PaymentMethodParam>) -> Self {
        self.inner.sunbit = Some(sunbit.into());
        self
    }
    /// Swish is a [real-time](https://docs.stripe.com/payments/real-time) payment method popular in Sweden.
    /// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
    /// Check this [page](https://docs.stripe.com/payments/swish) for more details.
    pub fn swish(mut self, swish: impl Into<PaymentMethodParam>) -> Self {
        self.inner.swish = Some(swish.into());
        self
    }
    /// Twint is a payment method popular in Switzerland.
    /// It allows customers to pay using their mobile phone.
    /// Check this [page](https://docs.stripe.com/payments/twint) for more details.
    pub fn twint(mut self, twint: impl Into<PaymentMethodParam>) -> Self {
        self.inner.twint = Some(twint.into());
        self
    }
    /// Unified Payment Interface (UPI) is India's leading payment method with exponential growth since it launched in 2016.
    pub fn upi(mut self, upi: impl Into<PaymentMethodParam>) -> Self {
        self.inner.upi = Some(upi.into());
        self
    }
    /// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
    /// Check this [page](https://docs.stripe.com/payments/ach-direct-debit) for more details.
    pub fn us_bank_account(mut self, us_bank_account: impl Into<PaymentMethodParam>) -> Self {
        self.inner.us_bank_account = Some(us_bank_account.into());
        self
    }
    /// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
    /// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
    /// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
    /// Check this [page](https://docs.stripe.com/payments/wechat-pay) for more details.
    pub fn wechat_pay(mut self, wechat_pay: impl Into<PaymentMethodParam>) -> Self {
        self.inner.wechat_pay = Some(wechat_pay.into());
        self
    }
    /// Zip gives your customers a way to split purchases over a series of payments.
    /// Check this [page](https://docs.stripe.com/payments/zip) for more details like country availability.
    pub fn zip(mut self, zip: impl Into<PaymentMethodParam>) -> Self {
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

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DisplayPreferenceParam {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<stripe_shared::Preference>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisplayPreferenceParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisplayPreferenceParam").finish_non_exhaustive()
    }
}
impl DisplayPreferenceParam {
    pub fn new() -> Self {
        Self { preference: None }
    }
}
impl Default for DisplayPreferenceParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PaymentMethodParam {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<DisplayPreferenceParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodParam").finish_non_exhaustive()
    }
}
impl PaymentMethodParam {
    pub fn new() -> Self {
        Self { display_preference: None }
    }
}
impl Default for PaymentMethodParam {
    fn default() -> Self {
        Self::new()
    }
}
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationBillieDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationBizumDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationBlikDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationCardDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationCashappDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationCryptoDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationEpsDisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationFpxDisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationIdealDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationJcbDisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationKrCardDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationLinkDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationMbWayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationNaverPayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationP24DisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationPaycoDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationPaytoDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationPixDisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationScalapayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationSofortDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationSunbitDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationSwishDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationTwintDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationUpiDisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type CreatePaymentMethodConfigurationZipDisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationBizumDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationCardDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationCryptoDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationKakaoPayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationKrCardDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationMbWayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationNaverPayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationP24DisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationPaycoDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationPaytoDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationPixDisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationSamsungPayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationScalapayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationSunbitDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationUpiDisplayPreferencePreference = stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference =
    stripe_shared::Preference;
/// The account's preference for whether or not to display this payment method.
pub type UpdatePaymentMethodConfigurationZipDisplayPreferencePreference = stripe_shared::Preference;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationAcssDebitDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationAffirmDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationAlipayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationAlmaDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationAmazonPayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationApplePayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationApplePayLaterDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationAuBecsDebitDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationBacsDebitDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationBancontactDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationBillieDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationBizumDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationBlikDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationBoletoDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationCardDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationCartesBancairesDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationCashappDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationCryptoDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationCustomerBalanceDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationEpsDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationFpxDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreference =
    DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationGiropayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationGooglePayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationGrabpayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationIdealDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationJcbDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationKakaoPayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationKlarnaDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationKonbiniDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationKrCardDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationLinkDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationMbWayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationMobilepayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationMultibancoDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationNaverPayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationNzBankAccountDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationOxxoDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationP24DisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationPayByBankDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationPaycoDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationPaynowDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationPaypalDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationPaytoDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationPixDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationPromptpayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationRevolutPayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationSamsungPayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationSatispayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationScalapayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationSepaDebitDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationSofortDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationSunbitDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationSwishDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationTwintDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationUpiDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationUsBankAccountDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationWechatPayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type CreatePaymentMethodConfigurationZipDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationAcssDebitDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationAffirmDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationAlipayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationAlmaDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationAmazonPayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationApplePayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationApplePayLaterDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationBacsDebitDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationBancontactDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationBillieDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationBizumDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationBlikDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationBoletoDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationCardDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationCartesBancairesDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationCashappDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationCryptoDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationEpsDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationFpxDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationFrMealVoucherConecsDisplayPreference =
    DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationGiropayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationGooglePayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationGrabpayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationIdealDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationJcbDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationKakaoPayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationKlarnaDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationKonbiniDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationKrCardDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationLinkDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationMbWayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationMobilepayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationMultibancoDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationNaverPayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationNzBankAccountDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationOxxoDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationP24DisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationPayByBankDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationPaycoDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationPaynowDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationPaypalDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationPaytoDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationPixDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationPromptpayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationRevolutPayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationSamsungPayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationSatispayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationScalapayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationSepaDebitDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationSofortDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationSunbitDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationSwishDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationTwintDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationUpiDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationUsBankAccountDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationWechatPayDisplayPreference = DisplayPreferenceParam;
/// Whether or not the payment method should be displayed.
pub type UpdatePaymentMethodConfigurationZipDisplayPreference = DisplayPreferenceParam;
/// Canadian pre-authorized debit payments, check this [page](https://docs.stripe.com/payments/acss-debit) for more details like country availability.
pub type CreatePaymentMethodConfigurationAcssDebit = PaymentMethodParam;
/// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
/// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
/// Check this [page](https://docs.stripe.com/payments/affirm) for more details like country availability.
pub type CreatePaymentMethodConfigurationAffirm = PaymentMethodParam;
/// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://docs.stripe.com/payments/afterpay-clearpay) for more details like country availability.
/// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
pub type CreatePaymentMethodConfigurationAfterpayClearpay = PaymentMethodParam;
/// Alipay is a digital wallet in China that has more than a billion active users worldwide.
/// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
/// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
/// Check this [page](https://docs.stripe.com/payments/alipay) for more details.
pub type CreatePaymentMethodConfigurationAlipay = PaymentMethodParam;
/// Alma is a Buy Now, Pay Later payment method that offers customers the ability to pay in 2, 3, or 4 installments.
pub type CreatePaymentMethodConfigurationAlma = PaymentMethodParam;
/// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
pub type CreatePaymentMethodConfigurationAmazonPay = PaymentMethodParam;
/// Stripe users can accept [Apple Pay](https://stripe.com/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
/// There are no additional fees to process Apple Pay payments, and the [pricing](https://stripe.com/pricing) is the same as other card transactions.
/// Check this [page](https://docs.stripe.com/apple-pay) for more details.
pub type CreatePaymentMethodConfigurationApplePay = PaymentMethodParam;
/// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
pub type CreatePaymentMethodConfigurationApplePayLater = PaymentMethodParam;
/// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
/// Check this [page](https://docs.stripe.com/payments/au-becs-debit) for more details.
pub type CreatePaymentMethodConfigurationAuBecsDebit = PaymentMethodParam;
/// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://docs.stripe.com/payments/payment-methods/bacs-debit) for more details.
pub type CreatePaymentMethodConfigurationBacsDebit = PaymentMethodParam;
/// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
/// [Customers](https://docs.stripe.com/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
/// Check this [page](https://docs.stripe.com/payments/bancontact) for more details.
pub type CreatePaymentMethodConfigurationBancontact = PaymentMethodParam;
/// Billie is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers businesses Pay by Invoice where they offer payment terms ranging from 7-120 days.
/// Customers are redirected from your website or app, authorize the payment with Billie, then return to your website or app.
/// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
pub type CreatePaymentMethodConfigurationBillie = PaymentMethodParam;
/// To enable Bizum, buyers need a Spanish IBAN from a bank connected to Bizum.
/// Within their banking app, they can enable Bizum and link their mobile number to their IBAN.
pub type CreatePaymentMethodConfigurationBizum = PaymentMethodParam;
/// BLIK is a [single use](https://docs.stripe.com/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
/// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
/// Check this [page](https://docs.stripe.com/payments/blik) for more details.
pub type CreatePaymentMethodConfigurationBlik = PaymentMethodParam;
/// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
/// Check this [page](https://docs.stripe.com/payments/boleto) for more details.
pub type CreatePaymentMethodConfigurationBoleto = PaymentMethodParam;
/// Cards are a popular way for consumers and businesses to pay online or in person.
/// Stripe supports global and local card networks.
pub type CreatePaymentMethodConfigurationCard = PaymentMethodParam;
/// Cartes Bancaires is France's local card network.
/// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
/// Check this [page](https://docs.stripe.com/payments/cartes-bancaires) for more details.
pub type CreatePaymentMethodConfigurationCartesBancaires = PaymentMethodParam;
/// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
/// Check this [page](https://docs.stripe.com/payments/cash-app-pay) for more details.
pub type CreatePaymentMethodConfigurationCashapp = PaymentMethodParam;
/// [Stablecoin payments](https://docs.stripe.com/payments/stablecoin-payments) enable customers to pay in stablecoins like USDC from 100s of wallets including Phantom and Metamask.
pub type CreatePaymentMethodConfigurationCrypto = PaymentMethodParam;
/// Uses a customer’s [cash balance](https://docs.stripe.com/payments/customer-balance) for the payment.
/// The cash balance can be funded via a bank transfer.
/// Check this [page](https://docs.stripe.com/payments/bank-transfers) for more details.
pub type CreatePaymentMethodConfigurationCustomerBalance = PaymentMethodParam;
/// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
/// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
/// Check this [page](https://docs.stripe.com/payments/eps) for more details.
pub type CreatePaymentMethodConfigurationEps = PaymentMethodParam;
/// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
/// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
/// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
/// Check this [page](https://docs.stripe.com/payments/fpx) for more details.
pub type CreatePaymentMethodConfigurationFpx = PaymentMethodParam;
/// Meal vouchers in France, or “titres-restaurant”, is a local benefits program commonly offered by employers for their employees to purchase prepared food and beverages on working days.
/// Check this [page](https://docs.stripe.com/payments/meal-vouchers/fr-meal-vouchers) for more details.
pub type CreatePaymentMethodConfigurationFrMealVoucherConecs = PaymentMethodParam;
/// giropay is a German payment method based on online banking, introduced in 2006.
/// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
/// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
/// giropay accounts for 10% of online checkouts in Germany.
/// Check this [page](https://docs.stripe.com/payments/giropay) for more details.
pub type CreatePaymentMethodConfigurationGiropay = PaymentMethodParam;
/// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
/// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
/// Check this [page](https://docs.stripe.com/google-pay) for more details.
pub type CreatePaymentMethodConfigurationGooglePay = PaymentMethodParam;
/// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
/// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
/// Check this [page](https://docs.stripe.com/payments/grabpay) for more details.
pub type CreatePaymentMethodConfigurationGrabpay = PaymentMethodParam;
/// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
/// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
/// Check this [page](https://docs.stripe.com/payments/ideal) for more details.
pub type CreatePaymentMethodConfigurationIdeal = PaymentMethodParam;
/// JCB is a credit card company based in Japan.
/// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
/// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
pub type CreatePaymentMethodConfigurationJcb = PaymentMethodParam;
/// Kakao Pay is a popular local wallet available in South Korea.
pub type CreatePaymentMethodConfigurationKakaoPay = PaymentMethodParam;
/// Klarna gives customers a range of [payment options](https://docs.stripe.com/payments/klarna#payment-options) during checkout.
/// Available payment options vary depending on the customer's billing address and the transaction amount.
/// These payment options make it convenient for customers to purchase items in all price ranges.
/// Check this [page](https://docs.stripe.com/payments/klarna) for more details.
pub type CreatePaymentMethodConfigurationKlarna = PaymentMethodParam;
/// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
/// Check this [page](https://docs.stripe.com/payments/konbini) for more details.
pub type CreatePaymentMethodConfigurationKonbini = PaymentMethodParam;
/// Korean cards let users pay using locally issued cards from South Korea.
pub type CreatePaymentMethodConfigurationKrCard = PaymentMethodParam;
/// [Link](https://docs.stripe.com/payments/link) is a payment method network.
/// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
pub type CreatePaymentMethodConfigurationLink = PaymentMethodParam;
/// MB WAY is the most popular wallet in Portugal.
/// After entering their phone number in your checkout, customers approve the payment directly in their MB WAY app.
/// Check this [page](https://docs.stripe.com/payments/mb-way) for more details.
pub type CreatePaymentMethodConfigurationMbWay = PaymentMethodParam;
/// MobilePay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
/// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the MobilePay app.
/// Check this [page](https://docs.stripe.com/payments/mobilepay) for more details.
pub type CreatePaymentMethodConfigurationMobilepay = PaymentMethodParam;
/// Stripe users in Europe and the United States can accept Multibanco payments from customers in Portugal using [Sources](https://stripe.com/docs/sources)—a single integration path for creating payments using any supported method.
pub type CreatePaymentMethodConfigurationMultibanco = PaymentMethodParam;
/// Naver Pay is a popular local wallet available in South Korea.
pub type CreatePaymentMethodConfigurationNaverPay = PaymentMethodParam;
/// Stripe users in New Zealand can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with a New Zeland bank account.
/// Check this [page](https://docs.stripe.com/payments/nz-bank-account) for more details.
pub type CreatePaymentMethodConfigurationNzBankAccount = PaymentMethodParam;
/// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
/// OXXO allows customers to pay bills and online purchases in-store with cash.
/// Check this [page](https://docs.stripe.com/payments/oxxo) for more details.
pub type CreatePaymentMethodConfigurationOxxo = PaymentMethodParam;
/// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
/// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
/// Check this [page](https://docs.stripe.com/payments/p24) for more details.
pub type CreatePaymentMethodConfigurationP24 = PaymentMethodParam;
/// Pay by bank is a redirect payment method backed by bank transfers.
/// A customer is redirected to their bank to authorize a bank transfer for a given amount.
/// This removes a lot of the error risks inherent in waiting for the customer to initiate a transfer themselves, and is less expensive than card payments.
pub type CreatePaymentMethodConfigurationPayByBank = PaymentMethodParam;
/// PAYCO is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
pub type CreatePaymentMethodConfigurationPayco = PaymentMethodParam;
/// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
/// Check this [page](https://docs.stripe.com/payments/paynow) for more details.
pub type CreatePaymentMethodConfigurationPaynow = PaymentMethodParam;
/// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
/// Check this [page](https://docs.stripe.com/payments/paypal) for more details.
pub type CreatePaymentMethodConfigurationPaypal = PaymentMethodParam;
/// PayTo is a [real-time](https://docs.stripe.com/payments/real-time) payment method that enables customers in Australia to pay by providing their bank account details.
/// Customers must accept a mandate authorizing you to debit their account.
/// Check this [page](https://docs.stripe.com/payments/payto) for more details.
pub type CreatePaymentMethodConfigurationPayto = PaymentMethodParam;
/// Pix is a payment method popular in Brazil.
/// When paying with Pix, customers authenticate and approve payments by scanning a QR code in their preferred banking app.
/// Check this [page](https://docs.stripe.com/payments/pix) for more details.
pub type CreatePaymentMethodConfigurationPix = PaymentMethodParam;
/// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
/// Check this [page](https://docs.stripe.com/payments/promptpay) for more details.
pub type CreatePaymentMethodConfigurationPromptpay = PaymentMethodParam;
/// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
/// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
pub type CreatePaymentMethodConfigurationRevolutPay = PaymentMethodParam;
/// Samsung Pay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
pub type CreatePaymentMethodConfigurationSamsungPay = PaymentMethodParam;
/// Satispay is a [single-use](/payments/payment-methods#usage) payment method where customers are required to [authenticate](/payments/payment-methods#customer-actions) their payment.
/// Customers pay by being redirected from your website or app, authorizing the payment with Satispay, then returning to your website or app.
/// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
pub type CreatePaymentMethodConfigurationSatispay = PaymentMethodParam;
/// Scalapay is a [single-use](/payments/payment-methods#usage) payment method that lets customers pay in 3 or 4 installments.
/// Customers are redirected from your website or app, authorize the payment with Scalapay, then return to your website or app.
/// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
pub type CreatePaymentMethodConfigurationScalapay = PaymentMethodParam;
/// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
/// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://docs.stripe.com/payments/sepa-debit) for more details.
pub type CreatePaymentMethodConfigurationSepaDebit = PaymentMethodParam;
/// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
/// Check this [page](https://docs.stripe.com/payments/sofort) for more details.
pub type CreatePaymentMethodConfigurationSofort = PaymentMethodParam;
/// Sunbit is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method where customers choose to pay in 3, 6, or 12 installments.
/// Customers are redirected from your website or app, authorize the payment with Sunbit, then return to your website or app.
/// You get [immediate notification](https://docs.stripe.com/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
pub type CreatePaymentMethodConfigurationSunbit = PaymentMethodParam;
/// Swish is a [real-time](https://docs.stripe.com/payments/real-time) payment method popular in Sweden.
/// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
/// Check this [page](https://docs.stripe.com/payments/swish) for more details.
pub type CreatePaymentMethodConfigurationSwish = PaymentMethodParam;
/// Twint is a payment method popular in Switzerland.
/// It allows customers to pay using their mobile phone.
/// Check this [page](https://docs.stripe.com/payments/twint) for more details.
pub type CreatePaymentMethodConfigurationTwint = PaymentMethodParam;
/// Unified Payment Interface (UPI) is India's leading payment method with exponential growth since it launched in 2016.
pub type CreatePaymentMethodConfigurationUpi = PaymentMethodParam;
/// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
/// Check this [page](https://docs.stripe.com/payments/ach-direct-debit) for more details.
pub type CreatePaymentMethodConfigurationUsBankAccount = PaymentMethodParam;
/// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
/// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
/// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
/// Check this [page](https://docs.stripe.com/payments/wechat-pay) for more details.
pub type CreatePaymentMethodConfigurationWechatPay = PaymentMethodParam;
/// Zip gives your customers a way to split purchases over a series of payments.
/// Check this [page](https://docs.stripe.com/payments/zip) for more details like country availability.
pub type CreatePaymentMethodConfigurationZip = PaymentMethodParam;
/// Canadian pre-authorized debit payments, check this [page](https://docs.stripe.com/payments/acss-debit) for more details like country availability.
pub type UpdatePaymentMethodConfigurationAcssDebit = PaymentMethodParam;
/// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
/// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
/// Check this [page](https://docs.stripe.com/payments/affirm) for more details like country availability.
pub type UpdatePaymentMethodConfigurationAffirm = PaymentMethodParam;
/// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://docs.stripe.com/payments/afterpay-clearpay) for more details like country availability.
/// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
pub type UpdatePaymentMethodConfigurationAfterpayClearpay = PaymentMethodParam;
/// Alipay is a digital wallet in China that has more than a billion active users worldwide.
/// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
/// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
/// Check this [page](https://docs.stripe.com/payments/alipay) for more details.
pub type UpdatePaymentMethodConfigurationAlipay = PaymentMethodParam;
/// Alma is a Buy Now, Pay Later payment method that offers customers the ability to pay in 2, 3, or 4 installments.
pub type UpdatePaymentMethodConfigurationAlma = PaymentMethodParam;
/// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
pub type UpdatePaymentMethodConfigurationAmazonPay = PaymentMethodParam;
/// Stripe users can accept [Apple Pay](https://stripe.com/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
/// There are no additional fees to process Apple Pay payments, and the [pricing](https://stripe.com/pricing) is the same as other card transactions.
/// Check this [page](https://docs.stripe.com/apple-pay) for more details.
pub type UpdatePaymentMethodConfigurationApplePay = PaymentMethodParam;
/// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
pub type UpdatePaymentMethodConfigurationApplePayLater = PaymentMethodParam;
/// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
/// Check this [page](https://docs.stripe.com/payments/au-becs-debit) for more details.
pub type UpdatePaymentMethodConfigurationAuBecsDebit = PaymentMethodParam;
/// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://docs.stripe.com/payments/payment-methods/bacs-debit) for more details.
pub type UpdatePaymentMethodConfigurationBacsDebit = PaymentMethodParam;
/// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
/// [Customers](https://docs.stripe.com/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
/// Check this [page](https://docs.stripe.com/payments/bancontact) for more details.
pub type UpdatePaymentMethodConfigurationBancontact = PaymentMethodParam;
/// Billie is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers businesses Pay by Invoice where they offer payment terms ranging from 7-120 days.
/// Customers are redirected from your website or app, authorize the payment with Billie, then return to your website or app.
/// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
pub type UpdatePaymentMethodConfigurationBillie = PaymentMethodParam;
/// To enable Bizum, buyers need a Spanish IBAN from a bank connected to Bizum.
/// Within their banking app, they can enable Bizum and link their mobile number to their IBAN.
pub type UpdatePaymentMethodConfigurationBizum = PaymentMethodParam;
/// BLIK is a [single use](https://docs.stripe.com/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
/// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
/// Check this [page](https://docs.stripe.com/payments/blik) for more details.
pub type UpdatePaymentMethodConfigurationBlik = PaymentMethodParam;
/// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
/// Check this [page](https://docs.stripe.com/payments/boleto) for more details.
pub type UpdatePaymentMethodConfigurationBoleto = PaymentMethodParam;
/// Cards are a popular way for consumers and businesses to pay online or in person.
/// Stripe supports global and local card networks.
pub type UpdatePaymentMethodConfigurationCard = PaymentMethodParam;
/// Cartes Bancaires is France's local card network.
/// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
/// Check this [page](https://docs.stripe.com/payments/cartes-bancaires) for more details.
pub type UpdatePaymentMethodConfigurationCartesBancaires = PaymentMethodParam;
/// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
/// Check this [page](https://docs.stripe.com/payments/cash-app-pay) for more details.
pub type UpdatePaymentMethodConfigurationCashapp = PaymentMethodParam;
/// [Stablecoin payments](https://docs.stripe.com/payments/stablecoin-payments) enable customers to pay in stablecoins like USDC from 100s of wallets including Phantom and Metamask.
pub type UpdatePaymentMethodConfigurationCrypto = PaymentMethodParam;
/// Uses a customer’s [cash balance](https://docs.stripe.com/payments/customer-balance) for the payment.
/// The cash balance can be funded via a bank transfer.
/// Check this [page](https://docs.stripe.com/payments/bank-transfers) for more details.
pub type UpdatePaymentMethodConfigurationCustomerBalance = PaymentMethodParam;
/// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
/// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
/// Check this [page](https://docs.stripe.com/payments/eps) for more details.
pub type UpdatePaymentMethodConfigurationEps = PaymentMethodParam;
/// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
/// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
/// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
/// Check this [page](https://docs.stripe.com/payments/fpx) for more details.
pub type UpdatePaymentMethodConfigurationFpx = PaymentMethodParam;
/// Meal vouchers in France, or “titres-restaurant”, is a local benefits program commonly offered by employers for their employees to purchase prepared food and beverages on working days.
/// Check this [page](https://docs.stripe.com/payments/meal-vouchers/fr-meal-vouchers) for more details.
pub type UpdatePaymentMethodConfigurationFrMealVoucherConecs = PaymentMethodParam;
/// giropay is a German payment method based on online banking, introduced in 2006.
/// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
/// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
/// giropay accounts for 10% of online checkouts in Germany.
/// Check this [page](https://docs.stripe.com/payments/giropay) for more details.
pub type UpdatePaymentMethodConfigurationGiropay = PaymentMethodParam;
/// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
/// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
/// Check this [page](https://docs.stripe.com/google-pay) for more details.
pub type UpdatePaymentMethodConfigurationGooglePay = PaymentMethodParam;
/// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
/// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
/// Check this [page](https://docs.stripe.com/payments/grabpay) for more details.
pub type UpdatePaymentMethodConfigurationGrabpay = PaymentMethodParam;
/// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
/// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
/// Check this [page](https://docs.stripe.com/payments/ideal) for more details.
pub type UpdatePaymentMethodConfigurationIdeal = PaymentMethodParam;
/// JCB is a credit card company based in Japan.
/// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
/// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
pub type UpdatePaymentMethodConfigurationJcb = PaymentMethodParam;
/// Kakao Pay is a popular local wallet available in South Korea.
pub type UpdatePaymentMethodConfigurationKakaoPay = PaymentMethodParam;
/// Klarna gives customers a range of [payment options](https://docs.stripe.com/payments/klarna#payment-options) during checkout.
/// Available payment options vary depending on the customer's billing address and the transaction amount.
/// These payment options make it convenient for customers to purchase items in all price ranges.
/// Check this [page](https://docs.stripe.com/payments/klarna) for more details.
pub type UpdatePaymentMethodConfigurationKlarna = PaymentMethodParam;
/// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
/// Check this [page](https://docs.stripe.com/payments/konbini) for more details.
pub type UpdatePaymentMethodConfigurationKonbini = PaymentMethodParam;
/// Korean cards let users pay using locally issued cards from South Korea.
pub type UpdatePaymentMethodConfigurationKrCard = PaymentMethodParam;
/// [Link](https://docs.stripe.com/payments/link) is a payment method network.
/// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
pub type UpdatePaymentMethodConfigurationLink = PaymentMethodParam;
/// MB WAY is the most popular wallet in Portugal.
/// After entering their phone number in your checkout, customers approve the payment directly in their MB WAY app.
/// Check this [page](https://docs.stripe.com/payments/mb-way) for more details.
pub type UpdatePaymentMethodConfigurationMbWay = PaymentMethodParam;
/// MobilePay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
/// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the MobilePay app.
/// Check this [page](https://docs.stripe.com/payments/mobilepay) for more details.
pub type UpdatePaymentMethodConfigurationMobilepay = PaymentMethodParam;
/// Stripe users in Europe and the United States can accept Multibanco payments from customers in Portugal using [Sources](https://stripe.com/docs/sources)—a single integration path for creating payments using any supported method.
pub type UpdatePaymentMethodConfigurationMultibanco = PaymentMethodParam;
/// Naver Pay is a popular local wallet available in South Korea.
pub type UpdatePaymentMethodConfigurationNaverPay = PaymentMethodParam;
/// Stripe users in New Zealand can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with a New Zeland bank account.
/// Check this [page](https://docs.stripe.com/payments/nz-bank-account) for more details.
pub type UpdatePaymentMethodConfigurationNzBankAccount = PaymentMethodParam;
/// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
/// OXXO allows customers to pay bills and online purchases in-store with cash.
/// Check this [page](https://docs.stripe.com/payments/oxxo) for more details.
pub type UpdatePaymentMethodConfigurationOxxo = PaymentMethodParam;
/// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
/// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
/// Check this [page](https://docs.stripe.com/payments/p24) for more details.
pub type UpdatePaymentMethodConfigurationP24 = PaymentMethodParam;
/// Pay by bank is a redirect payment method backed by bank transfers.
/// A customer is redirected to their bank to authorize a bank transfer for a given amount.
/// This removes a lot of the error risks inherent in waiting for the customer to initiate a transfer themselves, and is less expensive than card payments.
pub type UpdatePaymentMethodConfigurationPayByBank = PaymentMethodParam;
/// PAYCO is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
pub type UpdatePaymentMethodConfigurationPayco = PaymentMethodParam;
/// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
/// Check this [page](https://docs.stripe.com/payments/paynow) for more details.
pub type UpdatePaymentMethodConfigurationPaynow = PaymentMethodParam;
/// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
/// Check this [page](https://docs.stripe.com/payments/paypal) for more details.
pub type UpdatePaymentMethodConfigurationPaypal = PaymentMethodParam;
/// PayTo is a [real-time](https://docs.stripe.com/payments/real-time) payment method that enables customers in Australia to pay by providing their bank account details.
/// Customers must accept a mandate authorizing you to debit their account.
/// Check this [page](https://docs.stripe.com/payments/payto) for more details.
pub type UpdatePaymentMethodConfigurationPayto = PaymentMethodParam;
/// Pix is a payment method popular in Brazil.
/// When paying with Pix, customers authenticate and approve payments by scanning a QR code in their preferred banking app.
/// Check this [page](https://docs.stripe.com/payments/pix) for more details.
pub type UpdatePaymentMethodConfigurationPix = PaymentMethodParam;
/// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
/// Check this [page](https://docs.stripe.com/payments/promptpay) for more details.
pub type UpdatePaymentMethodConfigurationPromptpay = PaymentMethodParam;
/// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
/// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
pub type UpdatePaymentMethodConfigurationRevolutPay = PaymentMethodParam;
/// Samsung Pay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage local wallet available in South Korea.
pub type UpdatePaymentMethodConfigurationSamsungPay = PaymentMethodParam;
/// Satispay is a [single-use](/payments/payment-methods#usage) payment method where customers are required to [authenticate](/payments/payment-methods#customer-actions) their payment.
/// Customers pay by being redirected from your website or app, authorizing the payment with Satispay, then returning to your website or app.
/// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
pub type UpdatePaymentMethodConfigurationSatispay = PaymentMethodParam;
/// Scalapay is a [single-use](/payments/payment-methods#usage) payment method that lets customers pay in 3 or 4 installments.
/// Customers are redirected from your website or app, authorize the payment with Scalapay, then return to your website or app.
/// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
pub type UpdatePaymentMethodConfigurationScalapay = PaymentMethodParam;
/// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
/// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://docs.stripe.com/payments/sepa-debit) for more details.
pub type UpdatePaymentMethodConfigurationSepaDebit = PaymentMethodParam;
/// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
/// Check this [page](https://docs.stripe.com/payments/sofort) for more details.
pub type UpdatePaymentMethodConfigurationSofort = PaymentMethodParam;
/// Sunbit is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method where customers choose to pay in 3, 6, or 12 installments.
/// Customers are redirected from your website or app, authorize the payment with Sunbit, then return to your website or app.
/// You get [immediate notification](https://docs.stripe.com/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
pub type UpdatePaymentMethodConfigurationSunbit = PaymentMethodParam;
/// Swish is a [real-time](https://docs.stripe.com/payments/real-time) payment method popular in Sweden.
/// It allows customers to [authenticate and approve](https://docs.stripe.com/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
/// Check this [page](https://docs.stripe.com/payments/swish) for more details.
pub type UpdatePaymentMethodConfigurationSwish = PaymentMethodParam;
/// Twint is a payment method popular in Switzerland.
/// It allows customers to pay using their mobile phone.
/// Check this [page](https://docs.stripe.com/payments/twint) for more details.
pub type UpdatePaymentMethodConfigurationTwint = PaymentMethodParam;
/// Unified Payment Interface (UPI) is India's leading payment method with exponential growth since it launched in 2016.
pub type UpdatePaymentMethodConfigurationUpi = PaymentMethodParam;
/// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
/// Check this [page](https://docs.stripe.com/payments/ach-direct-debit) for more details.
pub type UpdatePaymentMethodConfigurationUsBankAccount = PaymentMethodParam;
/// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
/// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
/// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
/// Check this [page](https://docs.stripe.com/payments/wechat-pay) for more details.
pub type UpdatePaymentMethodConfigurationWechatPay = PaymentMethodParam;
/// Zip gives your customers a way to split purchases over a series of payments.
/// Check this [page](https://docs.stripe.com/payments/zip) for more details like country availability.
pub type UpdatePaymentMethodConfigurationZip = PaymentMethodParam;
