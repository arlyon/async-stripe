use crate::ids::{SourceId, TokenId};
use crate::params::Identifiable;
use crate::resources::{BankAccount, BankAccountParams, Card, CardParams, Source};
use serde_derive::{Deserialize, Serialize};

/// A PaymentSourceParams represents all of the supported ways that can
/// be used to creating a new customer with a payment method or creating
/// a payment method directly for a customer via `Customer::attach_source`.
///
/// Not to be confused with `SourceParams` which is used by `Source::create`
/// to create a source that is not necessarily attached to a customer.
#[derive(Clone, Debug)]
pub enum PaymentSourceParams<'a> {
    /// Creates a payment method (e.g. card or bank account) from tokenized data,
    /// using a token typically received from Stripe Elements.
    Token(TokenId),

    /// Attach an existing source to an existing customer or
    /// create a new customer from an existing source.
    Source(SourceId),

    /// Creates a `Card` payment method from the full card information
    /// (e.g. card number, expiration, etc).
    ///
    /// You usually want to use a `TokenId` received from
    /// [Stripe Elements](https://stripe.com/docs/stripe-js)
    /// instead.
    Card(CardParams<'a>),

    /// Creates a `BankAccount` payment method from the full bank information
    /// (e.g. account number, expiration, etc).
    ///
    /// You usually want to use a `TokenId` received from
    /// [Stripe Elements](https://stripe.com/docs/stripe-js)
    /// instead.
    BankAccount(BankAccountParams<'a>),
}

impl<'de> ::serde::Deserialize<'de> for PaymentSourceParams<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        use serde::de::{Deserialize, Error};
        use serde::private::de::{Content, ContentRefDeserializer};

        #[derive(Deserialize)]
        pub struct Any {}

        #[derive(Deserialize)]
        #[serde(tag = "object", rename_all = "snake_case")]
        pub enum PaymentSourceObjectType {
            Card(Any),
            BankAccount(Any),
        }

        // Try deserializing the untagged variants first
        let content = <Content<'_> as Deserialize>::deserialize(deserializer)?;
        let deserializer = ContentRefDeserializer::<D::Error>::new(&content);
        if let Ok(ok) = <SourceId as Deserialize>::deserialize(deserializer) {
            return Ok(PaymentSourceParams::Source(ok));
        }
        let deserializer = ContentRefDeserializer::<D::Error>::new(&content);
        if let Ok(ok) = <TokenId as Deserialize>::deserialize(deserializer) {
            return Ok(PaymentSourceParams::Token(ok));
        }

        // Deserialize just the tag of one of the tagged variants, then deserialize the matching variant
        let deserializer = ContentRefDeserializer::<D::Error>::new(&content);
        match <PaymentSourceObjectType as Deserialize>::deserialize(deserializer) {
            Ok(PaymentSourceObjectType::Card(_)) => {
                let deserializer = ContentRefDeserializer::<D::Error>::new(&content);
                return <CardParams<'_> as Deserialize>::deserialize(deserializer)
                    .map(PaymentSourceParams::Card);
            }
            Ok(PaymentSourceObjectType::BankAccount(_)) => {
                let deserializer = ContentRefDeserializer::<D::Error>::new(&content);
                return <BankAccountParams<'_> as Deserialize>::deserialize(deserializer)
                    .map(PaymentSourceParams::BankAccount);
            }
            _ => {}
        }

        Err(Error::custom("data did not match any variant of enum PaymentSourceParams"))
    }
}

impl<'a> ::serde::Serialize for PaymentSourceParams<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        #[derive(Serialize)]
        #[serde(tag = "object", rename_all = "snake_case")]
        enum PaymentSourceTagged<'a> {
            Card(&'a CardParams<'a>),
            BankAccount(&'a BankAccountParams<'a>),
        }

        match self {
            PaymentSourceParams::Source(id) => id.serialize(serializer),
            PaymentSourceParams::Token(id) => id.serialize(serializer),
            PaymentSourceParams::Card(card) => {
                PaymentSourceTagged::Card(card).serialize(serializer)
            }
            PaymentSourceParams::BankAccount(account) => {
                PaymentSourceTagged::BankAccount(account).serialize(serializer)
            }
        }
    }
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
    BankAccount(BankAccount),
}

impl Identifiable for PaymentSource {
    fn id(&self) -> &str {
        match self {
            PaymentSource::Card(c) => c.id(),
            PaymentSource::Source(s) => s.id(),
            PaymentSource::BankAccount(b) => b.id(),
        }
    }
}
