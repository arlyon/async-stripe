/// Result from an id_number check
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoIdNumberReport {
    /// Date of birth.
    pub dob: Option<stripe_misc::GelatoDataIdNumberReportDate>,
    /// Details on the verification error. Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoIdNumberReportError>,
    /// First name.
    pub first_name: Option<String>,
    /// ID number. When `id_number_type` is `us_ssn`, only the last 4 digits are present.
    pub id_number: Option<String>,
    /// Type of ID number.
    pub id_number_type: Option<stripe_shared::IdNumberType>,
    /// Last name.
    pub last_name: Option<String>,
    /// Status of this `id_number` check.
    pub status: stripe_shared::GelatoEmailReportStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoIdNumberReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoIdNumberReport").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoIdNumberReportBuilder {
    dob: Option<Option<stripe_misc::GelatoDataIdNumberReportDate>>,
    error: Option<Option<stripe_misc::GelatoIdNumberReportError>>,
    first_name: Option<Option<String>>,
    id_number: Option<Option<String>>,
    id_number_type: Option<Option<stripe_shared::IdNumberType>>,
    last_name: Option<Option<String>>,
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

    impl Deserialize for GelatoIdNumberReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoIdNumberReport>,
        builder: GelatoIdNumberReportBuilder,
    }

    impl Visitor for Place<GelatoIdNumberReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoIdNumberReportBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoIdNumberReportBuilder {
        type Out = GelatoIdNumberReport;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dob" => Deserialize::begin(&mut self.dob),
                "error" => Deserialize::begin(&mut self.error),
                "first_name" => Deserialize::begin(&mut self.first_name),
                "id_number" => Deserialize::begin(&mut self.id_number),
                "id_number_type" => Deserialize::begin(&mut self.id_number_type),
                "last_name" => Deserialize::begin(&mut self.last_name),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                dob: Some(None),
                error: Some(None),
                first_name: Some(None),
                id_number: Some(None),
                id_number_type: Some(None),
                last_name: Some(None),
                status: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(dob),
                Some(error),
                Some(first_name),
                Some(id_number),
                Some(id_number_type),
                Some(last_name),
                Some(status),
            ) = (
                self.dob,
                self.error.take(),
                self.first_name.take(),
                self.id_number.take(),
                self.id_number_type.take(),
                self.last_name.take(),
                self.status.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { dob, error, first_name, id_number, id_number_type, last_name, status })
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

    impl ObjectDeser for GelatoIdNumberReport {
        type Builder = GelatoIdNumberReportBuilder;
    }

    impl FromValueOpt for GelatoIdNumberReport {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoIdNumberReportBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "dob" => b.dob = FromValueOpt::from_value(v),
                    "error" => b.error = FromValueOpt::from_value(v),
                    "first_name" => b.first_name = FromValueOpt::from_value(v),
                    "id_number" => b.id_number = FromValueOpt::from_value(v),
                    "id_number_type" => b.id_number_type = FromValueOpt::from_value(v),
                    "last_name" => b.last_name = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of ID number.
pub type GelatoIdNumberReportIdNumberType = stripe_shared::IdNumberType;
/// Status of this `id_number` check.
pub type GelatoIdNumberReportStatus = stripe_shared::GelatoEmailReportStatus;
