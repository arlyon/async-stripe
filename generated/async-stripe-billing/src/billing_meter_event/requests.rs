use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct CreateBillingMeterEventBuilder {
    event_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<String>,
    payload: std::collections::HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<stripe_types::Timestamp>,
}
impl CreateBillingMeterEventBuilder {
    fn new(
        event_name: impl Into<String>,
        payload: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        Self {
            event_name: event_name.into(),
            expand: None,
            identifier: None,
            payload: payload.into(),
            timestamp: None,
        }
    }
}
/// Creates a billing meter event.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingMeterEvent {
    inner: CreateBillingMeterEventBuilder,
}
impl CreateBillingMeterEvent {
    /// Construct a new `CreateBillingMeterEvent`.
    pub fn new(
        event_name: impl Into<String>,
        payload: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        Self { inner: CreateBillingMeterEventBuilder::new(event_name.into(), payload.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A unique identifier for the event.
    /// If not provided, one is generated.
    /// We recommend using UUID-like identifiers.
    /// We will enforce uniqueness within a rolling period of at least 24 hours.
    /// The enforcement of uniqueness primarily addresses issues arising from accidental retries or other problems occurring within extremely brief time intervals.
    /// This approach helps prevent duplicate entries and ensures data integrity in high-frequency operations.
    pub fn identifier(mut self, identifier: impl Into<String>) -> Self {
        self.inner.identifier = Some(identifier.into());
        self
    }
    /// The time of the event.
    /// Measured in seconds since the Unix epoch.
    /// Must be within the past 35 calendar days or up to 5 minutes in the future.
    /// Defaults to current timestamp if not specified.
    pub fn timestamp(mut self, timestamp: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.timestamp = Some(timestamp.into());
        self
    }
}
impl CreateBillingMeterEvent {
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

impl StripeRequest for CreateBillingMeterEvent {
    type Output = stripe_billing::BillingMeterEvent;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/billing/meter_events").form(&self.inner)
    }
}
