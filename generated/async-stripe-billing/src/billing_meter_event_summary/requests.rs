use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListIdBillingMeterEventSummaryBuilder {
    customer: String,
    end_time: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    start_time: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_grouping_window: Option<stripe_shared::BillingMeterEventTimeWindow>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIdBillingMeterEventSummaryBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIdBillingMeterEventSummaryBuilder").finish_non_exhaustive()
    }
}
impl ListIdBillingMeterEventSummaryBuilder {
    fn new(
        customer: impl Into<String>,
        end_time: impl Into<stripe_types::Timestamp>,
        start_time: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        Self {
            customer: customer.into(),
            end_time: end_time.into(),
            ending_before: None,
            expand: None,
            limit: None,
            start_time: start_time.into(),
            starting_after: None,
            value_grouping_window: None,
        }
    }
}
/// Retrieve a list of billing meter event summaries.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListIdBillingMeterEventSummary {
    inner: ListIdBillingMeterEventSummaryBuilder,
    id: stripe_billing::BillingMeterId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIdBillingMeterEventSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIdBillingMeterEventSummary").finish_non_exhaustive()
    }
}
impl ListIdBillingMeterEventSummary {
    /// Construct a new `ListIdBillingMeterEventSummary`.
    pub fn new(
        id: impl Into<stripe_billing::BillingMeterId>,
        customer: impl Into<String>,
        end_time: impl Into<stripe_types::Timestamp>,
        start_time: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        Self {
            id: id.into(),
            inner: ListIdBillingMeterEventSummaryBuilder::new(
                customer.into(),
                end_time.into(),
                start_time.into(),
            ),
        }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Specifies what granularity to use when generating event summaries.
    /// If not specified, a single event summary would be returned for the specified time range.
    /// For hourly granularity, start and end times must align with hour boundaries (e.g., 00:00, 01:00, ..., 23:00).
    /// For daily granularity, start and end times must align with UTC day boundaries (00:00 UTC).
    pub fn value_grouping_window(
        mut self,
        value_grouping_window: impl Into<stripe_shared::BillingMeterEventTimeWindow>,
    ) -> Self {
        self.inner.value_grouping_window = Some(value_grouping_window.into());
        self
    }
}
impl ListIdBillingMeterEventSummary {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_billing::BillingMeterEventSummary>,
    > {
        let id = &self.id;

        stripe_client_core::ListPaginator::new_list(
            format!("/billing/meters/{id}/event_summaries"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListIdBillingMeterEventSummary {
    type Output = stripe_types::List<stripe_billing::BillingMeterEventSummary>;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/billing/meters/{id}/event_summaries"))
            .query(&self.inner)
    }
}

/// Specifies what granularity to use when generating event summaries.
/// If not specified, a single event summary would be returned for the specified time range.
/// For hourly granularity, start and end times must align with hour boundaries (e.g., 00:00, 01:00, ..., 23:00).
/// For daily granularity, start and end times must align with UTC day boundaries (00:00 UTC).
pub type ListIdBillingMeterEventSummaryValueGroupingWindow =
    stripe_shared::BillingMeterEventTimeWindow;
