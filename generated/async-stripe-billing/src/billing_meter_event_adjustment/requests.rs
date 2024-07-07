use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateBillingMeterEventAdjustmentBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel: Option<CreateBillingMeterEventAdjustmentCancel<'a>>,
    event_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(rename = "type")]
    type_: stripe_billing::BillingMeterEventAdjustmentType,
}
impl<'a> CreateBillingMeterEventAdjustmentBuilder<'a> {
    fn new(event_name: &'a str, type_: stripe_billing::BillingMeterEventAdjustmentType) -> Self {
        Self { cancel: None, event_name, expand: None, type_ }
    }
}
/// Specifies which event to cancel.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingMeterEventAdjustmentCancel<'a> {
    /// Unique identifier for the event.
    /// You can only cancel events within 24 hours of Stripe receiving them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<&'a str>,
}
impl<'a> CreateBillingMeterEventAdjustmentCancel<'a> {
    pub fn new() -> Self {
        Self { identifier: None }
    }
}
impl<'a> Default for CreateBillingMeterEventAdjustmentCancel<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Creates a billing meter event adjustment
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingMeterEventAdjustment<'a> {
    inner: CreateBillingMeterEventAdjustmentBuilder<'a>,
}
impl<'a> CreateBillingMeterEventAdjustment<'a> {
    /// Construct a new `CreateBillingMeterEventAdjustment`.
    pub fn new(
        event_name: &'a str,
        type_: stripe_billing::BillingMeterEventAdjustmentType,
    ) -> Self {
        Self { inner: CreateBillingMeterEventAdjustmentBuilder::new(event_name, type_) }
    }
    /// Specifies which event to cancel.
    pub fn cancel(mut self, cancel: CreateBillingMeterEventAdjustmentCancel<'a>) -> Self {
        self.inner.cancel = Some(cancel);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CreateBillingMeterEventAdjustment<'_> {
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

impl StripeRequest for CreateBillingMeterEventAdjustment<'_> {
    type Output = stripe_billing::BillingMeterEventAdjustment;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/billing/meter_event_adjustments")
            .form(&self.inner)
    }
}
