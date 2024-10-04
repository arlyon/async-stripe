// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Expandable};
use crate::resources::{Address, PaymentMethodDetailsCardPresent, SetupAttempt};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_card".
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

    /// The brand to use when displaying the card, this accounts for customer's brand choice on dual-branded cards.
    ///
    /// Can be `american_express`, `cartes_bancaires`, `diners_club`, `discover`, `eftpos_australia`, `interac`, `jcb`, `mastercard`, `union_pay`, `visa`, or `other` and may contain more values in the future.
    pub display_brand: Option<String>,

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

    /// Details of the original PaymentMethod that created this object.
    pub generated_from: Option<PaymentMethodCardGeneratedCard>,

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
pub struct PaymentMethodCardGeneratedCard {

    /// The charge that created this object.
    pub charge: Option<String>,

    /// Transaction-specific details of the payment method used in the payment.
    pub payment_method_details: Option<CardGeneratedFromPaymentMethodDetails>,

    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<Expandable<SetupAttempt>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CardGeneratedFromPaymentMethodDetails {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<PaymentMethodDetailsCardPresent>,

    /// The type of payment method transaction-specific details from the transaction that generated this `card` payment method.
    ///
    /// Always `card_present`.
    #[serde(rename = "type")]
    pub type_: String,
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
pub struct WalletAmexExpressCheckout {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WalletApplePay {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WalletGooglePay {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodCardWalletLink {
}

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
pub struct WalletSamsungPay {
}

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
pub struct ThreeDSecureUsage {

    /// Whether 3D Secure is supported on this card.
    pub supported: bool,
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
