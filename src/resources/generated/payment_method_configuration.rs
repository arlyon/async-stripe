// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{PaymentMethodConfigurationId};
use crate::params::{Expand, List, Object, Paginable};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "PaymentMethodConfigResourcePaymentMethodConfiguration".
///
/// For more details see <https://stripe.com/docs/api/payment_method_configurations/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodConfiguration {
    /// Unique identifier for the object.
    pub id: PaymentMethodConfigurationId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    /// Whether the configuration can be used for new payments.
    pub active: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alma: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    /// For child configs, the Connect application associated with the configuration.
    pub application: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billie: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    /// The default configuration is used whenever a payment method configuration is not specified.
    pub is_default: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    /// The configuration's name.
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    /// For child configs, the configuration's parent configuration.
    pub parent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub satispay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub twint: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
}

impl PaymentMethodConfiguration {

    /// List payment method configurations.
pub fn list(client: &Client, params: &ListPaymentMethodConfigurations<'_>) -> Response<List<PaymentMethodConfiguration>> {
   client.get_query("/payment_method_configurations", params)
}


    /// Creates a payment method configuration.
    pub fn create(client: &Client, params: CreatePaymentMethodConfiguration<'_>) -> Response<PaymentMethodConfiguration> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/payment_method_configurations", &params)
    }

    /// Retrieve payment method configuration.
    pub fn retrieve(client: &Client, id: &PaymentMethodConfigurationId, expand: &[&str]) -> Response<PaymentMethodConfiguration> {
        client.get_query(&format!("/payment_method_configurations/{}", id), Expand { expand })
    }

    /// Update payment method configuration.
    pub fn update(client: &Client, id: &PaymentMethodConfigurationId, params: UpdatePaymentMethodConfiguration<'_>) -> Response<PaymentMethodConfiguration> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/payment_method_configurations/{}", id), &params)
    }
}

