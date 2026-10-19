/// Result from a phone check
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoPhoneReport {
    /// Details on the verification error. Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoPhoneReportError>,
    /// Phone to be verified.
    pub phone: Option<String>,
    /// Status of this `phone` check.
    pub status: stripe_shared::GelatoEmailReportStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoPhoneReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoPhoneReport").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoPhoneReportBuilder {
    error: Option<Option<stripe_misc::GelatoPhoneReportError>>,
    phone: Option<Option<String>>,
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

    impl Deserialize for GelatoPhoneReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoPhoneReport>,
        builder: GelatoPhoneReportBuilder,
    }

    impl Visitor for Place<GelatoPhoneReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoPhoneReportBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoPhoneReportBuilder {
        type Out = GelatoPhoneReport;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "error" => Deserialize::begin(&mut self.error),
                "phone" => Deserialize::begin(&mut self.phone),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { error: Some(None), phone: Some(None), status: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(error), Some(phone), Some(status)) =
                (self.error.take(), self.phone.take(), self.status.take())
            else {
                return None;
            };
            Some(Self::Out { error, phone, status })
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

    impl ObjectDeser for GelatoPhoneReport {
        type Builder = GelatoPhoneReportBuilder;
    }

    impl FromValueOpt for GelatoPhoneReport {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoPhoneReportBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "error" => b.error = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Status of this `phone` check.
pub type GelatoPhoneReportStatus = stripe_shared::GelatoEmailReportStatus;
