/// If you have [scheduled a Sigma query](https://docs.stripe.com/sigma/scheduled-queries), you'll
/// receive a `sigma.scheduled_query_run.created` webhook each time the query
/// runs. The webhook contains a `ScheduledQueryRun` object, which you can use to
/// retrieve the query results.
///
/// For more details see <<https://stripe.com/docs/api/sigma/scheduled_queries/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ScheduledQueryRun {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// When the query was run, Sigma contained a snapshot of your Stripe data at this time.
    pub data_load_time: stripe_types::Timestamp,
    pub error: Option<stripe_misc::SigmaScheduledQueryRunError>,
    /// The file object representing the results of the query.
    pub file: Option<stripe_shared::File>,
    /// Unique identifier for the object.
    pub id: stripe_misc::ScheduledQueryRunId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Time at which the result expires and is no longer available for download.
    pub result_available_until: stripe_types::Timestamp,
    /// SQL for the query.
    pub sql: String,
    /// The query's execution status, which will be `completed` for successful runs, and `canceled`, `failed`, or `timed_out` otherwise.
    pub status: String,
    /// Title of the query.
    pub title: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ScheduledQueryRun {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ScheduledQueryRun").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ScheduledQueryRunBuilder {
    created: Option<stripe_types::Timestamp>,
    data_load_time: Option<stripe_types::Timestamp>,
    error: Option<Option<stripe_misc::SigmaScheduledQueryRunError>>,
    file: Option<Option<stripe_shared::File>>,
    id: Option<stripe_misc::ScheduledQueryRunId>,
    livemode: Option<bool>,
    result_available_until: Option<stripe_types::Timestamp>,
    sql: Option<String>,
    status: Option<String>,
    title: Option<String>,
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

    impl Deserialize for ScheduledQueryRun {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ScheduledQueryRun>,
        builder: ScheduledQueryRunBuilder,
    }

    impl Visitor for Place<ScheduledQueryRun> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ScheduledQueryRunBuilder {
                    created: Deserialize::default(),
                    data_load_time: Deserialize::default(),
                    error: Deserialize::default(),
                    file: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    result_available_until: Deserialize::default(),
                    sql: Deserialize::default(),
                    status: Deserialize::default(),
                    title: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "data_load_time" => Deserialize::begin(&mut self.builder.data_load_time),
                "error" => Deserialize::begin(&mut self.builder.error),
                "file" => Deserialize::begin(&mut self.builder.file),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "result_available_until" => {
                    Deserialize::begin(&mut self.builder.result_available_until)
                }
                "sql" => Deserialize::begin(&mut self.builder.sql),
                "status" => Deserialize::begin(&mut self.builder.status),
                "title" => Deserialize::begin(&mut self.builder.title),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(created),
                Some(data_load_time),
                Some(error),
                Some(file),
                Some(id),
                Some(livemode),
                Some(result_available_until),
                Some(sql),
                Some(status),
                Some(title),
            ) = (
                self.builder.created,
                self.builder.data_load_time,
                self.builder.error.take(),
                self.builder.file.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.result_available_until,
                self.builder.sql.take(),
                self.builder.status.take(),
                self.builder.title.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(ScheduledQueryRun {
                created,
                data_load_time,
                error,
                file,
                id,
                livemode,
                result_available_until,
                sql,
                status,
                title,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ScheduledQueryRun {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ScheduledQueryRun", 11)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("data_load_time", &self.data_load_time)?;
        s.serialize_field("error", &self.error)?;
        s.serialize_field("file", &self.file)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("result_available_until", &self.result_available_until)?;
        s.serialize_field("sql", &self.sql)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("title", &self.title)?;

        s.serialize_field("object", "scheduled_query_run")?;
        s.end()
    }
}
impl stripe_types::Object for ScheduledQueryRun {
    type Id = stripe_misc::ScheduledQueryRunId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ScheduledQueryRunId);