impl Object for PaymentMethodConfiguration {
    type Id = PaymentMethodConfigurationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "payment_method_configuration"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodConfigResourcePaymentMethodProperties {

    /// Whether this payment method may be offered at checkout.
    ///
    /// True if `display_preference` is `on` and the payment method's capability is active.
    pub available: bool,

    pub display_preference: PaymentMethodConfigResourceDisplayPreference,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodConfigResourceDisplayPreference {

    /// For child configs, whether or not the account's preference will be observed.
    ///
    /// If `false`, the parent configuration's default is used.
    pub overridable: Option<bool>,

    /// The account's display preference.
    pub preference: PaymentMethodConfigResourceDisplayPreferencePreference,

    /// The effective display preference value.
    pub value: PaymentMethodConfigResourceDisplayPreferenceValue,
}

/// The parameters for `PaymentMethodConfiguration::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreatePaymentMethodConfiguration<'a> {

    /// Canadian pre-authorized debit payments, check this [page](https://stripe.com/docs/payments/acss-debit) for more details like country availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreatePaymentMethodConfigurationAcssDebit>,

    /// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
    ///
    /// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
    /// Check this [page](https://stripe.com/docs/payments/affirm) for more details like country availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreatePaymentMethodConfigurationAffirm>,

    /// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://stripe.com/docs/payments/afterpay-clearpay) for more details like country availability.
    ///
    /// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreatePaymentMethodConfigurationAfterpayClearpay>,

    /// Alipay is a digital wallet in China that has more than a billion active users worldwide.
    ///
    /// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
    /// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
    /// Check this [page](https://stripe.com/docs/payments/alipay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreatePaymentMethodConfigurationAlipay>,

    /// Alma is a Buy Now, Pay Later payment method that offers customers the ability to pay in 2, 3, or 4 installments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alma: Option<CreatePaymentMethodConfigurationAlma>,

    /// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<CreatePaymentMethodConfigurationAmazonPay>,

    /// Stripe users can accept [Apple Pay](/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
    ///
    /// There are no additional fees to process Apple Pay payments, and the [pricing](/pricing) is the same as other card transactions.
    /// Check this [page](https://stripe.com/docs/apple-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<CreatePaymentMethodConfigurationApplePay>,

    /// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay_later: Option<CreatePaymentMethodConfigurationApplePayLater>,

    /// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
    ///
    /// Check this [page](https://stripe.com/docs/payments/au-becs-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreatePaymentMethodConfigurationAuBecsDebit>,

    /// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://stripe.com/docs/payments/payment-methods/bacs-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreatePaymentMethodConfigurationBacsDebit>,

    /// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
    ///
    /// [Customers](https://stripe.com/docs/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
    /// Check this [page](https://stripe.com/docs/payments/bancontact) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreatePaymentMethodConfigurationBancontact>,

    /// Billie is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers businesses Pay by Invoice where they offer payment terms ranging from 7-120 days.
    ///
    /// Customers are redirected from your website or app, authorize the payment with Billie, then return to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billie: Option<CreatePaymentMethodConfigurationBillie>,

    /// BLIK is a [single use](https://stripe.com/docs/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
    ///
    /// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
    /// Check this [page](https://stripe.com/docs/payments/blik) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<CreatePaymentMethodConfigurationBlik>,

    /// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
    ///
    /// Check this [page](https://stripe.com/docs/payments/boleto) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreatePaymentMethodConfigurationBoleto>,

    /// Cards are a popular way for consumers and businesses to pay online or in person.
    ///
    /// Stripe supports global and local card networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodParam>,

    /// Cartes Bancaires is France's local card network.
    ///
    /// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
    /// Check this [page](https://stripe.com/docs/payments/cartes-bancaires) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires: Option<CreatePaymentMethodConfigurationCartesBancaires>,

    /// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
    ///
    /// Check this [page](https://stripe.com/docs/payments/cash-app-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<CreatePaymentMethodConfigurationCashapp>,

    /// Uses a customer’s [cash balance](https://stripe.com/docs/payments/customer-balance) for the payment.
    ///
    /// The cash balance can be funded via a bank transfer.
    /// Check this [page](https://stripe.com/docs/payments/bank-transfers) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreatePaymentMethodConfigurationCustomerBalance>,

    /// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
    ///
    /// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
    /// Check this [page](https://stripe.com/docs/payments/eps) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreatePaymentMethodConfigurationEps>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
    ///
    /// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
    /// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
    /// Check this [page](https://stripe.com/docs/payments/fpx) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreatePaymentMethodConfigurationFpx>,

    /// giropay is a German payment method based on online banking, introduced in 2006.
    ///
    /// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
    /// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
    /// giropay accounts for 10% of online checkouts in Germany.
    /// Check this [page](https://stripe.com/docs/payments/giropay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreatePaymentMethodConfigurationGiropay>,

    /// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
    ///
    /// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
    /// Check this [page](https://stripe.com/docs/google-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<CreatePaymentMethodConfigurationGooglePay>,

    /// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
    ///
    /// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
    /// Check this [page](https://stripe.com/docs/payments/grabpay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreatePaymentMethodConfigurationGrabpay>,

    /// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
    ///
    /// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
    /// Check this [page](https://stripe.com/docs/payments/ideal) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreatePaymentMethodConfigurationIdeal>,

    /// JCB is a credit card company based in Japan.
    ///
    /// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
    /// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb: Option<CreatePaymentMethodConfigurationJcb>,

    /// Klarna gives customers a range of [payment options](https://stripe.com/docs/payments/klarna#payment-options) during checkout.
    ///
    /// Available payment options vary depending on the customer's billing address and the transaction amount.
    /// These payment options make it convenient for customers to purchase items in all price ranges.
    /// Check this [page](https://stripe.com/docs/payments/klarna) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreatePaymentMethodConfigurationKlarna>,

    /// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
    ///
    /// Check this [page](https://stripe.com/docs/payments/konbini) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreatePaymentMethodConfigurationKonbini>,

    /// [Link](https://stripe.com/docs/payments/link) is a payment method network.
    ///
    /// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreatePaymentMethodConfigurationLink>,

    /// MobilePay is a [single-use](https://stripe.com/docs/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
    ///
    /// It allows customers to [authenticate and approve](https://stripe.com/docs/payments/payment-methods#customer-actions) payments using the MobilePay app.
    /// Check this [page](https://stripe.com/docs/payments/mobilepay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<CreatePaymentMethodConfigurationMobilepay>,

    /// Stripe users in Europe and the United States can accept Multibanco payments from customers in Portugal using [Sources](https://stripe.com/docs/sources)—a single integration path for creating payments using any supported method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<CreatePaymentMethodConfigurationMultibanco>,

    /// Configuration name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    /// Stripe users in New Zealand can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with a New Zeland bank account.
    ///
    /// Check this [page](https://stripe.com/docs/payments/nz-bank-account) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<CreatePaymentMethodConfigurationNzBankAccount>,

    /// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
    ///
    /// OXXO allows customers to pay bills and online purchases in-store with cash.
    /// Check this [page](https://stripe.com/docs/payments/oxxo) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreatePaymentMethodConfigurationOxxo>,

    /// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
    ///
    /// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
    /// Check this [page](https://stripe.com/docs/payments/p24) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreatePaymentMethodConfigurationP24>,

    /// Configuration's parent configuration.
    ///
    /// Specify to create a child configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<&'a str>,

    /// Pay by bank is a redirect payment method backed by bank transfers.
    ///
    /// A customer is redirected to their bank to authorize a bank transfer for a given amount.
    /// This removes a lot of the error risks inherent in waiting for the customer to initiate a transfer themselves, and is less expensive than card payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<CreatePaymentMethodConfigurationPayByBank>,

    /// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
    ///
    /// Check this [page](https://stripe.com/docs/payments/paynow) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreatePaymentMethodConfigurationPaynow>,

    /// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
    ///
    /// Check this [page](https://stripe.com/docs/payments/paypal) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CreatePaymentMethodConfigurationPaypal>,

    /// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
    ///
    /// Check this [page](https://stripe.com/docs/payments/promptpay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<CreatePaymentMethodConfigurationPromptpay>,

    /// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
    ///
    /// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<CreatePaymentMethodConfigurationRevolutPay>,

    /// Satispay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method where customers are required to [authenticate](/payments/payment-methods#customer-actions) their payment.
    ///
    /// Customers pay by being redirected from your website or app, authorizing the payment with Satispay, then returning to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satispay: Option<CreatePaymentMethodConfigurationSatispay>,

    /// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
    ///
    /// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://stripe.com/docs/payments/sepa-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreatePaymentMethodConfigurationSepaDebit>,

    /// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
    ///
    /// Check this [page](https://stripe.com/docs/payments/sofort) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreatePaymentMethodConfigurationSofort>,

    /// Swish is a [real-time](https://stripe.com/docs/payments/real-time) payment method popular in Sweden.
    ///
    /// It allows customers to [authenticate and approve](https://stripe.com/docs/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
    /// Check this [page](https://stripe.com/docs/payments/swish) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<CreatePaymentMethodConfigurationSwish>,

    /// Twint is a payment method popular in Switzerland.
    ///
    /// It allows customers to pay using their mobile phone.
    /// Check this [page](https://docs.stripe.com/payments/twint) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twint: Option<CreatePaymentMethodConfigurationTwint>,

    /// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
    ///
    /// Check this [page](https://stripe.com/docs/payments/ach-direct-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreatePaymentMethodConfigurationUsBankAccount>,

    /// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
    ///
    /// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
    /// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
    /// Check this [page](https://stripe.com/docs/payments/wechat-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreatePaymentMethodConfigurationWechatPay>,

    /// Zip gives your customers a way to split purchases over a series of payments.
    ///
    /// Check this [page](https://stripe.com/docs/payments/zip) for more details like country availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<CreatePaymentMethodConfigurationZip>,
}

impl<'a> CreatePaymentMethodConfiguration<'a> {
    pub fn new() -> Self {
        CreatePaymentMethodConfiguration {
            acss_debit: Default::default(),
            affirm: Default::default(),
            afterpay_clearpay: Default::default(),
            alipay: Default::default(),
            alma: Default::default(),
            amazon_pay: Default::default(),
            apple_pay: Default::default(),
            apple_pay_later: Default::default(),
            au_becs_debit: Default::default(),
            bacs_debit: Default::default(),
            bancontact: Default::default(),
            billie: Default::default(),
            blik: Default::default(),
            boleto: Default::default(),
            card: Default::default(),
            cartes_bancaires: Default::default(),
            cashapp: Default::default(),
            customer_balance: Default::default(),
            eps: Default::default(),
            expand: Default::default(),
            fpx: Default::default(),
            giropay: Default::default(),
            google_pay: Default::default(),
            grabpay: Default::default(),
            ideal: Default::default(),
            jcb: Default::default(),
            klarna: Default::default(),
            konbini: Default::default(),
            link: Default::default(),
            mobilepay: Default::default(),
            multibanco: Default::default(),
            name: Default::default(),
            nz_bank_account: Default::default(),
            oxxo: Default::default(),
            p24: Default::default(),
            parent: Default::default(),
            pay_by_bank: Default::default(),
            paynow: Default::default(),
            paypal: Default::default(),
            promptpay: Default::default(),
            revolut_pay: Default::default(),
            satispay: Default::default(),
            sepa_debit: Default::default(),
            sofort: Default::default(),
            swish: Default::default(),
            twint: Default::default(),
            us_bank_account: Default::default(),
            wechat_pay: Default::default(),
            zip: Default::default(),
        }
    }
}

/// The parameters for `PaymentMethodConfiguration::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListPaymentMethodConfigurations<'a> {

