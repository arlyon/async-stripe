use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListIdBillingMeterEventSummaryBuilder<'a> {
    customer: &'a str,
    end_time: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    start_time: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_grouping_window: Option<ListIdBillingMeterEventSummaryValueGroupingWindow>,
}
impl<'a> ListIdBillingMeterEventSummaryBuilder<'a> {
    fn new(
        customer: &'a str,
        end_time: stripe_types::Timestamp,
        start_time: stripe_types::Timestamp,
    ) -> Self {
        Self {
            customer,
            end_time,
            ending_before: None,
            expand: None,
            limit: None,
            start_time,
            starting_after: None,
            value_grouping_window: None,
        }
    }
}
/// Specifies what granularity to use when generating event summaries.
/// If not specified, a single event summary would be returned for the specified time range.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListIdBillingMeterEventSummaryValueGroupingWindow {
    Hour,
}
impl ListIdBillingMeterEventSummaryValueGroupingWindow {
    pub fn as_str(self) -> &'static str {
        use ListIdBillingMeterEventSummaryValueGroupingWindow::*;
        match self {
            Hour => "hour",
        }
    }
}

impl std::str::FromStr for ListIdBillingMeterEventSummaryValueGroupingWindow {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListIdBillingMeterEventSummaryValueGroupingWindow::*;
        match s {
            "hour" => Ok(Hour),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ListIdBillingMeterEventSummaryValueGroupingWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListIdBillingMeterEventSummaryValueGroupingWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListIdBillingMeterEventSummaryValueGroupingWindow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListIdBillingMeterEventSummaryValueGroupingWindow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ListIdBillingMeterEventSummaryValueGroupingWindow",
            )
        })
    }
}
/// Retrieve a list of billing meter event summaries.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListIdBillingMeterEventSummary<'a> {
    inner: ListIdBillingMeterEventSummaryBuilder<'a>,
    id: &'a stripe_billing::BillingMeterId,
}
impl<'a> ListIdBillingMeterEventSummary<'a> {
    /// Construct a new `ListIdBillingMeterEventSummary`.
    pub fn new(
        id: &'a stripe_billing::BillingMeterId,
        customer: &'a str,
        end_time: stripe_types::Timestamp,
        start_time: stripe_types::Timestamp,
    ) -> Self {
        Self {
            id,
            inner: ListIdBillingMeterEventSummaryBuilder::new(customer, end_time, start_time),
        }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// Specifies what granularity to use when generating event summaries.
    /// If not specified, a single event summary would be returned for the specified time range.
    pub fn value_grouping_window(
        mut self,
        value_grouping_window: ListIdBillingMeterEventSummaryValueGroupingWindow,
    ) -> Self {
        self.inner.value_grouping_window = Some(value_grouping_window);
        self
    }
}
impl ListIdBillingMeterEventSummary<'_> {
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
        let id = self.id;

        stripe_client_core::ListPaginator::new_list(
            format!("/billing/meters/{id}/event_summaries"),
            self.inner,
        )
    }
}

impl StripeRequest for ListIdBillingMeterEventSummary<'_> {
    type Output = stripe_types::List<stripe_billing::BillingMeterEventSummary>;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/billing/meters/{id}/event_summaries"))
            .query(&self.inner)
    }
}
