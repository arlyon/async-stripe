#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceCodeVerificationFlow {
    /// The number of attempts remaining to authenticate the source object with a verification code.
    pub attempts_remaining: i64,
    /// The status of the code verification, either `pending` (awaiting verification, `attempts_remaining` should be greater than 0), `succeeded` (successful verification) or `failed` (failed verification, cannot be verified anymore as `attempts_remaining` should be 0).
    pub status: String,
}
#[doc(hidden)]
pub struct SourceCodeVerificationFlowBuilder {
    attempts_remaining: Option<i64>,
    status: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceCodeVerificationFlow {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceCodeVerificationFlow>,
        builder: SourceCodeVerificationFlowBuilder,
    }

    impl Visitor for Place<SourceCodeVerificationFlow> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceCodeVerificationFlowBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceCodeVerificationFlowBuilder {
        type Out = SourceCodeVerificationFlow;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "attempts_remaining" => Deserialize::begin(&mut self.attempts_remaining),
                "status" => Deserialize::begin(&mut self.status),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { attempts_remaining: Deserialize::default(), status: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                attempts_remaining: self.attempts_remaining?,
                status: self.status.take()?,
            })
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

    impl ObjectDeser for SourceCodeVerificationFlow {
        type Builder = SourceCodeVerificationFlowBuilder;
    }

    impl FromValueOpt for SourceCodeVerificationFlow {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceCodeVerificationFlowBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "attempts_remaining" => {
                        b.attempts_remaining = Some(FromValueOpt::from_value(v)?)
                    }
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
