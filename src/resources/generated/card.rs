// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::CardId;
use crate::params::{Expandable, Metadata, Object};
use crate::resources::{Account, Currency, Customer, Recipient};

/// The resource representing a Stripe "Card".
///
/// For more details see <https://stripe.com/docs/api/cards/object>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Card {
    /// Unique identifier for the object.
    pub id: CardId,

    /// The account this card belongs to.
    ///
    /// This attribute will not be in the card object if the card belongs to a customer or recipient instead.
    pub account: Box<Option<Expandable<Account>>>,

    /// City/District/Suburb/Town/Village.
    pub address_city: Box<Option<String>>,

    /// Billing address country, if provided when creating card.
    pub address_country: Box<Option<String>>,

    /// Address line 1 (Street address/PO Box/Company name).
    pub address_line1: Box<Option<String>>,

    /// If `address_line1` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Box<Option<String>>,

    /// Address line 2 (Apartment/Suite/Unit/Building).
    pub address_line2: Box<Option<String>>,

    /// State/County/Province/Region.
    pub address_state: Box<Option<String>>,

    /// ZIP or postal code.
    pub address_zip: Box<Option<String>>,

    /// If `address_zip` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_zip_check: Box<Option<String>>,

    /// A set of available payout methods for this card.
    ///
    /// Only values from this set should be passed as the `method` when creating a payout.
    pub available_payout_methods: Box<Option<Vec<CardAvailablePayoutMethods>>>,

    /// Card brand.
    ///
    /// Can be `American Express`, `Diners Club`, `Discover`, `JCB`, `MasterCard`, `UnionPay`, `Visa`, or `Unknown`.
    pub brand: Box<Option<String>>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Box<Option<String>>,

    /// Three-letter [ISO code for currency](https://stripe.com/docs/payouts).
    ///
    /// Only applicable on accounts (not customers or recipients).
    /// The card can be used as a transfer destination for funds in this currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The customer that this card belongs to.
    ///
    /// This attribute will not be in the card object if the card belongs to an account or recipient instead.
    pub customer: Box<Option<Expandable<Customer>>>,

    /// If a CVC was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    ///
    /// A result of unchecked indicates that CVC was provided but hasn't been checked yet.
    /// Checks are typically performed when attaching a card to a Customer object, or when creating a charge.
    /// For more details, see [Check if a card is valid without a charge](https://support.stripe.com/questions/check-if-a-card-is-valid-without-a-charge).
    pub cvc_check: Box<Option<String>>,

    /// Whether this card is the default external account for its currency.
    pub default_for_currency: Box<Option<bool>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Box<Option<String>>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: Box<Option<i64>>,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: Box<Option<i64>>,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.*.
    pub fingerprint: Box<Option<String>>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Box<Option<String>>,

    /// The last four digits of the card.
    pub last4: Box<Option<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// Cardholder name.
    pub name: Box<Option<String>>,

    /// The recipient that this card belongs to.
    ///
    /// This attribute will not be in the card object if the card belongs to a customer or account instead.
    pub recipient: Box<Option<Expandable<Recipient>>>,

    /// If the card number is tokenized, this is the method that was used.
    ///
    /// Can be `android_pay` (includes Google Pay), `apple_pay`, `masterpass`, `visa_checkout`, or null.
    pub tokenization_method: Box<Option<String>>,
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
