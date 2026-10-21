#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoProvidedDetails {
    /// Email of user being verified
    pub email: Option<String>,
    /// Phone number of user being verified
    pub phone: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoProvidedDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoProvidedDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoProvidedDetailsBuilder {
    email: Option<Option<String>>,
    phone: Option<Option<String>>,
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

    impl Deserialize for GelatoProvidedDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoProvidedDetails>,
        builder: GelatoProvidedDetailsBuilder,
    }

    impl Visitor for Place<GelatoProvidedDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoProvidedDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoProvidedDetailsBuilder {
        type Out = GelatoProvidedDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "email" => Deserialize::begin(&mut self.email),
                "phone" => Deserialize::begin(&mut self.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { email: Some(None), phone: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { email: Some(email), phone: Some(phone) } => Some(Self::Out { email, phone }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for GelatoProvidedDetails {
        type Builder = GelatoProvidedDetailsBuilder;
    }

    impl FromValueOpt for GelatoProvidedDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