    /// The Connect application to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<PaymentMethodConfigurationId>,

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
    pub starting_after: Option<PaymentMethodConfigurationId>,
}

impl<'a> ListPaymentMethodConfigurations<'a> {
    pub fn new() -> Self {
        ListPaymentMethodConfigurations {
            application: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListPaymentMethodConfigurations<'_> {
    type O = PaymentMethodConfiguration;
    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }}
/// The parameters for `PaymentMethodConfiguration::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdatePaymentMethodConfiguration<'a> {

    /// Canadian pre-authorized debit payments, check this [page](https://stripe.com/docs/payments/acss-debit) for more details like country availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdatePaymentMethodConfigurationAcssDebit>,

    /// Whether the configuration can be used for new payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// [Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments.
    ///
    /// Depending on the purchase, they can pay with four interest-free payments (Split Pay) or pay over a longer term (Installments), which might include interest.
    /// Check this [page](https://stripe.com/docs/payments/affirm) for more details like country availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<UpdatePaymentMethodConfigurationAffirm>,

    /// Afterpay gives your customers a way to pay for purchases in installments, check this [page](https://stripe.com/docs/payments/afterpay-clearpay) for more details like country availability.
    ///
    /// Afterpay is particularly popular among businesses selling fashion, beauty, and sports products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<UpdatePaymentMethodConfigurationAfterpayClearpay>,

