use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateBillingMeterEventAdjustmentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel: Option<CreateBillingMeterEventAdjustmentCancel>,
    event_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(rename = "type")]
    type_: stripe_billing::BillingMeterEventAdjustmentType,
}
impl CreateBillingMeterEventAdjustmentBuilder {
    fn new(
        event_name: impl Into<String>,
        type_: impl Into<stripe_billing::BillingMeterEventAdjustmentType>,
    ) -> Self {
        Self { cancel: None, event_name: event_name.into(), expand: None, type_: type_.into() }
    }
}
/// Specifies which event to cancel.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateBillingMeterEventAdjustmentCancel {
    /// Unique identifier for the event.
    /// You can only cancel events within 24 hours of Stripe receiving them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
impl CreateBillingMeterEventAdjustmentCancel {
    pub fn new() -> Self {
        Self { identifier: None }
    }
}
impl Default for CreateBillingMeterEventAdjustmentCancel {
    fn default() -> Self {
        Self::new()
    }
}
/// Creates a billing meter event adjustment
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateBillingMeterEventAdjustment {
    inner: CreateBillingMeterEventAdjustmentBuilder,
}
impl CreateBillingMeterEventAdjustment {
    /// Construct a new `CreateBillingMeterEventAdjustment`.
    pub fn new(
        event_name: impl Into<String>,
        type_: impl Into<stripe_billing::BillingMeterEventAdjustmentType>,
    ) -> Self {
        Self {
            inner: CreateBillingMeterEventAdjustmentBuilder::new(event_name.into(), type_.into()),
        }
    }
    /// Specifies which event to cancel.
    pub fn cancel(mut self, cancel: impl Into<CreateBillingMeterEventAdjustmentCancel>) -> Self {
        self.inner.cancel = Some(cancel.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CreateBillingMeterEventAdjustment {
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

impl StripeRequest for CreateBillingMeterEventAdjustment {
    type Output = stripe_billing::BillingMeterEventAdjustment;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/billing/meter_event_adjustments")
            .form(&self.inner)
    }
}
