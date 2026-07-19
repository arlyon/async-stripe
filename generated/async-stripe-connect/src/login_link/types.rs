/// Login Links are single-use URLs that takes an Express account to the login page for their Stripe dashboard.
/// A Login Link differs from an [Account Link](https://docs.stripe.com/api/account_links) in that it takes the user directly to their [Express dashboard for the specified account](https://docs.stripe.com/connect/integrate-express-dashboard#create-login-link).
///
/// For more details see <<https://stripe.com/docs/api/account/login_link>>.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LoginLink {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The URL for the login link.
    pub url: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LoginLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LoginLink").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct LoginLinkBuilder {
    created: Option<stripe_types::Timestamp>,
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

    impl Deserialize for LoginLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LoginLink>,
        builder: LoginLinkBuilder,
    }

    impl Visitor for Place<LoginLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: LoginLinkBuilder {
                    created: Deserialize::default(),
                    url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "url" => Deserialize::begin(&mut self.builder.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(created), Some(url)) = (self.builder.created, self.builder.url.take()) else {
                return Ok(());
            };
            *self.out = Some(LoginLink { created, url });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for LoginLink {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("LoginLink", 3)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("url", &self.url)?;

        s.serialize_field("object", "login_link")?;
        s.end()
    }
}
