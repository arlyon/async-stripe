#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoProvidedDetails {
    /// Email of user being verified
    pub email: Option<String>,
    /// Phone number of user being verified
    pub phone: Option<String>,
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
    use miniserde::{make_place, Deserialize, Result};
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
            Self { email: Deserialize::default(), phone: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(email), Some(phone)) = (self.email.take(), self.phone.take()) else {
                return None;
            };
            Some(Self::Out { email, phone })
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

    impl ObjectDeser for GelatoProvidedDetails {
        type Builder = GelatoProvidedDetailsBuilder;
    }

    impl FromValueOpt for GelatoProvidedDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoProvidedDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "email" => b.email = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
