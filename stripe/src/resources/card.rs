
use resources::Currency;
use params::Metadata;
use serde::ser::SerializeStruct;

#[derive(Debug, Default, Deserialize)]
pub struct CardParams<'a> {
    pub exp_month: &'a str, // eg. "12"
    pub exp_year: &'a str,  // eg. "17" or 2017"

    pub number: &'a str,       // card number
    pub name: Option<&'a str>, // cardholder's full name
    pub cvc: Option<&'a str>,  // card security code
}

impl<'a> ::serde::Serialize for CardParams<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
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

/// The resource representing a Stripe card object.
///
/// For more details see [https://stripe.com/docs/api#card_object](https://stripe.com/docs/api#card_object).
#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub id: String,
    pub account: Option<String>,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line1_check: Option<CheckResult>,
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<CheckResult>,
    pub available_payout_methods: Option<Vec<String>>,
    pub brand: CardBrand,
    pub country: String, // eg. "US"
    pub currency: Option<Currency>,
    pub customer: Option<String>,
    pub cvc_check: Option<CheckResult>,
    pub default_for_currency: Option<bool>,
    pub dynamic_last4: Option<String>,
    pub exp_month: u32,
    pub exp_year: u32,
    pub fingerprint: String,
    pub funding: CardType,
    pub last4: String,
    pub metadata: Metadata,
    pub name: Option<String>,
    pub recipient: Option<String>,
    pub tokenization_method: Option<TokenizationMethod>,
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
    #[serde(rename = "Unknown")]
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
pub enum CardType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "prepaid")]
    Prepaid,
    #[serde(rename = "unknown")]
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TokenizationMethod {
    ApplePay,
    AndroidPay,
    #[serde(other)]
    Other,
}

