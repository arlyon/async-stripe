#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoSessionMatchingOptions {
    /// Strictness of the DOB matching policy to apply.
    pub dob: Option<stripe_shared::GelatoSessionMatchingOptionsDob>,
    /// Strictness of the name matching policy to apply.
    pub name: Option<stripe_shared::GelatoSessionMatchingOptionsDob>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoSessionMatchingOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoSessionMatchingOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoSessionMatchingOptionsBuilder {
    dob: Option<Option<stripe_shared::GelatoSessionMatchingOptionsDob>>,
    name: Option<Option<stripe_shared::GelatoSessionMatchingOptionsDob>>,
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

    impl Deserialize for GelatoSessionMatchingOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoSessionMatchingOptions>,
        builder: GelatoSessionMatchingOptionsBuilder,
    }

    impl Visitor for Place<GelatoSessionMatchingOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoSessionMatchingOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoSessionMatchingOptionsBuilder {
        type Out = GelatoSessionMatchingOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dob" => Deserialize::begin(&mut self.dob),
                "name" => Deserialize::begin(&mut self.name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { dob: Some(None), name: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(dob), Some(name)) = (self.dob.take(), self.name.take()) else {
                return None;
            };
            Some(Self::Out { dob, name })
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

    impl ObjectDeser for GelatoSessionMatchingOptions {
        type Builder = GelatoSessionMatchingOptionsBuilder;
    }

    impl FromValueOpt for GelatoSessionMatchingOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoSessionMatchingOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "dob" => b.dob = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Strictness of the DOB matching policy to apply.
pub type GelatoSessionMatchingOptionsDob = stripe_shared::GelatoSessionMatchingOptionsDob;
/// Strictness of the name matching policy to apply.
pub type GelatoSessionMatchingOptionsName = stripe_shared::GelatoSessionMatchingOptionsDob;
