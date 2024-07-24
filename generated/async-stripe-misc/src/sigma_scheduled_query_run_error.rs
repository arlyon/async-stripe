#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SigmaScheduledQueryRunError {
    /// Information about the run failure.
    pub message: String,
}
#[doc(hidden)]
pub struct SigmaScheduledQueryRunErrorBuilder {
    message: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SigmaScheduledQueryRunError {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SigmaScheduledQueryRunError>,
        builder: SigmaScheduledQueryRunErrorBuilder,
    }

    impl Visitor for Place<SigmaScheduledQueryRunError> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SigmaScheduledQueryRunErrorBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SigmaScheduledQueryRunErrorBuilder {
        type Out = SigmaScheduledQueryRunError;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "message" => Deserialize::begin(&mut self.message),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { message: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { message: self.message.take()? })
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

    impl ObjectDeser for SigmaScheduledQueryRunError {
        type Builder = SigmaScheduledQueryRunErrorBuilder;
    }

    impl FromValueOpt for SigmaScheduledQueryRunError {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SigmaScheduledQueryRunErrorBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "message" => b.message = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
