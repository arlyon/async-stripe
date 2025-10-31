/// A Connection Token is used by the Stripe Terminal SDK to connect to a reader.
///
/// Related guide: [Fleet management](https://stripe.com/docs/terminal/fleet/locations)
///
/// For more details see <<https://stripe.com/docs/api/terminal/connection_tokens/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConnectionToken {
    /// The id of the location that this connection token is scoped to.
    /// Note that location scoping only applies to internet-connected readers.
    /// For more details, see [the docs on scoping connection tokens](https://docs.stripe.com/terminal/fleet/locations-and-zones?dashboard-or-api=api#connection-tokens).
    pub location: Option<String>,
    /// Your application should pass this token to the Stripe Terminal SDK.
    pub secret: String,
}
#[doc(hidden)]
pub struct TerminalConnectionTokenBuilder {
    location: Option<Option<String>>,
    secret: Option<String>,
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

    impl Deserialize for TerminalConnectionToken {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConnectionToken>,
        builder: TerminalConnectionTokenBuilder,
    }

    impl Visitor for Place<TerminalConnectionToken> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalConnectionTokenBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalConnectionTokenBuilder {
        type Out = TerminalConnectionToken;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "location" => Deserialize::begin(&mut self.location),
                "secret" => Deserialize::begin(&mut self.secret),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { location: Deserialize::default(), secret: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(location), Some(secret)) = (self.location.take(), self.secret.take()) else {
                return None;
            };
            Some(Self::Out { location, secret })
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

    impl ObjectDeser for TerminalConnectionToken {
        type Builder = TerminalConnectionTokenBuilder;
    }

    impl FromValueOpt for TerminalConnectionToken {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalConnectionTokenBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "location" => b.location = FromValueOpt::from_value(v),
                    "secret" => b.secret = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalConnectionToken {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TerminalConnectionToken", 3)?;
        s.serialize_field("location", &self.location)?;
        s.serialize_field("secret", &self.secret)?;

        s.serialize_field("object", "terminal.connection_token")?;
        s.end()
    }
}
