/// Tokenization is the process Stripe uses to collect sensitive card or bank
/// account details, or personally identifiable information (PII), directly from
/// your customers in a secure manner. A token representing this information is
/// returned to your server to use. Use our
/// [recommended payments integrations](https://stripe.com/docs/payments) to perform this process
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
/// information for later use, create [Customer](https://stripe.com/docs/api#customers)
/// objects or [External accounts](/api#external_accounts).
/// [Radar](https://stripe.com/docs/radar), our integrated solution for automatic fraud protection,
/// performs best with integrations that use client-side tokenization.
///
/// For more details see <<https://stripe.com/docs/api/tokens/object>>.
#[derive(Clone, Debug)]
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
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Type of the token: `account`, `bank_account`, `card`, or `pii`.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: String,
    /// Determines if you have already used this token (you can only use tokens once).
    pub used: bool,
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
            Ok(Box::new(Builder { out: &mut self.out, builder: TokenBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TokenBuilder {
        type Out = Token;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_account" => Deserialize::begin(&mut self.bank_account),
                "card" => Deserialize::begin(&mut self.card),
                "client_ip" => Deserialize::begin(&mut self.client_ip),
                "created" => Deserialize::begin(&mut self.created),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "type" => Deserialize::begin(&mut self.type_),
                "used" => Deserialize::begin(&mut self.used),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_account: Deserialize::default(),
                card: Deserialize::default(),
                client_ip: Deserialize::default(),
                created: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                type_: Deserialize::default(),
                used: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.bank_account.take(),
                self.card.take(),
                self.client_ip.take(),
                self.created,
                self.id.take(),
                self.livemode,
                self.type_.take(),
                self.used,
            )
            else {
                return None;
            };
            Some(Self::Out { bank_account, card, client_ip, created, id, livemode, type_, used })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for Token {
        type Builder = TokenBuilder;
    }

    impl FromValueOpt for Token {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TokenBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_account" => b.bank_account = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "client_ip" => b.client_ip = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "used" => b.used = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
