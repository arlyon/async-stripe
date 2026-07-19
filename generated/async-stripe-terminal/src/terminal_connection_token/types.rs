/// A Connection Token is used by the Stripe Terminal SDK to connect to a reader.
///
/// Related guide: [Fleet management](https://docs.stripe.com/terminal/fleet/locations)
///
/// For more details see <<https://stripe.com/docs/api/terminal/connection_tokens/object>>.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConnectionToken {
    /// The id of the location that this connection token is scoped to.
    /// Note that location scoping only applies to internet-connected readers.
    /// For more details, see [the docs on scoping connection tokens](https://docs.stripe.com/terminal/fleet/locations-and-zones?dashboard-or-api=api#connection-tokens).
    pub location: Option<String>,
    /// Your application should pass this token to the Stripe Terminal SDK.
    pub secret: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalConnectionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalConnectionToken").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalConnectionTokenBuilder {
    location: Option<Option<String>>,
    secret: Option<String>,
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
                builder: TerminalConnectionTokenBuilder {
                    location: Deserialize::default(),
                    secret: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "location" => Deserialize::begin(&mut self.builder.location),
                "secret" => Deserialize::begin(&mut self.builder.secret),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(location), Some(secret)) =
                (self.builder.location.take(), self.builder.secret.take())
            else {
                return Ok(());
            };
            *self.out = Some(TerminalConnectionToken { location, secret });
            Ok(())
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