    /// Alipay is a digital wallet in China that has more than a billion active users worldwide.
    ///
    /// Alipay users can pay on the web or on a mobile device using login credentials or their Alipay app.
    /// Alipay has a low dispute rate and reduces fraud by authenticating payments using the customer's login credentials.
    /// Check this [page](https://stripe.com/docs/payments/alipay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<UpdatePaymentMethodConfigurationAlipay>,

    /// Alma is a Buy Now, Pay Later payment method that offers customers the ability to pay in 2, 3, or 4 installments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alma: Option<UpdatePaymentMethodConfigurationAlma>,

    /// Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<UpdatePaymentMethodConfigurationAmazonPay>,

    /// Stripe users can accept [Apple Pay](/payments/apple-pay) in iOS applications in iOS 9 and later, and on the web in Safari starting with iOS 10 or macOS Sierra.
    ///
    /// There are no additional fees to process Apple Pay payments, and the [pricing](/pricing) is the same as other card transactions.
    /// Check this [page](https://stripe.com/docs/apple-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<UpdatePaymentMethodConfigurationApplePay>,

    /// Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchases into four installments across six weeks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay_later: Option<UpdatePaymentMethodConfigurationApplePayLater>,

    /// Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with an Australian bank account.
    ///
    /// Check this [page](https://stripe.com/docs/payments/au-becs-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<UpdatePaymentMethodConfigurationAuBecsDebit>,

    /// Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this [page](https://stripe.com/docs/payments/payment-methods/bacs-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<UpdatePaymentMethodConfigurationBacsDebit>,

    /// Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation.
    ///
    /// [Customers](https://stripe.com/docs/api/customers) use a Bancontact card or mobile app linked to a Belgian bank account to make online payments that are secure, guaranteed, and confirmed immediately.
    /// Check this [page](https://stripe.com/docs/payments/bancontact) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<UpdatePaymentMethodConfigurationBancontact>,

    /// Billie is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers businesses Pay by Invoice where they offer payment terms ranging from 7-120 days.
    ///
    /// Customers are redirected from your website or app, authorize the payment with Billie, then return to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billie: Option<UpdatePaymentMethodConfigurationBillie>,

    /// BLIK is a [single use](https://stripe.com/docs/payments/payment-methods#usage) payment method that requires customers to authenticate their payments.
    ///
    /// When customers want to pay online using BLIK, they request a six-digit code from their banking application and enter it into the payment collection form.
    /// Check this [page](https://stripe.com/docs/payments/blik) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<UpdatePaymentMethodConfigurationBlik>,

    /// Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil.
    ///
    /// Check this [page](https://stripe.com/docs/payments/boleto) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<UpdatePaymentMethodConfigurationBoleto>,

    /// Cards are a popular way for consumers and businesses to pay online or in person.
    ///
    /// Stripe supports global and local card networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodParam>,

    /// Cartes Bancaires is France's local card network.
    ///
    /// More than 95% of these cards are co-branded with either Visa or Mastercard, meaning you can process these cards over either Cartes Bancaires or the Visa or Mastercard networks.
    /// Check this [page](https://stripe.com/docs/payments/cartes-bancaires) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires: Option<UpdatePaymentMethodConfigurationCartesBancaires>,

    /// Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using their digital wallet.
    ///
    /// Check this [page](https://stripe.com/docs/payments/cash-app-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<UpdatePaymentMethodConfigurationCashapp>,

    /// Uses a customer’s [cash balance](https://stripe.com/docs/payments/customer-balance) for the payment.
    ///
    /// The cash balance can be funded via a bank transfer.
    /// Check this [page](https://stripe.com/docs/payments/bank-transfers) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<UpdatePaymentMethodConfigurationCustomerBalance>,

    /// EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credentials.
    ///
    /// EPS is supported by all Austrian banks and is accepted by over 80% of Austrian online retailers.
    /// Check this [page](https://stripe.com/docs/payments/eps) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<UpdatePaymentMethodConfigurationEps>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions online using their bank credentials.
    ///
    /// Bank Negara Malaysia (BNM), the Central Bank of Malaysia, and eleven other major Malaysian financial institutions are members of the PayNet Group, which owns and operates FPX.
    /// It is one of the most popular online payment methods in Malaysia, with nearly 90 million transactions in 2018 according to BNM.
    /// Check this [page](https://stripe.com/docs/payments/fpx) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<UpdatePaymentMethodConfigurationFpx>,

    /// giropay is a German payment method based on online banking, introduced in 2006.
    ///
    /// It allows customers to complete transactions online using their online banking environment, with funds debited from their bank account.
    /// Depending on their bank, customers confirm payments on giropay using a second factor of authentication or a PIN.
    /// giropay accounts for 10% of online checkouts in Germany.
    /// Check this [page](https://stripe.com/docs/payments/giropay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<UpdatePaymentMethodConfigurationGiropay>,

