/// The Report Run object represents an instance of a report type generated with
/// specific run parameters. Once the object is created, Stripe begins processing the report.
/// When the report has finished running, it will give you a reference to a file
/// where you can retrieve your results. For an overview, see
/// [API Access to Reports](https://docs.stripe.com/reporting/statements/api).
///
/// Note that certain report types can only be run based on your live-mode data (not test-mode
/// data), and will error when queried without a [live-mode API key](https://docs.stripe.com/keys#test-live-modes).
///
/// For more details see <<https://stripe.com/docs/api/reporting/report_run/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ReportingReportRun {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// If something should go wrong during the run, a message about the failure (populated when
    ///  `status=failed`).
    pub error: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::ReportingReportRunId,
    /// `true` if the report is run on live mode data and `false` if it is run on test mode data.
    pub livemode: bool,
    pub parameters: stripe_misc::FinancialReportingFinanceReportRunRunParameters,
    /// The ID of the [report type](https://docs.stripe.com/reports/report-types) to run, such as `"balance.summary.1"`.
    pub report_type: String,
    /// The file object representing the result of the report run (populated when
    ///  `status=succeeded`).
    pub result: Option<stripe_shared::File>,
    /// Status of this report run. This will be `pending` when the run is initially created.
    ///  When the run finishes, this will be set to `succeeded` and the `result` field will be populated.
    /// Rarely, we may encounter an error, at which point this will be set to `failed` and the `error` field will be populated.
    pub status: String,
    /// Timestamp at which this run successfully finished (populated when
    ///  `status=succeeded`). Measured in seconds since the Unix epoch.
    pub succeeded_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReportingReportRun {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReportingReportRun").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ReportingReportRunBuilder {
    created: Option<stripe_types::Timestamp>,
    error: Option<Option<String>>,
    id: Option<stripe_misc::ReportingReportRunId>,
    livemode: Option<bool>,
    parameters: Option<stripe_misc::FinancialReportingFinanceReportRunRunParameters>,
    report_type: Option<String>,
    result: Option<Option<stripe_shared::File>>,
    status: Option<String>,
    succeeded_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for ReportingReportRun {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReportingReportRun>,
        builder: ReportingReportRunBuilder,
    }

    impl Visitor for Place<ReportingReportRun> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ReportingReportRunBuilder {
                    created: Deserialize::default(),
                    error: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    parameters: Deserialize::default(),
                    report_type: Deserialize::default(),
                    result: Deserialize::default(),
                    status: Deserialize::default(),
                    succeeded_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "error" => Deserialize::begin(&mut self.builder.error),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "parameters" => Deserialize::begin(&mut self.builder.parameters),
                "report_type" => Deserialize::begin(&mut self.builder.report_type),
                "result" => Deserialize::begin(&mut self.builder.result),
                "status" => Deserialize::begin(&mut self.builder.status),
                "succeeded_at" => Deserialize::begin(&mut self.builder.succeeded_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(created),
                Some(error),
                Some(id),
                Some(livemode),
                Some(parameters),
                Some(report_type),
                Some(result),
                Some(status),
                Some(succeeded_at),
            ) = (
                self.builder.created,
                self.builder.error.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.parameters.take(),
                self.builder.report_type.take(),
                self.builder.result.take(),
                self.builder.status.take(),
                self.builder.succeeded_at,
            )
            else {
                return Ok(());
            };
            *self.out = Some(ReportingReportRun {
                created,
                error,
                id,
                livemode,
                parameters,
                report_type,
                result,
                status,
                succeeded_at,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ReportingReportRun {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ReportingReportRun", 10)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("error", &self.error)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("parameters", &self.parameters)?;
        s.serialize_field("report_type", &self.report_type)?;
        s.serialize_field("result", &self.result)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("succeeded_at", &self.succeeded_at)?;

        s.serialize_field("object", "reporting.report_run")?;
        s.end()
    }
}
impl stripe_types::Object for ReportingReportRun {
    type Id = stripe_misc::ReportingReportRunId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ReportingReportRunId);
