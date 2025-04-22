#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoSessionPhoneOptions {
    /// Request one time password verification of `provided_details.phone`.
    pub require_verification: Option<bool>,
}
#[doc(hidden)]
pub struct GelatoSessionPhoneOptionsBuilder {
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoSessionPhoneOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoSessionPhoneOptions>,
        builder: GelatoSessionPhoneOptionsBuilder,
    }

    impl Visitor for Place<GelatoSessionPhoneOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoSessionPhoneOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoSessionPhoneOptionsBuilder {
        type Out = GelatoSessionPhoneOptions;
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

    impl ObjectDeser for GelatoSessionPhoneOptions {
        type Builder = GelatoSessionPhoneOptionsBuilder;
    }

    impl FromValueOpt for GelatoSessionPhoneOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoSessionPhoneOptionsBuilder::deser_default();
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