    /// Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Google Account, including those from Google Play, YouTube, Chrome, or an Android device.
    ///
    /// Use the Google Pay API to request any credit or debit card stored in your customer's Google account.
    /// Check this [page](https://stripe.com/docs/google-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<UpdatePaymentMethodConfigurationGooglePay>,

    /// GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/).
    ///
    /// GrabPay is a digital wallet - customers maintain a balance in their wallets that they pay out with.
    /// Check this [page](https://stripe.com/docs/payments/grabpay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<UpdatePaymentMethodConfigurationGrabpay>,

    /// iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank credentials.
    ///
    /// All major Dutch banks are members of Currence, the scheme that operates iDEAL, making it the most popular online payment method in the Netherlands with a share of online transactions close to 55%.
    /// Check this [page](https://stripe.com/docs/payments/ideal) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<UpdatePaymentMethodConfigurationIdeal>,

    /// JCB is a credit card company based in Japan.
    ///
    /// JCB is currently available in Japan to businesses approved by JCB, and available to all businesses in Australia, Canada, Hong Kong, Japan, New Zealand, Singapore, Switzerland, United Kingdom, United States, and all countries in the European Economic Area except Iceland.
    /// Check this [page](https://support.stripe.com/questions/accepting-japan-credit-bureau-%28jcb%29-payments) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb: Option<UpdatePaymentMethodConfigurationJcb>,

    /// Klarna gives customers a range of [payment options](https://stripe.com/docs/payments/klarna#payment-options) during checkout.
    ///
    /// Available payment options vary depending on the customer's billing address and the transaction amount.
    /// These payment options make it convenient for customers to purchase items in all price ranges.
    /// Check this [page](https://stripe.com/docs/payments/klarna) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<UpdatePaymentMethodConfigurationKlarna>,

    /// Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash.
    ///
    /// Check this [page](https://stripe.com/docs/payments/konbini) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<UpdatePaymentMethodConfigurationKonbini>,

    /// [Link](https://stripe.com/docs/payments/link) is a payment method network.
    ///
    /// With Link, users save their payment details once, then reuse that information to pay with one click for any business on the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<UpdatePaymentMethodConfigurationLink>,

    /// MobilePay is a [single-use](https://stripe.com/docs/payments/payment-methods#usage) card wallet payment method used in Denmark and Finland.
    ///
    /// It allows customers to [authenticate and approve](https://stripe.com/docs/payments/payment-methods#customer-actions) payments using the MobilePay app.
    /// Check this [page](https://stripe.com/docs/payments/mobilepay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<UpdatePaymentMethodConfigurationMobilepay>,

    /// Stripe users in Europe and the United States can accept Multibanco payments from customers in Portugal using [Sources](https://stripe.com/docs/sources)—a single integration path for creating payments using any supported method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<UpdatePaymentMethodConfigurationMultibanco>,

    /// Configuration name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    /// Stripe users in New Zealand can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with a New Zeland bank account.
    ///
    /// Check this [page](https://stripe.com/docs/payments/nz-bank-account) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<UpdatePaymentMethodConfigurationNzBankAccount>,

    /// OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly 20% of online transactions in Mexico.
    ///
    /// OXXO allows customers to pay bills and online purchases in-store with cash.
    /// Check this [page](https://stripe.com/docs/payments/oxxo) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<UpdatePaymentMethodConfigurationOxxo>,

    /// Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using bank transfers and other methods.
    ///
    /// Bank transfers account for 30% of online payments in Poland and Przelewy24 provides a way for customers to pay with over 165 banks.
    /// Check this [page](https://stripe.com/docs/payments/p24) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<UpdatePaymentMethodConfigurationP24>,

    /// Pay by bank is a redirect payment method backed by bank transfers.
    ///
    /// A customer is redirected to their bank to authorize a bank transfer for a given amount.
    /// This removes a lot of the error risks inherent in waiting for the customer to initiate a transfer themselves, and is less expensive than card payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<UpdatePaymentMethodConfigurationPayByBank>,

    /// PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from participating banks and participating non-bank financial institutions.
    ///
    /// Check this [page](https://stripe.com/docs/payments/paynow) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<UpdatePaymentMethodConfigurationPaynow>,

    /// PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal account.
    ///
    /// Check this [page](https://stripe.com/docs/payments/paypal) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<UpdatePaymentMethodConfigurationPaypal>,

    /// PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from participating banks.
    ///
    /// Check this [page](https://stripe.com/docs/payments/promptpay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<UpdatePaymentMethodConfigurationPromptpay>,

    /// Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method.
    ///
    /// Revolut Pay uses the customer’s stored balance or cards to fund the payment, and offers the option for non-Revolut customers to save their details after their first purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<UpdatePaymentMethodConfigurationRevolutPay>,

    /// Satispay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method where customers are required to [authenticate](/payments/payment-methods#customer-actions) their payment.
    ///
    /// Customers pay by being redirected from your website or app, authorizing the payment with Satispay, then returning to your website or app.
    /// You get [immediate notification](/payments/payment-methods#payment-notification) of whether the payment succeeded or failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satispay: Option<UpdatePaymentMethodConfigurationSatispay>,

