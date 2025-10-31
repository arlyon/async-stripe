#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoSessionEmailOptions {
    /// Request one time password verification of `provided_details.email`.
    pub require_verification: Option<bool>,
}
#[doc(hidden)]
pub struct GelatoSessionEmailOptionsBuilder {
    require_verification: Option<Option<bool>>,
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

    impl Deserialize for GelatoSessionEmailOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoSessionEmailOptions>,
        builder: GelatoSessionEmailOptionsBuilder,
    }

    impl Visitor for Place<GelatoSessionEmailOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoSessionEmailOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoSessionEmailOptionsBuilder {
        type Out = GelatoSessionEmailOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "require_verification" => Deserialize::begin(&mut self.require_verification),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { require_verification: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(require_verification),) = (self.require_verification,) else {
                return None;
            };
            Some(Self::Out { require_verification })
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

    impl ObjectDeser for GelatoSessionEmailOptions {
        type Builder = GelatoSessionEmailOptionsBuilder;
    }

    impl FromValueOpt for GelatoSessionEmailOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoSessionEmailOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "require_verification" => b.require_verification = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
