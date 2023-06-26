impl stripe_misc::sigma::scheduled_query_run::ScheduledQueryRun {
    /// Returns a list of scheduled query runs.
    pub fn list(
        client: &stripe::Client,
        params: ListScheduledQueryRun,
    ) -> stripe::Response<
        stripe_types::List<stripe_misc::sigma::scheduled_query_run::ScheduledQueryRun>,
    > {
        client.get_query("/sigma/scheduled_query_runs", params)
    }
    /// Retrieves the details of an scheduled query run.
    pub fn retrieve(
        client: &stripe::Client,
        scheduled_query_run: &stripe_misc::sigma::scheduled_query_run::ScheduledQueryRunId,
        params: RetrieveScheduledQueryRun,
    ) -> stripe::Response<stripe_misc::sigma::scheduled_query_run::ScheduledQueryRun> {
        client.get_query(
            &format!(
                "/sigma/scheduled_query_runs/{scheduled_query_run}",
                scheduled_query_run = scheduled_query_run
            ),
            params,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListScheduledQueryRun<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListScheduledQueryRun<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveScheduledQueryRun<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveScheduledQueryRun<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
