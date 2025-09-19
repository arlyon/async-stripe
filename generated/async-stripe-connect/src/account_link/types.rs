/// Account Links are the means by which a Connect platform grants a connected account permission to access.
/// Stripe-hosted applications, such as Connect Onboarding.
///
/// Related guide: [Connect Onboarding](https://stripe.com/docs/connect/custom/hosted-onboarding)
///
/// For more details see <<https://stripe.com/docs/api/account_links/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountLink {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The timestamp at which this account link will expire.
    pub expires_at: stripe_types::Timestamp,
    /// The URL for the account link.
    pub url: String,
}
#[doc(hidden)]
pub struct AccountLinkBuilder {
    created: Option<stripe_types::Timestamp>,
    expires_at: Option<stripe_types::Timestamp>,
    url: Option<String>,
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

    impl Deserialize for AccountLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountLink>,
        builder: AccountLinkBuilder,
    }

    impl Visitor for Place<AccountLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountLinkBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountLinkBuilder {
        type Out = AccountLink;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "url" => Deserialize::begin(&mut self.url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                expires_at: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(created), Some(expires_at), Some(url)) =
                (self.created, self.expires_at, self.url.take())
            else {
                return None;
            };
            Some(Self::Out { created, expires_at, url })
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

    impl ObjectDeser for AccountLink {
        type Builder = AccountLinkBuilder;
    }

    impl FromValueOpt for AccountLink {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountLinkBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountLink {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("AccountLink", 4)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("url", &self.url)?;

        s.serialize_field("object", "account_link")?;
        s.end()
    }
}
