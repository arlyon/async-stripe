#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceCodeVerificationFlow {
    /// The number of attempts remaining to authenticate the source object with a verification code.
    pub attempts_remaining: i64,
    /// The status of the code verification, either `pending` (awaiting verification, `attempts_remaining` should be greater than 0), `succeeded` (successful verification) or `failed` (failed verification, cannot be verified anymore as `attempts_remaining` should be 0).
    pub status: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceCodeVerificationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceCodeVerificationFlow").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceCodeVerificationFlowBuilder {
    attempts_remaining: Option<i64>,
    status: Option<String>,
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
                builder: SourceCodeVerificationFlowBuilder {
                    attempts_remaining: Deserialize::default(),
                    status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "attempts_remaining" => Deserialize::begin(&mut self.builder.attempts_remaining),
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(attempts_remaining), Some(status)) =
                (self.builder.attempts_remaining, self.builder.status.take())
            else {
                return Ok(());
            };
            *self.out = Some(SourceCodeVerificationFlow { attempts_remaining, status });
            Ok(())
        }
    }
};
