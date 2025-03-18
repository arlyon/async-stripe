// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::CardId;
use crate::params::{Expandable, Metadata, Object};
use crate::resources::{Account, Currency, Customer};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Card".
///
/// For more details see <https://stripe.com/docs/api/cards/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Card {
    /// Unique identifier for the object.
    pub id: CardId,

    /// The account this card belongs to.
    ///
    /// This attribute will not be in the card object if the card belongs to a customer or recipient instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Expandable<Account>>,

    /// City/District/Suburb/Town/Village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,

    /// Billing address country, if provided when creating card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,

    /// Address line 1 (Street address/PO Box/Company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,

    /// If `address_line1` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<String>,

    /// Address line 2 (Apartment/Suite/Unit/Building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,

    /// State/County/Province/Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,

    /// If `address_zip` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip_check: Option<String>,

    /// A set of available payout methods for this card.
    ///
    /// Only values from this set should be passed as the `method` when creating a payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payout_methods: Option<Vec<CardAvailablePayoutMethods>>,

    /// Card brand.
    ///
    /// Can be `American Express`, `Diners Club`, `Discover`, `Eftpos Australia`, `JCB`, `MasterCard`, `UnionPay`, `Visa`, or `Unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Three-letter [ISO code for currency](https://stripe.com/docs/payouts).
    ///
    /// Only applicable on accounts (not customers or recipients).
    /// The card can be used as a transfer destination for funds in this currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The customer that this card belongs to.
    ///
    /// This attribute will not be in the card object if the card belongs to an account or recipient instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// If a CVC was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    ///
    /// A result of unchecked indicates that CVC was provided but hasn't been checked yet.
    /// Checks are typically performed when attaching a card to a Customer object, or when creating a charge.
    /// For more details, see [Check if a card is valid without a charge](https://support.stripe.com/questions/check-if-a-card-is-valid-without-a-charge).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,

    /// Whether this card is the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// A high-level description of the type of cards issued in this range.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// (For tokenized numbers only.) The last four digits of the device account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,

    /// Two-digit number representing the card's expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,

    /// Four-digit number representing the card's expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// For external accounts that are cards, possible values are `new` and `errored`.
    ///
    /// If a payout fails, the status is set to `errored` and [scheduled payouts](https://stripe.com/docs/payouts#payout-schedule) are stopped until account details are updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// If the card number is tokenized, this is the method that was used.
    ///
    /// Can be `android_pay` (includes Google Pay), `apple_pay`, `masterpass`, `visa_checkout`, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenization_method: Option<String>,
}

impl Object for Card {
    type Id = CardId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "card"
    }
}

/// An enum representing the possible values of an `Card`'s `available_payout_methods` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CardAvailablePayoutMethods {
    Instant,
    Standard,
}

impl CardAvailablePayoutMethods {
    pub fn as_str(self) -> &'static str {
        match self {
            CardAvailablePayoutMethods::Instant => "instant",
            CardAvailablePayoutMethods::Standard => "standard",
        }
    }
}

impl AsRef<str> for CardAvailablePayoutMethods {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardAvailablePayoutMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CardAvailablePayoutMethods {
    fn default() -> Self {
        Self::Instant
    }
}
