use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateBillingMeterEventBuilder<'a> {
    event_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<&'a str>,
    payload: &'a std::collections::HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<stripe_types::Timestamp>,
}
impl<'a> CreateBillingMeterEventBuilder<'a> {
    fn new(event_name: &'a str, payload: &'a std::collections::HashMap<String, String>) -> Self {
        Self { event_name, expand: None, identifier: None, payload, timestamp: None }
    }
}
/// Creates a billing meter event
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingMeterEvent<'a> {
    inner: CreateBillingMeterEventBuilder<'a>,
}
impl<'a> CreateBillingMeterEvent<'a> {
    /// Construct a new `CreateBillingMeterEvent`.
    pub fn new(
        event_name: &'a str,
        payload: &'a std::collections::HashMap<String, String>,
    ) -> Self {
        Self { inner: CreateBillingMeterEventBuilder::new(event_name, payload) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A unique identifier for the event.
    /// If not provided, one will be generated.
    /// We recommend using a globally unique identifier for this.
    /// We'll enforce uniqueness within a rolling 24 hour period.
    pub fn identifier(mut self, identifier: &'a str) -> Self {
        self.inner.identifier = Some(identifier);
        self
    }
    /// The time of the event.
    /// Measured in seconds since the Unix epoch.
    /// Must be within the past 35 calendar days or up to 5 minutes in the future.
    /// Defaults to current timestamp if not specified.
    pub fn timestamp(mut self, timestamp: stripe_types::Timestamp) -> Self {
        self.inner.timestamp = Some(timestamp);
        self
    }
}
impl CreateBillingMeterEvent<'_> {
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

impl StripeRequest for CreateBillingMeterEvent<'_> {
    type Output = stripe_billing::BillingMeterEvent;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/billing/meter_events").form(&self.inner)
    }
}
