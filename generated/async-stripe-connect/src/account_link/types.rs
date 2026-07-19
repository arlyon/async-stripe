/// Account Links are the means by which a Connect platform grants a connected account permission to access.
/// Stripe-hosted applications, such as Connect Onboarding.
///
/// Related guide: [Connect Onboarding](https://docs.stripe.com/connect/custom/hosted-onboarding)
///
/// For more details see <<https://stripe.com/docs/api/account_links/object>>.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountLink {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The timestamp at which this account link will expire.
    pub expires_at: stripe_types::Timestamp,
    /// The URL for the account link.
    pub url: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountLink").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: AccountLinkBuilder {
                    created: Deserialize::default(),
                    expires_at: Deserialize::default(),
                    url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "url" => Deserialize::begin(&mut self.builder.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(created), Some(expires_at), Some(url)) =
                (self.builder.created, self.builder.expires_at, self.builder.url.take())
            else {
                return Ok(());
            };
            *self.out = Some(AccountLink { created, expires_at, url });
            Ok(())
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
