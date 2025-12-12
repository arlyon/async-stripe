/// If you have [scheduled a Sigma query](https://docs.stripe.com/sigma/scheduled-queries), you'll
/// receive a `sigma.scheduled_query_run.created` webhook each time the query
/// runs. The webhook contains a `ScheduledQueryRun` object, which you can use to
/// retrieve the query results.
///
/// For more details see <<https://stripe.com/docs/api/sigma/scheduled_queries/object>>.
#[derive(Clone, Debug)]
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
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
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
                builder: ScheduledQueryRunBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ScheduledQueryRunBuilder {
        type Out = ScheduledQueryRun;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "data_load_time" => Deserialize::begin(&mut self.data_load_time),
                "error" => Deserialize::begin(&mut self.error),
                "file" => Deserialize::begin(&mut self.file),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "result_available_until" => Deserialize::begin(&mut self.result_available_until),
                "sql" => Deserialize::begin(&mut self.sql),
                "status" => Deserialize::begin(&mut self.status),
                "title" => Deserialize::begin(&mut self.title),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.created,
                self.data_load_time,
                self.error.take(),
                self.file.take(),
                self.id.take(),
                self.livemode,
                self.result_available_until,
                self.sql.take(),
                self.status.take(),
                self.title.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
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
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for ScheduledQueryRun {
        type Builder = ScheduledQueryRunBuilder;
    }

    impl FromValueOpt for ScheduledQueryRun {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ScheduledQueryRunBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "data_load_time" => b.data_load_time = FromValueOpt::from_value(v),
                    "error" => b.error = FromValueOpt::from_value(v),
                    "file" => b.file = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "result_available_until" => {
                        b.result_available_until = FromValueOpt::from_value(v)
                    }
                    "sql" => b.sql = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "title" => b.title = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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
