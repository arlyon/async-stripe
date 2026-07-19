/// Tokenization is the process Stripe uses to collect sensitive card or bank
/// account details, or personally identifiable information (PII), directly from
/// your customers in a secure manner. A token representing this information is
/// returned to your server to use. Use our
/// [recommended payments integrations](https://docs.stripe.com/payments) to perform this process
/// on the client-side. This guarantees that no sensitive card data touches your server,
/// and allows your integration to operate in a PCI-compliant way.
///
/// If you can't use client-side tokenization, you can also create tokens using
/// the API with either your publishable or secret API key. If
/// your integration uses this method, you're responsible for any PCI compliance
/// that it might require, and you must keep your secret API key safe. Unlike with
/// client-side tokenization, your customer's information isn't sent directly to
/// Stripe, so we can't determine how it's handled or stored.
///
/// You can't store or use tokens more than once. To store card or bank account
/// information for later use, create [Customer](https://docs.stripe.com/api#customers)
/// objects or [External accounts](/api#external_accounts).
/// [Radar](https://docs.stripe.com/radar), our integrated solution for automatic fraud protection,
/// performs best with integrations that use client-side tokenization.
///
/// For more details see <<https://stripe.com/docs/api/tokens/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Token {
    pub bank_account: Option<stripe_shared::BankAccount>,
    pub card: Option<stripe_shared::Card>,
    /// IP address of the client that generates the token.
    pub client_ip: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_core::TokenId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Type of the token: `account`, `bank_account`, `card`, or `pii`.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: String,
    /// Determines if you have already used this token (you can only use tokens once).
    pub used: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Token").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TokenBuilder {
    bank_account: Option<Option<stripe_shared::BankAccount>>,
    card: Option<Option<stripe_shared::Card>>,
    client_ip: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    id: Option<stripe_core::TokenId>,
    livemode: Option<bool>,
    type_: Option<String>,
    used: Option<bool>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for Token {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Token>,
        builder: TokenBuilder,
    }

    impl Visitor for Place<Token> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TokenBuilder {
                    bank_account: Deserialize::default(),
                    card: Deserialize::default(),
                    client_ip: Deserialize::default(),
                    created: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    type_: Deserialize::default(),
                    used: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_account" => Deserialize::begin(&mut self.builder.bank_account),
                "card" => Deserialize::begin(&mut self.builder.card),
                "client_ip" => Deserialize::begin(&mut self.builder.client_ip),
                "created" => Deserialize::begin(&mut self.builder.created),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "used" => Deserialize::begin(&mut self.builder.used),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(bank_account),
                Some(card),
                Some(client_ip),
                Some(created),
                Some(id),
                Some(livemode),
                Some(type_),
                Some(used),
            ) = (
                self.builder.bank_account.take(),
                self.builder.card.take(),
                self.builder.client_ip.take(),
                self.builder.created,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.type_.take(),
                self.builder.used,
            )
            else {
                return Ok(());
            };
            *self.out =
                Some(Token { bank_account, card, client_ip, created, id, livemode, type_, used });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Token {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Token", 9)?;
        s.serialize_field("bank_account", &self.bank_account)?;
        s.serialize_field("card", &self.card)?;
        s.serialize_field("client_ip", &self.client_ip)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("used", &self.used)?;

        s.serialize_field("object", "token")?;
        s.end()
    }
}
impl stripe_types::Object for Token {
    type Id = stripe_core::TokenId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TokenId);
