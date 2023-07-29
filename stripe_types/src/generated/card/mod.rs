/// You can store multiple cards on a customer in order to charge the customer
/// later.
///
/// You can also store multiple debit cards on a recipient in order to transfer to those cards later.  Related guide: [Card Payments with Sources](https://stripe.com/docs/sources/cards).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Card {
    /// The account this card belongs to.
    ///
    /// This attribute will not be in the card object if the card belongs to a customer or recipient instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types::Expandable<stripe_types::account::Account>>,
    /// City/District/Suburb/Town/Village.
    pub address_city: Option<String>,
    /// Billing address country, if provided when creating card.
    pub address_country: Option<String>,
    /// Address line 1 (Street address/PO Box/Company name).
    pub address_line1: Option<String>,
    /// If `address_line1` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Option<String>,
    /// Address line 2 (Apartment/Suite/Unit/Building).
    pub address_line2: Option<String>,
    /// State/County/Province/Region.
    pub address_state: Option<String>,
    /// ZIP or postal code.
    pub address_zip: Option<String>,
    /// If `address_zip` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_zip_check: Option<String>,
    /// A set of available payout methods for this card.
    ///
    /// Only values from this set should be passed as the `method` when creating a payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payout_methods: Option<Vec<CardAvailablePayoutMethods>>,
    /// Card brand.
    ///
    /// Can be `American Express`, `Diners Club`, `Discover`, `JCB`, `MasterCard`, `UnionPay`, `Visa`, or `Unknown`.
    pub brand: String,
    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    /// Three-letter [ISO code for currency](https://stripe.com/docs/payouts).
    ///
    /// Only applicable on accounts (not customers or recipients).
    /// The card can be used as a transfer destination for funds in this currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The customer that this card belongs to.
    ///
    /// This attribute will not be in the card object if the card belongs to an account or recipient instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<stripe_types::Expandable<stripe_types::customer::Customer>>,
    /// If a CVC was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    ///
    /// A result of unchecked indicates that CVC was provided but hasn't been checked yet.
    /// Checks are typically performed when attaching a card to a Customer object, or when creating a charge.
    /// For more details, see [Check if a card is valid without a charge](https://support.stripe.com/questions/check-if-a-card-is-valid-without-a-charge).
    pub cvc_check: Option<String>,
    /// Whether this card is the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,
    /// A high-level description of the type of cards issued in this range.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: String,
    /// Unique identifier for the object.
    pub id: stripe_types::card::CardId,
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
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Cardholder name.
    pub name: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CardObject,
    /// For external accounts, possible values are `new` and `errored`.
    ///
    /// If a transfer fails, the status is set to `errored` and transfers are stopped until account details are updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// If the card number is tokenized, this is the method that was used.
    ///
    /// Can be `android_pay` (includes Google Pay), `apple_pay`, `masterpass`, `visa_checkout`, or null.
    pub tokenization_method: Option<String>,
}
/// A set of available payout methods for this card.
///
/// Only values from this set should be passed as the `method` when creating a payout.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardAvailablePayoutMethods {
    Instant,
    Standard,
}

impl CardAvailablePayoutMethods {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Instant => "instant",
            Self::Standard => "standard",
        }
    }
}

impl std::str::FromStr for CardAvailablePayoutMethods {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "instant" => Ok(Self::Instant),
            "standard" => Ok(Self::Standard),

            _ => Err(()),
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
impl serde::Serialize for CardAvailablePayoutMethods {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardAvailablePayoutMethods {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CardAvailablePayoutMethods"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardObject {
    Card,
}

impl CardObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Card => "card",
        }
    }
}

impl std::str::FromStr for CardObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "card" => Ok(Self::Card),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CardObject"))
    }
}
impl stripe_types::Object for Card {
    type Id = stripe_types::card::CardId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(CardId, "card_");
pub mod deleted;
pub use deleted::DeletedCard;
