// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{CustomerId, PaymentMethodId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Timestamp};
use crate::resources::{
    Address, BillingDetails, Charge, Customer, PaymentMethodDetailsCardPresent, SetupAttempt,
};

/// The resource representing a Stripe "PaymentMethod".
///
/// For more details see <https://stripe.com/docs/api/payment_methods/object>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethod {
    /// Unique identifier for the object.
    pub id: PaymentMethodId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<Box<PaymentMethodAcssDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<Box<PaymentMethodAfterpayClearpay>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<Box<PaymentFlowsPrivatePaymentMethodsAlipay>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<Box<PaymentMethodAuBecsDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<Box<PaymentMethodBacsDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<Box<PaymentMethodBancontact>>,

    pub billing_details: BillingDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<Box<PaymentMethodBoleto>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Box<CardDetails>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<Box<CardPresent>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The ID of the Customer to which this PaymentMethod is saved.
    ///
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<Expandable<Customer>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<Box<PaymentMethodEps>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<Box<PaymentMethodFpx>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<Box<PaymentMethodGiropay>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<Box<PaymentMethodGrabpay>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<Box<PaymentMethodIdeal>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<Box<PaymentMethodInteracPresent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<Box<PaymentMethodKlarna>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<Box<PaymentMethodOxxo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<Box<PaymentMethodP24>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<Box<PaymentMethodSepaDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<Box<PaymentMethodSofort>>,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<Box<PaymentMethodWechatPay>>,
}

impl PaymentMethod {
    /// Returns a list of PaymentMethods.
    ///
    /// For listing a customer’s payment methods, you should use [List a Customer’s PaymentMethods](https://stripe.com/docs/api/payment_methods/customer_list).
    pub fn list(client: &Client, params: ListPaymentMethods<'_>) -> Response<List<PaymentMethod>> {
        client.get_query("/payment_methods", &params)
    }

    /// Creates a PaymentMethod object.
    ///
    /// Read the [Stripe.js reference](https://stripe.com/docs/stripe-js/reference#stripe-create-payment-method) to learn how to create PaymentMethods via Stripe.js.  Instead of creating a PaymentMethod directly, we recommend using the [PaymentIntents](https://stripe.com/docs/payments/accept-a-payment) API to accept a payment immediately or the [SetupIntent](https://stripe.com/docs/payments/save-and-reuse) API to collect payment method details ahead of a future payment.
    pub fn create(client: &Client, params: CreatePaymentMethod<'_>) -> Response<PaymentMethod> {
        client.post_form("/payment_methods", &params)
    }

    /// Retrieves a PaymentMethod object.
    pub fn retrieve(
        client: &Client,
        id: &PaymentMethodId,
        expand: &[&str],
    ) -> Response<PaymentMethod> {
        client.get_query(&format!("/payment_methods/{}", id), &Expand { expand })
    }

