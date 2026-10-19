/// Result from a selfie check
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoSelfieReport {
    /// ID of the [File](https://docs.stripe.com/api/files) holding the image of the identity document used in this check.
    pub document: Option<String>,
    /// Details on the verification error. Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoSelfieReportError>,
    /// ID of the [File](https://docs.stripe.com/api/files) holding the image of the selfie used in this check.
    pub selfie: Option<String>,
    /// Status of this `selfie` check.
    pub status: stripe_shared::GelatoEmailReportStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoSelfieReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoSelfieReport").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoSelfieReportBuilder {
    document: Option<Option<String>>,
    error: Option<Option<stripe_misc::GelatoSelfieReportError>>,
    selfie: Option<Option<String>>,
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

    impl Deserialize for GelatoSelfieReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoSelfieReport>,
        builder: GelatoSelfieReportBuilder,
    }

    impl Visitor for Place<GelatoSelfieReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoSelfieReportBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoSelfieReportBuilder {
        type Out = GelatoSelfieReport;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "document" => Deserialize::begin(&mut self.document),
                "error" => Deserialize::begin(&mut self.error),
                "selfie" => Deserialize::begin(&mut self.selfie),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { document: Some(None), error: Some(None), selfie: Some(None), status: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(document), Some(error), Some(selfie), Some(status)) =
                (self.document.take(), self.error.take(), self.selfie.take(), self.status.take())
            else {
                return None;
            };
            Some(Self::Out { document, error, selfie, status })
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

    impl ObjectDeser for GelatoSelfieReport {
        type Builder = GelatoSelfieReportBuilder;
    }

    impl FromValueOpt for GelatoSelfieReport {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoSelfieReportBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "document" => b.document = FromValueOpt::from_value(v),
                    "error" => b.error = FromValueOpt::from_value(v),
                    "selfie" => b.selfie = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Status of this `selfie` check.
pub type GelatoSelfieReportStatus = stripe_shared::GelatoEmailReportStatus;
