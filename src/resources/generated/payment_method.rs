// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{CustomerId, PaymentMethodId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, Timestamp};
use crate::resources::{
    Address, BillingDetails, Charge, Customer, PaymentMethodCardPresentNetworks, RadarRadarOptions,
    SetupAttempt,
};
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
    pub fn retrieve(
        client: &Client,
        id: &PaymentMethodId,
        expand: &[&str],
    ) -> Response<PaymentMethod> {
        client.get_query(&format!("/payment_methods/{}", id), Expand { expand })
    }

    /// Updates a PaymentMethod object.
    ///
    /// A PaymentMethod must be attached a customer to be updated.
    pub fn update(
        client: &Client,
        id: &PaymentMethodId,
        params: UpdatePaymentMethod<'_>,
    ) -> Response<PaymentMethod> {
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsAlipay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodAcssDebit {
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Institution number of the bank account.
    pub institution_number: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// Transit number of the bank account.
    pub transit_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodAffirm {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodAfterpayClearpay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodAuBecsDebit {
    /// Six-digit number identifying bank and branch associated with this bank account.
    pub bsb_number: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodBacsDebit {
    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    pub sort_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodBancontact {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodBlik {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodBoleto {
    /// Uniquely identifies the customer tax id (CNPJ or CPF).
    pub tax_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CardDetails {
    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: String,

    /// Checks on Card address and CVC if provided.
    pub checks: Option<PaymentMethodCardChecks>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,

    /// A high-level description of the type of cards issued in this range.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: String,

    /// Issuer identification number of the card.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,

    /// The name of the card's issuing bank.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// The last four digits of the card.
    pub last4: String,

    /// Contains information about card networks that can be used to process the payment.
    pub networks: Option<Networks>,

    /// Contains details on how this Card may be used for 3D Secure authentication.
    pub three_d_secure_usage: Option<ThreeDSecureUsage>,

    /// If this Card is part of a card wallet, this contains the details of the card wallet.
    pub wallet: Option<WalletDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Networks {
    /// All available networks for the card.
    pub available: Vec<String>,

    /// The preferred network for co-branded cards.
    ///
    /// Can be `cartes_bancaires`, `mastercard`, `visa` or `invalid_preference` if requested network is not valid for the card.
    pub preferred: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodCardChecks {
    /// If a address line1 was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Option<String>,

    /// If a address postal code was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_postal_code_check: Option<String>,

    /// If a CVC was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub cvc_check: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CardPresent {
    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: Option<String>,

    /// The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format.
    ///
    /// May include alphanumeric characters, special characters and first/last name separator (`/`).
    /// In some cases, the cardholder name may not be available depending on how the issuer has configured the card.
    /// Cardholder name is typically not available on swipe or contactless payments, such as those made with Apple Pay and Google Pay.
    pub cardholder_name: Option<String>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,

    /// A high-level description of the type of cards issued in this range.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
    pub fingerprint: Option<String>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,

    /// Issuer identification number of the card.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,

    /// The name of the card's issuing bank.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// The last four digits of the card.
    pub last4: Option<String>,

    /// Contains information about card networks that can be used to process the payment.
    pub networks: Option<PaymentMethodCardPresentNetworks>,

    /// How card details were read in this transaction.
    pub read_method: Option<CardPresentReadMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WalletDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout: Option<WalletAmexExpressCheckout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<WalletApplePay>,

    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<WalletGooglePay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodCardWalletLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<WalletMasterpass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<WalletSamsungPay>,

    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, `visa_checkout`, or `link`.
    ///
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[serde(rename = "type")]
    pub type_: WalletDetailsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_checkout: Option<WalletVisaCheckout>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WalletAmexExpressCheckout {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WalletApplePay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WalletGooglePay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodCardWalletLink {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WalletMasterpass {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub billing_address: Option<Address>,

    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub email: Option<String>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub name: Option<String>,

    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub shipping_address: Option<Address>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WalletSamsungPay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WalletVisaCheckout {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub billing_address: Option<Address>,

    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub email: Option<String>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub name: Option<String>,

    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub shipping_address: Option<Address>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodCashapp {
    /// A unique and immutable identifier assigned by Cash App to every buyer.
    pub buyer_id: Option<String>,

    /// A public identifier for buyers using Cash App.
    pub cashtag: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodCustomerBalance {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodEps {
    /// The customer's bank.
    ///
    /// Should be one of `arzte_und_apotheker_bank`, `austrian_anadi_bank_ag`, `bank_austria`, `bankhaus_carl_spangler`, `bankhaus_schelhammer_und_schattera_ag`, `bawag_psk_ag`, `bks_bank_ag`, `brull_kallmus_bank_ag`, `btv_vier_lander_bank`, `capital_bank_grawe_gruppe_ag`, `deutsche_bank_ag`, `dolomitenbank`, `easybank_ag`, `erste_bank_und_sparkassen`, `hypo_alpeadriabank_international_ag`, `hypo_noe_lb_fur_niederosterreich_u_wien`, `hypo_oberosterreich_salzburg_steiermark`, `hypo_tirol_bank_ag`, `hypo_vorarlberg_bank_ag`, `hypo_bank_burgenland_aktiengesellschaft`, `marchfelder_bank`, `oberbank_ag`, `raiffeisen_bankengruppe_osterreich`, `schoellerbank_ag`, `sparda_bank_wien`, `volksbank_gruppe`, `volkskreditbank_ag`, or `vr_bank_braunau`.
    pub bank: Option<PaymentMethodEpsBank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodFpx {
    /// Account holder type, if provided.
    ///
    /// Can be one of `individual` or `company`.
    pub account_holder_type: Option<PaymentMethodFpxAccountHolderType>,

    /// The customer's bank, if provided.
    ///
    /// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, `pb_enterprise`, or `bank_of_china`.
    pub bank: PaymentMethodFpxBank,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodGiropay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodGrabpay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodIdeal {
    /// The customer's bank, if provided.
    ///
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `n26`, `nn`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
    pub bank: Option<PaymentMethodIdealBank>,

    /// The Bank Identifier Code of the customer's bank, if the bank was provided.
    pub bic: Option<PaymentMethodIdealBic>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodInteracPresent {
    /// Card brand.
    ///
    /// Can be `interac`, `mastercard` or `visa`.
    pub brand: Option<String>,

    /// The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format.
    ///
    /// May include alphanumeric characters, special characters and first/last name separator (`/`).
    /// In some cases, the cardholder name may not be available depending on how the issuer has configured the card.
    /// Cardholder name is typically not available on swipe or contactless payments, such as those made with Apple Pay and Google Pay.
    pub cardholder_name: Option<String>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,

    /// A high-level description of the type of cards issued in this range.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
    pub fingerprint: Option<String>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,

    /// Issuer identification number of the card.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,

    /// The name of the card's issuing bank.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// The last four digits of the card.
    pub last4: Option<String>,

    /// Contains information about card networks that can be used to process the payment.
    pub networks: Option<PaymentMethodCardPresentNetworks>,

    /// EMV tag 5F2D.
    ///
    /// Preferred languages specified by the integrated circuit chip.
    pub preferred_locales: Option<Vec<String>>,

    /// How card details were read in this transaction.
    pub read_method: Option<PaymentMethodInteracPresentReadMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodKlarna {
    /// The customer's date of birth, if provided.
    pub dob: Option<PaymentFlowsPrivatePaymentMethodsKlarnaDob>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: Option<i64>,

    /// The month of birth, between 1 and 12.
    pub month: Option<i64>,

    /// The four-digit year of birth.
    pub year: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodKonbini {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodLink {
    /// Account owner's email address.
    pub email: Option<String>,

    /// [Deprecated] This is a legacy parameter that no longer has any function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodOxxo {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodP24 {
    /// The customer's bank, if provided.
    pub bank: Option<PaymentMethodP24Bank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodPaynow {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodPaypal {
    /// Owner's email.
    ///
    /// Values are provided by PayPal directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub payer_email: Option<String>,

    /// PayPal account PayerID.
    ///
    /// This identifier uniquely identifies the PayPal customer.
    pub payer_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodPix {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodPromptpay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodRevolutPay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodSepaDebit {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,

    /// Branch code of bank associated with the bank account.
    pub branch_code: Option<String>,

    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Information about the object that generated this PaymentMethod.
    pub generated_from: Option<SepaDebitGeneratedFrom>,

    /// Last four characters of the IBAN.
    pub last4: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodSwish {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type: Option<PaymentMethodUsBankAccountAccountHolderType>,

    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    pub account_type: Option<PaymentMethodUsBankAccountAccountType>,

    /// The name of the bank.
    pub bank_name: Option<String>,

    /// The ID of the Financial Connections Account used to create the payment method.
    pub financial_connections_account: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// Contains information about US bank account networks that can be used.
    pub networks: Option<UsBankAccountNetworks>,

    /// Routing number of the bank account.
    pub routing_number: Option<String>,

    /// Contains information about the future reusability of this PaymentMethod.
    pub status_details: Option<PaymentMethodUsBankAccountStatusDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodUsBankAccountStatusDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<PaymentMethodUsBankAccountBlocked>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodUsBankAccountBlocked {
    /// The ACH network code that resulted in this block.
    pub network_code: Option<PaymentMethodUsBankAccountBlockedNetworkCode>,

    /// The reason why this PaymentMethod's fingerprint has been blocked.
    pub reason: Option<PaymentMethodUsBankAccountBlockedReason>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodWechatPay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodZip {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SepaDebitGeneratedFrom {
    /// The ID of the Charge that generated this PaymentMethod, if any.
    pub charge: Option<Expandable<Charge>>,

    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<Expandable<SetupAttempt>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ThreeDSecureUsage {
    /// Whether 3D Secure is supported on this card.
    pub supported: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UsBankAccountNetworks {
    /// The preferred network.
    pub preferred: Option<String>,

    /// All supported networks.
    pub supported: Vec<UsBankAccountNetworksSupported>,
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
    }
}
/// The parameters for `PaymentMethod::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdatePaymentMethod<'a> {
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
pub struct CreatePaymentMethodAffirm {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodAfterpayClearpay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodAlipay {}

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
pub struct CreatePaymentMethodBancontact {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodBlik {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodBoleto {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodCashapp {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodCustomerBalance {}

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
pub struct CreatePaymentMethodGiropay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodGrabpay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentMethodIdealBank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodInteracPresent {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodKlarna {
    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreatePaymentMethodKlarnaDob>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodKonbini {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodLink {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodOxxo {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentMethodP24Bank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodPaynow {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodPaypal {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodPix {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodPromptpay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodRadarOptions {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodRevolutPay {}

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
pub struct CreatePaymentMethodSwish {}

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
pub struct CreatePaymentMethodWechatPay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentMethodZip {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentMethodLink {}

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

/// An enum representing the possible values of an `CardPresent`'s `read_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CardPresentReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}

impl CardPresentReadMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CardPresentReadMethod::ContactEmv => "contact_emv",
            CardPresentReadMethod::ContactlessEmv => "contactless_emv",
            CardPresentReadMethod::ContactlessMagstripeMode => "contactless_magstripe_mode",
            CardPresentReadMethod::MagneticStripeFallback => "magnetic_stripe_fallback",
            CardPresentReadMethod::MagneticStripeTrack2 => "magnetic_stripe_track2",
        }
    }
}

impl AsRef<str> for CardPresentReadMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardPresentReadMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CardPresentReadMethod {
    fn default() -> Self {
        Self::ContactEmv
    }
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
            CreatePaymentMethodEpsBank::BankhausSchelhammerUndSchatteraAg => {
                "bankhaus_schelhammer_und_schattera_ag"
            }
            CreatePaymentMethodEpsBank::BawagPskAg => "bawag_psk_ag",
            CreatePaymentMethodEpsBank::BksBankAg => "bks_bank_ag",
            CreatePaymentMethodEpsBank::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            CreatePaymentMethodEpsBank::BtvVierLanderBank => "btv_vier_lander_bank",
            CreatePaymentMethodEpsBank::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            CreatePaymentMethodEpsBank::DeutscheBankAg => "deutsche_bank_ag",
            CreatePaymentMethodEpsBank::Dolomitenbank => "dolomitenbank",
            CreatePaymentMethodEpsBank::EasybankAg => "easybank_ag",
            CreatePaymentMethodEpsBank::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            CreatePaymentMethodEpsBank::HypoAlpeadriabankInternationalAg => {
                "hypo_alpeadriabank_international_ag"
            }
            CreatePaymentMethodEpsBank::HypoBankBurgenlandAktiengesellschaft => {
                "hypo_bank_burgenland_aktiengesellschaft"
            }
            CreatePaymentMethodEpsBank::HypoNoeLbFurNiederosterreichUWien => {
                "hypo_noe_lb_fur_niederosterreich_u_wien"
            }
            CreatePaymentMethodEpsBank::HypoOberosterreichSalzburgSteiermark => {
                "hypo_oberosterreich_salzburg_steiermark"
            }
            CreatePaymentMethodEpsBank::HypoTirolBankAg => "hypo_tirol_bank_ag",
            CreatePaymentMethodEpsBank::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            CreatePaymentMethodEpsBank::MarchfelderBank => "marchfelder_bank",
            CreatePaymentMethodEpsBank::OberbankAg => "oberbank_ag",
            CreatePaymentMethodEpsBank::RaiffeisenBankengruppeOsterreich => {
                "raiffeisen_bankengruppe_osterreich"
            }
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

/// An enum representing the possible values of an `PaymentMethodEps`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodEpsBank {
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

impl PaymentMethodEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodEpsBank::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            PaymentMethodEpsBank::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            PaymentMethodEpsBank::BankAustria => "bank_austria",
            PaymentMethodEpsBank::BankhausCarlSpangler => "bankhaus_carl_spangler",
            PaymentMethodEpsBank::BankhausSchelhammerUndSchatteraAg => {
                "bankhaus_schelhammer_und_schattera_ag"
            }
            PaymentMethodEpsBank::BawagPskAg => "bawag_psk_ag",
            PaymentMethodEpsBank::BksBankAg => "bks_bank_ag",
            PaymentMethodEpsBank::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            PaymentMethodEpsBank::BtvVierLanderBank => "btv_vier_lander_bank",
            PaymentMethodEpsBank::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            PaymentMethodEpsBank::DeutscheBankAg => "deutsche_bank_ag",
            PaymentMethodEpsBank::Dolomitenbank => "dolomitenbank",
            PaymentMethodEpsBank::EasybankAg => "easybank_ag",
            PaymentMethodEpsBank::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            PaymentMethodEpsBank::HypoAlpeadriabankInternationalAg => {
                "hypo_alpeadriabank_international_ag"
            }
            PaymentMethodEpsBank::HypoBankBurgenlandAktiengesellschaft => {
                "hypo_bank_burgenland_aktiengesellschaft"
            }
            PaymentMethodEpsBank::HypoNoeLbFurNiederosterreichUWien => {
                "hypo_noe_lb_fur_niederosterreich_u_wien"
            }
            PaymentMethodEpsBank::HypoOberosterreichSalzburgSteiermark => {
                "hypo_oberosterreich_salzburg_steiermark"
            }
            PaymentMethodEpsBank::HypoTirolBankAg => "hypo_tirol_bank_ag",
            PaymentMethodEpsBank::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            PaymentMethodEpsBank::MarchfelderBank => "marchfelder_bank",
            PaymentMethodEpsBank::OberbankAg => "oberbank_ag",
            PaymentMethodEpsBank::RaiffeisenBankengruppeOsterreich => {
                "raiffeisen_bankengruppe_osterreich"
            }
            PaymentMethodEpsBank::SchoellerbankAg => "schoellerbank_ag",
            PaymentMethodEpsBank::SpardaBankWien => "sparda_bank_wien",
            PaymentMethodEpsBank::VolksbankGruppe => "volksbank_gruppe",
            PaymentMethodEpsBank::VolkskreditbankAg => "volkskreditbank_ag",
            PaymentMethodEpsBank::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for PaymentMethodEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodEpsBank {
    fn default() -> Self {
        Self::ArzteUndApothekerBank
    }
}

/// An enum representing the possible values of an `PaymentMethodFpx`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodFpxAccountHolderType {
    Company,
    Individual,
}

impl PaymentMethodFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodFpxAccountHolderType::Company => "company",
            PaymentMethodFpxAccountHolderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for PaymentMethodFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodFpxAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `PaymentMethodFpx`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodFpxBank {
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

impl PaymentMethodFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodFpxBank::AffinBank => "affin_bank",
            PaymentMethodFpxBank::Agrobank => "agrobank",
            PaymentMethodFpxBank::AllianceBank => "alliance_bank",
            PaymentMethodFpxBank::Ambank => "ambank",
            PaymentMethodFpxBank::BankIslam => "bank_islam",
            PaymentMethodFpxBank::BankMuamalat => "bank_muamalat",
            PaymentMethodFpxBank::BankOfChina => "bank_of_china",
            PaymentMethodFpxBank::BankRakyat => "bank_rakyat",
            PaymentMethodFpxBank::Bsn => "bsn",
            PaymentMethodFpxBank::Cimb => "cimb",
            PaymentMethodFpxBank::DeutscheBank => "deutsche_bank",
            PaymentMethodFpxBank::HongLeongBank => "hong_leong_bank",
            PaymentMethodFpxBank::Hsbc => "hsbc",
            PaymentMethodFpxBank::Kfh => "kfh",
            PaymentMethodFpxBank::Maybank2e => "maybank2e",
            PaymentMethodFpxBank::Maybank2u => "maybank2u",
            PaymentMethodFpxBank::Ocbc => "ocbc",
            PaymentMethodFpxBank::PbEnterprise => "pb_enterprise",
            PaymentMethodFpxBank::PublicBank => "public_bank",
            PaymentMethodFpxBank::Rhb => "rhb",
            PaymentMethodFpxBank::StandardChartered => "standard_chartered",
            PaymentMethodFpxBank::Uob => "uob",
        }
    }
}

impl AsRef<str> for PaymentMethodFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodFpxBank {
    fn default() -> Self {
        Self::AffinBank
    }
}

/// An enum representing the possible values of an `PaymentMethodIdeal`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodIdealBank {
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

impl PaymentMethodIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodIdealBank::AbnAmro => "abn_amro",
            PaymentMethodIdealBank::AsnBank => "asn_bank",
            PaymentMethodIdealBank::Bunq => "bunq",
            PaymentMethodIdealBank::Handelsbanken => "handelsbanken",
            PaymentMethodIdealBank::Ing => "ing",
            PaymentMethodIdealBank::Knab => "knab",
            PaymentMethodIdealBank::Moneyou => "moneyou",
            PaymentMethodIdealBank::N26 => "n26",
            PaymentMethodIdealBank::Nn => "nn",
            PaymentMethodIdealBank::Rabobank => "rabobank",
            PaymentMethodIdealBank::Regiobank => "regiobank",
            PaymentMethodIdealBank::Revolut => "revolut",
            PaymentMethodIdealBank::SnsBank => "sns_bank",
            PaymentMethodIdealBank::TriodosBank => "triodos_bank",
            PaymentMethodIdealBank::VanLanschot => "van_lanschot",
            PaymentMethodIdealBank::Yoursafe => "yoursafe",
        }
    }
}

impl AsRef<str> for PaymentMethodIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodIdealBank {
    fn default() -> Self {
        Self::AbnAmro
    }
}

/// An enum representing the possible values of an `PaymentMethodIdeal`'s `bic` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodIdealBic {
    #[serde(rename = "ABNANL2A")]
    Abnanl2a,
    #[serde(rename = "ASNBNL21")]
    Asnbnl21,
    #[serde(rename = "BITSNL2A")]
    Bitsnl2a,
    #[serde(rename = "BUNQNL2A")]
    Bunqnl2a,
    #[serde(rename = "FVLBNL22")]
    Fvlbnl22,
    #[serde(rename = "HANDNL2A")]
    Handnl2a,
    #[serde(rename = "INGBNL2A")]
    Ingbnl2a,
    #[serde(rename = "KNABNL2H")]
    Knabnl2h,
    #[serde(rename = "MOYONL21")]
    Moyonl21,
    #[serde(rename = "NNBANL2G")]
    Nnbanl2g,
    #[serde(rename = "NTSBDEB1")]
    Ntsbdeb1,
    #[serde(rename = "RABONL2U")]
    Rabonl2u,
    #[serde(rename = "RBRBNL21")]
    Rbrbnl21,
    #[serde(rename = "REVOIE23")]
    Revoie23,
    #[serde(rename = "REVOLT21")]
    Revolt21,
    #[serde(rename = "SNSBNL2A")]
    Snsbnl2a,
    #[serde(rename = "TRIONL2U")]
    Trionl2u,
}

impl PaymentMethodIdealBic {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodIdealBic::Abnanl2a => "ABNANL2A",
            PaymentMethodIdealBic::Asnbnl21 => "ASNBNL21",
            PaymentMethodIdealBic::Bitsnl2a => "BITSNL2A",
            PaymentMethodIdealBic::Bunqnl2a => "BUNQNL2A",
            PaymentMethodIdealBic::Fvlbnl22 => "FVLBNL22",
            PaymentMethodIdealBic::Handnl2a => "HANDNL2A",
            PaymentMethodIdealBic::Ingbnl2a => "INGBNL2A",
            PaymentMethodIdealBic::Knabnl2h => "KNABNL2H",
            PaymentMethodIdealBic::Moyonl21 => "MOYONL21",
            PaymentMethodIdealBic::Nnbanl2g => "NNBANL2G",
            PaymentMethodIdealBic::Ntsbdeb1 => "NTSBDEB1",
            PaymentMethodIdealBic::Rabonl2u => "RABONL2U",
            PaymentMethodIdealBic::Rbrbnl21 => "RBRBNL21",
            PaymentMethodIdealBic::Revoie23 => "REVOIE23",
            PaymentMethodIdealBic::Revolt21 => "REVOLT21",
            PaymentMethodIdealBic::Snsbnl2a => "SNSBNL2A",
            PaymentMethodIdealBic::Trionl2u => "TRIONL2U",
        }
    }
}

impl AsRef<str> for PaymentMethodIdealBic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodIdealBic {
    fn default() -> Self {
        Self::Abnanl2a
    }
}

/// An enum representing the possible values of an `PaymentMethodInteracPresent`'s `read_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodInteracPresentReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}

impl PaymentMethodInteracPresentReadMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodInteracPresentReadMethod::ContactEmv => "contact_emv",
            PaymentMethodInteracPresentReadMethod::ContactlessEmv => "contactless_emv",
            PaymentMethodInteracPresentReadMethod::ContactlessMagstripeMode => {
                "contactless_magstripe_mode"
            }
            PaymentMethodInteracPresentReadMethod::MagneticStripeFallback => {
                "magnetic_stripe_fallback"
            }
            PaymentMethodInteracPresentReadMethod::MagneticStripeTrack2 => "magnetic_stripe_track2",
        }
    }
}

impl AsRef<str> for PaymentMethodInteracPresentReadMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodInteracPresentReadMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodInteracPresentReadMethod {
    fn default() -> Self {
        Self::ContactEmv
    }
}

/// An enum representing the possible values of an `PaymentMethodP24`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodP24Bank {
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

impl PaymentMethodP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodP24Bank::AliorBank => "alior_bank",
            PaymentMethodP24Bank::BankMillennium => "bank_millennium",
            PaymentMethodP24Bank::BankNowyBfgSa => "bank_nowy_bfg_sa",
            PaymentMethodP24Bank::BankPekaoSa => "bank_pekao_sa",
            PaymentMethodP24Bank::BankiSpbdzielcze => "banki_spbdzielcze",
            PaymentMethodP24Bank::Blik => "blik",
            PaymentMethodP24Bank::BnpParibas => "bnp_paribas",
            PaymentMethodP24Bank::Boz => "boz",
            PaymentMethodP24Bank::CitiHandlowy => "citi_handlowy",
            PaymentMethodP24Bank::CreditAgricole => "credit_agricole",
            PaymentMethodP24Bank::Envelobank => "envelobank",
            PaymentMethodP24Bank::EtransferPocztowy24 => "etransfer_pocztowy24",
            PaymentMethodP24Bank::GetinBank => "getin_bank",
            PaymentMethodP24Bank::Ideabank => "ideabank",
            PaymentMethodP24Bank::Ing => "ing",
            PaymentMethodP24Bank::Inteligo => "inteligo",
            PaymentMethodP24Bank::MbankMtransfer => "mbank_mtransfer",
            PaymentMethodP24Bank::NestPrzelew => "nest_przelew",
            PaymentMethodP24Bank::NoblePay => "noble_pay",
            PaymentMethodP24Bank::PbacZIpko => "pbac_z_ipko",
            PaymentMethodP24Bank::PlusBank => "plus_bank",
            PaymentMethodP24Bank::SantanderPrzelew24 => "santander_przelew24",
            PaymentMethodP24Bank::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            PaymentMethodP24Bank::ToyotaBank => "toyota_bank",
            PaymentMethodP24Bank::Velobank => "velobank",
            PaymentMethodP24Bank::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for PaymentMethodP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodP24Bank {
    fn default() -> Self {
        Self::AliorBank
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
}

impl PaymentMethodType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodType::AcssDebit => "acss_debit",
            PaymentMethodType::Affirm => "affirm",
            PaymentMethodType::AfterpayClearpay => "afterpay_clearpay",
            PaymentMethodType::Alipay => "alipay",
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
}

impl PaymentMethodTypeFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodTypeFilter::AcssDebit => "acss_debit",
            PaymentMethodTypeFilter::Affirm => "affirm",
            PaymentMethodTypeFilter::AfterpayClearpay => "afterpay_clearpay",
            PaymentMethodTypeFilter::Alipay => "alipay",
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

/// An enum representing the possible values of an `PaymentMethodUsBankAccount`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl PaymentMethodUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodUsBankAccountAccountHolderType::Company => "company",
            PaymentMethodUsBankAccountAccountHolderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for PaymentMethodUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodUsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `PaymentMethodUsBankAccount`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodUsBankAccountAccountType {
    Checking,
    Savings,
}

impl PaymentMethodUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodUsBankAccountAccountType::Checking => "checking",
            PaymentMethodUsBankAccountAccountType::Savings => "savings",
        }
    }
}

impl AsRef<str> for PaymentMethodUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodUsBankAccountAccountType {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `PaymentMethodUsBankAccountBlocked`'s `network_code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodUsBankAccountBlockedNetworkCode {
    #[serde(rename = "R02")]
    R02,
    #[serde(rename = "R03")]
    R03,
    #[serde(rename = "R04")]
    R04,
    #[serde(rename = "R05")]
    R05,
    #[serde(rename = "R07")]
    R07,
    #[serde(rename = "R08")]
    R08,
    #[serde(rename = "R10")]
    R10,
    #[serde(rename = "R11")]
    R11,
    #[serde(rename = "R16")]
    R16,
    #[serde(rename = "R20")]
    R20,
    #[serde(rename = "R29")]
    R29,
    #[serde(rename = "R31")]
    R31,
}

impl PaymentMethodUsBankAccountBlockedNetworkCode {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodUsBankAccountBlockedNetworkCode::R02 => "R02",
            PaymentMethodUsBankAccountBlockedNetworkCode::R03 => "R03",
            PaymentMethodUsBankAccountBlockedNetworkCode::R04 => "R04",
            PaymentMethodUsBankAccountBlockedNetworkCode::R05 => "R05",
            PaymentMethodUsBankAccountBlockedNetworkCode::R07 => "R07",
            PaymentMethodUsBankAccountBlockedNetworkCode::R08 => "R08",
            PaymentMethodUsBankAccountBlockedNetworkCode::R10 => "R10",
            PaymentMethodUsBankAccountBlockedNetworkCode::R11 => "R11",
            PaymentMethodUsBankAccountBlockedNetworkCode::R16 => "R16",
            PaymentMethodUsBankAccountBlockedNetworkCode::R20 => "R20",
            PaymentMethodUsBankAccountBlockedNetworkCode::R29 => "R29",
            PaymentMethodUsBankAccountBlockedNetworkCode::R31 => "R31",
        }
    }
}

impl AsRef<str> for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn default() -> Self {
        Self::R02
    }
}

/// An enum representing the possible values of an `PaymentMethodUsBankAccountBlocked`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodUsBankAccountBlockedReason {
    BankAccountClosed,
    BankAccountFrozen,
    BankAccountInvalidDetails,
    BankAccountRestricted,
    BankAccountUnusable,
    DebitNotAuthorized,
}

impl PaymentMethodUsBankAccountBlockedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodUsBankAccountBlockedReason::BankAccountClosed => "bank_account_closed",
            PaymentMethodUsBankAccountBlockedReason::BankAccountFrozen => "bank_account_frozen",
            PaymentMethodUsBankAccountBlockedReason::BankAccountInvalidDetails => {
                "bank_account_invalid_details"
            }
            PaymentMethodUsBankAccountBlockedReason::BankAccountRestricted => {
                "bank_account_restricted"
            }
            PaymentMethodUsBankAccountBlockedReason::BankAccountUnusable => "bank_account_unusable",
            PaymentMethodUsBankAccountBlockedReason::DebitNotAuthorized => "debit_not_authorized",
        }
    }
}

impl AsRef<str> for PaymentMethodUsBankAccountBlockedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodUsBankAccountBlockedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodUsBankAccountBlockedReason {
    fn default() -> Self {
        Self::BankAccountClosed
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

/// An enum representing the possible values of an `UsBankAccountNetworks`'s `supported` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UsBankAccountNetworksSupported {
    Ach,
    UsDomesticWire,
}

impl UsBankAccountNetworksSupported {
    pub fn as_str(self) -> &'static str {
        match self {
            UsBankAccountNetworksSupported::Ach => "ach",
            UsBankAccountNetworksSupported::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for UsBankAccountNetworksSupported {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UsBankAccountNetworksSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UsBankAccountNetworksSupported {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `WalletDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WalletDetailsType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Link,
    Masterpass,
    SamsungPay,
    VisaCheckout,
}

impl WalletDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            WalletDetailsType::AmexExpressCheckout => "amex_express_checkout",
            WalletDetailsType::ApplePay => "apple_pay",
            WalletDetailsType::GooglePay => "google_pay",
            WalletDetailsType::Link => "link",
            WalletDetailsType::Masterpass => "masterpass",
            WalletDetailsType::SamsungPay => "samsung_pay",
            WalletDetailsType::VisaCheckout => "visa_checkout",
        }
    }
}

impl AsRef<str> for WalletDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for WalletDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for WalletDetailsType {
    fn default() -> Self {
        Self::AmexExpressCheckout
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
    CardDetailsParams(CardDetailsParams),
    TokenParams(TokenParams),
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
    /// The card number, as a string without any separators.
    pub number: String,
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
}
