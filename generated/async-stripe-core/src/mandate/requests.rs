use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveMandateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveMandateBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveMandateBuilder").finish_non_exhaustive()
    }
}
impl RetrieveMandateBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a Mandate object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveMandate {
    inner: RetrieveMandateBuilder,
    mandate: stripe_shared::MandateId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveMandate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveMandate").finish_non_exhaustive()
    }
}
impl RetrieveMandate {
    /// Construct a new `RetrieveMandate`.
    pub fn new(mandate: impl Into<stripe_shared::MandateId>) -> Self {
        Self { mandate: mandate.into(), inner: RetrieveMandateBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveMandate {
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

impl StripeRequest for RetrieveMandate {
    type Output = stripe_shared::Mandate;

    fn build(&self) -> RequestBuilder {
        let mandate = &self.mandate;
        RequestBuilder::new(StripeMethod::Get, format!("/mandates/{mandate}")).query(&self.inner)
    }
}
