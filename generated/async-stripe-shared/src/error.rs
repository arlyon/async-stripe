/// An error response from the Stripe API
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Error {
    pub error: Box<stripe_shared::ApiErrors>,
}
#[doc(hidden)]
pub struct ErrorBuilder {
    error: Option<Box<stripe_shared::ApiErrors>>,
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

    impl Deserialize for Error {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Error>,
        builder: ErrorBuilder,
    }

    impl Visitor for Place<Error> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ErrorBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ErrorBuilder {
        type Out = Error;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "error" => Deserialize::begin(&mut self.error),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { error: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(error),) = (self.error.take(),) else {
                return None;
            };
            Some(Self::Out { error })
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

    impl ObjectDeser for Error {
        type Builder = ErrorBuilder;
    }

    impl FromValueOpt for Error {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ErrorBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "error" => b.error = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