    /// The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of the European Union to simplify payments within and across member countries.
    ///
    /// SEPA established and enforced banking standards to allow for the direct debiting of every EUR-denominated bank account within the SEPA region, check this [page](https://stripe.com/docs/payments/sepa-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdatePaymentMethodConfigurationSepaDebit>,

    /// Stripe users in Europe and the United States can use the [Payment Intents API](https://stripe.com/docs/payments/payment-intents)—a single integration path for creating payments using any supported method—to accept [Sofort](https://www.sofort.com/) payments from customers.
    ///
    /// Check this [page](https://stripe.com/docs/payments/sofort) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<UpdatePaymentMethodConfigurationSofort>,

    /// Swish is a [real-time](https://stripe.com/docs/payments/real-time) payment method popular in Sweden.
    ///
    /// It allows customers to [authenticate and approve](https://stripe.com/docs/payments/payment-methods#customer-actions) payments using the Swish mobile app and the Swedish BankID mobile app.
    /// Check this [page](https://stripe.com/docs/payments/swish) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<UpdatePaymentMethodConfigurationSwish>,

    /// Twint is a payment method popular in Switzerland.
    ///
    /// It allows customers to pay using their mobile phone.
    /// Check this [page](https://docs.stripe.com/payments/twint) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twint: Option<UpdatePaymentMethodConfigurationTwint>,

    /// Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using the Automated Clearing House (ACH) payments system operated by Nacha.
    ///
    /// Check this [page](https://stripe.com/docs/payments/ach-direct-debit) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdatePaymentMethodConfigurationUsBankAccount>,

    /// WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users.
    ///
    /// Chinese consumers can use WeChat Pay to pay for goods and services inside of businesses' apps and websites.
    /// WeChat Pay users buy most frequently in gaming, e-commerce, travel, online education, and food/nutrition.
    /// Check this [page](https://stripe.com/docs/payments/wechat-pay) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<UpdatePaymentMethodConfigurationWechatPay>,

    /// Zip gives your customers a way to split purchases over a series of payments.
    ///
    /// Check this [page](https://stripe.com/docs/payments/zip) for more details like country availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<UpdatePaymentMethodConfigurationZip>,
}

