use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListEventBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<&'a [&'a str]>,
}
impl<'a> ListEventBuilder<'a> {
    fn new() -> Self {
        Self {
            created: None,
            delivery_success: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_: None,
            types: None,
        }
    }
}
/// List events, going back up to 30 days.
/// Each event data is rendered according to Stripe API version at its creation time, specified in [event object](https://docs.stripe.com/api/events/object) `api_version` attribute (not according to your current Stripe API version or `Stripe-Version` header).
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListEvent<'a> {
    inner: ListEventBuilder<'a>,
}
impl<'a> ListEvent<'a> {
    /// Construct a new `ListEvent`.
    pub fn new() -> Self {
        Self { inner: ListEventBuilder::new() }
    }
    /// Only return events that were created during the given date interval.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// Filter events by whether all webhooks were successfully delivered.
    /// If false, events which are still pending or have failed all delivery attempts to a webhook endpoint will be returned.
    pub fn delivery_success(mut self, delivery_success: bool) -> Self {
        self.inner.delivery_success = Some(delivery_success);
        self
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
    /// A string containing a specific event name, or group of events using * as a wildcard.
    /// The list will be filtered to include only events with a matching event property.
    pub fn type_(mut self, type_: &'a str) -> Self {
        self.inner.type_ = Some(type_);
        self
    }
    /// An array of up to 20 strings containing specific event names.
    /// The list will be filtered to include only events with a matching event property.
    /// You may pass either `type` or `types`, but not both.
    pub fn types(mut self, types: &'a [&'a str]) -> Self {
        self.inner.types = Some(types);
        self
    }
}
impl<'a> Default for ListEvent<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListEvent<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Event>> {
        stripe_client_core::ListPaginator::new_list("/events", self.inner)
    }
}

impl StripeRequest for ListEvent<'_> {
    type Output = stripe_types::List<stripe_shared::Event>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/events").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveEventBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveEventBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an event.
/// Supply the unique identifier of the event, which you might have received in a webhook.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveEvent<'a> {
    inner: RetrieveEventBuilder<'a>,
    id: &'a stripe_shared::EventId,
}
impl<'a> RetrieveEvent<'a> {
    /// Construct a new `RetrieveEvent`.
    pub fn new(id: &'a stripe_shared::EventId) -> Self {
        Self { id, inner: RetrieveEventBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveEvent<'_> {
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
}

impl StripeRequest for RetrieveEvent<'_> {
    type Output = stripe_shared::Event;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/events/{id}")).query(&self.inner)
    }
}
