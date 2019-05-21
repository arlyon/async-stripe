use crate::ids::CardId;
use crate::params::{Expandable, Metadata, Object};
use crate::resources::{Account, Currency, Customer, Recipient};
use serde::ser::SerializeStruct;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CardParams<'a> {
    pub exp_month: &'a str, // eg. "12"
    pub exp_year: &'a str,  // eg. "17" or 2017"

    pub number: &'a str,       // card number
    pub name: Option<&'a str>, // cardholder's full name
    pub cvc: Option<&'a str>,  // card security code
}

impl<'a> serde::Serialize for CardParams<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut s = serializer.serialize_struct("CardParams", 6)?;
        s.serialize_field("object", "card")?;
        s.serialize_field("exp_month", &self.exp_month)?;
        s.serialize_field("exp_year", &self.exp_year)?;
        s.serialize_field("number", &self.number)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("cvc", &self.cvc)?;
        s.end()
    }
}

/// The resource representing a Stripe "Card".
///
/// For more details see [https://stripe.com/docs/api/cards/object](https://stripe.com/docs/api/cards/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub address_line1_check: Option<CheckResult>,

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
    pub address_zip_check: Option<CheckResult>,

    /// A set of available payout methods for this card.
    ///
    /// Will be either `["standard"]` or `["standard", "instant"]`.
    /// Only values from this set should be passed as the `method` when creating a transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payout_methods: Option<Vec<String>>,

    /// Card brand.
    ///
    /// Can be `American Express`, `Diners Club`, `Discover`, `JCB`, `MasterCard`, `UnionPay`, `Visa`, or `Unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<CardBrand>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The customer that this card belongs to.
    ///
    /// This attribute will not be in the card object if the card belongs to an account or recipient instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// If a CVC was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<CheckResult>,

    /// Whether this card is the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<Currency>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

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
    /// You can use this attribute to check whether two customers who've signed up with you are using the same card number, for example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<CardType>,

    /// The last four digits of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The recipient that this card belongs to.
    ///
    /// This attribute will not be in the card object if the card belongs to a customer or account instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Expandable<Recipient>>,

    /// If the card number is tokenized, this is the method that was used.
    ///
    /// Can be `apple_pay` or `google_pay`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenization_method: Option<TokenizationMethod>,
}

/// Card details returned as part of a `Source` object
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardShort {
    pub exp_month: i32,
    pub exp_year: i32,
    pub address_zip_check: Option<CheckResult>,
    pub brand: String,
    pub country: String,
    pub cvc_check: Option<CheckResult>,
    pub fingerprint: Option<String>,
    pub funding: CardType,
    pub last4: String,
    pub three_d_secure: Option<String>,
    pub name: Option<String>,
    pub address_line1_check: Option<CheckResult>,
    pub tokenization_method: Option<TokenizationMethod>,
    pub dynamic_last4: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
pub enum CheckResult {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "fail")]
    Failed,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "unchecked")]
    Unchecked,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
pub enum CardBrand {
    #[serde(rename = "American Express")]
    AmericanExpress,
    #[serde(rename = "Diners Club")]
    DinersClub,
    #[serde(rename = "Discover")]
    Discover,
    #[serde(rename = "JCB")]
    JCB,
    #[serde(rename = "Visa")]
    Visa,
    #[serde(rename = "MasterCard")]
    MasterCard,
    #[serde(rename = "UnionPay")]
    UnionPay,

    /// An unknown card brand.
    ///
    /// May also be a variant not yet supported by the library.
    #[serde(other)]
    #[serde(rename = "Unknown")]
    Unknown,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
pub enum CardType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "prepaid")]
    Prepaid,

    /// An unknown card type.
    ///
    /// May also be a variant not yet supported by the library.
    #[serde(other)]
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TokenizationMethod {
    ApplePay,
    AndroidPay,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

impl Object for Card {
    type Id = CardId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
    fn object(&self) -> &'static str {
        "card"
    }
}
