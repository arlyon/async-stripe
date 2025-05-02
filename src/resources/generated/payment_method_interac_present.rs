// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::{PaymentMethodCardPresentNetworks};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_interac_present".
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
            PaymentMethodInteracPresentReadMethod::ContactlessMagstripeMode => "contactless_magstripe_mode",
            PaymentMethodInteracPresentReadMethod::MagneticStripeFallback => "magnetic_stripe_fallback",
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