    /// Updates a PaymentMethod object.
    ///
    /// A PaymentMethod must be attached a customer to be updated.
    pub fn update(
        client: &Client,
        id: &PaymentMethodId,
        params: UpdatePaymentMethod<'_>,
    ) -> Response<PaymentMethod> {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsAlipay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodAcssDebit {
    /// Name of the bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<Box<String>>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Institution number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_number: Option<Box<String>>,

    /// Last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,

    /// Transit number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_number: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodAfterpayClearpay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodAuBecsDebit {
    /// Six-digit number identifying bank and branch associated with this bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsb_number: Option<Box<String>>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodBacsDebit {
    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,

    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodBancontact {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodBoleto {
    /// Uniquely identifies the customer tax id (CNPJ or CPF).
    pub tax_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardDetails {
    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: String,

    /// Checks on Card address and CVC if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Box<PaymentMethodCardChecks>>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: String,

    /// Details of the original PaymentMethod that created this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_from: Option<Box<PaymentMethodCardGeneratedCard>>,

    /// The last four digits of the card.
    pub last4: String,

    /// Contains information about card networks that can be used to process the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Box<Networks>>,

    /// Contains details on how this Card maybe be used for 3D Secure authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure_usage: Option<Box<ThreeDSecureUsage>>,

    /// If this Card is part of a card wallet, this contains the details of the card wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<Box<WalletDetails>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Networks {
    /// All available networks for the card.
    pub available: Vec<String>,

    /// The preferred network for the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodCardChecks {
    /// If a address line1 was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<Box<String>>,

    /// If a address postal code was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_postal_code_check: Option<Box<String>>,

    /// If a CVC was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodCardGeneratedCard {
    /// The charge that created this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<Box<String>>,

    /// Transaction-specific details of the payment method used in the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<Box<CardGeneratedFromPaymentMethodDetails>>,

    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_attempt: Option<Box<Expandable<SetupAttempt>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardGeneratedFromPaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<Box<PaymentMethodDetailsCardPresent>>,

    /// The type of payment method transaction-specific details from the transaction that generated this `card` payment method.
    ///
    /// Always `card_present`.
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardPresent {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout: Option<Box<WalletAmexExpressCheckout>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<Box<WalletApplePay>>,

    /// (For tokenized numbers only.) The last four digits of the device account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<Box<WalletGooglePay>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<Box<WalletMasterpass>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<Box<WalletSamsungPay>>,

    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, or `visa_checkout`.
    ///
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[serde(rename = "type")]
    pub type_: WalletDetailsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_checkout: Option<Box<WalletVisaCheckout>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletAmexExpressCheckout {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletApplePay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletGooglePay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletMasterpass {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Box<Address>>,

    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<String>>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<String>>,

    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Box<Address>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletSamsungPay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletVisaCheckout {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Box<Address>>,

    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<String>>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<String>>,

    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Box<Address>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodEps {
    /// The customer's bank.
    ///
    /// Should be one of `arzte_und_apotheker_bank`, `austrian_anadi_bank_ag`, `bank_austria`, `bankhaus_carl_spangler`, `bankhaus_schelhammer_und_schattera_ag`, `bawag_psk_ag`, `bks_bank_ag`, `brull_kallmus_bank_ag`, `btv_vier_lander_bank`, `capital_bank_grawe_gruppe_ag`, `dolomitenbank`, `easybank_ag`, `erste_bank_und_sparkassen`, `hypo_alpeadriabank_international_ag`, `hypo_noe_lb_fur_niederosterreich_u_wien`, `hypo_oberosterreich_salzburg_steiermark`, `hypo_tirol_bank_ag`, `hypo_vorarlberg_bank_ag`, `hypo_bank_burgenland_aktiengesellschaft`, `marchfelder_bank`, `oberbank_ag`, `raiffeisen_bankengruppe_osterreich`, `schoellerbank_ag`, `sparda_bank_wien`, `volksbank_gruppe`, `volkskreditbank_ag`, or `vr_bank_braunau`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<Box<PaymentMethodEpsBank>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodFpx {
    /// The customer's bank, if provided.
    ///
    /// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, or `pb_enterprise`.
    pub bank: PaymentMethodFpxBank,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodGiropay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodGrabpay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodIdeal {
    /// The customer's bank, if provided.
    ///
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, or `van_lanschot`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<Box<PaymentMethodIdealBank>>,

    /// The Bank Identifier Code of the customer's bank, if the bank was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<Box<PaymentMethodIdealBic>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodInteracPresent {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodKlarna {
    /// The customer's date of birth, if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<Box<PaymentFlowsPrivatePaymentMethodsKlarnaDob>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsKlarnaDob {
    /// The day of birth, between 1 and 31.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<Box<i64>>,

    /// The month of birth, between 1 and 12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Box<i64>>,

    /// The four-digit year of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<Box<i64>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodOxxo {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodP24 {
    /// The customer's bank, if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<Box<PaymentMethodP24Bank>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodSepaDebit {
    /// Bank code of bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<Box<String>>,

    /// Branch code of bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<Box<String>>,

    /// Two-letter ISO code representing the country the bank account is located in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Information about the object that generated this PaymentMethod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_from: Option<Box<SepaDebitGeneratedFrom>>,

    /// Last four characters of the IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodWechatPay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SepaDebitGeneratedFrom {
    /// The ID of the Charge that generated this PaymentMethod, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<Box<Expandable<Charge>>>,

    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_attempt: Option<Box<Expandable<SetupAttempt>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreeDSecureUsage {
    /// Whether 3D Secure is supported on this card.
    pub supported: bool,
}

/// The parameters for `PaymentMethod::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreatePaymentMethod<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<Box<CreatePaymentMethodAcssDebit>>,

    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<Box<CreatePaymentMethodAfterpayClearpay>>,

    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<Box<CreatePaymentMethodAlipay>>,

    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<Box<CreatePaymentMethodAuBecsDebit>>,

    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<Box<CreatePaymentMethodBacsDebit>>,

    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<Box<CreatePaymentMethodBancontact>>,

    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,

    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<Box<CreatePaymentMethodBoleto>>,

    /// The `Customer` to whom the original PaymentMethod is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<Box<CreatePaymentMethodEps>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<Box<CreatePaymentMethodFpx>>,

    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<Box<CreatePaymentMethodGiropay>>,

    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<Box<CreatePaymentMethodGrabpay>>,

    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<Box<CreatePaymentMethodIdeal>>,

    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<Box<CreatePaymentMethodInteracPresent>>,

    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<Box<CreatePaymentMethodKlarna>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<Box<CreatePaymentMethodOxxo>>,

    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<Box<CreatePaymentMethodP24>>,

    /// The PaymentMethod to share.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodId>,

    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<Box<CreatePaymentMethodSepaDebit>>,

    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<Box<CreatePaymentMethodSofort>>,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PaymentMethodTypeFilter>,

    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<Box<CreatePaymentMethodWechatPay>>,
}

impl<'a> CreatePaymentMethod<'a> {
    pub fn new() -> Self {
        CreatePaymentMethod {
            acss_debit: Default::default(),
            afterpay_clearpay: Default::default(),
            alipay: Default::default(),
            au_becs_debit: Default::default(),
            bacs_debit: Default::default(),
            bancontact: Default::default(),
            billing_details: Default::default(),
            boleto: Default::default(),
            customer: Default::default(),
            eps: Default::default(),
            expand: Default::default(),
            fpx: Default::default(),
            giropay: Default::default(),
            grabpay: Default::default(),
            ideal: Default::default(),
            interac_present: Default::default(),
            klarna: Default::default(),
            metadata: Default::default(),
            oxxo: Default::default(),
            p24: Default::default(),
            payment_method: Default::default(),
            sepa_debit: Default::default(),
            sofort: Default::default(),
            type_: Default::default(),
            wechat_pay: Default::default(),
        }
    }
}

/// The parameters for `PaymentMethod::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListPaymentMethods<'a> {
    /// The ID of the customer whose PaymentMethods will be retrieved.
    ///
    /// If not provided, the response list will be empty.
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

    /// A required filter on the list, based on the object `type` field.
    #[serde(rename = "type")]
    pub type_: PaymentMethodTypeFilter,
}

impl<'a> ListPaymentMethods<'a> {
    pub fn new(type_: PaymentMethodTypeFilter) -> Self {
        ListPaymentMethods {
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            type_,
        }
    }
}

/// The parameters for `PaymentMethod::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdatePaymentMethod<'a> {
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl<'a> UpdatePaymentMethod<'a> {
    pub fn new() -> Self {
        UpdatePaymentMethod {
            billing_details: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodAcssDebit {
    pub account_number: String,

    pub institution_number: String,

    pub transit_number: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodAfterpayClearpay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodAlipay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodAuBecsDebit {
    pub account_number: String,

    pub bsb_number: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodBacsDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodBancontact {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodBoleto {
    pub tax_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodEps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<Box<CreatePaymentMethodEpsBank>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodFpx {
    pub bank: CreatePaymentMethodFpxBank,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodGiropay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodGrabpay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodIdeal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<Box<CreatePaymentMethodIdealBank>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodInteracPresent {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodKlarna {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<Box<CreatePaymentMethodKlarnaDob>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodOxxo {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodP24 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<Box<CreatePaymentMethodP24Bank>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodSepaDebit {
    pub iban: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodSofort {
    pub country: CreatePaymentMethodSofortCountry,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodWechatPay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentMethodKlarnaDob {
    pub day: i64,

    pub month: i64,

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
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
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
            CreatePaymentMethodIdealBank::Rabobank => "rabobank",
            CreatePaymentMethodIdealBank::Regiobank => "regiobank",
            CreatePaymentMethodIdealBank::Revolut => "revolut",
            CreatePaymentMethodIdealBank::SnsBank => "sns_bank",
            CreatePaymentMethodIdealBank::TriodosBank => "triodos_bank",
            CreatePaymentMethodIdealBank::VanLanschot => "van_lanschot",
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
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
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
            PaymentMethodIdealBank::Rabobank => "rabobank",
            PaymentMethodIdealBank::Regiobank => "regiobank",
            PaymentMethodIdealBank::Revolut => "revolut",
            PaymentMethodIdealBank::SnsBank => "sns_bank",
            PaymentMethodIdealBank::TriodosBank => "triodos_bank",
            PaymentMethodIdealBank::VanLanschot => "van_lanschot",
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

/// An enum representing the possible values of an `PaymentMethodIdeal`'s `bic` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodIdealBic {
    #[serde(rename = "ABNANL2A")]
    Abnanl2a,
    #[serde(rename = "ASNBNL21")]
    Asnbnl21,
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
    #[serde(rename = "RABONL2U")]
    Rabonl2u,
    #[serde(rename = "RBRBNL21")]
    Rbrbnl21,
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
            PaymentMethodIdealBic::Bunqnl2a => "BUNQNL2A",
            PaymentMethodIdealBic::Fvlbnl22 => "FVLBNL22",
            PaymentMethodIdealBic::Handnl2a => "HANDNL2A",
            PaymentMethodIdealBic::Ingbnl2a => "INGBNL2A",
            PaymentMethodIdealBic::Knabnl2h => "KNABNL2H",
            PaymentMethodIdealBic::Moyonl21 => "MOYONL21",
            PaymentMethodIdealBic::Rabonl2u => "RABONL2U",
            PaymentMethodIdealBic::Rbrbnl21 => "RBRBNL21",
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

/// An enum representing the possible values of an `PaymentMethod`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodType {
    AcssDebit,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    CardPresent,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    InteracPresent,
    Klarna,
    Oxxo,
    P24,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl PaymentMethodType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodType::AcssDebit => "acss_debit",
            PaymentMethodType::AfterpayClearpay => "afterpay_clearpay",
            PaymentMethodType::Alipay => "alipay",
            PaymentMethodType::AuBecsDebit => "au_becs_debit",
            PaymentMethodType::BacsDebit => "bacs_debit",
            PaymentMethodType::Bancontact => "bancontact",
            PaymentMethodType::Boleto => "boleto",
            PaymentMethodType::Card => "card",
            PaymentMethodType::CardPresent => "card_present",
            PaymentMethodType::Eps => "eps",
            PaymentMethodType::Fpx => "fpx",
            PaymentMethodType::Giropay => "giropay",
            PaymentMethodType::Grabpay => "grabpay",
            PaymentMethodType::Ideal => "ideal",
            PaymentMethodType::InteracPresent => "interac_present",
            PaymentMethodType::Klarna => "klarna",
            PaymentMethodType::Oxxo => "oxxo",
            PaymentMethodType::P24 => "p24",
            PaymentMethodType::SepaDebit => "sepa_debit",
            PaymentMethodType::Sofort => "sofort",
            PaymentMethodType::WechatPay => "wechat_pay",
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

/// An enum representing the possible values of an `CreatePaymentMethod`'s `type_` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodTypeFilter {
    AcssDebit,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Oxxo,
    P24,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl PaymentMethodTypeFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodTypeFilter::AcssDebit => "acss_debit",
            PaymentMethodTypeFilter::AfterpayClearpay => "afterpay_clearpay",
            PaymentMethodTypeFilter::Alipay => "alipay",
            PaymentMethodTypeFilter::AuBecsDebit => "au_becs_debit",
            PaymentMethodTypeFilter::BacsDebit => "bacs_debit",
            PaymentMethodTypeFilter::Bancontact => "bancontact",
            PaymentMethodTypeFilter::Boleto => "boleto",
            PaymentMethodTypeFilter::Card => "card",
            PaymentMethodTypeFilter::Eps => "eps",
            PaymentMethodTypeFilter::Fpx => "fpx",
            PaymentMethodTypeFilter::Giropay => "giropay",
            PaymentMethodTypeFilter::Grabpay => "grabpay",
            PaymentMethodTypeFilter::Ideal => "ideal",
            PaymentMethodTypeFilter::Klarna => "klarna",
            PaymentMethodTypeFilter::Oxxo => "oxxo",
            PaymentMethodTypeFilter::P24 => "p24",
            PaymentMethodTypeFilter::SepaDebit => "sepa_debit",
            PaymentMethodTypeFilter::Sofort => "sofort",
            PaymentMethodTypeFilter::WechatPay => "wechat_pay",
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

/// An enum representing the possible values of an `WalletDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WalletDetailsType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
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
