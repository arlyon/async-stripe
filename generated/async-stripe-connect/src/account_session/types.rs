/// An AccountSession allows a Connect platform to grant access to a connected account in Connect embedded components.
///
/// We recommend that you create an AccountSession each time you need to display an embedded component
/// to your user. Do not save AccountSessions to your database as they expire relatively
/// quickly, and cannot be used more than once.
///
/// Related guide: [Connect embedded components](https://stripe.com/docs/connect/get-started-connect-embedded-components).
///
/// For more details see <<https://stripe.com/docs/api/account_sessions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountSession {
    /// The ID of the account the AccountSession was created for
    pub account: String,
    /// The client secret of this AccountSession.
    /// Used on the client to set up secure access to the given `account`.
    ///
    /// The client secret can be used to provide access to `account` from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the connected account.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    ///
    /// Refer to our docs to [setup Connect embedded components](https://stripe.com/docs/connect/get-started-connect-embedded-components) and learn about how `client_secret` should be handled.
    pub client_secret: String,
    pub components: stripe_connect::ConnectEmbeddedAccountSessionCreateComponents,
    /// The timestamp at which this AccountSession will expire.
    pub expires_at: stripe_types::Timestamp,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
#[doc(hidden)]
pub struct AccountSessionBuilder {
    account: Option<String>,
    client_secret: Option<String>,
    components: Option<stripe_connect::ConnectEmbeddedAccountSessionCreateComponents>,
    expires_at: Option<stripe_types::Timestamp>,
    livemode: Option<bool>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountSession {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountSession>,
        builder: AccountSessionBuilder,
    }

    impl Visitor for Place<AccountSession> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountSessionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountSessionBuilder {
        type Out = AccountSession;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "client_secret" => Deserialize::begin(&mut self.client_secret),
                "components" => Deserialize::begin(&mut self.components),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "livemode" => Deserialize::begin(&mut self.livemode),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account: Deserialize::default(),
                client_secret: Deserialize::default(),
                components: Deserialize::default(),
                expires_at: Deserialize::default(),
                livemode: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account),
                Some(client_secret),
                Some(components),
                Some(expires_at),
                Some(livemode),
            ) = (
                self.account.take(),
                self.client_secret.take(),
                self.components,
                self.expires_at,
                self.livemode,
            )
            else {
                return None;
            };
            Some(Self::Out { account, client_secret, components, expires_at, livemode })
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

    impl ObjectDeser for AccountSession {
        type Builder = AccountSessionBuilder;
    }

    impl FromValueOpt for AccountSession {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountSessionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = FromValueOpt::from_value(v),
                    "client_secret" => b.client_secret = FromValueOpt::from_value(v),
                    "components" => b.components = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountSession {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("AccountSession", 6)?;
        s.serialize_field("account", &self.account)?;
        s.serialize_field("client_secret", &self.client_secret)?;
        s.serialize_field("components", &self.components)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("livemode", &self.livemode)?;

        s.serialize_field("object", "account_session")?;
        s.end()
    }
}
