use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveMandateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveMandateBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a Mandate object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveMandate {
    inner: RetrieveMandateBuilder,
    mandate: stripe_shared::MandateId,
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
