/// A Connection Token is used by the Stripe Terminal SDK to connect to a reader.
///
/// Related guide: [Fleet management](https://stripe.com/docs/terminal/fleet/locations)
///
/// For more details see <<https://stripe.com/docs/api/terminal/connection_tokens/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TerminalConnectionToken {
    /// The id of the location that this connection token is scoped to.
    /// Note that location scoping only applies to internet-connected readers.
    /// For more details, see [the docs on scoping connection tokens](https://stripe.com/docs/terminal/fleet/locations#connection-tokens).
    pub location: Option<String>,
    /// Your application should pass this token to the Stripe Terminal SDK.
    pub secret: String,
}
#[cfg(feature = "min-ser")]
pub struct TerminalConnectionTokenBuilder {
    location: Option<Option<String>>,
    secret: Option<String>,
}

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
            Ok(Box::new(Builder { out: &mut self.out, builder: TerminalConnectionTokenBuilder::deser_default() }))
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
            let location = self.location.take()?;
            let secret = self.secret.take()?;

            Some(Self::Out { location, secret })
        }
    }

    impl<'a> Map for Builder<'a> {
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
                    "location" => b.location = Some(FromValueOpt::from_value(v)?),
                    "secret" => b.secret = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
