use ids::{SourceId, TokenId};
use params::Identifiable;
use resources::{BankAccount, BankAccountParams, Card, CardParams, Source};

#[derive(Clone, Debug)]
pub enum PaymentSourceParams<'a> {
    Source(SourceId),
    Token(TokenId),
    Card(CardParams<'a>),
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
        let content = <Content as Deserialize>::deserialize(deserializer)?;
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
                return <CardParams as Deserialize>::deserialize(deserializer)
                    .map(PaymentSourceParams::Card);
            }
            Ok(PaymentSourceObjectType::BankAccount(_)) => {
                let deserializer = ContentRefDeserializer::<D::Error>::new(&content);
                return <BankAccountParams as Deserialize>::deserialize(deserializer)
                    .map(PaymentSourceParams::BankAccount);
            },
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
            BankAccount(&'a BankAccountParams<'a>)
        }

        match self {
            PaymentSourceParams::Source(id) => id.serialize(serializer),
            PaymentSourceParams::Token(id) => id.serialize(serializer),
            PaymentSourceParams::Card(card) => {
                PaymentSourceTagged::Card(card).serialize(serializer)
            },
            PaymentSourceParams::BankAccount(account) => {
                PaymentSourceTagged::BankAccount(account).serialize(serializer)
            },
        }
    }
}

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
