/// Result from a email check
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoEmailReport {
    /// Email to be verified.
    pub email: Option<String>,
    /// Details on the verification error. Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoEmailReportError>,
    /// Status of this `email` check.
    pub status: stripe_shared::GelatoEmailReportStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoEmailReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoEmailReport").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoEmailReportBuilder {
    email: Option<Option<String>>,
    error: Option<Option<stripe_misc::GelatoEmailReportError>>,
    status: Option<stripe_shared::GelatoEmailReportStatus>,
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

    impl Deserialize for GelatoEmailReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoEmailReport>,
        builder: GelatoEmailReportBuilder,
    }

    impl Visitor for Place<GelatoEmailReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoEmailReportBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoEmailReportBuilder {
        type Out = GelatoEmailReport;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "email" => Deserialize::begin(&mut self.email),
                "error" => Deserialize::begin(&mut self.error),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { email: Some(None), error: Some(None), status: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(email), Some(error), Some(status)) =
                (self.email.take(), self.error.take(), self.status.take())
            else {
                return None;
            };
            Some(Self::Out { email, error, status })
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

    impl ObjectDeser for GelatoEmailReport {
        type Builder = GelatoEmailReportBuilder;
    }

    impl FromValueOpt for GelatoEmailReport {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoEmailReportBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "email" => b.email = FromValueOpt::from_value(v),
                    "error" => b.error = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Status of this `email` check.
pub type GelatoEmailReportStatus = stripe_shared::GelatoEmailReportStatus;
