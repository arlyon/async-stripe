/// Login Links are single-use URLs that takes an Express account to the login page for their Stripe dashboard.
/// A Login Link differs from an [Account Link](https://stripe.com/docs/api/account_links) in that it takes the user directly to their [Express dashboard for the specified account](https://stripe.com/docs/connect/integrate-express-dashboard#create-login-link).
///
/// For more details see <<https://stripe.com/docs/api/account/login_link>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LoginLink {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The URL for the login link.
    pub url: String,
}
#[doc(hidden)]
pub struct LoginLinkBuilder {
    created: Option<stripe_types::Timestamp>,
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
            Ok(Box::new(Builder { out: &mut self.out, builder: LoginLinkBuilder::deser_default() }))
        }
    }

    impl MapBuilder for LoginLinkBuilder {
        type Out = LoginLink;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "url" => Deserialize::begin(&mut self.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { created: Deserialize::default(), url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(created), Some(url)) = (self.created, self.url.take()) else {
                return None;
            };
            Some(Self::Out { created, url })
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

    impl ObjectDeser for LoginLink {
        type Builder = LoginLinkBuilder;
    }

    impl FromValueOpt for LoginLink {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = LoginLinkBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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