impl<'a> UpdatePaymentMethodConfiguration<'a> {
    pub fn new() -> Self {
        UpdatePaymentMethodConfiguration {
            acss_debit: Default::default(),
            active: Default::default(),
            affirm: Default::default(),
            afterpay_clearpay: Default::default(),
            alipay: Default::default(),
            alma: Default::default(),
            amazon_pay: Default::default(),
            apple_pay: Default::default(),
            apple_pay_later: Default::default(),
            au_becs_debit: Default::default(),
            bacs_debit: Default::default(),
            bancontact: Default::default(),
            billie: Default::default(),
            blik: Default::default(),
            boleto: Default::default(),
            card: Default::default(),
            cartes_bancaires: Default::default(),
            cashapp: Default::default(),
            customer_balance: Default::default(),
            eps: Default::default(),
            expand: Default::default(),
            fpx: Default::default(),
            giropay: Default::default(),
            google_pay: Default::default(),
            grabpay: Default::default(),
            ideal: Default::default(),
            jcb: Default::default(),
            klarna: Default::default(),
            konbini: Default::default(),
            link: Default::default(),
            mobilepay: Default::default(),
            multibanco: Default::default(),
            name: Default::default(),
            nz_bank_account: Default::default(),
            oxxo: Default::default(),
            p24: Default::default(),
            pay_by_bank: Default::default(),
            paynow: Default::default(),
            paypal: Default::default(),
            promptpay: Default::default(),
            revolut_pay: Default::default(),
            satispay: Default::default(),
            sepa_debit: Default::default(),
            sofort: Default::default(),
            swish: Default::default(),
            twint: Default::default(),
            us_bank_account: Default::default(),
            wechat_pay: Default::default(),
            zip: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAcssDebit {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAcssDebitDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAffirm {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAffirmDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAfterpayClearpay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAlipay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAlipayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAlma {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAlmaDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAmazonPay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAmazonPayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationApplePay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationApplePayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationApplePayLater {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationApplePayLaterDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAuBecsDebit {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationAuBecsDebitDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationBacsDebit {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationBacsDebitDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationBancontact {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationBancontactDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationBillie {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationBillieDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationBlik {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationBlikDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationBoleto {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationBoletoDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationCartesBancaires {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationCartesBancairesDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationCashapp {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationCashappDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationCustomerBalance {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationCustomerBalanceDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationEps {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationEpsDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationFpx {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationFpxDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationGiropay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationGiropayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationGooglePay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationGooglePayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationGrabpay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationGrabpayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationIdeal {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationIdealDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationJcb {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationJcbDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationKlarna {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationKlarnaDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationKonbini {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationKonbiniDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationLink {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationLinkDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationMobilepay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationMobilepayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationMultibanco {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationMultibancoDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationNzBankAccount {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationNzBankAccountDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationOxxo {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationOxxoDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationP24 {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationP24DisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationPayByBank {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationPayByBankDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationPaynow {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationPaynowDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationPaypal {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationPaypalDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationPromptpay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationPromptpayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationRevolutPay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationRevolutPayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationSatispay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationSatispayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationSepaDebit {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationSepaDebitDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationSofort {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationSofortDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationSwish {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationSwishDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationTwint {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationTwintDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationUsBankAccount {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationUsBankAccountDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationWechatPay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationWechatPayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationZip {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<CreatePaymentMethodConfigurationZipDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAcssDebit {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAcssDebitDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAffirm {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAffirmDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAfterpayClearpay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAlipay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAlipayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAlma {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAlmaDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAmazonPay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAmazonPayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationApplePay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationApplePayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationApplePayLater {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationApplePayLaterDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAuBecsDebit {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationBacsDebit {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationBacsDebitDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationBancontact {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationBancontactDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationBillie {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationBillieDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationBlik {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationBlikDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationBoleto {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationBoletoDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationCartesBancaires {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationCartesBancairesDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationCashapp {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationCashappDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationCustomerBalance {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationEps {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationEpsDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationFpx {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationFpxDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationGiropay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationGiropayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationGooglePay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationGooglePayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationGrabpay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationGrabpayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationIdeal {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationIdealDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationJcb {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationJcbDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationKlarna {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationKlarnaDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationKonbini {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationKonbiniDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationLink {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationLinkDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationMobilepay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationMobilepayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationMultibanco {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationMultibancoDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationNzBankAccount {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationNzBankAccountDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationOxxo {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationOxxoDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationP24 {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationP24DisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationPayByBank {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationPayByBankDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationPaynow {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationPaynowDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationPaypal {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationPaypalDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationPromptpay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationPromptpayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationRevolutPay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationRevolutPayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationSatispay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationSatispayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationSepaDebit {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationSepaDebitDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationSofort {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationSofortDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationSwish {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationSwishDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationTwint {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationTwintDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationUsBankAccount {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationUsBankAccountDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationWechatPay {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationWechatPayDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationZip {

    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<UpdatePaymentMethodConfigurationZipDisplayPreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAcssDebitDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAffirmDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAlipayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAlmaDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAmazonPayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationApplePayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationApplePayLaterDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationAuBecsDebitDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationBacsDebitDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationBancontactDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationBillieDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationBillieDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationBlikDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationBlikDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationBoletoDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationCartesBancairesDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationCashappDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationCashappDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationCustomerBalanceDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationEpsDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationEpsDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationFpxDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationFpxDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationGiropayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationGooglePayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationGrabpayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationIdealDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationIdealDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationJcbDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationJcbDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationKlarnaDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationKonbiniDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationLinkDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationLinkDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationMobilepayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationMultibancoDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationNzBankAccountDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationOxxoDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationP24DisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationP24DisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationPayByBankDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationPaynowDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationPaypalDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationPromptpayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationRevolutPayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationSatispayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationSepaDebitDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationSofortDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationSofortDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationSwishDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationSwishDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationTwintDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationTwintDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationUsBankAccountDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationWechatPayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodConfigurationZipDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<CreatePaymentMethodConfigurationZipDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAcssDebitDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAffirmDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAlipayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAlmaDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAmazonPayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationApplePayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationApplePayLaterDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationBacsDebitDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationBancontactDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationBillieDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationBlikDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationBoletoDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationCartesBancairesDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationCashappDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationEpsDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationFpxDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationGiropayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationGooglePayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationGrabpayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationIdealDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationJcbDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationKlarnaDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationKonbiniDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationLinkDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationMobilepayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationMultibancoDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationNzBankAccountDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationOxxoDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationP24DisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationP24DisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationPayByBankDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationPaynowDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationPaypalDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationPromptpayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationRevolutPayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationSatispayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationSepaDebitDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationSofortDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationSwishDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationTwintDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationUsBankAccountDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationWechatPayDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodConfigurationZipDisplayPreference {

    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<UpdatePaymentMethodConfigurationZipDisplayPreferencePreference>,
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationAcssDebitDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationAffirmDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationAlipayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationAlmaDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationAmazonPayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationApplePayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationApplePayLaterDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationAuBecsDebitDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationBacsDebitDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationBancontactDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationBillieDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationBillieDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationBillieDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationBillieDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationBlikDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationBlikDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationBlikDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationBlikDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationBoletoDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationCartesBancairesDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationCashappDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationCashappDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationCashappDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationCashappDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationCustomerBalanceDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationEpsDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationEpsDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationEpsDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationEpsDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationFpxDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationFpxDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationFpxDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationFpxDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationGiropayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationGooglePayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationGrabpayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationIdealDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationIdealDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationIdealDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationIdealDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationJcbDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationJcbDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationJcbDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationJcbDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationKlarnaDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationKonbiniDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationLinkDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationLinkDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationLinkDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationLinkDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationMobilepayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationMultibancoDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationNzBankAccountDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationOxxoDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationP24DisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationP24DisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationP24DisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationP24DisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationP24DisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationPayByBankDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationPaynowDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationPaypalDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationPromptpayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationRevolutPayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationSatispayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationSepaDebitDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationSofortDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationSofortDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationSofortDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationSofortDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationSwishDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationSwishDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationSwishDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationSwishDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationTwintDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationTwintDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationTwintDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationTwintDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationUsBankAccountDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationWechatPayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentMethodConfigurationZipDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentMethodConfigurationZipDisplayPreferencePreference::None => "none",
            CreatePaymentMethodConfigurationZipDisplayPreferencePreference::Off => "off",
            CreatePaymentMethodConfigurationZipDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentMethodConfigurationZipDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `PaymentMethodConfigResourceDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodConfigResourceDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl PaymentMethodConfigResourceDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodConfigResourceDisplayPreferencePreference::None => "none",
            PaymentMethodConfigResourceDisplayPreferencePreference::Off => "off",
            PaymentMethodConfigResourceDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for PaymentMethodConfigResourceDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodConfigResourceDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodConfigResourceDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `PaymentMethodConfigResourceDisplayPreference`'s `value` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodConfigResourceDisplayPreferenceValue {
    Off,
    On,
}

impl PaymentMethodConfigResourceDisplayPreferenceValue {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodConfigResourceDisplayPreferenceValue::Off => "off",
            PaymentMethodConfigResourceDisplayPreferenceValue::On => "on",
        }
    }
}

impl AsRef<str> for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn default() -> Self {
        Self::Off
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationAcssDebitDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationAcssDebitDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationAffirmDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationAffirmDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationAfterpayClearpayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationAlipayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationAlipayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationAlmaDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationAlmaDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationAmazonPayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationAmazonPayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationApplePayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationApplePayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationApplePayLaterDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationApplePayLaterDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationAuBecsDebitDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationBacsDebitDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationBacsDebitDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationBancontactDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationBancontactDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationBillieDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationBillieDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationBlikDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationBlikDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationBoletoDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationBoletoDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationCartesBancairesDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationCartesBancairesDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationCashappDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationCashappDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationCustomerBalanceDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationEpsDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationEpsDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationFpxDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationFpxDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationGiropayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationGiropayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationGooglePayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationGooglePayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationGrabpayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationGrabpayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationIdealDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationIdealDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationJcbDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationJcbDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationKlarnaDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationKlarnaDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationKonbiniDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationKonbiniDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationLinkDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationLinkDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationMobilepayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationMobilepayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationMultibancoDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationMultibancoDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationNzBankAccountDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationNzBankAccountDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationOxxoDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationOxxoDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationP24DisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationP24DisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationP24DisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationP24DisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationP24DisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationPayByBankDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationPayByBankDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationPaynowDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationPaynowDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationPaypalDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationPaypalDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationPromptpayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationPromptpayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationRevolutPayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationRevolutPayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationSatispayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationSatispayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationSepaDebitDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationSepaDebitDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationSofortDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationSofortDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationSwishDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationSwishDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationTwintDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationTwintDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationUsBankAccountDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationUsBankAccountDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationWechatPayDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationWechatPayDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdatePaymentMethodConfigurationZipDisplayPreference`'s `preference` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    None,
    Off,
    On,
}

impl UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentMethodConfigurationZipDisplayPreferencePreference::None => "none",
            UpdatePaymentMethodConfigurationZipDisplayPreferencePreference::Off => "off",
            UpdatePaymentMethodConfigurationZipDisplayPreferencePreference::On => "on",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentMethodConfigurationZipDisplayPreferencePreference {
    fn default() -> Self {
        Self::None
    }
}

/// Cards are a popular way for consumers and businesses to pay online or in person.
///
/// Stripe supports global and local card networks.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodParam {
    /// Whether or not the payment method should be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<DisplayPreference>,
}

/// Whether or not the payment method should be displayed.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisplayPreference {
    /// The account's preference for whether or not to display this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<String>,
}
