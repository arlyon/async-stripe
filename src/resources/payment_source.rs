use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

use crate::ids::{PaymentSourceId, SourceId, TokenId};
use crate::params::Object;
use crate::resources::{Account, BankAccount, Card, Currency, Source};

/// A PaymentSourceParams represents all of the supported ways that can
/// be used.
///
/// This includes creating a new customer with a payment method or creating
/// a payment method directly for a customer via `Customer::attach_source`.
/// Not to be confused with `SourceParams` which is used by `Source::create`
/// to create a source that is not necessarily attached to a customer.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentSourceParams {
    /// Creates a payment method (e.g. card or bank account) from tokenized data,
    /// using a token typically received from Stripe Elements.
    Token(TokenId),

    /// Attach an existing source to an existing customer or
    /// create a new customer from an existing source.
    Source(SourceId),
}

/// A PaymentSource represents a payment method _associated with a customer or charge_.
/// This value is usually returned as a subresource on another request.
///
/// Not to be confused with `Source` which represents a "generic" payment method
/// returned by the `Source::get` (which could still be a credit card, etc)
/// but is not necessarily attached to either a customer or charge.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum PaymentSource {
    Card(Card),
    Source(Source),
    Account(Account),
    BankAccount(BankAccount),
}

impl Object for PaymentSource {
    type Id = PaymentSourceId;
    fn id(&self) -> Self::Id {
        match self {
            PaymentSource::Card(x) => PaymentSourceId::Card(x.id()),
            PaymentSource::Source(x) => PaymentSourceId::Source(x.id()),
            PaymentSource::Account(x) => PaymentSourceId::Account(x.id()),
            PaymentSource::BankAccount(x) => PaymentSourceId::BankAccount(x.id()),
        }
    }
    fn object(&self) -> &'static str {
        match self {
            PaymentSource::Card(x) => x.object(),
            PaymentSource::Source(x) => x.object(),
            PaymentSource::Account(x) => x.object(),
            PaymentSource::BankAccount(x) => x.object(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct BankAccountParams<'a> {
    pub country: &'a str,
    pub currency: Currency,
    pub account_holder_name: Option<&'a str>,
    pub account_holder_type: Option<&'a str>,
    pub routing_number: Option<&'a str>,
    pub account_number: &'a str,
}

impl<'a> serde::ser::Serialize for BankAccountParams<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut s = serializer.serialize_struct("BankAccountParams", 6)?;
        s.serialize_field("object", "bank_account")?;
        s.serialize_field("country", &self.country)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("account_holder_name", &self.account_holder_name)?;
        s.serialize_field("routing_number", &self.routing_number)?;
        s.serialize_field("account_number", &self.account_number)?;
        s.end()
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CardParams<'a> {
    pub exp_month: &'a str, // eg. "12"
    pub exp_year: &'a str,  // eg. "17" or 2017"

    pub number: &'a str,       // card number
    pub name: Option<&'a str>, // cardholder's full name
    pub cvc: Option<&'a str>,  // card security code
}

impl<'a> serde::ser::Serialize for CardParams<'a> {
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
