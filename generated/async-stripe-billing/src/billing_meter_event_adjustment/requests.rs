use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateBillingMeterEventAdjustmentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel: Option<CreateBillingMeterEventAdjustmentCancel>,
    event_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(rename = "type")]
    type_: stripe_billing::BillingMeterEventAdjustmentType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingMeterEventAdjustmentBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingMeterEventAdjustmentBuilder").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateBillingMeterEventAdjustmentCancel {
    /// Unique identifier for the event.
    /// You can only cancel events within 24 hours of Stripe receiving them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingMeterEventAdjustmentCancel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingMeterEventAdjustmentCancel").finish_non_exhaustive()
    }
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
/// Creates a billing meter event adjustment.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateBillingMeterEventAdjustment {
    inner: CreateBillingMeterEventAdjustmentBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingMeterEventAdjustment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingMeterEventAdjustment").finish_non_exhaustive()
    }
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
