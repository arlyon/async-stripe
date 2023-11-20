#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListPaymentMethodConfigResourcePaymentMethodConfiguration<'a> {
    /// The Connect application to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ListPaymentMethodConfigResourcePaymentMethodConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListPaymentMethodConfigResourcePaymentMethodConfiguration<'a> {
    /// List payment method configurations.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<
        stripe_types::List<stripe_payment::PaymentMethodConfigResourcePaymentMethodConfiguration>,
    > {
        client.get_query("/payment_method_configurations", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_payment::PaymentMethodConfigResourcePaymentMethodConfiguration>
    {
        stripe::ListPaginator::from_params("/payment_method_configurations", self)
    }
}
impl<'a> stripe::PaginationParams
    for ListPaymentMethodConfigResourcePaymentMethodConfiguration<'a>
{
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePaymentMethodConfigResourcePaymentMethodConfiguration<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentMethodConfigResourcePaymentMethodConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrievePaymentMethodConfigResourcePaymentMethodConfiguration<'a> {
    /// Retrieve payment method configuration.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration:&stripe_payment::payment_method_config_resource_payment_method_configuration::PaymentMethodConfigurationId,
    ) -> stripe::Response<stripe_payment::PaymentMethodConfigResourcePaymentMethodConfiguration>
    {
        client.get_query(
            &format!(
                "/payment_method_configurations/{configuration}",
                configuration = configuration
            ),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfiguration<'a> {
    /// Canadian pre-authorized debit payments, check this [page](https://stripe.com/docs/payments/acss-debit) for more details like country availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebit>,
    /// Whether the configuration can be used for new payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
    ///
    /// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
    /// Check this [page](https://stripe.com/docs/payments/affirm) for more details like country availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirm>,
    /// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://stripe.com/docs/payments/afterpay-clearpay) for more details like country availability.
    ///
    /// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpay>,
    /// Alipay is a digital wallet in China that has more than a billion active users worldwide.
    ///
    /// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
    /// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
    /// Check this [page](https://stripe.com/docs/payments/alipay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipay>,
    /// Stripe users can accept [Apple Pay](/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
    ///
    /// There are no additional fees to process Apple Pay payments, and the [pricing](/pricing) is the same as other card transactions.
    /// Check this [page](https://stripe.com/docs/apple-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePay>,
    /// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay_later:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLater>,
    /// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
    ///
    /// Check this [page](https://stripe.com/docs/payments/au-becs-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebit>,
    /// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://stripe.com/docs/payments/payment-methods/bacs-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebit>,
    /// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
    ///
    /// [Customers](https://stripe.com/docs/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
    /// Check this [page](https://stripe.com/docs/payments/bancontact) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontact>,
    /// BLIK is a [single use](https://stripe.com/docs/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
    ///
    /// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
    /// Check this [page](https://stripe.com/docs/payments/blik) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlik>,
    /// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
    ///
    /// Check this [page](https://stripe.com/docs/payments/boleto) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoleto>,
    /// Cards are a popular way for consumers and businesses to pay online or in person.
    ///
    /// Stripe supports global and local card networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCard>,
    /// Cartes Bancaires is France's local card network.
    ///
    /// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
    /// Check this [page](https://stripe.com/docs/payments/cartes-bancaires) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancaires>,
    /// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
    ///
    /// Check this [page](https://stripe.com/docs/payments/cash-app-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashapp>,
    /// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
    ///
    /// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
    /// Check this [page](https://stripe.com/docs/payments/eps) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEps>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
    ///
    /// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
    /// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
    /// Check this [page](https://stripe.com/docs/payments/fpx) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpx>,
    /// giropay is a German payment method based on online banking, introduced in 2006.
    ///
    /// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
    /// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
    /// giropay accounts for 10% of online checkouts in Germany.
    /// Check this [page](https://stripe.com/docs/payments/giropay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropay>,
    /// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
    ///
    /// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
    /// Check this [page](https://stripe.com/docs/google-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePay>,
    /// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
    ///
    /// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
    /// Check this [page](https://stripe.com/docs/payments/grabpay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpay>,
    /// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
    ///
    /// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
    /// Check this [page](https://stripe.com/docs/payments/ideal) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdeal>,
    /// JCB is a credit card company based in Japan.
    ///
    /// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
    /// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcb>,
    /// Klarna gives customers a range of [payment options](https://stripe.com/docs/payments/klarna#payment-options) during checkout.
    ///
    /// Available payment options vary depending on the customer's billing address and the transaction amount.
    /// These payment options make it convenient for customers to purchase items in all price ranges.
    /// Check this [page](https://stripe.com/docs/payments/klarna) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarna>,
    /// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
    ///
    /// Check this [page](https://stripe.com/docs/payments/konbini) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbini>,
    /// [Link](https://stripe.com/docs/payments/link) is a payment method network.
    ///
    /// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLink>,
    /// Configuration name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
    ///
    /// OXXO allows customers to pay bills and online purchases in-store with cash.
    /// Check this [page](https://stripe.com/docs/payments/oxxo) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxo>,
    /// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
    ///
    /// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
    /// Check this [page](https://stripe.com/docs/payments/p24) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24>,
    /// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
    ///
    /// Check this [page](https://stripe.com/docs/payments/paynow) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynow>,
    /// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
    ///
    /// Check this [page](https://stripe.com/docs/payments/paypal) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypal>,
    /// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
    ///
    /// Check this [page](https://stripe.com/docs/payments/promptpay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpay>,
    /// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
    ///
    /// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://stripe.com/docs/payments/sepa-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebit>,
    /// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
    ///
    /// Check this [page](https://stripe.com/docs/payments/sofort) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofort>,
    /// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
    ///
    /// Check this [page](https://stripe.com/docs/payments/ach-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccount>,
    /// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
    ///
    /// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
    /// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
    /// Check this [page](https://stripe.com/docs/payments/wechat-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPay>,
}
impl<'a> UpdatePaymentMethodConfigResourcePaymentMethodConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Canadian pre-authorized debit payments, check this [page](https://stripe.com/docs/payments/acss-debit) for more details like country availability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
///
/// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
/// Check this [page](https://stripe.com/docs/payments/affirm) for more details like country availability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirm {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://stripe.com/docs/payments/afterpay-clearpay) for more details like country availability.
///
/// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpay {
    /// Whether or not the payment method should be displayed.
#[serde(skip_serializing_if = "Option::is_none")]
pub display_preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::*;
        match self {
None => "none",
Off => "off",
On => "on",

        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Alipay is a digital wallet in China that has more than a billion active users worldwide.
///
/// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
/// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
/// Check this [page](https://stripe.com/docs/payments/alipay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Stripe users can accept [Apple Pay](/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
///
/// There are no additional fees to process Apple Pay payments, and the [pricing](/pricing) is the same as other card transactions.
/// Check this [page](https://stripe.com/docs/apple-pay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLater {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLater {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::*;
        match self {
None => "none",
Off => "off",
On => "on",

        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
///
/// Check this [page](https://stripe.com/docs/payments/au-becs-debit) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::*;
        match self {
None => "none",
Off => "off",
On => "on",

        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://stripe.com/docs/payments/payment-methods/bacs-debit) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
///
/// [Customers](https://stripe.com/docs/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
/// Check this [page](https://stripe.com/docs/payments/bancontact) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontact {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// BLIK is a [single use](https://stripe.com/docs/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
///
/// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
/// Check this [page](https://stripe.com/docs/payments/blik) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlik {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlik {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
///
/// Check this [page](https://stripe.com/docs/payments/boleto) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoleto {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoleto {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Cards are a popular way for consumers and businesses to pay online or in person.
///
/// Stripe supports global and local card networks.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCard {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCard {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Cartes Bancaires is France's local card network.
///
/// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
/// Check this [page](https://stripe.com/docs/payments/cartes-bancaires) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancaires {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancaires {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::*;
        match self {
None => "none",
Off => "off",
On => "on",

        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
///
/// Check this [page](https://stripe.com/docs/payments/cash-app-pay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashapp {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashapp {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
///
/// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
/// Check this [page](https://stripe.com/docs/payments/eps) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEps {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
///
/// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
/// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
/// Check this [page](https://stripe.com/docs/payments/fpx) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpx {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpx {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// giropay is a German payment method based on online banking, introduced in 2006.
///
/// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
/// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
/// giropay accounts for 10% of online checkouts in Germany.
/// Check this [page](https://stripe.com/docs/payments/giropay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
///
/// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
/// Check this [page](https://stripe.com/docs/google-pay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
///
/// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
/// Check this [page](https://stripe.com/docs/payments/grabpay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
///
/// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
/// Check this [page](https://stripe.com/docs/payments/ideal) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdeal {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// JCB is a credit card company based in Japan.
///
/// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
/// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcb {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcb {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Klarna gives customers a range of [payment options](https://stripe.com/docs/payments/klarna#payment-options) during checkout.
///
/// Available payment options vary depending on the customer's billing address and the transaction amount.
/// These payment options make it convenient for customers to purchase items in all price ranges.
/// Check this [page](https://stripe.com/docs/payments/klarna) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarna {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
///
/// Check this [page](https://stripe.com/docs/payments/konbini) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbini {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// [Link](https://stripe.com/docs/payments/link) is a payment method network.
///
/// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLink {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLink {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
///
/// OXXO allows customers to pay bills and online purchases in-store with cash.
/// Check this [page](https://stripe.com/docs/payments/oxxo) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxo {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
///
/// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
/// Check this [page](https://stripe.com/docs/payments/p24) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24 {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
///
/// Check this [page](https://stripe.com/docs/payments/paynow) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynow {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
///
/// Check this [page](https://stripe.com/docs/payments/paypal) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypal {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
///
/// Check this [page](https://stripe.com/docs/payments/promptpay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
///
/// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://stripe.com/docs/payments/sepa-debit) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
///
/// Check this [page](https://stripe.com/docs/payments/sofort) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofort {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreference>,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofort {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
///
/// Check this [page](https://stripe.com/docs/payments/ach-debit) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccount {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::*;
        match self {
None => "none",
Off => "off",
On => "on",

        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
///
/// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
/// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
/// Check this [page](https://stripe.com/docs/payments/wechat-pay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreference,
    >,
}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference>,

}
impl UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> UpdatePaymentMethodConfigResourcePaymentMethodConfiguration<'a> {
    /// Update payment method configuration.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration:&stripe_payment::payment_method_config_resource_payment_method_configuration::PaymentMethodConfigurationId,
    ) -> stripe::Response<stripe_payment::PaymentMethodConfigResourcePaymentMethodConfiguration>
    {
        client.send_form(
            &format!(
                "/payment_method_configurations/{configuration}",
                configuration = configuration
            ),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfiguration<'a> {
    /// Canadian pre-authorized debit payments, check this [page](https://stripe.com/docs/payments/acss-debit) for more details like country availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebit>,
    /// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
    ///
    /// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
    /// Check this [page](https://stripe.com/docs/payments/affirm) for more details like country availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirm>,
    /// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://stripe.com/docs/payments/afterpay-clearpay) for more details like country availability.
    ///
    /// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpay>,
    /// Alipay is a digital wallet in China that has more than a billion active users worldwide.
    ///
    /// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
    /// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
    /// Check this [page](https://stripe.com/docs/payments/alipay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipay>,
    /// Stripe users can accept [Apple Pay](/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
    ///
    /// There are no additional fees to process Apple Pay payments, and the [pricing](/pricing) is the same as other card transactions.
    /// Check this [page](https://stripe.com/docs/apple-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePay>,
    /// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay_later:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLater>,
    /// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
    ///
    /// Check this [page](https://stripe.com/docs/payments/au-becs-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebit>,
    /// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://stripe.com/docs/payments/payment-methods/bacs-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebit>,
    /// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
    ///
    /// [Customers](https://stripe.com/docs/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
    /// Check this [page](https://stripe.com/docs/payments/bancontact) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontact>,
    /// BLIK is a [single use](https://stripe.com/docs/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
    ///
    /// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
    /// Check this [page](https://stripe.com/docs/payments/blik) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlik>,
    /// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
    ///
    /// Check this [page](https://stripe.com/docs/payments/boleto) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoleto>,
    /// Cards are a popular way for consumers and businesses to pay online or in person.
    ///
    /// Stripe supports global and local card networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationCard>,
    /// Cartes Bancaires is France's local card network.
    ///
    /// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
    /// Check this [page](https://stripe.com/docs/payments/cartes-bancaires) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancaires>,
    /// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
    ///
    /// Check this [page](https://stripe.com/docs/payments/cash-app-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashapp>,
    /// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
    ///
    /// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
    /// Check this [page](https://stripe.com/docs/payments/eps) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationEps>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
    ///
    /// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
    /// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
    /// Check this [page](https://stripe.com/docs/payments/fpx) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpx>,
    /// giropay is a German payment method based on online banking, introduced in 2006.
    ///
    /// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
    /// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
    /// giropay accounts for 10% of online checkouts in Germany.
    /// Check this [page](https://stripe.com/docs/payments/giropay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropay>,
    /// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
    ///
    /// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
    /// Check this [page](https://stripe.com/docs/google-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePay>,
    /// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
    ///
    /// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
    /// Check this [page](https://stripe.com/docs/payments/grabpay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpay>,
    /// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
    ///
    /// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
    /// Check this [page](https://stripe.com/docs/payments/ideal) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdeal>,
    /// JCB is a credit card company based in Japan.
    ///
    /// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
    /// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcb>,
    /// Klarna gives customers a range of [payment options](https://stripe.com/docs/payments/klarna#payment-options) during checkout.
    ///
    /// Available payment options vary depending on the customer's billing address and the transaction amount.
    /// These payment options make it convenient for customers to purchase items in all price ranges.
    /// Check this [page](https://stripe.com/docs/payments/klarna) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarna>,
    /// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
    ///
    /// Check this [page](https://stripe.com/docs/payments/konbini) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbini>,
    /// [Link](https://stripe.com/docs/payments/link) is a payment method network.
    ///
    /// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationLink>,
    /// Configuration name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
    ///
    /// OXXO allows customers to pay bills and online purchases in-store with cash.
    /// Check this [page](https://stripe.com/docs/payments/oxxo) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxo>,
    /// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
    ///
    /// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
    /// Check this [page](https://stripe.com/docs/payments/p24) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24>,
    /// Configuration's parent configuration.
    ///
    /// Specify to create a child configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<&'a str>,
    /// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
    ///
    /// Check this [page](https://stripe.com/docs/payments/paynow) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynow>,
    /// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
    ///
    /// Check this [page](https://stripe.com/docs/payments/paypal) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypal>,
    /// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
    ///
    /// Check this [page](https://stripe.com/docs/payments/promptpay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpay>,
    /// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
    ///
    /// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://stripe.com/docs/payments/sepa-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebit>,
    /// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
    ///
    /// Check this [page](https://stripe.com/docs/payments/sofort) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofort>,
    /// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
    ///
    /// Check this [page](https://stripe.com/docs/payments/ach-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccount>,
    /// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
    ///
    /// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
    /// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
    /// Check this [page](https://stripe.com/docs/payments/wechat-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPay>,
}
impl<'a> CreatePaymentMethodConfigResourcePaymentMethodConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Canadian pre-authorized debit payments, check this [page](https://stripe.com/docs/payments/acss-debit) for more details like country availability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
///
/// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
/// Check this [page](https://stripe.com/docs/payments/affirm) for more details like country availability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirm {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAffirmDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://stripe.com/docs/payments/afterpay-clearpay) for more details like country availability.
///
/// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpay {
    /// Whether or not the payment method should be displayed.
#[serde(skip_serializing_if = "Option::is_none")]
pub display_preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::*;
        match self {
None => "none",
Off => "off",
On => "on",

        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Alipay is a digital wallet in China that has more than a billion active users worldwide.
///
/// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
/// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
/// Check this [page](https://stripe.com/docs/payments/alipay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAlipayDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Stripe users can accept [Apple Pay](/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
///
/// There are no additional fees to process Apple Pay payments, and the [pricing](/pricing) is the same as other card transactions.
/// Check this [page](https://stripe.com/docs/apple-pay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLater {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLater {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::*;
        match self {
None => "none",
Off => "off",
On => "on",

        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
///
/// Check this [page](https://stripe.com/docs/payments/au-becs-debit) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::*;
        match self {
None => "none",
Off => "off",
On => "on",

        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://stripe.com/docs/payments/payment-methods/bacs-debit) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
///
/// [Customers](https://stripe.com/docs/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
/// Check this [page](https://stripe.com/docs/payments/bancontact) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontact {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// BLIK is a [single use](https://stripe.com/docs/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
///
/// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
/// Check this [page](https://stripe.com/docs/payments/blik) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlik {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlik {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBlikDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
///
/// Check this [page](https://stripe.com/docs/payments/boleto) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoleto {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoleto {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationBoletoDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Cards are a popular way for consumers and businesses to pay online or in person.
///
/// Stripe supports global and local card networks.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationCard {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationCard {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCardDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Cartes Bancaires is France's local card network.
///
/// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
/// Check this [page](https://stripe.com/docs/payments/cartes-bancaires) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancaires {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancaires {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::*;
        match self {
None => "none",
Off => "off",
On => "on",

        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
///
/// Check this [page](https://stripe.com/docs/payments/cash-app-pay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashapp {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashapp {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
///
/// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
/// Check this [page](https://stripe.com/docs/payments/eps) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationEps {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationEpsDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
///
/// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
/// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
/// Check this [page](https://stripe.com/docs/payments/fpx) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpx {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpx {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationFpxDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// giropay is a German payment method based on online banking, introduced in 2006.
///
/// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
/// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
/// giropay accounts for 10% of online checkouts in Germany.
/// Check this [page](https://stripe.com/docs/payments/giropay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
///
/// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
/// Check this [page](https://stripe.com/docs/google-pay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
///
/// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
/// Check this [page](https://stripe.com/docs/payments/grabpay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
///
/// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
/// Check this [page](https://stripe.com/docs/payments/ideal) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdeal {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationIdealDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// JCB is a credit card company based in Japan.
///
/// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
/// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcb {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcb {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationJcbDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Klarna gives customers a range of [payment options](https://stripe.com/docs/payments/klarna#payment-options) during checkout.
///
/// Available payment options vary depending on the customer's billing address and the transaction amount.
/// These payment options make it convenient for customers to purchase items in all price ranges.
/// Check this [page](https://stripe.com/docs/payments/klarna) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarna {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationKlarnaDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
///
/// Check this [page](https://stripe.com/docs/payments/konbini) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbini {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// [Link](https://stripe.com/docs/payments/link) is a payment method network.
///
/// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationLink {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationLink {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationLinkDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
///
/// OXXO allows customers to pay bills and online purchases in-store with cash.
/// Check this [page](https://stripe.com/docs/payments/oxxo) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxo {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationOxxoDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
///
/// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
/// Check this [page](https://stripe.com/docs/payments/p24) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24 {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationP24DisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
///
/// Check this [page](https://stripe.com/docs/payments/paynow) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynow {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaynowDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
///
/// Check this [page](https://stripe.com/docs/payments/paypal) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypal {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPaypalDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
///
/// Check this [page](https://stripe.com/docs/payments/promptpay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
///
/// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://stripe.com/docs/payments/sepa-debit) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebit {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
///
/// Check this [page](https://stripe.com/docs/payments/sofort) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofort {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference:
        Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreference>,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofort {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentMethodConfigResourcePaymentMethodConfigurationSofortDisplayPreferencePreference
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
///
/// Check this [page](https://stripe.com/docs/payments/ach-debit) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccount {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::*;
        match self {
None => "none",
Off => "off",
On => "on",

        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
///
/// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
/// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
/// Check this [page](https://stripe.com/docs/payments/wechat-pay) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPay {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<
        CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreference,
    >,
}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether or not the payment method should be displayed.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreference {
    /// The account's preference for whether or not to display this payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub preference: Option<CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference>,

}
impl CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreference {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The account's preference for whether or not to display this payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference
{
    None,
    Off,
    On,
}

impl
    CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference
{
    pub fn as_str(self) -> &'static str {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference::*;
        match s {
    "none" => Ok(None),
"off" => Ok(Off),
"on" => Ok(On),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentMethodConfigResourcePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreatePaymentMethodConfigResourcePaymentMethodConfiguration<'a> {
    /// Creates a payment method configuration.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_payment::PaymentMethodConfigResourcePaymentMethodConfiguration>
    {
        client.send_form("/payment_method_configurations", self, http_types::Method::Post)
    }
}
