// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Timestamp};
use crate::resources::{PaymentFlowsPrivatePaymentMethodsCardPresentCommonWallet, PaymentMethodDetailsCardPresentOffline};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_details_card_present".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardPresent {

    /// The authorized amount.
    pub amount_authorized: Option<i64>,

    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: Option<String>,

    /// The [product code](https://stripe.com/docs/card-product-codes) that identifies the specific program or product associated with a card.
    pub brand_product: Option<String>,

    /// When using manual capture, a future timestamp after which the charge will be automatically refunded if uncaptured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_before: Option<Timestamp>,

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

    /// Authorization response cryptogram.
    pub emv_auth_data: Option<String>,

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

    /// ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions.
    ///
    /// Only present if it was possible to generate a card PaymentMethod.
    pub generated_card: Option<String>,

    /// Issuer identification number of the card.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,

    /// Whether this [PaymentIntent](https://stripe.com/docs/api/payment_intents) is eligible for incremental authorizations.
    ///
    /// Request support using [request_incremental_authorization_support](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-payment_method_options-card_present-request_incremental_authorization_support).
    pub incremental_authorization_supported: bool,

    /// The name of the card's issuing bank.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// The last four digits of the card.
    pub last4: Option<String>,

    /// Identifies which network this charge was processed on.
    ///
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub network: Option<String>,

    /// This is used by the financial networks to identify a transaction.
    ///
    /// Visa calls this the Transaction ID, Mastercard calls this the Trace ID, and American Express calls this the Acquirer Reference Data.
    /// The first three digits of the Trace ID is the Financial Network Code, the next 6 digits is the Banknet Reference Number, and the last 4 digits represent the date (MM/DD).
    /// This field will be available for successful Visa, Mastercard, or American Express transactions and always null for other card brands.
    pub network_transaction_id: Option<String>,

    /// Details about payments collected offline.
    pub offline: Option<PaymentMethodDetailsCardPresentOffline>,

    /// Defines whether the authorized amount can be over-captured or not.
    pub overcapture_supported: bool,

    /// EMV tag 5F2D.
    ///
    /// Preferred languages specified by the integrated circuit chip.
    pub preferred_locales: Option<Vec<String>>,

    /// How card details were read in this transaction.
    pub read_method: Option<PaymentMethodDetailsCardPresentReadMethod>,

    /// A collection of fields required to be displayed on receipts.
    ///
    /// Only required for EMV transactions.
    pub receipt: Option<PaymentMethodDetailsCardPresentReceipt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<PaymentFlowsPrivatePaymentMethodsCardPresentCommonWallet>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardPresentReceipt {

    /// The type of account being debited or credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<PaymentMethodDetailsCardPresentReceiptAccountType>,

    /// EMV tag 9F26, cryptogram generated by the integrated circuit chip.
    pub application_cryptogram: Option<String>,

    /// Mnenomic of the Application Identifier.
    pub application_preferred_name: Option<String>,

    /// Identifier for this transaction.
    pub authorization_code: Option<String>,

    /// EMV tag 8A.
    ///
    /// A code returned by the card issuer.
    pub authorization_response_code: Option<String>,

    /// Describes the method used by the cardholder to verify ownership of the card.
    ///
    /// One of the following: `approval`, `failure`, `none`, `offline_pin`, `offline_pin_and_signature`, `online_pin`, or `signature`.
    pub cardholder_verification_method: Option<String>,

    /// EMV tag 84.
    ///
    /// Similar to the application identifier stored on the integrated circuit chip.
    pub dedicated_file_name: Option<String>,

    /// The outcome of a series of EMV functions performed by the card reader.
    pub terminal_verification_results: Option<String>,

    /// An indication of various EMV functions performed during the transaction.
    pub transaction_status_information: Option<String>,
}

/// An enum representing the possible values of an `PaymentMethodDetailsCardPresent`'s `read_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardPresentReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}

impl PaymentMethodDetailsCardPresentReadMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsCardPresentReadMethod::ContactEmv => "contact_emv",
            PaymentMethodDetailsCardPresentReadMethod::ContactlessEmv => "contactless_emv",
            PaymentMethodDetailsCardPresentReadMethod::ContactlessMagstripeMode => "contactless_magstripe_mode",
            PaymentMethodDetailsCardPresentReadMethod::MagneticStripeFallback => "magnetic_stripe_fallback",
            PaymentMethodDetailsCardPresentReadMethod::MagneticStripeTrack2 => "magnetic_stripe_track2",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardPresentReadMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardPresentReadMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodDetailsCardPresentReadMethod {
    fn default() -> Self {
        Self::ContactEmv
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsCardPresentReceipt`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardPresentReceiptAccountType {
    Checking,
    Credit,
    Prepaid,
    Unknown,
}

impl PaymentMethodDetailsCardPresentReceiptAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsCardPresentReceiptAccountType::Checking => "checking",
            PaymentMethodDetailsCardPresentReceiptAccountType::Credit => "credit",
            PaymentMethodDetailsCardPresentReceiptAccountType::Prepaid => "prepaid",
            PaymentMethodDetailsCardPresentReceiptAccountType::Unknown => "unknown",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn default() -> Self {
        Self::Checking
    }
}
